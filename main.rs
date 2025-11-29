use serde::{Serialize, Deserialize};
use std::io;
use chrono::Local;

fn print_banner() {
    println!("
        ███╗   ██╗ ██████╗ ████████╗███████╗███████╗
        ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
        ██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
        ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
        ██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
        ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝");
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: u32,
    body: String,
    timestamp: String,
}

fn print_menu() {
    println!("1. Add Note");
    println!("2. Remove Note");
    println!("3. View Notes");
    println!("4. Quit");
    println!("Enter choice: ");
}

fn add_note(notes: &mut Vec<Note>) -> io::Result<()> {
    println!("Enter note: ");
    let mut body = String::new();
    io::stdin().read_line(&mut body)?;

    let now = Local::now();
    let now_formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    let new_note = Note {
        id: notes.len() as u32 + 1,
        body: body.trim().to_string(),
        timestamp: now_formatted,
    };

    notes.push(new_note);
    println!("Note added");
    Ok(())
}

fn main() -> io::Result<()> {
    print_banner();
    let mut notes: Vec<Note> = Vec::new();

    loop {
        print_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let clean_input: i32 = input.trim().parse().expect("Not a valid choice");

        if clean_input == 1 {
            add_note(&mut notes);
        } else if clean_input == 2 {
            //remove note
        } else if clean_input == 3 {
            //view notes
        } else if clean_input == 4 {
            break; //quit
        } else {
            println!("Invalid choice.");
        }
    }
    Ok(())
}


