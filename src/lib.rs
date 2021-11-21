use std::fs;

pub fn read_capacity(path: std::path::PathBuf) -> anyhow::Result<u8> {
    let capa: u8 = fs::read_to_string(path)?
        .lines()
        .next()
        .unwrap_or_default()
        .parse()?;
    anyhow::ensure!((0..=100).contains(&capa), "illegal number");

    Ok(capa)
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
            eprintln!("Error: Failed to create temp file while testing: {:?}", e);
        };
        let path = temp_file.into_parts().1;
        let path: &Path = path.as_ref();

        let res = read_capacity(path.to_path_buf());
        println!("{:?}", res);

        res.is_ok()
    }
}
