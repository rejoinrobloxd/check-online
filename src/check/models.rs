use tabled::Tabled;

pub struct BanResult {
    pub index: usize,
    pub is_banned: Option<bool>,
    pub id: u64,
}

#[derive(Tabled)]
pub struct Row {
    #[tabled(rename = "Status")]
    pub category: String,
    #[tabled(rename = "Count")]
    pub count: usize,
}


