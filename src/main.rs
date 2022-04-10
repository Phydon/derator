use rand::Rng;
use std::process::Command;
use std::time::Duration;
use std::{io, thread};

const NUM: usize = 10;

fn main() {
    let lst: [String; NUM] = [
        "Max".to_string(), 
        "Günther".to_string(), 
        "Julia".to_string(), 
        "Gustav".to_string(), 
        "Marie".to_string(), 
        "Uwe".to_string(), 
        "Luise".to_string(), 
        "Knut".to_string(), 
        "Pauline".to_string(), 
        "Ludwig".to_string()
    ];

    let mut n: u64 = 1;
    for i in 1..50 {
        exec_clear().expect("Failed to clear screen");

        println!("\\\\ GENERATOR //\n");
        let name: String = generate(&lst);
        
        if i == 49 {
            println!("{} << Du als nächstes", name.to_uppercase());
        } else if i < 49 && i > 45 {
            println!(">> {}", name);
            sleep(n);
            n += 100;
        } else if i < 45 && i > 40 {
            println!(">> {}", name);
            sleep(n);
            n += 60;
        } else if i < 40 && i > 33 {
            println!(">> {}", name);
            sleep(n);
            n += 30;
        } else if i < 33 && i > 22 {
            println!(">> {}", name);
            sleep(n);
            n += 10;
        } else if i < 22 {
            println!(">> {}", name);
            sleep(n);
            n += 6;
        }
    }
}

fn sleep(num: u64) {
    thread::sleep(Duration::from_millis(num));
}

fn exec_clear() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()?;
        Ok(())
    } else {
        Command::new("clear").status()?;
        Ok(())
    }
}

fn generate(lst: &[String; NUM]) -> String {
    let r = rand::thread_rng().gen_range(1..NUM);
    let name =  &lst[r];
    return name.to_string();
}
