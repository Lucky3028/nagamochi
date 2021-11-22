use serde::{Deserialize, Serialize};
use std::{cmp::PartialEq, fmt::Debug, fs, path::Path};

#[derive(Deserialize, PartialEq, Serialize, Debug)]
enum TriggerType {
    Above,
    Below,
    Equal,
    Charging,
    Discharging,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Trigger {
    percentage: Option<u8>,
    when: TriggerType,
    message: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    check_interval: i32,
    triggers: Vec<Trigger>,
}

impl Config {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let str = fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&str)?;
        let triggers: Vec<Trigger> = config
            .triggers
            .into_iter()
            .map(|trigger| Trigger {
                percentage: match trigger.when {
                    TriggerType::Charging | TriggerType::Discharging => None,
                    _ => trigger.percentage,
                },
                ..trigger
            })
            .collect();
        let config = Self { triggers, ..config };

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            check_interval: 60,
            triggers: vec![
                Trigger {
                    percentage: Some(100),
                    when: TriggerType::Equal,
                    message: String::from("Battery Full"),
                },
                Trigger {
                    percentage: Some(80),
                    when: TriggerType::Above,
                    message: String::from("Battery Upper Limit"),
                },
                Trigger {
                    percentage: Some(20),
                    when: TriggerType::Below,
                    message: String::from("Battery Lower Limit"),
                },
                Trigger {
                    percentage: None,
                    when: TriggerType::Charging,
                    message: String::from("Plugged"),
                },
                Trigger {
                    percentage: None,
                    when: TriggerType::Discharging,
                    message: String::from("Unplugged"),
                },
            ],
        }
    }
}

