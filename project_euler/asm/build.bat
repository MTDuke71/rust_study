@echo off
setlocal

set ML=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x86\ml.exe
set LINK=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe
set MSVCLIB=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\lib\x86
set SDKLIB=C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x86

cd /d "%~dp0"

echo === Assembling ===
"%ML%" /c /Fo p018_brute.obj p018_brute.asm
if errorlevel 1 goto :fail

echo === Linking ===
"%LINK%" /subsystem:console /machine:x86 /out:p018_brute.exe p018_brute.obj /LIBPATH:"%MSVCLIB%" /LIBPATH:"%SDKLIB%" kernel32.lib
if errorlevel 1 goto :fail

echo.
echo === Running ===
powershell -NoProfile -Command "$sw=[Diagnostics.Stopwatch]::StartNew(); .\p018_brute.exe; $sw.Stop(); Write-Host ('Wall time: {0:N3} ms' -f $sw.Elapsed.TotalMilliseconds)"
goto :done

:fail
echo *** Build failed ***

:done
endlocal
