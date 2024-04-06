#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
use std::env;
use std::process::Command;

fn main() {
  let args: Vec<String> = env::args().collect();

  let base_path = &args[1];
  let new_sys_link = format!("{}\\New Sys Link", base_path);

  let target_folder = tfd::select_folder_dialog("Select a Folder", "");

  match target_folder {
    Some(folder_path) => {
      let mklink_args = vec![
        "/d".to_string(),
        new_sys_link,
        folder_path,
      ];

      println!("{:?}", mklink_args);

      let _ = Command::new("cmd")
        .arg("/c")
        .arg("mklink")
        .args(&mklink_args)
        .output()
        .unwrap();

    }
    None => return,
  }
}
