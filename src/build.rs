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

pub fn build_image(image_tag: &String, dir: &str) {
    println!("Building image: {}", image_tag);
    let output = Command::new("docker")
        .args(["build", "-t", image_tag, dir])
        .output()
        .expect("Could not build Image");
}

pub fn push_image(image_tag: &String) {
    println!("Pushing image: {}", image_tag);
    let output = Command::new("docker")
        .args(["push", image_tag])
        .output()
        .expect("Could not push Image");
}
