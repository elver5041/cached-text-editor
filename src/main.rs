use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug, Clone)]
struct WrongParamsErrror;
impl std::error::Error for WrongParamsErrror{}

impl fmt::Display for WrongParamsErrror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wrong parameters passed, it usess one extra parameter to specify the file to open")
    }
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    if env::args().count()!=2 {
        return Err(Box::new(WrongParamsErrror))
    }
    let filename = env::args().last().unwrap();
    let f: File = File::open(filename).expect("input file");
    let config_dir = match env::consts::OS {
        "linux" => {
            r"/home/.elver/console/" 
        },
        "windows" => {
            r"%appdata%\elver\console\"
        }
        _ => {panic!("os not supported");}
    };
    fs::create_dir_all(config_dir)?;
    let config = match File::open(config_dir.to_owned()+"config.json") {
        Ok(file) => file,
        Err(_) => File::create(config_dir.to_owned()+"config.json")?
    };
    let lines: Vec<String> = read_lines(f)?;
    for i in lines{
        println!("{}",i);
    }   
    loop{}
    Ok(())
}

fn read_lines(file: File) -> Result<Vec<String>, std::io::Error> {
    Ok(
        BufReader::new(file).lines()
            .collect::<Result<Vec<String>, std::io::Error>>()?
    )
}