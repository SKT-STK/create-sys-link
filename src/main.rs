#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
use std::{env, path::Path, process::Command};

fn main() {
  let target_folder = tfd::select_folder_dialog("Select a Folder", "");

  if target_folder.is_none() { return; }

  let target_folder = target_folder.unwrap();

  let mklink_args = vec![
    "/d".to_string(),
    format!("{}\\{}", env::args().last().unwrap().to_string(), Path::new(&target_folder).components().last().unwrap().as_os_str().to_str().unwrap().to_string()),
    target_folder
  ];

  let _ = Command::new("cmd")
    .arg("/c")
    .arg("mklink")
    .args(&mklink_args)
    .spawn();
}
