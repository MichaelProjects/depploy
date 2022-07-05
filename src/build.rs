use crate::io::Config;
use log::{debug, info, trace, warn};
use std::process::Command;

pub fn create_tag(image_conf: &Config, mut docker_registry: String) -> Vec<String> {
    if docker_registry.clone().ne(&String::from("")) {
        docker_registry = format!("{}/", docker_registry)
    }

    let name = format!(
        "{}{}",
        docker_registry.trim().to_lowercase(),
        image_conf.name.trim().to_lowercase(),
    );

    let tag = format!("{}:{}", name, image_conf.version.trim());
    debug!("Docker image-tag: {}", tag);
    return vec![name, tag];
}

pub fn set_latest_tag(image_name: &String, image_tag: &String) -> String {
    let latest_tag: String = format!("{}:{}", image_name, "latest");
    return latest_tag;
}

pub fn build_image(image_tag: &String, dir: &str, dockerfile_name: &String, no_latest: &bool, latest_img_tag: &String) {
    debug!("Building image: {}", image_tag);
    let mut output;
    if no_latest == &true {
        output = Command::new("docker")
            .args(["build", "-f", dockerfile_name, "-t", image_tag, dir])
            .output()
            .expect("Could not build Image");
    } else {
        output = Command::new("docker")
            .args(["build", "-f", dockerfile_name, "-t", image_tag, "-t", latest_img_tag, dir])
            .output()
            .expect("Could not build Image");
    }
    if output.stderr.len() != 0 {
        // panic!(
        //     "{}",
        //     String::from_utf8(output.stderr).expect("Could not decode process output")
        // );
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
        // panic!(
        //     "{}",
        //     String::from_utf8(output.stderr).expect("Could not decode process output")
        // );
    }
    let output_str = String::from_utf8(output.stdout).expect("Could not decode process output");
    debug!("Pushing Output: {:?}", output_str);
}
