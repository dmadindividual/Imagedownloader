use error_chain::error_chain;
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;
use dirs;
use reqwest;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set the target URL for the image
    let target = "https://pbs.twimg.com/media/GR0A0c5WgAES9E9?format=jpg&name=small";
    
    // Make an HTTP GET request to the target URL
    let response = reqwest::get(target).await?;

    // Extract the file name from the URL
    let fname = response.url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .map(|name| format!("{}.jpg", name)) // Ensure the file name has .jpg extension
        .unwrap_or("downloaded_image.jpg".to_string());

    // Print the file name to be downloaded
    println!("File to download: '{}'", fname);

    // Get the path to the Downloads directory
    let mut path = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(fname);

    // Print the file location
    println!("File will be located: '{}'", path.display());

    // Create the destination file in the Downloads directory
    let mut dest = File::create(&path)?;

    // Get the content of the response as bytes
    let content = response.bytes().await?;
    
    // Write the content to the destination file
    dest.write_all(&content)?;

    // Print a completion message
    println!("Download and save complete.");

    Ok(())
}
