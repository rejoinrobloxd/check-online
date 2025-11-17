use indicatif::ProgressBar;
use ansi_term::Colour;
use tokio::sync::Semaphore;
use std::sync::Arc;

use crate::check::data::PhaseData;
use crate::check::types::UserResponse;
use crate::check::models::BanResult;

pub async fn phase2_ban_check(data: &mut PhaseData, num_threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(Semaphore::new(num_threads));

    let pb2 = ProgressBar::new(data.accounts.len() as u64);
    pb2.set_style(indicatif::ProgressStyle::default_bar().template("{spinner} {msg} [{bar:40}] {pos}/{len}")?.progress_chars("=> "));
    pb2.set_message("Checking banned status");
    let mut handles = vec![];

    for (acc_index, _, id, _, _) in &data.accounts {
        let client = data.client.clone();
        let semaphore = Arc::clone(&semaphore);
        let index = *acc_index;
        let id = *id;

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let url = format!("https://users.roblox.com/v1/users/{}", id);
            let resp = client.get(&url).send().await;
            match resp {
                Ok(r) => {
                    if r.status() == 429 {
                        let url_roproxy = format!("https://users.roproxy.com/v1/users/{}", id);

                        let resp2 = client.get(&url_roproxy).send().await;
                        match resp2 {
                            Ok(r2) if r2.status().is_success() => {
                if let Ok(user) = r2.json::<UserResponse>().await {
                    BanResult { index, is_banned: Some(user.is_banned), id }
                } else {
                    BanResult { index, is_banned: None, id }
                }
                            }
                            _ => BanResult { index, is_banned: None, id }
                        }
                    } else if r.status().is_success() {
                        if let Ok(user) = r.json::<UserResponse>().await {
                            BanResult { index, is_banned: Some(user.is_banned), id }
                        } else {
                            BanResult { index, is_banned: None, id }
                        }
                    } else {
                        BanResult { index, is_banned: None, id }
                    }
                }
                Err(_) => BanResult { index, is_banned: None, id }
            }
        });
        handles.push(handle);
    }

    let mut ban_results = vec![];
    for handle in handles {
        ban_results.push(handle.await.unwrap());
    }

    ban_results.sort_by_key(|r| r.index);

    for r in &ban_results {
        if let Some(is_banned) = r.is_banned {
            if is_banned {
                data.banned += 1;
            } else {
                data.live += 1;
                data.live_ids.lock().unwrap().insert(r.id);
            }
        } else {
            data.error += 1;
        }
        pb2.inc(1);
    }
    pb2.finish_and_clear();

    for r in &ban_results {
        let cookie_i = data.accounts[r.index].1;
        let status_str = if let Some(is_banned) = r.is_banned {
            if is_banned { "BANNED" } else { "LIVE" }
        } else { "ERROR" };
        let prefix = match status_str {
            "LIVE" => Colour::Green.bold().paint("[LIVE]"),
            "BANNED" => Colour::Red.bold().paint("[BANNED]"),
            _ => Colour::Fixed(246).paint(format!("[{}]", status_str)),
        };
        data.status_log.insert(cookie_i, format!("{} Account {} (ID: {})", prefix, data.accounts[r.index].3, r.id));
    }

    Ok(())
}


