use tauri::Manager;

#[tauri::command]
pub async fn show(window: tauri::Window) {
  window.get_window("main").unwrap().show().unwrap();
}

use futures_util::StreamExt;
use reqwest::header::USER_AGENT;
use std::fs::File;
use std::io::Write;

#[tauri::command]
pub async fn download(url: String, path: String) -> Result<(), String> {
  let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36";
  let client = reqwest::Client::new();
  let response = client
    .get(&url)
    .header(USER_AGENT, user_agent)
    .send()
    .await
    .or(Err(format!("Failed to GET from '{}'", &url)))?;

  let source_size = response
    .content_length()
    .ok_or(format!("Failed to get content length from '{}'", &url))?;

  let mut download_size: u64 = 0;
  let mut stream = response.bytes_stream();
  let mut file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;

  while let Some(item) = stream.next().await {
    let chunk = item.or(Err(format!("Error while downloading file")))?;
    download_size += chunk.len() as u64;
    file
      .write(&chunk)
      .or(Err(format!("Error while writing to file")))?;
  }

  if download_size == 0 || download_size < source_size {
    Err(format!("Failed to download file from '{}'", &url))
  } else {
    Ok(())
  }
}
