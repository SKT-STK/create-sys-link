Truly simple Rust project to make it easier to use the SysLinks (directory ones at least) in Windows.

###

powershell Start-Process powershell -Verb RunAs -ArgumentList '-NoProfile -Command "& {F:\programming-projects\rust\create-sys-link\target\release\create-sys-link.exe %V}"'"

HKEY_CLASSES_ROOT\Directory\Background\shell\CreateSysLink -> Default: Create Sys Link
\Command -> Default: the command above
