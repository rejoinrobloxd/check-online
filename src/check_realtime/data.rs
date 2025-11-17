use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct RealtimeStats {
    pub live: usize,
    pub dead: usize,
    pub banned: usize,
    pub error: usize,
    pub duplicates: usize,
}

pub struct PresenceStats {
    pub online: usize,
    pub offline: usize,
    pub in_game: usize,
    pub in_studio: usize,
}

pub struct HardwareInfo {
    pub cpu_name: String,
    pub cpu_usage: f32,
    pub ram_used: u64,
    pub ram_total: u64,
    pub ping: Option<u32>,
}

pub struct RealtimeData {
    pub cookies: Vec<String>,
    pub stats: Arc<Mutex<RealtimeStats>>,
    pub presence: Arc<Mutex<PresenceStats>>,
    pub hardware: Arc<Mutex<HardwareInfo>>,
    pub logs: Arc<Mutex<Vec<String>>>,
    pub client: reqwest::Client,
    pub seen_ids: Arc<Mutex<HashSet<u64>>>,
    pub live_ids: Arc<Mutex<HashSet<u64>>>,
    pub countdown: Arc<Mutex<u32>>,
    pub check_interval: Arc<Mutex<u32>>,
}

impl RealtimeData {
    pub fn new(cookies: Vec<String>, check_interval: u32) -> Self {
        Self {
            cookies,
            stats: Arc::new(Mutex::new(RealtimeStats {
                live: 0,
                dead: 0,
                banned: 0,
                error: 0,
                duplicates: 0,
            })),
            presence: Arc::new(Mutex::new(PresenceStats {
                online: 0,
                offline: 0,
                in_game: 0,
                in_studio: 0,
            })),
            hardware: Arc::new(Mutex::new(HardwareInfo {
                cpu_name: String::new(),
                cpu_usage: 0.0,
                ram_used: 0,
                ram_total: 0,
                ping: None,
            })),
            logs: Arc::new(Mutex::new(Vec::new())),
            client: reqwest::Client::new(),
            seen_ids: Arc::new(Mutex::new(HashSet::new())),
            live_ids: Arc::new(Mutex::new(HashSet::new())),
            countdown: Arc::new(Mutex::new(check_interval)),
            check_interval: Arc::new(Mutex::new(check_interval)),
        }
    }
}
