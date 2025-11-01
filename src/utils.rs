use anyhow::{Context, bail};
use goose::goose::TransactionError;
use rand::Rng;
use std::{fmt::Display, io::Read, path::PathBuf};
use xz2::read::XzDecoder;

/// Truncate from the middle if the string is too long.
pub fn truncate_middle(s: impl Display, max_len: usize) -> String {
    let s = s.to_string();

    if s.len() <= max_len {
        s.to_string()
    } else {
        let keep = (max_len - 1) / 2; // Number of characters to keep from start and end
        let end_keep = max_len - keep - 1; // Ensure total length is exactly max_len
        format!("{}…{}", &s[..keep], &s[s.len() - end_keep..])
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DisplayVec<T>(pub Vec<T>);

impl<T: std::fmt::Display> std::fmt::Display for DisplayVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strs: Vec<String> = self.0.iter().map(|item| item.to_string()).collect();
        write!(f, "{}", strs.join(","))
    }
}

/// This struct is used to store user-specific data that is needed for Goose transactions.
///
/// The advisory ID is randomly selected from the available advisories.
#[derive(Clone)]
pub struct GooseUserData {
    pub advisory_id: Option<String>,
}

/// Load advisory file from UPLOAD_FILE_PATH directory, or fall back to CARGO_MANIFEST_DIR
pub fn load_advisory_files() -> anyhow::Result<PathBuf> {
    // Try UPLOAD_FILE_PATH first, then fall back to CARGO_MANIFEST_DIR
    let base_path = std::env::var("UPLOAD_FILE_PATH")
        .or_else(|_| Ok::<String, std::env::VarError>(env!("CARGO_MANIFEST_DIR").to_string()))
        .ok()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Neither UPLOAD_FILE_PATH nor CARGO_MANIFEST_DIR environment variables are set"
            )
        })?;

    let advisories_dir: PathBuf = [&base_path, "advisories"].iter().collect();

    let entries = std::fs::read_dir(&advisories_dir).with_context(|| {
        format!(
            "failed to read advisories directory: {}",
            advisories_dir.display()
        )
    })?;

    let files: Vec<PathBuf> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? == "xz" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if let Some(f) = files.first() {
        Ok(f.clone())
    } else {
        bail!("No advisory files found in {}", advisories_dir.display());
    }
}

/// Read upload file, handling both regular JSON and XZ compressed files
pub async fn read_upload_file(file_path: &str) -> Result<Vec<u8>, anyhow::Error> {
    let file_bytes = tokio::fs::read(file_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file_path, e))?;

    // Check if file is XZ compressed by looking at the magic bytes
    if file_path.ends_with(".xz") || is_xz_compressed(&file_bytes) {
        decompress_xz(&file_bytes)
    } else {
        Ok(file_bytes)
    }
}

/// Check if data is XZ compressed by examining magic bytes
pub fn is_xz_compressed(data: &[u8]) -> bool {
    data.len() >= 6 && data[0..6] == [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]
}

/// Decompress XZ compressed data
pub fn decompress_xz(compressed_data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut decoder = XzDecoder::new(compressed_data);
    let mut decompressed = Vec::new();

    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| anyhow::anyhow!("Failed to decompress XZ data: {}", e))?;
    Ok(decompressed)
}

/// Generate a new advisory with modified ID
pub fn generate_advisory_content(base_content: Vec<u8>) -> Result<Vec<u8>, Box<TransactionError>> {
    let mut advisory: serde_json::Value = serde_json::from_slice(&base_content).map_err(|e| {
        Box::new(TransactionError::Custom(format!(
            "Failed to parse advisory JSON: {}",
            e
        )))
    })?;

    // Generate random suffix
    let mut rng = rand::rng();
    let random_suffix: String = (0..6)
        .map(|_| rng.sample(rand::distr::Alphanumeric) as char)
        .collect();

    // Modify the ID field
    if let Some(id) = advisory["document"]["tracking"]["id"].as_str() {
        let new_id = format!("{}-{}", id, random_suffix);
        advisory["document"]["tracking"]["id"] = serde_json::Value::String(new_id);
    }

    // Convert back to bytes
    serde_json::to_vec(&advisory).map_err(|e| {
        Box::new(TransactionError::Custom(format!(
            "Failed to serialize advisory: {}",
            e
        )))
    })
}

/// Generate a new advisory with modified ID asynchronously
pub async fn generate_advisory_content_async(
    base_content: Vec<u8>,
) -> Result<Vec<u8>, Box<TransactionError>> {
    tokio::task::spawn_blocking(move || generate_advisory_content(base_content))
        .await
        .map_err(|e| {
            Box::new(TransactionError::Custom(format!(
                "Blocking task error: {}",
                e
            )))
        })?
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!("", truncate_middle("", 8));
        assert_eq!("0123456", truncate_middle("0123456", 8));
        assert_eq!("01234567", truncate_middle("01234567", 8));
        assert_eq!("012…5678", truncate_middle("012345678", 8),);
        assert_eq!("012…5678", truncate_middle("012345678012345678", 8));
    }
}
