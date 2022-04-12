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
  // println!("download: url: {} | path: {}", url, path);

  let client = reqwest::Client::new();

  let res = client
    .get(&url)
    .header(USER_AGENT,"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36")
    .send()
    .await
    .or(Err(format!("Failed to GET from '{}'", &url)))?;

  // let total_size = res
  //   .content_length()
  //   .ok_or(format!("Failed to get content length from '{}'", &url))?;

  // println!("total_size: {}", total_size);

  let mut stream = res.bytes_stream();

  let mut file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;

  // let mut downloaded: u64 = 0;

  while let Some(item) = stream.next().await {
    let chunk = item.or(Err(format!("Error while downloading file")))?;
    // println!("Chunk: size = {}", chunk.len());

    // downloaded += chunk.len() as u64;

    file
      .write(&chunk)
      .or(Err(format!("Error while writing to file")))?;
  }

  // println!("downloaded: {}", downloaded);

  Ok(())
}
