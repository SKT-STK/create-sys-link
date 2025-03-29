#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
use std::{env, path::Path, process::Command};
use winreg::{enums::{HKEY_CLASSES_ROOT, KEY_ALL_ACCESS}, RegKey};

fn main() {
  if env::args().len() == 1 {
    install_app();
    return;
  }

  make_link();
}

fn make_link() {
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

fn install_app() {
  let mut exe_path = env::current_exe().unwrap();
  exe_path.pop();

  let background_key = RegKey::predef(HKEY_CLASSES_ROOT)
    .open_subkey_with_flags(r#"Directory\Background\shell"#, KEY_ALL_ACCESS).unwrap();

  let create_link_background = background_key.create_subkey("create-sys-link").unwrap().0;
  create_link_background.set_value("", &"Create System Link".to_string()).unwrap();

  let command_background = create_link_background.create_subkey("command").unwrap().0;
  let exe_path_str = exe_path.to_string_lossy().replace('/', "\\");
  command_background.set_value(
    "", 
    &format!("\"{}\\create-sys-link.exe\" \"%V\"", exe_path_str)
  ).unwrap();

  let dir_key = RegKey::predef(HKEY_CLASSES_ROOT)
    .open_subkey_with_flags(r#"Directory\shell"#, KEY_ALL_ACCESS).unwrap();

  let create_link_dir = dir_key.create_subkey("create-sys-link").unwrap().0;
  create_link_dir.set_value("", &"Create System Link".to_string()).unwrap();

  let command_dir = create_link_dir.create_subkey("command").unwrap().0;
  command_dir.set_value(
    "", 
    &format!("\"{}\\create-sys-link.exe\" \"%V\"", exe_path_str)
  ).unwrap();

  println!("Registry entries created successfully!");
}
