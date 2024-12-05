use log::info;
use std::collections::HashMap;
use std::error::Error;

static ENV_FILE: &str = include_str!("../env.ini");

#[derive(Debug, Default)]
pub struct Env {
    pub curseforge_api_key: String,
}

impl Env {
    pub fn new() -> Result<Env, Box<dyn Error>> {
        info!("Reading environment variables from embedded file");
        // get an array of lines
        let lines = ENV_FILE.lines();

        // remove any lines that start with '#'
        let lines = lines.filter(|line| !line.starts_with('#'));

        // remove any data after '#'
        // Ex: key=value #some other data
        // the #some other data will be trimmed
        let lines: Vec<&str> = lines
            .map(|line| line.split('#').next().unwrap_or(line))
            .collect();

        // get key-value pairs
        let kvp: HashMap<String, String> = lines
            .iter()
            .map(|line| {
                let mut split = line.split("=");
                let key = split.next().unwrap_or("").to_string();
                let value = split.next().unwrap_or("").to_string();
                (key, value)
            })
            .collect();

        let mut env: Env = Env::default();

        for (key, value) in kvp {
            if key.eq_ignore_ascii_case("curseforge_api_key") {
                env.curseforge_api_key = value;
            }
        }

        Ok(env)
    }
}
