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
            TriggerType::Equal => self.percentage == capacity,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(TriggerType::Below, 20, 1 => true)]
    #[test_case(TriggerType::Below, 1, 20 => false)]
    #[test_case(TriggerType::Below, 20, 20 => true)]
    #[test_case(TriggerType::Above, 1, 20 => true)]
    #[test_case(TriggerType::Above, 20, 1 => false)]
    #[test_case(TriggerType::Above, 20, 20 => true)]
    #[test_case(TriggerType::Equal, 20, 20 => true)]
    #[test_case(TriggerType::Equal, 20, 30 => false)]
    fn trigger_is_fired(when: TriggerType, percentage: u8, capa: u8) -> bool {
        let trigger = Trigger {
            percentage,
            when,
            message: "".to_string(),
        };

        trigger.is_fired(capa)
    }
}
