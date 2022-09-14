use std::{path::PathBuf, error::Error, str::FromStr, fs};



pub fn create_project_cache(project_path: &String) -> Result<(), Box<dyn Error>>{
    let raw_path = format!("{}/.depploy", project_path);
    let path = PathBuf::from_str(&raw_path)?;
    if path.exists() && path.is_dir() {
        return Ok(())
    }
    if path.exists() && path.is_file() {
        fs::remove_file(&path)?;
    }
    fs::create_dir(&path)?;
    return Ok(())
}