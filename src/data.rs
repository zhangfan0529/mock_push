use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Deserialize,Serialize};

pub struct RuleData {
    pub data: RwLock<HashMap<i64, PushRule>>,
}


pub struct PushList {
    pub data: RwLock<HashMap<i64, Vec<PushData>>>,
}


#[derive(Deserialize)]
pub struct PushRule {
    pub status_code: i32,
    pub response: String,
}

#[derive(Deserialize,Serialize)]
pub struct PushData {
    pub push_header: HashMap<String, String>,
    pub raw_body: String,
    pub body: Option<String>,
}



