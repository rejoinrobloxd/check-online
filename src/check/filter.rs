use std::io::{self, Write};

use super::data::PhaseData;
use super::models::Row;

pub fn filter_options(data: &mut PhaseData) -> Result<(), Box<dyn std::error::Error>> {
    print!("Do you want to add advanced option? (Y/N): ");
    io::stdout().flush().unwrap();
    let mut adv_input = String::new();
    io::stdin().read_line(&mut adv_input).unwrap();
    if adv_input.trim().to_lowercase() == "y" {
        print!("Do you want to filter cookie accounts by friends count? (Y/N): ");
        io::stdout().flush().unwrap();
        let mut filter_input = String::new();
        io::stdin().read_line(&mut filter_input).unwrap();
        if filter_input.trim().to_lowercase() == "y" {
            println!("1. Below");
            println!("2. Between");
            println!("3. High");
            print!("Select option: ");
            io::stdout().flush().unwrap();
            let mut opt_input = String::new();
            io::stdin().read_line(&mut opt_input).unwrap();
            match opt_input.trim() {
                "1" => {
                    print!("Enter max friends count: ");
                    io::stdout().flush().unwrap();
                    let mut val_input = String::new();
                    io::stdin().read_line(&mut val_input).unwrap();
                    if let Ok(max) = val_input.trim().parse::<i64>() {
                        data.final_ids.retain(|&id| data.friends_count.get(&id).map_or(false, |&c| c <= max));
                    }
                }
                "2" => {
                    print!("Enter min and max friends count (e.g., 30 45): ");
                    io::stdout().flush().unwrap();
                    let mut val_input = String::new();
                    io::stdin().read_line(&mut val_input).unwrap();
                    let parts: Vec<&str> = val_input.trim().split_whitespace().collect();
                    if parts.len() == 2 {
                        if let (Ok(min), Ok(max)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                            data.final_ids.retain(|&id| data.friends_count.get(&id).map_or(false, |&c| c >= min && c <= max));
                        }
                    }
                }
                "3" => {
                    print!("Enter min friends count: ");
                    io::stdout().flush().unwrap();
                    let mut val_input = String::new();
                    io::stdin().read_line(&mut val_input).unwrap();
                    if let Ok(min) = val_input.trim().parse::<i64>() {
                        data.final_ids.retain(|&id| data.friends_count.get(&id).map_or(false, |&c| c >= min));
                    }
                }
                _ => {}
            }
            let new_live = data.final_ids.len();
            let rows = vec![
                Row { category: "LIVE".to_string(), count: new_live },
                Row { category: "DEAD".to_string(), count: data.dead },
                Row { category: "BANNED".to_string(), count: data.banned },
                Row { category: "ERROR".to_string(), count: data.error },
                Row { category: "DUPLICATE".to_string(), count: data.duplicates },
            ];
            println!("{}", tabled::Table::new(&rows).with(tabled::settings::Style::rounded()).to_string());
        }
    }

    Ok(())
}


