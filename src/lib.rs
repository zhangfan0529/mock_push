use std::collections::HashMap;
use crate::data::{RuleData, PushList};
use std::sync::RwLock;

pub mod data;
pub mod req;


pub fn init_rule_data() -> RuleData {
    let map = HashMap::new();
    RuleData {
        data: RwLock::new(map),
    }
}

pub fn init_data() -> PushList {
    let map = HashMap::new();
    PushList {
        data: RwLock::new(map),
    }
}