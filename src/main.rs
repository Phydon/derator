use rand::Rng;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

use std::cmp::min;
// use std::process::Command;
use std::time::Duration;
use std::io::Read;
use std::{io, fs, thread, char, str};

const NUM_SYM: usize = 10;
const FILEPATH: &str = "./names.txt";

fn main() {

    loop {
        let lst = read_file();

        let symbols: [char; NUM_SYM] = [
            char::from_u32(127917).unwrap(),
            char::from_u32(128520).unwrap(),
            char::from_u32(128545).unwrap(),
            char::from_u32(128565).unwrap(),
            char::from_u32(9875).unwrap(),
            char::from_u32(9889).unwrap(),
            char::from_u32(9762).unwrap(),
            char::from_u32(9763).unwrap(),
            char::from_u32(9760).unwrap(),
            char::from_u32(9883).unwrap()
        ];

        let mut n: u64 = 1;
        for i in 1..50 {
            // exec_clear().expect("Failed to clear screen");
            clear_screen();

            let title = title().blue();
            println!("{}", title);
            // let title = "GENERATOR".blue().underline().bold();
            // println!("\\\\ {} //\n", title);

            // println!("\\\\ GENERATOR //\n");

            let name: String = generate(&lst);
            
            if i == 49 {
                generate_symbol(&symbols);
                let txt = "du als nächstes".dimmed().italic();
                println!("                      >>   {}", name.to_uppercase().red().blink());
                println!("                      {}", txt);
            } else if i < 49 && i > 45 {
                hourglass(1);
                println!("                      >>   {}", name.red());
                sleep(n);
                n += 100;
            } else if i < 45 && i > 40 {
                hourglass(2);
                println!("                      >>   {}", name);
                sleep(n);
                n += 60;
            } else if i < 40 && i > 33 {
                hourglass(2);
                println!("                      >>   {}", name);
                sleep(n);
                n += 30;
            } else if i < 33 && i > 22 {
                hourglass(2);
                println!("                      >>   {}", name);
                sleep(n);
                n += 10;
            } else if i < 22 {
                hourglass(2);
                println!("                      >>   {}", name);
                sleep(n);
                n += 6;
            }
        }

        let done = quit();
        if done {break;}
    }

    std::process::exit(0);
}

fn read_file() -> Vec<String> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open(FILEPATH)
        .expect("Failed to open file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read file");

    let storage = content.split_whitespace().map(str::to_string).collect();

    storage
    }

fn sleep(num: u64) {
    thread::sleep(Duration::from_millis(num));
}

// fn exec_clear() -> io::Result<()> {
//     if cfg!(target_os = "windows") {
//         Command::new("cmd").args(["/C", "cls"]).status()?;
//         Ok(())
//     } else {
//         Command::new("clear").status()?;
//         Ok(())
//     }
// }

fn clear_screen() {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    println!("\x1Bc");
}

fn progress_bar() {
    let mut idx = 0;
    let end = 1000;

    let pb = ProgressBar::new(end);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{wide_bar:.cyan/blue}] ({eta})")
        .progress_chars("#>-"));

    while idx < end {
        let new = min(idx + 6, end);
        idx = new;
        pb.set_position(new);
        sleep(15);
    }

    pb.finish_with_message("done");

    // exec_clear().expect("Failed to clear screen");
    clear_screen();
}

fn generate(lst: &Vec<String>) -> String {
    let len_lst = lst.len();
    let r = rand::thread_rng().gen_range(1..len_lst);
    let name =  &lst[r];
    return name.to_string();
}

fn hourglass(a: u8) {
    if a == 1 {
        let hglass = char::from_u32(8987).unwrap();
        println!("                            {}\n", hglass);
    } else if a == 2 {
        let hglass = char::from_u32(9203).unwrap();
        println!("                            {}\n", hglass);
    } else {
        eprintln!("no symbol available");
    }

}

fn generate_symbol(symbols: &[char; NUM_SYM]) {
    let r = rand::thread_rng().gen_range(1..NUM_SYM);
    let sym =  &symbols[r];
    println!("                            {}\n", sym);
}

fn quit() -> bool {
    loop {
        println!("\n\nBeenden? (J/N):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "j" | "J" => return true,
            "n" | "N" => return false,
            _ => {
                println!("Keine gültige Eingabe");
                println!("Bitte \"J\" für Ja eingeben oder \"N\" für Nein.\n");
                progress_bar();
            }
        }
    }
}

fn title() -> String {
let title = "
██████╗ ███████╗██████╗  █████╗ ████████╗ ██████╗ ██████╗ 
██╔══██╗██╔════╝██╔══██╗██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗
██║  ██║█████╗  ██████╔╝███████║   ██║   ██║   ██║██████╔╝
██║  ██║██╔══╝  ██╔══██╗██╔══██║   ██║   ██║   ██║██╔══██╗
██████╔╝███████╗██║  ██║██║  ██║   ██║   ╚██████╔╝██║  ██║
╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝
".to_string();
 title
}
