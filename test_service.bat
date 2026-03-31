@echo off
echo Test service is running...
echo Press Ctrl+C to stop.
:loop
timeout /t 10 /nobreak >nul
echo [%date% %time%] Service is still running...
goto loop
