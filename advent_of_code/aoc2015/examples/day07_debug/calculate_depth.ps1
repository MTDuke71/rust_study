#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Calculate the maximum dependency depth for wire 'a'
.DESCRIPTION
    Analyzes the complete dependency tree to determine the deepest path
    from wire 'a' to the leaf nodes (constants or inputs).
.PARAMETER ShowPath
    Show the actual path of maximum depth
.PARAMETER Verbose
    Show detailed analysis
.EXAMPLE
    .\calculate_depth.ps1
    .\calculate_depth.ps1 -ShowPath -Verbose
#>

param(
    [switch]$ShowPath = $false,
    [switch]$Verbose = $false
)

Write-Host "🔍 Calculating Maximum Dependency Depth for Wire 'a'" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Gray

# Load gate definitions
$inputFile = "inputs\day07_example.txt"
if (-not (Test-Path $inputFile)) {
    Write-Host "❌ Input file not found: $inputFile" -ForegroundColor Red
    exit 1
}

$allGates = Get-Content $inputFile
Write-Host "📁 Loaded $($allGates.Count) gate definitions" -ForegroundColor Yellow

# Parse gate definitions into lookup table
$gateTable = @{}
foreach ($gateDef in $allGates) {
    if ($gateDef -match "^(.+) -> (\w+)$") {
        $expression = $matches[1]
        $wire = $matches[2]
        
        # Parse the expression to get inputs
        $inputs = @()
        
        if ($expression -match "^(\w+)$") {
            # Direct assignment: x -> wire or 123 -> wire
            $inputs = @($matches[1])
        }
        elseif ($expression -match "^(\w+) AND (\w+)$") {
            $inputs = @($matches[1], $matches[2])
        }
        elseif ($expression -match "^(\w+) OR (\w+)$") {
            $inputs = @($matches[1], $matches[2])
        }
        elseif ($expression -match "^(\w+) LSHIFT (\d+)$") {
            $inputs = @($matches[1])  # Only count wire inputs, not constants
        }
        elseif ($expression -match "^(\w+) RSHIFT (\d+)$") {
            $inputs = @($matches[1])  # Only count wire inputs, not constants
        }
        elseif ($expression -match "^NOT (\w+)$") {
            $inputs = @($matches[1])
        }
        
        $gateTable[$wire] = @{
            Inputs = $inputs
            Definition = $gateDef
        }
    }
}

Write-Host "🔧 Parsed $($gateTable.Keys.Count) gates" -ForegroundColor Green

# Get evaluated wires from debug output to focus only on relevant gates
$debugFile = "day07_debug_output_2025-09-30_19-33-06.txt"
if (Test-Path $debugFile) {
    $evalLines = Select-String "Evaluating wire '(\w+)'" $debugFile | Where-Object { $_.LineNumber -lt 118940 }
    $evaluatedWires = $evalLines | ForEach-Object { 
        if ($_.Line -match "Evaluating wire '(\w+)'") { $matches[1] } 
    } | Sort-Object -Unique
    Write-Host "⚡ Found $($evaluatedWires.Count) evaluated wires" -ForegroundColor Cyan
} else {
    $evaluatedWires = $gateTable.Keys
    Write-Host "⚠️  Debug file not found, analyzing all gates" -ForegroundColor Yellow
}

# Memoization for depth calculation
$depthCache = @{}
$pathCache = @{}

function Get-WireDepth {
    param($wireName, $visitedPath = @())
    
    # Check for cycles
    if ($visitedPath -contains $wireName) {
        if ($Verbose) {
            Write-Host "⚠️  Cycle detected at $wireName in path: $($visitedPath -join ' -> ')" -ForegroundColor Yellow
        }
        return 0
    }
    
    # Check cache
    if ($depthCache.ContainsKey($wireName)) {
        return $depthCache[$wireName]
    }
    
    # If it's a constant or undefined wire, depth is 0
    if ($wireName -match "^\d+$" -or -not $gateTable.ContainsKey($wireName)) {
        $depthCache[$wireName] = 0
        $pathCache[$wireName] = @($wireName)
        return 0
    }
    
    # Skip if wire wasn't evaluated (not part of 'a' computation)
    if ($evaluatedWires -and $evaluatedWires -notcontains $wireName) {
        $depthCache[$wireName] = 0
        $pathCache[$wireName] = @($wireName)
        return 0
    }
    
    $gate = $gateTable[$wireName]
    $maxDepth = 0
    $deepestPath = @($wireName)
    
    # Calculate depth for each input
    foreach ($inputWire in $gate.Inputs) {
        if ($inputWire -match "^\w+$" -and $inputWire -notmatch "^\d+$") {
            # Only follow wire names, not constants
            $newVisitedPath = $visitedPath + $wireName
            $inputDepth = Get-WireDepth $inputWire $newVisitedPath
            
            if ($inputDepth + 1 -gt $maxDepth) {
                $maxDepth = $inputDepth + 1
                if ($pathCache.ContainsKey($inputWire)) {
                    $deepestPath = @($wireName) + $pathCache[$inputWire]
                } else {
                    $deepestPath = @($wireName, $inputWire)
                }
            }
        }
    }
    
    $depthCache[$wireName] = $maxDepth
    $pathCache[$wireName] = $deepestPath
    
    if ($Verbose) {
        Write-Host "🔍 $wireName -> depth: $maxDepth" -ForegroundColor Gray
    }
    
    return $maxDepth
}

# Calculate depth starting from 'a'
Write-Host "`n🎯 Calculating dependency depth..." -ForegroundColor Magenta

$startTime = Get-Date
$totalDepth = Get-WireDepth "a"
$endTime = Get-Date
$duration = ($endTime - $startTime).TotalMilliseconds

Write-Host "`n📊 Results:" -ForegroundColor Green
Write-Host "=" * 40 -ForegroundColor Gray
Write-Host "Maximum dependency depth: $totalDepth levels" -ForegroundColor Yellow
Write-Host "Calculation time: $([math]::Round($duration, 2)) ms" -ForegroundColor Gray
Write-Host "Gates analyzed: $($depthCache.Keys.Count)" -ForegroundColor Gray

if ($ShowPath -and $pathCache.ContainsKey("a")) {
    Write-Host "`n🛤️  Longest dependency path:" -ForegroundColor Magenta
    $path = $pathCache["a"]
    for ($i = 0; $i -lt $path.Length; $i++) {
        $indent = "  " * $i
        $wire = $path[$i]
        
        if ($gateTable.ContainsKey($wire)) {
            $def = $gateTable[$wire].Definition
            Write-Host "$indent$wire -> $def" -ForegroundColor White
        } else {
            Write-Host "$indent$wire (leaf node)" -ForegroundColor Cyan
        }
    }
}

# Show depth distribution
Write-Host "`n📈 Depth Distribution:" -ForegroundColor Blue
$depthGroups = $depthCache.GetEnumerator() | Group-Object Value | Sort-Object Name
foreach ($group in $depthGroups) {
    $percentage = [math]::Round(($group.Count / $depthCache.Keys.Count) * 100, 1)
    Write-Host "  Depth $($group.Name): $($group.Count) wires ($percentage%)" -ForegroundColor White
}

# Find some other deep paths for comparison
Write-Host "`n🏔️  Other Deep Paths (depth >= $([math]::Max($totalDepth - 3, 1))):" -ForegroundColor Magenta
$deepWires = $depthCache.GetEnumerator() | Where-Object { $_.Value -ge ($totalDepth - 3) -and $_.Key -ne "a" } | Sort-Object Value -Descending | Select-Object -First 5

foreach ($entry in $deepWires) {
    $wire = $entry.Key
    $depth = $entry.Value
    if ($gateTable.ContainsKey($wire)) {
        Write-Host "  $wire (depth $depth): $($gateTable[$wire].Definition)" -ForegroundColor Gray
    }
}

Write-Host "`n🎉 Depth analysis complete!" -ForegroundColor Green

# Save results to file
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$outputFile = "depth_analysis_$timestamp.txt"

$output = @(
    "# Dependency Depth Analysis for Wire 'a' - Generated $(Get-Date)",
    "# AoC 2015 Day 7 Circuit Analysis",
    "",
    "## Summary:",
    "Maximum Dependency Depth: $totalDepth levels",
    "Gates Analyzed: $($depthCache.Keys.Count)",
    "Calculation Time: $([math]::Round($duration, 2)) ms",
    "",
    "## Longest Dependency Path:"
)

if ($pathCache.ContainsKey("a")) {
    $path = $pathCache["a"]
    for ($i = 0; $i -lt $path.Length; $i++) {
        $indent = "  " * $i
        $wire = $path[$i]
        if ($gateTable.ContainsKey($wire)) {
            $def = $gateTable[$wire].Definition
            $output += "$indent$wire -> $def"
        } else {
            $output += "$indent$wire (leaf node)"
        }
    }
}

$output += ""
$output += "## Depth Distribution:"
foreach ($group in $depthGroups) {
    $percentage = [math]::Round(($group.Count / $depthCache.Keys.Count) * 100, 1)
    $output += "Depth $($group.Name): $($group.Count) wires ($percentage%)"
}

$output | Out-File -FilePath $outputFile -Encoding UTF8
Write-Host "💾 Analysis saved to: $outputFile" -ForegroundColor Green