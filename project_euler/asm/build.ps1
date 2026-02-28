# build.ps1 — assemble and link p018_brute.asm
$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot

$ml   = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x86\ml.exe'
$link = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe'
$msvcLib = 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207\lib\x86'
$sdkLib  = 'C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x86'

Write-Host '=== Assembling ==='
& $ml /c /Fo p018_brute.obj p018_brute.asm
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host '=== Linking ==='
& $link /subsystem:console /machine:x86 /out:p018_brute.exe `
    p018_brute.obj `
    "/LIBPATH:$msvcLib" "/LIBPATH:$sdkLib" `
    kernel32.lib
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host ''
Write-Host '=== Running ==='
$sw = [Diagnostics.Stopwatch]::StartNew()
& .\p018_brute.exe
$sw.Stop()
Write-Host ("Wall time (incl. startup): {0:N3} ms" -f $sw.Elapsed.TotalMilliseconds)
