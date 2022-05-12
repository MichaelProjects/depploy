use std::{
    error::Error,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

fn get_filesnames(path: &PathBuf) -> Result<Option<Vec<OsString>>, Box<dyn Error>> {
    //! gets a path to a directory and recrusively gets root files but also all file from the subdirectorys.
    //! appends the paths to a list and returns it
    let mut list = Vec::new();
    // check if the current path is a directory
    if path.is_dir() {
        for element in fs::read_dir(path)? {
            let y = element?;
            // calls it self here again
            let mut result = get_filesnames(&y.path())?;
            if result.is_some() {
                list.append(&mut result.unwrap());
            }
        }
    } else {
        list.push(path.as_os_str().into())
    }
    return Ok(Some(list));
}

fn analyzse_dir_struct(files: Vec<OsString>) {
    for file in files.iter() {
        let filename = file.to_str().unwrap();
        if filename.contains(".") {
            let y = filename.split(".").last();
            println!("{:?}", y.unwrap());
        }
    }
}

pub fn get_project_language() {}

#[test]
fn test_dir_structre() {
    let path = PathBuf::from_str("/home/michaell/development/depploy").unwrap();
    let abc = get_filesnames(&path);
    println!("{:?}", abc.unwrap().unwrap());
    assert_eq!(false, true)
}

#[test]
fn test_analyse_dir_structures() {
    let path = PathBuf::from_str("/home/michaell/development/depploy").unwrap();
    let abc = get_filesnames(&path);
    analyzse_dir_struct(abc.unwrap().unwrap());
    assert_eq!(false, true)
}
