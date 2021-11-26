use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;

pub fn serialize<S>(value: &Option<PathBuf>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'a>(#[serde(with = "PathBuf")] &'a PathBuf);

    value.as_ref().map(Helper).serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(with = "PathBuf")] PathBuf);

    let helper = Option::deserialize(deserializer)?;

    Ok(helper.map(|Helper(path)| path))
}
