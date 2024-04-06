@echo off
setlocal

:: Define the path to your application's executable
set "appPath=F:\programming-projects\rust\create-sys-link\target\release\create-sys-link.exe"

:: Define the name of your application as it will appear in the context menu
set "appName=Create Sys Link"

:: Define the registry key for the context menu
set "regKey=HKEY_CLASSES_ROOT\Directory\Background\shell\CreateSysLink"

:: Define the registry key for the command
set "commandKey=%regKey%\command"

:: Create the registry key for the context menu
reg add "%regKey%" /ve /d "%appName%" /f

:: Create the registry key for the command
reg add "%commandKey%" /ve /d "\"%appPath%\" \"%%V\"" /f

echo Done.
pause
