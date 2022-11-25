use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct GHRelease {
    pub url: String,
    pub tag_name: String,
    pub assets: Vec<Asset>
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String
}