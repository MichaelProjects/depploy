use crate::io::Config;
use std::process::{Command};

pub fn create_tag(image_conf: &Config, mut docker_registry: String) -> String {
    if docker_registry.clone().ne(&String::from("")){
        docker_registry = format!("{}/", docker_registry)
    }

    let tag = format!(
        "{}{}:{}",
        docker_registry.trim().to_lowercase(),
        image_conf.name.trim().to_lowercase(),
        image_conf.version.trim()
    );
    println!("Docker image-tag: {}", tag);
    return tag;
}

pub fn set_latest_tag(image_tag: &String) -> String{
    let latest_tag: String = format!("{}:{}", image_tag, "latest");
    let output = Command::new("docker")
        .arg("tag")
        .arg(image_tag)
        .arg(&latest_tag)
        .output()
        .expect("Could not set latest tag");
    if output.stderr.len() > 0 {
        println!("{}", String::from_utf8(output.stderr).expect("Could not decode process output"));
    }
    let output_str = String::from_utf8(output.stdout).expect("Could not decode process output");
    println!("Building Output: {:?}", output_str);
    return latest_tag;
}


pub fn build_image(image_tag: &String, dir: &str) {
    println!("Building image: {}", image_tag);
    let output = Command::new("docker")
        .args(["build", "-t", image_tag, dir])
        .output()
        .expect("Could not build Image");
    if output.stderr.len() > 0 {
        println!("{}", String::from_utf8(output.stderr).expect("Could not decode process output"));
    }
    let output_str = String::from_utf8(output.stdout).expect("Could not decode process output");
    println!("Building Output: {:?}", output_str);
}

pub fn push_image(image_tag: &String) {
    println!("Pushing image: {}", image_tag);
    let output = Command::new("docker")
        .args(["push", image_tag])
        .output()
        .expect("Could not push Image");
    if output.stderr.len() > 0 {
        println!("{}", String::from_utf8(output.stderr).expect("Could not decode process output"));
    }
    let output_str = String::from_utf8(output.stdout).expect("Could not decode process output");
    println!("Pushing Output: {:?}", output_str);
}
