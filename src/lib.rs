use soloud::{audio, AudioExt, LoadExt, Soloud};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

mod config;
pub use config::Config;
mod suppressor;
pub use suppressor::Suppressor;
mod trigger;
mod serde_helper;

pub fn find_config() -> anyhow::Result<Config> {
    env::var_os("HOME")
        .map(find_config_path)
        .map_or(Err(anyhow::anyhow!("config not found")), |path| {
            Config::from_file(&path.unwrap())
        })
}

pub fn find_config_path(home_dir: std::ffi::OsString) -> Option<PathBuf> {
    vec![
        format!("{:?}/.config/nagamochi/nagamochi.yml", home_dir),
        format!("{:?}/nagamochi.yml", home_dir),
        "./nagamochi.yml".to_string(),
    ]
    .iter()
    .map(PathBuf::from)
    .find(|path| path.exists())
}

pub fn read_capacity(path: std::path::PathBuf) -> anyhow::Result<u8> {
    let capa: u8 = fs::read_to_string(path)?
        .lines()
        .next()
        .unwrap_or_default()
        .parse()?;
    anyhow::ensure!((0..=100).contains(&capa), "illegal number");

    Ok(capa)
}

pub fn is_ac_connected(path: std::path::PathBuf) -> anyhow::Result<bool> {
    let status: u8 = fs::read_to_string(path)?
        .lines()
        .next()
        .unwrap_or_default()
        .parse()?;

    Ok(status % 2 == 1)
}

pub fn play_sound(path: &Path) -> anyhow::Result<()> {
    anyhow::ensure!(path.exists(), "Audio file not found!");

    let sound_player = Soloud::default()?;
    let mut wav = audio::Wav::default();
    wav.load(path)?;
    sound_player.play(&wav);
    while sound_player.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{io::Write, path::Path};
    use tempfile::NamedTempFile;
    use test_case::test_case;

    #[test_case("0" => true)]
    #[test_case("30" => true)]
    #[test_case("100" => true)]
    #[test_case("200" => false)]
    #[test_case("-10" => false)]
    #[test_case("a" => false)]
    #[test_case("" => false)]
    fn read_capa(content: &str) -> bool {
        let mut temp_file = NamedTempFile::new().unwrap();
        if let Err(e) = write!(temp_file, "{}", content) {
            panic!("Error: Failed to create temp file while testing: {:?}", e);
        };
        let path = temp_file.into_parts().1;
        let path: &Path = path.as_ref();

        read_capacity(path.to_path_buf()).is_ok()
    }

    #[test_case(0 => false)]
    #[test_case(1 => true)]
    fn test_is_ac_connected(content: u8) -> bool {
        let mut temp_file = NamedTempFile::new().unwrap();
        if let Err(e) = write!(temp_file, "{}", content) {
            panic!("Error: Failed to create temp file while testing: {:?}", e);
        };
        let path = temp_file.into_parts().1;
        let path: &Path = path.as_ref();
        let res = is_ac_connected(path.to_path_buf());

        res.is_ok() && res.unwrap()
    }
}
