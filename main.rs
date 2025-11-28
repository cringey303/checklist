use serde::{Serialize, Deserealize};
use std::io;
#[derive(Serialize, Deserealize)]
struct Note {
    id: u32,
    body: String,
    timestamp: String,
}

fn print_menu() {
    println!("1. Add Note");
    println!("2. Remove Note");
    println!("3. Quit");
    println!("Enter choice: ");
}

fn print_banner() {
    println!("
        ███╗   ██╗ ██████╗ ████████╗███████╗███████╗\n
        ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝\n
        ██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗\n
        ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║\n
        ██║ ╚████║╚██████╔╝   ██║   ███████╗███████║\n
        ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝");
}

fn main() -> io::Result<()> {
    printBanner();
    let mut notes: Vec<Note> = Vec::new();
    loop {
        printMenu();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let clean_input: i32  = input.trim().parse().expect("Not a valid choice");

        if clean_input == 1 {
            //add note
        } else if clean_input == 2 {
            //quit
        }
    }

}


