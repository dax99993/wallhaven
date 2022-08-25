use clap::Parser;
use crate::api;

#[derive(Parser, Debug)]
#[clap(author = "Dax99993 <Dax99993@gmail.com>",
version,
about = "Download wallpapers from WallHaven.cc with custom settings",
long_about = "A CLI for quickly access WallHaven API for downloading wallpapers, according
to preferences, with optional use of api key for remembering the preferences and access NSFW wallpapers."
)]
pub struct Args {
    /// Path To Save Wallpapers
    #[clap(short = 'p', long, verbatim_doc_comment)]
    #[clap(required_unless_present = "set-api")]
    #[clap(default_value = ".")]
    pub path: String,

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
    #[clap(required_unless_present_any = ["colors", "set-api"])]
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
    #[clap(required_unless_present_any = ["query", "set-api"])]
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
    pub set_api: String,

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


pub fn create_url(args: Args, api_path: &str) -> String {
    let base_url = String::from("https://wallhaven.cc/api/v1/");
    /* Try to get API KEY */
    let api = 
    if let Some(api) = api::get_key(api_path) {
        api
    }
    else {
        String::from("")
    };

    /* Create search query and preferences */
    let mut search = String::from("search?");

    /* Add API key */
    if !args.ignore_api && api != "" {
        search.push_str(&format!("apikey={}&", &api));
    }


    /* Method of requesting wallpapers Message */
    if args.no_api_settings && !args.ignore_api {
        println!("Overwriting Default API Preferences!.");
    }
    if args.ignore_api {
        println!("Ignoring API Key, Using Default Non-User Access and Preferences!.");
    }
    if api == "" {
        println!("Invalid API key!, Using Default Non-User Access and Preferences!.");
    }
    
    /* Add fields when using invalid API key or
     * ignoring API key or
     * Requesting a one time overwrite of API key preferences */
    if args.no_api_settings || args.ignore_api || api == "" {
        search.push_str(&format!("categories={}&", args.categories));
        search.push_str(&format!("purity={}&", args.purity));
        search.push_str(&format!("resolutions={}&", args.resolutions.to_ascii_lowercase()));
        search.push_str(&format!("ratios={}&", args.ratios.to_ascii_lowercase()));
    }

    if args.sorting == "TOPLIST" && (args.no_api_settings || args.ignore_api || api == "") {
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

