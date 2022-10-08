@call scripts\build.bat
@if %errorlevel% neq 0 exit /b %errorlevel%

@echo Starting game...
@start /w /d ..\..\ ..\..\Oni.exe
@if %errorlevel% neq 0 exit /b %errorlevel%
