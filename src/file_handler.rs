use std::fs;
use sha2::{Sha256, Digest};
use log::{info, warn};
use crate::database::Database;
use anyhow::{Context, Result}; // Import anyhow Result

pub fn process_file(file_path: &str, db: &Database) -> Result<()> {
    // Read file content
    let content = fs::read(file_path).context("Failed to read file")?;
    
    // Calculate SHA256 hash
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = format!("{:x}", hasher.finalize());
    
    info!("File hash: {}", hash);
    
    // Check if file already exists in database
    if let Some(existing_path) = db.get_file_path(&hash)? {
        warn!("File already processed. Output: {}", existing_path);
        return Ok(());
    }
    
    // Rename and store file
    let new_path = format!("processed/{}.bin", hash);
    fs::create_dir_all("processed").context("Failed to create processed directory")?;
    fs::rename(file_path, &new_path).context("Failed to rename file")?;
    
    // Save to database
    db.save_file(&hash, &new_path)
        .context("Failed to save file information in database")?;
    
    info!("File saved to {}", new_path);
    Ok(())
}
