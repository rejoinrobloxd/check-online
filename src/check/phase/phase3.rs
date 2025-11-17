use indicatif::ProgressBar;
use tokio::sync::Semaphore;
use std::sync::Arc;
use serde_json::Value as JsonValue;

use crate::check::data::PhaseData;

pub async fn phase3_friends_fetch(data: &mut PhaseData, num_threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let live_list: Vec<u64> = data.live_ids.lock().unwrap().iter().cloned().collect();

    if !live_list.is_empty() {
        let pb3 = ProgressBar::new(live_list.len() as u64);
        pb3.set_style(indicatif::ProgressStyle::default_bar().template("{spinner} {msg} [{bar:40}] {pos}/{len}")?.progress_chars("=> "));
        pb3.set_message("Fetching friends count for LIVE accounts");
        let semaphore = Arc::new(Semaphore::new(num_threads));
        let mut handles = vec![];
        for &id in &live_list {
            let client = data.client.clone();
            let semaphore = Arc::clone(&semaphore);
            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let url = format!("https://friends.roblox.com/v1/users/{}/friends/count", id);

                let resp = client.get(&url).send().await;
                match resp {
                    Ok(r) => {
                        if r.status().is_success() {
                            if let Ok(json) = r.json::<JsonValue>().await {
                                if let Some(count) = json["count"].as_i64() {
                                    Some((id, count))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(_) => None
                }
            });
            handles.push(handle);
        }
        for (_, handle) in handles.into_iter().enumerate() {
            if let Ok(Some((id, count))) = handle.await {
                data.friends_count.insert(id, count);
            }
            pb3.inc(1);
        }
        pb3.finish_and_clear();
    }

    Ok(())
}


