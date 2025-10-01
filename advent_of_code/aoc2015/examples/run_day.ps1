# PowerShell script to run AoC days easily
param(
    [Parameter(Mandatory=$true)]
    [int]$Day,
    [string]$InputFile = ""
)

if ($InputFile -eq "") {
    $InputFile = "inputs/day{0:D2}_example.txt" -f $Day
}

Write-Host "Running Day $Day with input: $InputFile" -ForegroundColor Green
cargo run -- $Day $InputFile