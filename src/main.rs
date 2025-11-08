use reqwest::blocking::get;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{cursor, terminal::{self, Clear, ClearType}, ExecutableCommand};
use crossterm::cursor::MoveTo;

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
                let (cols, _) = terminal::size().unwrap();
                let mut stdout = stdout();
                let full = format!("{}{}", args.url, contenido);
                let response = get(&full).unwrap();
                let y = 20;
                if response.status() != args.null{
                    let (local, _) = terminal::size().unwrap();
                    stdout.execute(cursor::MoveTo(cols - 90, y)).unwrap();
                    stdout.execute(MoveTo(cols, y + 1)).unwrap();
                    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                    println!(" {}{} - {}", args.url, contenido, response.status());
                    stdout.flush().unwrap();

                }
                if response.status() == args.null || response.status() != args.null{
                    stdout.execute(cursor::MoveTo(cols - 100, 50)).unwrap();
                    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                    print!("[TRY] {}{} - {}", args.url, contenido, response.status());
                    stdout.flush().unwrap();
                    //sleep(Duration::from_secs(1));
                }
            },
            Err(e) => eprintln!("Error al leer archivo {}", e),
        }
    }

}