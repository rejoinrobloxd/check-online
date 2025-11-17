use rfd::FileDialog;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::sync::Mutex;
use tabled::Table;
use chrono::Utc;

use super::data::PhaseData;
use super::models::Row;
use super::phase::{phase1_auth, phase2_ban_check, phase3_friends_fetch};
use super::filter::filter_options;

pub async fn check_accounts() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter number of async threads (default 10): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_threads = input.trim().parse::<usize>().unwrap_or(10);

    let file_dialog = FileDialog::new()
        .add_filter("Text files", &["txt"])
        .set_directory("/");

    let cookie_file_path = match file_dialog.pick_file() {
        Some(path) => path,
        None => {
            println!("No file selected.");
            return Ok(());
        }
    };

    let content = fs::read_to_string(cookie_file_path)?;
    let cookies_content: Vec<String> = if content.contains("\n\n") {
        content.split("\n\n").filter(|s| !s.trim().is_empty()).map(|s| s.replace("\n", "")).collect()
    } else {
        content.lines().filter(|s| !s.trim().is_empty()).map(|s| s.to_string()).collect()
    };

    let mut data = PhaseData {
        cookies: cookies_content,
        client: reqwest::Client::new(),
        status_log: HashMap::new(),
        accounts: Vec::new(),
        live:0,
        dead:0,
        banned:0,
        error:0,
        duplicates:0,
        friends_count: HashMap::new(),
        live_ids: Mutex::new(HashSet::new()),
        final_ids: HashSet::new(),
    };

    phase1_auth(&mut data).await?;
    phase2_ban_check(&mut data, num_threads).await?;
    phase3_friends_fetch(&mut data, num_threads).await?;
    data.final_ids=data.live_ids.lock().unwrap().clone();

    for i in 0..data.cookies.len() {
        if let Some(log) = data.status_log.get(&i) {
            println!("{}", log);
        }
    }

    let rows = vec![
        Row { category: "LIVE".to_string(), count: data.live },
        Row { category: "DEAD".to_string(), count: data.dead },
        Row { category: "BANNED".to_string(), count: data.banned },
        Row { category: "ERROR".to_string(), count: data.error },
        Row { category: "DUPLICATE".to_string(), count: data.duplicates },
    ];

    println!("{}", Table::new(&rows).with(tabled::settings::Style::rounded()).to_string());

    if data.live == 0 {
        println!("No LIVE accounts to filter.");    
    } else {
        filter_options(&mut data)?;
    }

    if data.live > 0 {
        print!("Filter final LIVE accounts to file? (Y/N): ");
        io::stdout().flush().unwrap();
        let mut save_input = String::new();
        io::stdin().read_line(&mut save_input).unwrap();
        if save_input.trim().to_lowercase() == "y" {
            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
            let default_filename = format!("cookie_{}.txt", timestamp);
            let file_path = FileDialog::new().set_file_name(&default_filename).save_file();
            if let Some(path) = file_path {
                let final_cookies: Vec<String> = data.accounts.iter().filter_map(|(_, _, id, _, cookie)| if data.final_ids.contains(id) { Some(cookie.clone()) } else { None }).collect();
                let content = final_cookies.join("\n\n");
                fs::write(path, content)?;
                println!("Final LIVE cookies saved to file.");
            }
        }
    }

    Ok(())
}

