use std::fs::OpenOptions;
use std::io::Write;

pub fn get_key(path: &str) -> Option<String> {
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

pub fn save_key(path: &str, api: &str) -> Result<(), String> {
    if !api.chars().all(|c| c.is_ascii_alphanumeric()) && api.len() < 50 {
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
