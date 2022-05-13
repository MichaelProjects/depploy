use crate::io::Config;
use std::process::{Command};
use log::{info, debug, trace, warn};

pub fn create_tag(image_conf: &Config, mut docker_registry: String) -> Vec<String> {
    if docker_registry.clone().ne(&String::from("")){
        docker_registry = format!("{}/", docker_registry)
    }

    let name = format!(
        "{}{}",
        docker_registry.trim().to_lowercase(),
        image_conf.name.trim().to_lowercase(),
    );

    let tag = format!(
        "{}:{}",
        name,
        image_conf.version.trim()
    );
    debug!("Docker image-tag: {}", tag);
    return vec![name, tag];
}

pub fn set_latest_tag(image_name: &String, image_tag: &String) -> String {

    let latest_tag: String = format!("{}:{}", image_name, "latest");
    
    // uses the docker deamon to set the latest tag
    let output = Command::new("docker")
        .arg("tag")
        .arg(image_tag)
        .arg(&latest_tag)
        .output()
        .expect("Could not set latest tag");
    if output.stderr.len() >0 {
        panic!("{}", String::from_utf8(output.stderr).expect("Could not decode process output"));

    }
    let output_str = String::from_utf8(output.stdout).expect("Could not decode process output");
    info!("Building Output: {:?}", output_str);
    return latest_tag;
}


pub fn build_image(image_tag: &String, dir: &str, dockerfile_name: &String) {
    debug!("Building image: {}", image_tag);
    let output = Command::new("docker")
        .args(["build", "-f", dockerfile_name, "-t", image_tag, dir])
        .output()
        .expect("Could not build Image");
    if output.stderr.len() != 0 {
        panic!("{}", String::from_utf8(output.stderr).expect("Could not decode process output"));
    }
    let output_str = String::from_utf8(output.stdout).expect("Could not decode process output");
    debug!("Building Output: {:?}", output_str);
}

pub fn push_image(image_tag: &String) {
    info!("Pushing image: {}", image_tag);
    let output = Command::new("docker")
        .args(["push", image_tag])
        .output()
        .expect("Could not push Image");
    if output.stderr.len() > 0 {
        panic!("{}", String::from_utf8(output.stderr).expect("Could not decode process output"));
    }
    let output_str = String::from_utf8(output.stdout).expect("Could not decode process output");
    debug!("Pushing Output: {:?}", output_str);
}
