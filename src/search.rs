use reqwest;

pub async fn get_category(registry_url: &String) -> Result<String, reqwest::Error> {
    let url = format!("{}{}",registry_url, "/v2/_catalog");
    let response = reqwest::get(url.as_str()).await?;
    let data = response.json().await?;
    Ok(data)
}

pub async fn search_for_tag(registry_url: &String, tag: &String) -> Result<String, reqwest::Error> {
    let url = format!("{}{}{}{}",registry_url, "/v2/", tag, "/list");
    let response = reqwest::get(url.as_str()).await?;
    let data = response.json().await?;
    Ok(data)
}