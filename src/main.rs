use reqwest::blocking::get;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
struct Args {
    url:String,
    wordlist:String,
    null:u16,
}

fn banner(){
    println!("

@@@       @@@@@@   @@@@@@  @@@@@@@  @@@@@@@@ @@@  @@@ @@@@@@@@ @@@@@@@@ 
@@!      @@!  @@@ @@!  @@@ @@!  @@@ @@!      @@!  @@@      @@!      @@! 
@!!      @!@  !@! @!@  !@! @!@@!@!  @!!!:!   @!@  !@!    @!!      @!!   
!!:      !!:  !!! !!:  !!! !!:      !!:      !!:  !!!  !!:      !!:     
: ::.: :  : :. :   : :. :   :        :        :.:: :  :.::.: : :.::.: : 
                                                                        
        ");
}
fn main(){
    banner();
    let args = Args::parse();
    let file = File::open(&args.wordlist).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        match line {
            Ok(contenido) => {
                let full = format!("{}{}", args.url, contenido);
                let response = get(&full).unwrap();
                if response.status() != args.null{
                    println!("{} - {}", contenido, response.status());
                }
            },
            Err(e) => eprintln!("Error al leer archivo {}", e),
        }
    }

}