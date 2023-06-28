use std::fs::OpenOptions;
use std::io::Write;

pub fn get_key(path: &str) -> Option<String> {
    let api = 
    match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => { println!("Cannot read/access API User key in {}: {:?}.",path, e);
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



struct APIBuilder {
    query: Vec<Query>,
    categories: Vec<Category>,
    purity: Vec<Purity>,
    sorting: Sorting,
    order: Order,
    toprange: TopRange, //sorting must be TopList to apply this
    atleast: Option<Resolution>,
    resolutions: Vec<Resolution>,
    ratios: Vec<Resolution>, //maybe create type Ratio
    colors: Vec<HexColor>,
    page: usize, //should start at 1
    seed: AlphaNumericalSeed,
}

enum Query {
    Tags(Vec<Tag>),
    Username(String),
    Id(i32), //Cant be combined
    Type(), //type:{png/jpg}
    Like(String), 
}

enum Tag {
    Include(String),
    Exclude(String),
}

use std::convert::AsRef;
use strum::AsRefStr;

#[derive(Debug, AsRefStr, Default, PartialEq)]
//#[derive(Debug, Default)]
enum Sorting {
    #[default]
    #[strum(serialize = "date_added")]
    DateAdded,
    Relevance,
    Random,
    Views,
    Favorites,
    Toplist
}

#[derive(Default)]
enum Order {
    #[default]
    Desc,
    Asc,
}

#[derive(Default)]
enum TopRange {
    OneDay,
    ThreeDays,
    OneWeek,
    #[default]
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
}

#[derive(Default)]
enum Category {
    #[default]
    General,
    Anime,
    People,
}

#[derive(Debug, Default, PartialEq)]
enum Purity{
    #[default]
    Sfw,
    Sketchy,
    Nsfw,
}

struct Resolution(usize, usize);

struct HexColor(String);

struct AlphaNumericalSeed(String);




#[cfg(test)]
mod tests {
    use crate::api::{Purity, Sorting};

    #[test]
    fn it_works() {
        let p = Purity::default();
        println!("{:?}", p);
        assert_eq!(p, Purity::Sfw);
    }
    #[test]
    fn sorting_as_str() {
        let s = Sorting::default();
        println!("{:?}", s.as_ref());
        assert_eq!(s.as_ref(), "date_added");
    }
}
