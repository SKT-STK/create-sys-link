#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
use std::env;
use std::process::Command;

fn main() {
  let args: Vec<String> = env::args().collect();

  let base_path = &args[1];

  let target_folder = tfd::select_folder_dialog("Select a Folder", "");

  match target_folder {
    Some(folder_path) => {
      let dest_file_name: Vec<&str> = folder_path.split("\\").collect();
      let dest_file_name = dest_file_name[dest_file_name.len() - 1];

      let mklink_args = vec![
        "/d".to_string(),
        format!("{}\\{}", base_path, dest_file_name),
        folder_path,
      ];

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
