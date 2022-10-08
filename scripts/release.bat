@cargo build --release
@if %errorlevel% neq 0 exit /b %errorlevel%

@copy /b /y target\i686-pc-windows-msvc\release\dinput.dll ..\..\
@copy /b /y ..\..\shina.ini ..\..\shina.ini.bak
@copy /b /y assets\shina.toml ..\..\shina.ini
@echo Assets copied to game folder.
