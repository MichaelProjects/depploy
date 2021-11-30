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

pub fn build_image(image_tag: &String) {
    let output = Command::new("docker")
        .args(["build", "-t", image_tag, "."])
        .output()
        .unwrap();
    if output.stdout.is_empty() {
        panic!("Could not find a dockerfile")
    }
}

pub fn push_image(image_tag: &String) {
    let output = Command::new("docker")
        .args(["push", image_tag])
        .output()
        .expect("Could not build image");
    println!("{:?}", output);
}
