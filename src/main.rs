use std::io::{self, Write, stdout};
use crossterm::terminal::{Clear, ClearType};
use crossterm::execute;
use crossterm;
use tokio::signal;
use figlet_rs::FIGfont;

mod check;
mod check_realtime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    tokio::spawn(async {
        signal::ctrl_c().await.unwrap();
        println!("\n[EXIT] Graceful shutdown requested by user. Exiting...");
        std::process::exit(0);
    });
    let s = FIGfont::standard().unwrap();
    let s = s.convert("Dawn Checker").unwrap();
    println!("{}", s);
    println!("[ 1 ] - Check Account (cookie)");
    println!("[ 2 ] - Check Real-time Cookie");
    print!("Select option: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice == "1" {
        check::check_accounts().await?;
        print!("Press Enter to exit...");
        io::stdout().flush().unwrap();
        let mut _dummy = String::new();
        io::stdin().read_line(&mut _dummy).unwrap();
    } else if choice == "2" {
        check_realtime::check_realtime().await?;
    } else {
        println!("Invalid option. Exiting...");
    }

    Ok(())
}
