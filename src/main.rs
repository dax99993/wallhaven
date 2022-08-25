use clap::Parser;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use futures::stream::StreamExt;

#[derive(Parser, Debug)]
#[clap(author = "Dax99993 <Dax99993@gmail.com>",
version,
about = "Download wallpapers from WallHaven.cc with custom settings",
long_about = "A CLI for quickly access WallHaven API for downloading wallpapers, according
to preferences, with optional use of api key for remembering the preferences and access NSFW wallpapers."
)]
struct Args {
    /// Path To Save Wallpapers
    #[clap(short = 'p', long, verbatim_doc_comment)]
    #[clap(required_unless_present = "set-api")]
    path: String,

    /// Query String
    ///
    ///    tagname - search fuzzily for a tag/keyword
    ///    -tagname - exclude a tag/keyword
    ///    +tag1 +tag2 - must have tag1 and tag2
    ///    +tag1 -tag2 - must have tag1 and NOT tag2
    ///    @username - user uploads
    ///    id:123 - Exact tag search (can not be combined)
    ///    type:{png/jpg} - Search for file type (jpg = jpeg)
    ///    like:wallpaper ID - Find wallpapers with similar tags
    ///
    ///    Ex. "anime +city -mountain type:png"
    #[clap(short = 'q', long, verbatim_doc_comment)]    
    #[clap(default_value = "")]
    #[clap(required_unless_present = "colors")]
    #[clap(required_unless_present = "set-api")]
    query: String,

    /// Categories
    ///
    ///    Turn Categories on(1) Or Off(0)
    ///    (general/anime/people).
    #[clap(short = 'c', long, verbatim_doc_comment)]    
    #[clap(value_parser = ["100", "101", "110", "111"] )]
    #[clap(default_value = "111")]
    categories: String,

    /// Purity
    ///
    ///    Turn Purities On(1) Or Off(0)
    ///    *NSFW Requires A Valid API Key*
    ///    (sfw/sketchy/nsfw).
    #[clap(short = 'x', long, verbatim_doc_comment)]    
    #[clap(value_parser = ["100", "101", "110", "111"] )]
    #[clap(default_value = "100")]
    purity: String,

    /// Sorting 
    ///
    #[clap(short = 's', long, verbatim_doc_comment)]    
    #[clap(value_parser = ["DATE_ADDED", "RELEVANCE", "RANDOM", "VIEWS", "FAVORITES", "TOPLIST"] )]
    #[clap(default_value = "DATE_ADDED")]
    #[clap(ignore_case = true)]
    sorting: String,

    /// Sorting order 
    ///
    #[clap(short = 'o', long, verbatim_doc_comment)]    
    #[clap(value_parser = ["ASC", "DESC"])]
    #[clap(default_value = "DESC")]
    #[clap(ignore_case = true)]
    order: String,

    /// Range Of Search
    ///
    ///    Sorting MUST Be Set To 'TOPLIST'
    #[clap(short = 't', long, verbatim_doc_comment)]    
    #[clap(value_parser = ["1D", "3D", "1W", "1M", "3M", "6M", "1Y"])]
    #[clap(default_value = "1M")]
    #[clap(ignore_case = true)]
    toprange: String,

    /// Atleast
    ///
    ///    Set The Minimum Resolution Allowed
    ///    Ex. 1920x1080.
    #[clap(short = 'a', long, verbatim_doc_comment)]    
    #[clap(default_value = "")]
    atleast: String,

    /// Resolutions
    ///
    ///    List Of Exact Wallpaper Resolutions
    ///    Single Resolution Allowed.
    #[clap(short = 'r', long, verbatim_doc_comment)]    
    #[clap(default_value = "1920x1080,1920x1200")]
    resolutions: String,

    /// Ratios
    ///
    ///    List Of Aspect Ratios
    ///    Single Ratio Allowed.
    #[clap(short = 'R', long, verbatim_doc_comment)]    
    #[clap(default_value = "16x9,16x10")]
    ratios: String,

    /// Color
    ///
    ///    Search By Hex Color
    ///    Ex.  --colors 0066cc
    #[clap(short = 'C', long, verbatim_doc_comment)]    
    #[clap(default_value = "")]
    #[clap(required_unless_present = "query")]
    #[clap(required_unless_present = "set-api")]
    colors: String,

    /// Page 
    ///
    ///    Select Page Of Results
    ///    (1..)
    #[clap(short = 'P', long, verbatim_doc_comment)]    
    #[clap(value_parser = clap::value_parser!(u32).range(1..))]
    #[clap(default_value_t = 1)] 
    page: u32,

    /// Set API Key
    ///
    ///    Set API Key For Future Quering With User Preferences
    ///    (categories, purity, resolutions, ratios and toprange).
    #[clap(short = 'S', long = "set-api", verbatim_doc_comment)]    
    #[clap(default_value = "")] 
    set_api: String,

    /// Overwrite API Wallpaper Preferences 
    ///
    ///    Query With API Access But Using Given Wallpaper Preferences.
    #[clap(short = 'n', long = "no-api-settings", verbatim_doc_comment)]    
    #[clap(takes_value = false)]
    no_api_settings: bool,


    /// Ignore API Key 
    ///
    ///    Query with non-user access
    #[clap(short = 'i', long = "ignore-api", verbatim_doc_comment)]    
    #[clap(takes_value = false)]
    ignore_api: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thumb {
    large: String,
    original: String,
    small: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde (rename = "")]
pub struct Wallpaper {
    id: String,
    url: String,
    short_url: String,
    views: i32,
    favorites: i32,
    source: String,
    purity: String,
    category: String,
    dimension_x: i32,
    dimension_y: i32,
    resolution: String,
    ratio: String,
    file_size: i32,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    path: String,
    thumbs: Thumb,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde (rename = "data")]
    pub data: Vec<Wallpaper>
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    /* Path for Storing the API key */
    let api_path = shellexpand::tilde("~/.wallhaven").to_string();

    let args = Args::parse();
    println!("{:#?}", args);

    /* Set API if possible and exit */
    if args.set_api != "" {
        match save_api_key(&api_path, &args.set_api) {
            Ok(()) => (),
            Err(e) => println!("Error: {}", e),
        }
        return Ok(());
    }

    /* Copy save path for wallpapers */
    let save_path = args.path.clone();

    /* Create request url with given preferences */
    let request_url = create_url(args, &api_path);
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
            println!("Verify validity of API key");
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
        let file_type: Vec<&str> = wallpaper.file_type.split("/").collect();
        let extension = file_type[1];
        let path = format!("{}/{}.{}", save_path, wallpaper.id, extension);
        download_file(&client, &wallpaper.path, &path).await
            .unwrap();
    }


    Ok(())
}

fn create_url(args: Args, api_path: &str) -> String {
    let base_url = String::from("https://wallhaven.cc/api/v1/");
    /* Try to get API KEY */
    let api = 
    if let Some(api) = get_api_key(api_path) {
        api
    }
    else {
        println!("Using default non-user access");
        String::from("")
    };

    /* Create search query and preferences */
    let mut search = String::from("search?");

    /* Add API key */
    if !args.no_api_settings && !args.ignore_api && api != "" {
        search.push_str(&format!("apikey={}&", &api));
    }

    if args.no_api_settings || args.ignore_api || api == "" {
        println!("Overwriting/Ignoring Default API Preferences!");
        search.push_str(&format!("categories={}&", args.categories));
        search.push_str(&format!("purity={}&", args.purity));
        search.push_str(&format!("resolutions={}&", args.resolutions.to_ascii_lowercase()));
        search.push_str(&format!("ratios={}&", args.ratios.to_ascii_lowercase()));
    }

    if args.sorting == "TOPLIST" && (args.no_api_settings || args.ignore_api) {
        search.push_str(&format!("topRange={}&", args.toprange.to_ascii_lowercase()));
    }

    /* Add non-api general settings */
    search.push_str(&format!("order={}&", args.order.to_ascii_lowercase()));
    search.push_str(&format!("page={}&", args.page.to_string()));
    search.push_str(&format!("sorting={}&", args.sorting.to_ascii_lowercase()));

    if args.atleast != "" {
        search.push_str(&format!("atleast={}&", args.atleast.to_ascii_lowercase()));
    }

    /* Add search method */
    if args.colors == "" {
        search.push_str(&format!("q={}", args.query));
    }
    else{
        search.push_str(&format!("colors={}", args.colors.to_ascii_lowercase()));
    }

    let mut url = base_url;
    url.push_str(&search);

    url
}

fn get_api_key(path: &str) -> Option<String> {
    let api = 
    match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => { println!("No stored API in {}: {:?}.",path, e);
            return None;
        },
    };

    if !api.chars().all(|c| c.is_ascii_alphanumeric()) {
        println!("Not valid API!");
        return None;
    }
        
    Some(api)
}

fn save_api_key(path: &str, api: &str) -> Result<(), String> {
    if !api.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!("Not valid API!."));
    }

    let mut file = OpenOptions::new().write(true).truncate(true)
        .create(true).open(path)
        .unwrap();

    match write!(file, "{}", api) {
        Ok(_) => println!("API key written to {}", path),
        Err(e) => return Err(format!("Cannot write API key: {:?}", e)),
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
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
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
