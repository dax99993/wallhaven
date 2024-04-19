mod api;
mod args;

use api::WallhavenClient;
use clap::Parser;
use crate::args::CLIArgs;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Get cli arguments
    let args = CLIArgs::parse();
    //println!("{:#?}", args);
    
    // Create Api client
    let wallhaven = WallhavenClient::new(args.commands)?;

    // Execute request
    let response = wallhaven.execute().await;

    // IDK if I should handle the error like this
    // since error can contains custom formated string not json
    match response {
        Ok(res) => {println!("{}", res)},
        Err(e) => {eprintln!("{}", e)},
    }

    Ok(())
}
