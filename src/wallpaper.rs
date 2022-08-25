use serde::{Serialize, Deserialize};

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

impl Wallpaper {
    pub fn get_savepath(&self, path: &str) -> String {
        let file_type: Vec<&str> = self.file_type.split("/").collect();
        let extension = file_type[1];

        format!("{}/{}.{}", path, self.id, extension)
    }

    pub fn get_url(&self) -> &str {
        //format!("{}", &self.path)
        &self.path
    }

}


