use regex::Regex;
use std::{fs, io::ErrorKind};

pub const CONFIG_FILE: &str = ".ballercliconfig";

// A config file is expected on the users local machine
// .ballercliconfig is expected in HOME

#[derive(Debug)]
pub struct Creds {
    pub email: String,
    pub token: String,
}

pub fn get_jira_creds() -> Result<Creds, &'static str> {
    // Get user home path
    let path = match home::home_dir() {
        Some(path) => Ok(path),
        None => Err("Problem finding HOME dir"),
    };

    // Change it to string
    let path_string = path.unwrap().into_os_string().into_string().unwrap();
    println!("{}", &path_string);

    // Read the .ballercliconfig file
    let contents =
        fs::read_to_string(format!("{}/{}", path_string, CONFIG_FILE)).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                panic!(".ballercliconfig file not found in HOME dir")
            } else {
                panic!("Problem opeing the file: {:?}", error)
            }
        });

    // Parse to struct and return
    parse_creds(contents)
}

fn parse_creds(contents: String) -> Result<Creds, &'static str> {
    let creds = contents.split('\n').collect::<Vec<_>>();
    let re = Regex::new(r"^[^_]*=").unwrap();
    
    if creds.len() <= 2 {
        return Err("JIRA email or token missing, please check .ballercliconfig")
    }
    
    let email = re.replace_all(creds[0], "").to_string();
    let token = re.replace_all(creds[1], "").to_string();
    
    Ok(Creds { email, token })
}
