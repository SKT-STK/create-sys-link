#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
use std::env;
use std::process::Command;

fn main() {
  let target_folder = tfd::select_folder_dialog("Select a Folder", "");

  match target_folder {
    Some(folder_path) => {
      let dest_file_name: Vec<&str> = folder_path.split("\\").collect();
      let dest_file_name = dest_file_name[dest_file_name.len() - 1];

      let mklink_args = vec![
        "/d".to_string(),
        format!("{}\\..\\..\\..\\Desktop\\{}", env::temp_dir().to_str().unwrap(), dest_file_name),
        folder_path,
      ];

      let _ = Command::new("mklink.exe")
        .args(&mklink_args)
        .output()
        .unwrap();

    }
    None => return,
  }
}
