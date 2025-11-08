use reqwest::blocking::get;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
struct Args {
    url:String,
    wordlist:String,
    null:String,
}

fn main(){
    let args = Args::parse();
    let file = File::open(&args.wordlist).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        match line {
            Ok(contenido) => {
                let full = format!("{}{}", args.url, contenido);
                let response = get(&full).unwrap();
                if response.status() == 200{
                    println!("{} - {}", contenido, response.status());
                }
            },
            Err(e) => eprintln!("Error al leer archivo {}", e),
        }
    }

}