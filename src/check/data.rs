use reqwest::Client;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

pub struct PhaseData {
    pub cookies: Vec<String>,
    pub client: Client,
    pub status_log: HashMap<usize, String>,
    pub accounts: Vec<(usize, usize, u64, String, String)>,
    pub live: usize,
    pub dead: usize,
    pub banned: usize,
    pub error: usize,
    pub duplicates: usize,
    pub friends_count: HashMap<u64, i64>,
    pub live_ids: Mutex<HashSet<u64>>,
    pub final_ids: HashSet<u64>,
}


