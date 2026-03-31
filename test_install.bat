@echo off
setlocal enabledelayedexpansion

echo ====================================
echo NSSM Service Install Test Script
echo ====================================
echo.

set /p LOOP_COUNT=Enter number of loops (default 5): 
if "!LOOP_COUNT!"=="" set LOOP_COUNT=5

set /p SLEEP_TIME=Enter sleep time in seconds between loops (default 2): 
if "!SLEEP_TIME!"=="" set SLEEP_TIME=2

set SERVICE_NAME=TestService
set EXE_PATH=C:\Windows\System32\notepad.exe

echo.
echo Starting test with %LOOP_COUNT% loops...
echo Service name: %SERVICE_NAME%
echo Executable: %EXE_PATH%
echo Sleep time: %SLEEP_TIME% seconds
echo.

for /L %%i in (1,1,%LOOP_COUNT%) do (
    echo.
    echo ========================================
    echo Loop %%i of %LOOP_COUNT%
    echo ========================================
    
    echo [%%i/%LOOP_COUNT%] Installing service...
    src-tauri\resources\win64\nssm.exe install %SERVICE_NAME% %EXE_PATH%
    if !errorlevel! neq 0 (
        echo ERROR: Failed to install service
    ) else (
        echo SUCCESS: Service installed
        
        echo [%%i/%LOOP_COUNT%] Starting service...
        src-tauri\resources\win64\nssm.exe start %SERVICE_NAME%
        if !errorlevel! neq 0 (
            echo WARNING: Failed to start service
        ) else (
            echo SUCCESS: Service started
        )
        
        echo [%%i/%LOOP_COUNT%] Waiting %SLEEP_TIME% seconds...
        timeout /t %SLEEP_TIME% /nobreak >nul
        
        echo [%%i/%LOOP_COUNT%] Stopping service...
        src-tauri\resources\win64\nssm.exe stop %SERVICE_NAME%
        if !errorlevel! neq 0 (
            echo WARNING: Failed to stop service
        ) else (
            echo SUCCESS: Service stopped
        )
        
        echo [%%i/%LOOP_COUNT%] Removing service...
        src-tauri\resources\win64\nssm.exe remove %SERVICE_NAME% confirm
        if !errorlevel! neq 0 (
            echo ERROR: Failed to remove service
        ) else (
            echo SUCCESS: Service removed
        )
    )
    
    echo.
    if %%i lss %LOOP_COUNT% (
        echo Waiting 1 second before next loop...
        timeout /t 1 /nobreak >nul
    )
)

echo.
echo ====================================
echo Test completed!
echo ====================================

pause
