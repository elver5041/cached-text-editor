use std::env;
use std::io::Error;

fn main() -> Result<(),Error> {
    for arg in env::args(){
        println!("{}", arg);
    }
    Ok(())
}
