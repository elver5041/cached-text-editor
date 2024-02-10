use std::io::{BufReader, BufRead, stdin};
use std::path::Path;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;


#[derive(Debug, Clone)]
struct WrongParamsErrror;
impl std::error::Error for WrongParamsErrror{}

impl fmt::Display for WrongParamsErrror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wrong parameters passed, it usess one extra parameter to specify the file to open")
    }
}

enum Mode {
    Menu,
    InFile,
    OpenFile,
    NewFile
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let mut mode ;
    let config_dir ;
    let home_dir;
    let file_sep;
    match env::consts::OS {
        "linux" => {
            config_dir="/home/.elver/console/";
            let user=env::var("UID")?.parse::<u32>()?;
            home_dir=format!("/home/{}/",user);
            file_sep="/" 
        },
        "windows" => {
            config_dir=r"%appdata%\elver\console\";
            let user = env::var("USERNAME")?;
            home_dir = format!("C:\\Users\\{}\\", user);
            file_sep=r"\";
        }
        _ => {panic!("os not supported");}
    }; 
    let file;
    if env::args().count()==1 {
        mode = Mode::Menu;
        let mut input= String::new();
        'base: loop {
            match mode {
                Mode::Menu => {
                    stdin().read_line(&mut input).expect("failed to read input");
                    mode = match input.as_str() {
                        "new" => Mode::NewFile,
                        "open" => Mode::OpenFile,
                        inp => {
                            println!("input \"{}\" not valid",inp);
                            Mode::Menu
                        }
                    }
                },
                Mode::NewFile => {
                    println!("input filename: ");
                    let mut filename = String::new();
                    stdin().read_line(&mut filename).expect("failed to read input");
                    let mut directory = String::new();
                    loop {
                        stdin().read_line(&mut directory).expect("failed to read input");
                        directory = match directory.to_ascii_lowercase().as_str() {
                            "desk" | "desktop" | "e" | "Escritorio" | "es" | "escri" | "esc" => {
                                format!("{}{}{}", home_dir, "Desktop", file_sep)
                            }
                            "down" | "downloa" | "downloads" | "do" | "desc" | "descargas" => {
                                format!("{}{}{}", home_dir, "Downloads", file_sep)
                            }
                            "docs" | "documents" | "documentos" | "doc" | "docum" => {
                                format!("{}{}{}", home_dir, "Documents", file_sep)
                            }
                            _ => {
                                if Path::new(&directory).is_dir() {
                                    directory
                                } else {
                                    println!(r#"directory is not valid, you can use keywords like "desktop", "downloads" or "documents""#);
                                    continue;
                                }
                            }
                        };
                        file = format!("{}{}",directory,filename);
                        break 'base;

                    }
                    
                }
                _=>panic!()
            }
        }

    }

    if env::args().count()!=2 {
        return Err(Box::new(WrongParamsErrror))
    }
    mode = Mode::InFile;
    let filename = env::args().last().unwrap();
    let f: File = File::open(filename).expect("input file");
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
}

fn read_lines(file: File) -> Result<Vec<String>, std::io::Error> {
    Ok(
        BufReader::new(file).lines()
            .collect::<Result<Vec<String>, std::io::Error>>()?
    )
}