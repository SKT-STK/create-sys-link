Truly simple Rust project to make it easier to use the SysLinks (directory ones at least) in Windows.

### INSTALL

build the Rust project using "cargo build --release" -> open registry editor -> HKEY_CLASSES_ROOT\Directory\Background\shell -> create new key "create-sys-link" -> in the (Default) value type the name that is to be displayed in the right click context menu -> then create another key inside "create-sys-link" called "command" and in its (Default) value paste: ""YOUR_PATH\target\release\create-sys-link.exe"" (without the most outer quotation marks, and remember to change YOUR_PATH to the actual path which you have downloaded and builded the .zip in (must use backslashes)) -> do the same at HKEY_CLASSES_ROOT\Directory\shell -> go to "YOUR_PATH\target\release\" and make create-sys-link.exe always run as admin (right click -> compatibility -> change settings for all users -> run this program as an administrator)
