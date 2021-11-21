use std::process::{Command, Output};
use crate::io::Config;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::env;

pub fn create_tag(image_conf: &Config, cli_settings: String) -> String{
    let tag = format!("{}/{}:{}", cli_settings.trim(), image_conf.name.trim(), image_conf.version.trim());
    println!("{}", tag);
    return tag
}

pub fn build_image(image_tag: &String){
    let output = Command::new("docker")
        .args(["build", "-t", image_tag, "."])
        .output()
        .unwrap();
    if output.stdout.is_empty(){
        panic!("Could not find a dockerfile")
    }
}

pub fn push_image(image_tag: &String){
    let output = Command::new("docker")
        .args(["push", image_tag])
        .output()
        .expect("Could not build image");
    println!("{:?}", output);
}