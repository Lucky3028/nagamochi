use serde::{Deserialize, Serialize};
use std::{cmp::PartialEq, fmt::Debug, fs, path::Path};

use super::trigger::{Trigger, TriggerType};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub check_interval: u64,
    pub triggers: Vec<Trigger>,
}

impl Config {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let str = fs::read_to_string(path)?;

        Ok(serde_yaml::from_str::<Self>(&str)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            check_interval: 60,
            triggers: vec![
                Trigger {
                    percentage: 100,
                    when: TriggerType::Equal,
                    message: String::from("Battery Full"),
                    can_suppress: true
                },
                Trigger {
                    percentage: 80,
                    when: TriggerType::Above,
                    message: String::from("Battery Upper Limit"),
                    can_suppress: true,
                },
                Trigger {
                    percentage: 20,
                    when: TriggerType::Below,
                    message: String::from("Battery Lower Limit"),
                    can_suppress: true
                },
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn general_config() {
        let expected_config = Config {
            check_interval: 60,
            triggers: vec![
                Trigger {
                    message: "qwerty".to_string(),
                    percentage: 20,
                    when: TriggerType::Equal,
                    can_suppress: true,
                },
                Trigger {
                    message: "qwerty".to_string(),
                    percentage: 10,
                    when: TriggerType::Above,
                    can_suppress: true,
                },
                Trigger {
                    message: "qwerty".to_string(),
                    percentage: 80,
                    when: TriggerType::Below,
                    can_suppress: true,
                },
            ],
        };
        let path = Path::new("./src/tests/configs/general_config.yml");
        let res = Config::from_file(path).unwrap();

        assert_eq!(res, expected_config);
    }

    #[test]
    fn bad_config() {
        let path = Path::new("./src/tests/configs/bad_config.yml");
        let res = Config::from_file(path);

        assert!(res.is_err());
    }
}
