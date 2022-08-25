use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use futures::stream::StreamExt;

mod api;
mod args;
mod wallpaper;
use crate::args::Args;
use crate::wallpaper::ApiResponse;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    /* Path for Storing the API key */
    let api_path = shellexpand::tilde("~/.wallhaven").to_string();

    let args = Args::parse();
    //println!("{:#?}", args);

    /* Set API if possible and exit */
    if args.set_user_key != "" {
        match api::save_key(&api_path, &args.set_user_key) {
            Ok(()) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
        return Ok(());
    }

    /* Copy save path for wallpapers */
    let save_path = args.path.clone();

    /* Create request url with given preferences */
    let request_url = args::create_url(args, &api_path);
    eprintln!("Fetching {:#?}", request_url);

    /* Request wallpaper data */
    let client = reqwest::Client::new();
    let response = client.get(request_url)
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .header(reqwest::header::ACCEPT, "application/json")
                    .send()
                    .await
                    .unwrap();
    //eprintln!("Response: {:#?}", response);
    
    /* Check for correct response */
    let api_response =
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("Successful GET request!");
            match response.json::<ApiResponse>().await {
                Ok(resp) => resp,
                Err(err) => panic!("Cannot parse json: {:?}", err),
            }
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Verify validity of API key or use --ignore-api");
            return Ok(());
        }
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    };

    /* Did we found any wallpaper with given query? */
    let wallpapers= api_response.data;
    if wallpapers.is_empty() {
        println!("No wallpapers found for given query and preferences!");
    }

    /* Download and store wallpapers */
    for wallpaper in wallpapers {
        let path = wallpaper.get_savepath(&save_path);
        download_file(&client, &wallpaper.get_url(), &path).await
            .unwrap();
    }


    Ok(())
}


pub async fn download_file(client: &reqwest::Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    // Download chunks
    //let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut file = OpenOptions::new().write(true).append(true)
        .create(true).open(path)
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = u64::min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));

    Ok(())
}
