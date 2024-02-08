use std::{env, fs};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;


#[derive(Debug, Clone)]
struct WrongParamsErrror;
impl Error for WrongParamsErrror{}

impl fmt::Display for WrongParamsErrror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wrong parameters passed, it usess one extra parameter to specify the file to open")
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    if env::args().count()!=2 {
        return Err(Box::new(WrongParamsErrror))
    }
    //let f: File = File::open(env::args().last().unwrap())?;
    println!("{}",fs::read_to_string(env::args().last().unwrap())?);
    loop{}
    Ok(())
}
