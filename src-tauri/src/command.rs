use tauri::Manager;

#[tauri::command]
pub async fn show(window: tauri::Window) {
  window.get_window("main").unwrap().show().unwrap();
}

use std::path::Path;

#[tauri::command]
pub async fn exists(path: String) -> bool {
  Path::new(&path).exists()
}

use crypto::digest::Digest;
use crypto::md5::Md5;
use futures_util::StreamExt;
use reqwest::header::USER_AGENT;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[tauri::command]
pub async fn download(
  url: String,
  folder: String,
  name_type: i32,
  name: String,
  ext: String,
) -> Result<(), String> {
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
  let mut md5 = Md5::new();

  let id = Uuid::new_v4();
  let path = format!("{}/{}", &folder, id);
  let mut file = File::create(&path).or(Err(format!("Failed to create file '{}'", path)))?;

  while let Some(item) = stream.next().await {
    let chunk = item.or(Err(format!("Error while downloading file")))?;
    download_size += chunk.len() as u64;

    if name_type == 4 {
      md5.input(&chunk);
    }

    file
      .write(&chunk)
      .or(Err(format!("Error while writing to file")))?;
  }

  if download_size == 0 || download_size < source_size {
    Err(format!("Failed to download file from '{}'", &url))
  } else {
    let path2 = match name_type {
      4 => format!("{}/{}{}", folder, md5.result_str(), ext),
      _ => format!("{}/{}{}", folder, name, ext),
    };

    fs::rename(&path, &path2).or(Err(format!("Failed to rename '{}'", path2)))?;

    Ok(())
  }
}

use sysinfo::{DiskExt, System, SystemExt};

#[tauri::command]
pub async fn disk_free_size(path: String) -> u64 {
  let sys = System::new_all();
  for disk in sys.disks() {
    if Path::new(&path).starts_with(disk.mount_point()) {
      return disk.available_space();
    }
  }
  return 0;
}
