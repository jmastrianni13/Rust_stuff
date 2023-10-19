use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String>{
        if args.len() < 3 {
            let err_msg = format!("expect 2 arguments, got {}", args.len()-1);
            return Err(err_msg);
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config{query, file_path});
    }
}

