use std::{fs::File, io::copy};

use anyhow::{Ok, Result};
use url::Url;

pub fn extract_filename_from_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    let segments = parsed_url.path_segments()?;
    let filename = segments.last()?.to_string();
    if filename.is_empty() {
        None
    } else {
        Some(filename)
    }
}

pub async fn download_file(url: &str, save_path: &str) -> Result<()> {
    let url = Url::parse(url)?;
    let response = reqwest::get(url).await?;
    let mut file = File::create(save_path)?;
    let content = response.bytes().await?;
    copy(&mut &content[..], &mut file)?;
    Ok(())
}

mod tests {

    #[test]
    fn test_extract_filename_from_url() {
        assert_eq!(
            super::extract_filename_from_url("https://example.com/files/example.txt"),
            Some("example.txt".to_string())
        );
        assert_eq!(
            super::extract_filename_from_url("https://example.com/files/example.txt?a=1&b=2&c=3"),
            Some("example.txt".to_string())
        );
    }
}
