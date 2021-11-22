use serde::{Deserialize, Serialize};
use std::{cmp::PartialEq, fmt::Debug};

#[derive(Deserialize, PartialEq, Serialize, Debug)]
pub enum TriggerType {
    Above,
    Below,
    Equal,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Trigger {
    pub percentage: u8,
    pub when: TriggerType,
    pub message: String,
}

impl Trigger {
    pub fn is_fired(&self, capacity: u8) -> bool {
        match self.when {
            TriggerType::Above => self.percentage <= capacity,
            TriggerType::Below => self.percentage >= capacity,
            TriggerType::Equal => self.percentage == capacity
        }
    }
}
