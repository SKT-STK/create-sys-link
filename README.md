Truly simple Rust project to make it easier to use the SysLinks (directory ones at least) in Windows.

### INSTALL

build the Rust project using "cargo build --release" -> open registry editor -> HKEY_CLASSES_ROOT\Directory\Background\shell -> create new key "create-sys-link" -> in the (Default) value type the name that is to be displayed in the right click context menu -> then create another key inside "create-sys-link" called "command" and in its (Default) value paste: "powershell Start-Process powershell -Verb RunAs -ArgumentList '-ExecutionPolicy Bypass -NoProfile -Command "& {YOUR_PATH\target\release\create-sys-link.exe}"'"" (without the most outer quotation marks, and remember to change YOUR_PATH to the actual path which you have downloaded and builded the .zip in (cannot contain ') (must use backslashes)) -> do the same at HKEY_CLASSES_ROOT\Directory\shell -> reboot your system -> (sometimes might require clicking the right click context menu option 2 times (idk why, i guess just the beauty of win11)) -> (administrative privileges are required because the 'mklink /d' command which is ultimately used to create the link is only available without them if the 'Developer Mode' on windows is on, also for security reasons the link will always be created on the Desktop, you can just cut & paste it wherever you need it to be at)
