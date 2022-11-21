use std::{path::PathBuf, str::FromStr};

use crate::{commands::Prototype, common::models::PrototypeConfig, build::{create_tag, set_latest_tag}, io::{ProjectConf, build_depploy_path, match_config, load_project_file, get_info}, conf::{read_depploy_conf}};

use super::{generate::{send_creation_prototype, CreatePrototype, presist_creation_prototype, check_for_prototype}, upload::{read_project_file, upload_config}, list::list_running_services};



pub async fn prototype_logic(cmd: &Prototype) {
    match cmd {
        Prototype::Create { dir, debug } => {
            let x = dir.as_os_str().to_string_lossy().to_string();
            check_for_prototype(&x).expect("Codebase already has a prototype configured.");

            let cfg = read_depploy_conf(&PathBuf::from_str(build_depploy_path().as_str()).unwrap()).await.unwrap();
            
            let filename = match_config(&dir);
            let config_data = match load_project_file(&dir, &filename) {
                Ok(data) => data,
                Err(err) => panic!("Error: {}", err),
            };
            let data = get_info(config_data);

            let image_tag = create_tag(&data, cfg.registry.docker_registry);
            let latest = set_latest_tag(&image_tag[0]);
            let cpt = CreatePrototype::new(data.name, String::new(), latest);
            let res = send_creation_prototype(&cpt, &x, &cfg.prototype.clone().unwrap().prototype_host, &cfg.prototype.unwrap().prototype_app_token).await.expect("Error");


            presist_creation_prototype(x, PrototypeConfig::new(res, cpt.docker_registry_uri.clone())).unwrap();
        }
        Prototype::Upload { dir, debug } => {
            let cfg = read_depploy_conf(&PathBuf::from_str(build_depploy_path().as_str()).unwrap()).await.unwrap();
            let x = dir.as_os_str().to_string_lossy().to_string();
            let pcfg = read_project_file(&x).unwrap();
            upload_config(pcfg,&cfg.prototype.clone().unwrap().prototype_host, cfg.prototype.unwrap().prototype_app_token).await.expect("An error occured while uploading the config");
        }
        Prototype::List => {
            let cfg = read_depploy_conf(&PathBuf::from_str(build_depploy_path().as_str()).unwrap()).await.unwrap();
            list_running_services(&cfg.prototype.clone().unwrap().prototype_host, cfg.prototype.unwrap().prototype_app_token).await.unwrap();
        }
    }
}