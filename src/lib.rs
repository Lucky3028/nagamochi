use std::fs;

pub fn read_capacity(path: std::path::PathBuf) -> anyhow::Result<u8> {
    let capa: u8 = fs::read_to_string(path)?.lines().next().unwrap_or_default().parse()?;
    anyhow::ensure!((0..=100).contains(&capa), "illegal number");

    Ok(capa)
}
