use indicatif::ProgressBar;
use ansi_term::Colour;
use std::collections::HashSet;

use crate::check::data::PhaseData;
use crate::check::types::AuthResponse;

pub async fn phase1_auth(data: &mut PhaseData) -> Result<(), Box<dyn std::error::Error>> {
    let mut seen_ids = HashSet::new();
    let mut account_index = 0;

    let pb1 = ProgressBar::new(data.cookies.len() as u64);
    pb1.set_style(indicatif::ProgressStyle::default_bar().template("{spinner} {msg} [{bar:40}] {pos}/{len}")?.progress_chars("=> "));
    pb1.set_message("Checking authentication");

    for (i, cookie) in data.cookies.iter().enumerate() {
        if cookie.trim().is_empty() {
            pb1.inc(1);
            continue;
        }
        let response = data.client
            .get("https://users.roblox.com/v1/users/authenticated")
            .header("Cookie", format!(".ROBLOSECURITY={}", cookie))
            .send()
            .await;

        match response {
            Ok(resp) => {
        let text = resp.text().await?;
        if text.contains("User is moderated") {
            data.banned += 1;
            data.status_log.insert(i, format!("{} Account {} (ID: {}) - Moderated", Colour::Red.bold().paint("[BANNED]"), "Unknown", "Unknown"));
        } else {
            match serde_json::from_str::<AuthResponse>(&text) {
                Ok(auth) => {
                    if seen_ids.insert(auth.id) {
                        data.accounts.push((account_index, i, auth.id, auth.name, cookie.to_string()));
                        account_index += 1;
                    } else {
                        data.status_log.insert(i, format!("{} Account {} (ID: {})", Colour::Fixed(246).paint("[DUPLICATE]"), auth.name, auth.id));
                        data.duplicates += 1;
                    }
                }
                Err(_) => {
                    data.dead += 1;
                    data.status_log.insert(i, format!("{} Cookie {} - Authentication failed", Colour::Yellow.bold().paint("[DEAD]"), i + 1));
                }
            }
        }
            }
            Err(_) => {
                data.dead += 1;
                data.status_log.insert(i, format!("{} Cookie {} - Authentication failed", Colour::Yellow.bold().paint("[DEAD]"), i + 1));
            }
        }
        pb1.inc(1);
    }
    pb1.finish_and_clear();
    Ok(())
}


