use crate::{io::{ProjectConf, match_config, load_project_file}, common::error::PTGenError};
use log::{debug, info, error};
use std::{process::Command, path::PathBuf, str::FromStr};


pub fn create_tag(image_conf: &ProjectConf, mut docker_registry: String) -> Vec<String> {
    if docker_registry.ne(&String::from("")) {
        docker_registry = format!("{docker_registry}/")
    }

    let name = format!(
        "{}{}",
        docker_registry.trim().to_lowercase(),
        image_conf.name.trim().to_lowercase(),
    );

    let tag = format!("{}:{}", name, image_conf.version.trim());
    debug!("Docker image-tag: {}", tag);
    vec![name, tag]
}

pub fn set_latest_tag(image_name: &String) -> String {
    let latest_tag: String = format!("{}:{}", image_name, "latest");
    latest_tag
}

pub async fn build_image(image_tag: &String, dir: &str, dockerfile_name: &String, no_latest: &bool, latest_img_tag: &String, platform: &String) -> Result<(), PTGenError> {
    debug!("Building image: {}", image_tag);
    let p = PathBuf::from_str(dir).unwrap();
    let filename = match_config(&p);

    let mut project_path = dir.to_string();

    let config_data = match load_project_file(&p, &filename) {
        Ok(data) => data,
        Err(err) => panic!("Error: {err}"),
    };
    if config_data.contains(", path ="){
        let mut x = PathBuf::from_str(dir).unwrap();
        x.pop();
        project_path = x.to_str().unwrap().to_string();
    }
    println!("{project_path:?}");
    let dockerfile_name_path = format!("{project_path}/{dockerfile_name}");
    let mut args = vec!["build", "-f", dockerfile_name_path.as_str(), "-t", image_tag, "-t", latest_img_tag, project_path.as_str()];
    if platform != ""{
        args.insert(1, "--platform");
        args.insert(2, &platform);
    }
    if no_latest == &true {
        args = vec!["build", "-f", dockerfile_name_path.as_str(), "-t", image_tag, project_path.as_str()];
    } 

    debug!("{:?}", args);
    let output = Command::new("docker")
        .args(args)
        .output()
        .expect("Could not build Image");
    

    let error_str = String::from_utf8(output.stderr).expect("Could not decode process output");
    if error_str.contains("exporting") && error_str.contains("writing"){
        debug!("Building Output: {:?}", error_str);
        return Ok(());
    }
    error!("Failed to build container, output: {:?}", error_str);

    Err(PTGenError::FailedBuilding)
}

pub fn push_image(image_tag: &String) {
    info!("Pushing image: {}", image_tag);

    let output = Command::new("docker")
        .args(["push", image_tag])
        .output()
        .expect("Could not push Image");
    let output_str = String::from_utf8(output.stderr).expect("Could not decode process output");
    if output_str.contains("not exist locally"){
        error!("Failed to Push container: {}", image_tag);
    }else{
        debug!("Pushing Output: {:?}", output_str);
    }
}