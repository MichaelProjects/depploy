use crate::{
    common::error::PTGenError,
    io::{load_project_file, match_config, ProjectConf},
};
use log::{debug, error, info};
use std::{
    io::{self, BufRead},
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

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

pub async fn build_image(
    image_tag: &String,
    dir: &str,
    dockerfile_name: &String,
    no_latest: &bool,
    latest_img_tag: &String,
    platform: &String,
) -> Result<(), PTGenError> {
    info!("Building image: {}", image_tag);
    let p = PathBuf::from_str(dir).unwrap();
    let filename = match_config(&p);

    let mut project_path = dir.to_string();

    let config_data = match load_project_file(&p, &filename) {
        Ok(data) => data,
        Err(err) => panic!("Error: {err}"),
    };
    if config_data.contains(", path =") {
        let mut x = PathBuf::from_str(dir).unwrap();
        x.pop();
        project_path = x.to_str().unwrap().to_string();
    }
    let dockerfile_name_path = format!("{project_path}/{dockerfile_name}");
    let mut args = vec![
        "buildx",
        "build",
        "-f",
        dockerfile_name_path.as_str(),
        "-t",
        image_tag,
        "-t",
        latest_img_tag,
        project_path.as_str(),
    ];
    if platform != "" {
        args.insert(2, "--platform");
        args.insert(3, &platform);
    }
    if no_latest == &true {
        args = vec![
            "buildx",
            "build",
            "-f",
            dockerfile_name_path.as_str(),
            "-t",
            image_tag,
            project_path.as_str(),
        ];
    }

    debug!("{:?}", args);
    let mut cmd = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Read the output of the command
    if let Some(stdout) = cmd.stdout.take() {
        let reader = io::BufReader::new(stdout);

        // Iterate over each line of the output
        for line in reader.lines() {
            println!("{}", line.unwrap()); // Print each line in real-time
        }
    }

    // Wait for the command to finish and get the exit status
    let status = cmd.wait().unwrap();
    match status.exit_ok() {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("Failed to build container");
            return Err(PTGenError::FailedBuilding);
        }
    }
}

pub fn push_image(image_tag: &String) -> Result<(), PTGenError>  {
    info!("Pushing image: {}", image_tag);

    let mut cmd = Command::new("docker")
        .args(["push", image_tag])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let status = cmd.wait().unwrap();
    match status.exit_ok() {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("Failed to push container");
            return Err(PTGenError::FailedPushing);
        }
    }
}
