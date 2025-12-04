use std::env;

//struct to hold configuration
pub struct Config{
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{

        if args.len() < 3 {
            return Err("Not enough arguments. usage: programname, query string & filepath")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        OK(Config {query, file_path})
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();

    let config=  Config::build(&args).expect("Problem occured while parsing argument");

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);
}