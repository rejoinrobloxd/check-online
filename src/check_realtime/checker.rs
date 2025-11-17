use crate::check::types::*;
use super::data::{RealtimeData, RealtimeStats};
use super::utils::format_log_message;

impl RealtimeData {
    pub async fn check_single_cookie(&self, cookie: &str, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .get("https://users.roblox.com/v1/users/authenticated")
            .header("Cookie", format!(".ROBLOSECURITY={}", cookie))
            .send()
            .await;

        let text = match response {
            Ok(resp) => resp.text().await?,
            Err(_) => {
                let mut logs = self.logs.lock().unwrap();
                let mut stats = self.stats.lock().unwrap();
                stats.dead += 1;
                logs.push(format_log_message(&format!("[DEAD] Cookie {} - Authentication failed", index + 1)));
                if logs.len() > 100 {
                    logs.remove(0);
                }
                return Ok(());
            }
        };

        if text.contains("User is moderated") {
            let mut logs = self.logs.lock().unwrap();
            let mut stats = self.stats.lock().unwrap();
            stats.banned += 1;
            logs.push(format_log_message("[BANNED] Account Unknown (ID: Unknown) - Moderated"));
            if logs.len() > 100 {
                logs.remove(0);
            }
            return Ok(());
        }

        let auth = match serde_json::from_str::<AuthResponse>(&text) {
            Ok(auth) => auth,
            Err(_) => {
                let mut logs = self.logs.lock().unwrap();
                let mut stats = self.stats.lock().unwrap();
                stats.dead += 1;
                logs.push(format_log_message(&format!("[DEAD] Cookie {} - Authentication failed", index + 1)));
                if logs.len() > 100 {
                    logs.remove(0);
                }
                return Ok(());
            }
        };

        let is_new_id = {
            let mut seen_ids = self.seen_ids.lock().unwrap();
            seen_ids.insert(auth.id)
        };

        if !is_new_id {
            let mut logs = self.logs.lock().unwrap();
            let mut stats = self.stats.lock().unwrap();
            stats.duplicates += 1;
            logs.push(format_log_message(&format!("[DUPLICATE] Account {} (ID: {})", auth.name, auth.id)));
            if logs.len() > 100 {
                logs.remove(0);
            }
            return Ok(());
        }
        let ban_check = self.client
            .get(&format!("https://users.roblox.com/v1/users/{}", auth.id))
            .send()
            .await;

        let user_result = match ban_check {
            Ok(resp) if resp.status().is_success() => {
                resp.json::<UserResponse>().await.ok()
            }
            _ => None
        };

        let mut logs = self.logs.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        match user_result {
            Some(user) => {
                if user.is_banned {
                    stats.banned += 1;
                    logs.push(format_log_message(&format!("[BANNED] Account {} (ID: {})", auth.name, auth.id)));
                } else {
                    stats.live += 1;
                    self.live_ids.lock().unwrap().insert(auth.id);
                    logs.push(format_log_message(&format!("[LIVE] Account {} (ID: {})", auth.name, auth.id)));
                }
            }
            None => {
                stats.error += 1;
                logs.push(format_log_message(&format!("[ERROR] Account {} (ID: {}) - Failed to check ban status", auth.name, auth.id)));
            }
        }

        if logs.len() > 100 {
            logs.remove(0);
        }
        Ok(())
    }

    pub async fn check_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut stats = self.stats.lock().unwrap();
            *stats = RealtimeStats {
                live: 0,
                dead: 0,
                banned: 0,
                error: 0,
                duplicates: 0,
            };
        }
        {
            let mut seen_ids = self.seen_ids.lock().unwrap();
            seen_ids.clear();
        }


        {
            let interval = *self.check_interval.lock().unwrap();
            let mut countdown = self.countdown.lock().unwrap();
            *countdown = interval;
        }


        {  let mut live_ids = self.live_ids.lock().unwrap();
            live_ids.clear(); }

        {
            let mut presence = self.presence.lock().unwrap();
            *presence = super::data::PresenceStats {
                online: 0,
                offline: 0,
                in_game: 0,
                in_studio: 0,
            };
        }
        for (i, cookie) in self.cookies.iter().enumerate() {
            if cookie.trim().is_empty() {
                continue;
            }
            if let Err(e) = self.check_single_cookie(cookie, i).await {
                let mut logs = self.logs.lock().unwrap();
                logs.push(format_log_message(&format!("[ERROR] Cookie {} - {}", i + 1, e)));
                if logs.len() > 100 {
                    logs.remove(0);
                }
            }
        }

        self.check_presence().await?;

        Ok(())
    }
    pub async fn check_presence(&self) -> Result<(), Box<dyn std::error::Error>> {
        let live_ids: Vec<u64> = {
            let live_ids = self.live_ids.lock().unwrap();
            live_ids.iter().cloned().collect()
        };

        if live_ids.is_empty() {
            return Ok(());
        }

        for chunk in live_ids.chunks(100) {
            let request_body = serde_json::json!({
                "userIds": chunk
            });

            let response = self.client
                .post("https://presence.roblox.com/v1/presence/users")
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
                .await;

            match response {
                Ok(resp) if resp.status().is_success() => {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(user_presences) = json["userPresences"].as_array() {
                            let mut presence = self.presence.lock().unwrap();
                            for presence_data in user_presences {
                                if let Some(presence_type) = presence_data["userPresenceType"].as_u64() {
                                    match presence_type {
                                        0 => presence.offline += 1,
                                        1 => presence.online += 1,
                                        2 => presence.in_game += 1,
                                        3 => presence.in_studio += 1,
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                }
            }
        }

        Ok(())
    }
}

