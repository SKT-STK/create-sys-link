Truly simple Rust project to make it easier to use the SysLinks (directory ones at least) in Windows.

### INSTALL

build the Rust project using "cargo build --release" -> open registry editor -> HKEY_CLASSES_ROOT\Directory\Background\shell -> create new key "CreateSysLink" -> in the (Default) value type the name that is to be displayed in the right click context menu -> then create another key inside "CreateSysLink" called "Command" and in its (Default) value paste: "powershell Start-Process powershell -Verb RunAs -ArgumentList '-NoProfile -Command "& {YOUR_PATH\create-sys-link\target\release\create-sys-link.exe %V}"'"" (without the most outer quotation marks, and remember to change YOUR_PATH to the actual path which you have downloaded and builded the .zip in (cannot contain ') (must use backslashes))
