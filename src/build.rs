use std::process::{Command, Output};
use crate::io::Config;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::env;

pub fn create_tag(image_conf: &Config, cli_settings: String) -> String{
    let tag = format!("{}/{}:{}", cli_settings, image_conf.name, image_conf.version);
    return tag
}

pub fn build_image(image_tag: &String){
    let mut stream = UnixStream::connect("/var/run/docker.sock").unwrap();
    let path = env::current_dir().unwrap();
    let data = format!("GET /build?dockerfile={}/dockerfile HTTP/1.1\r\n\r\n", path.as_os_str().to_str().unwrap());
    stream.write_all(data.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);
}

pub fn push_image(image_tag: &String){
}