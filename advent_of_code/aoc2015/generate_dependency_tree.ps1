#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Generate a detailed dependency tree for wire 'a'
.DESCRIPTION
    Creates a comprehensive tree visualization showing the complete
    dependency chain for computing wire 'a' in the AoC Day 7 circuit.
.PARAMETER MaxDepth
    Maximum depth to traverse (default: 10)
.PARAMETER SaveToFile
    Save the tree to a file
.EXAMPLE
    .\generate_dependency_tree.ps1
    .\generate_dependency_tree.ps1 -MaxDepth 15 -SaveToFile
#>

param(
    [int]$MaxDepth = 10,
    [switch]$SaveToFile = $false
)

Write-Host "🌳 Generating Dependency Tree for Wire 'a'" -ForegroundColor Cyan
Write-Host "=" * 50 -ForegroundColor Gray

# Load gate definitions
$inputFile = "inputs\day07_example.txt"
$allGates = Get-Content $inputFile

# Parse gate definitions into lookup table
$gateTable = @{}
foreach ($gateDef in $allGates) {
    if ($gateDef -match "^(.+) -> (\w+)$") {
        $expression = $matches[1]
        $wire = $matches[2]
        
        # Parse the expression
        $gateType = ""
        $inputs = @()
        
        if ($expression -match "^(\w+)$") {
            # Direct assignment: x -> wire or 123 -> wire
            $gateType = if ($matches[1] -match "^\d+$") { "CONST" } else { "ASSIGN" }
            $inputs = @($matches[1])
        }
        elseif ($expression -match "^(\w+) AND (\w+)$") {
            $gateType = "AND"
            $inputs = @($matches[1], $matches[2])
        }
        elseif ($expression -match "^(\w+) OR (\w+)$") {
            $gateType = "OR" 
            $inputs = @($matches[1], $matches[2])
        }
        elseif ($expression -match "^(\w+) LSHIFT (\d+)$") {
            $gateType = "LSHIFT"
            $inputs = @($matches[1], $matches[2])
        }
        elseif ($expression -match "^(\w+) RSHIFT (\d+)$") {
            $gateType = "RSHIFT"
            $inputs = @($matches[1], $matches[2])
        }
        elseif ($expression -match "^NOT (\w+)$") {
            $gateType = "NOT"
            $inputs = @($matches[1])
        }
        
        $gateTable[$wire] = @{
            Type = $gateType
            Inputs = $inputs
            Definition = $gateDef
        }
    }
}

Write-Host "📊 Loaded $($gateTable.Keys.Count) gate definitions" -ForegroundColor Yellow

# Get evaluated wires from debug output
$debugFile = "day07_debug_output_2025-09-30_19-33-06.txt"
$evalLines = Select-String "Evaluating wire '(\w+)'" $debugFile | Where-Object { $_.LineNumber -lt 118940 }
$evaluatedWires = $evalLines | ForEach-Object { 
    if ($_.Line -match "Evaluating wire '(\w+)'") { $matches[1] } 
} | Sort-Object -Unique

Write-Host "⚡ Found $($evaluatedWires.Count) evaluated wires for 'a'" -ForegroundColor Green

# Generate tree output
$treeOutput = @()

function New-TreeNode {
    param($wireName, $level = 0)
    
    if ($level -gt $MaxDepth) {
        return
    }
    
    $indent = "  " * $level
    $gate = $gateTable[$wireName]
    
    if ($gate -and $evaluatedWires -contains $wireName) {
        $color = switch ($gate.Type) {
            "AND" { "🔴" }
            "OR" { "🔵" } 
            "NOT" { "🟡" }
            "LSHIFT" { "🟢" }
            "RSHIFT" { "🟣" }
            "ASSIGN" { "⚪" }
            "CONST" { "🔶" }
            default { "⚫" }
        }
        
        $node = "$indent$color $wireName [$($gate.Type)]"
        $treeOutput += $node
        Write-Host $node -ForegroundColor White
        
        # Add definition as comment
        $comment = "$indent   // $($gate.Definition)"
        $treeOutput += $comment
        Write-Host $comment -ForegroundColor Gray
        
        # Recursively process inputs
        foreach ($inputWire in $gate.Inputs) {
            if ($inputWire -match "^\w+$" -and $inputWire -notmatch "^\d+$") {
                # Only follow wire names, not constants
                New-TreeNode $inputWire ($level + 1)
            } else {
                # Show constants as leaf nodes
                $constNode = "$indent  🔶 $inputWire [CONST]"
                $treeOutput += $constNode
                Write-Host $constNode -ForegroundColor Cyan
            }
        }
    } elseif ($gate) {
        # Gate exists but wasn't evaluated
        $unusedNode = "$indent❌ $wireName [$($gate.Type)] (unused)"
        $treeOutput += $unusedNode
        Write-Host $unusedNode -ForegroundColor Red
    } else {
        # No gate definition found
        $missingNode = "$indent❓ $wireName (undefined)"
        $treeOutput += $missingNode
        Write-Host $missingNode -ForegroundColor Magenta
    }
}

Write-Host "`n🌳 Dependency Tree (Max Depth: $MaxDepth):" -ForegroundColor Magenta
Write-Host "=" * 80 -ForegroundColor Gray
Write-Host ""

# Generate the tree starting from 'a'
New-TreeNode "a"

Write-Host ""
Write-Host "🔤 Legend:" -ForegroundColor Cyan
Write-Host "  🔴 AND gate    🔵 OR gate     🟡 NOT gate" -ForegroundColor White
Write-Host "  🟢 LSHIFT     🟣 RSHIFT     ⚪ ASSIGN" -ForegroundColor White  
Write-Host "  🔶 CONST      ❌ Unused     ❓ Undefined" -ForegroundColor White

if ($SaveToFile) {
    $timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
    $outputFile = "dependency_tree_wire_a_$timestamp.txt"
    
    $header = @(
        "# Dependency Tree for Wire 'a' - Generated $(Get-Date)",
        "# Maximum Depth: $MaxDepth",
        "# Total Gates: $($gateTable.Keys.Count)",
        "# Evaluated Gates: $($evaluatedWires.Count)",
        "",
        "Legend:",
        "  🔴 AND gate    🔵 OR gate     🟡 NOT gate",
        "  🟢 LSHIFT     🟣 RSHIFT     ⚪ ASSIGN", 
        "  🔶 CONST      ❌ Unused     ❓ Undefined",
        "",
        "Dependency Tree:"
    )
    
    ($header + $treeOutput) | Out-File -FilePath $outputFile -Encoding UTF8
    Write-Host "`n💾 Tree saved to: $outputFile" -ForegroundColor Green
}

Write-Host "`n🎉 Tree generation complete!" -ForegroundColor Green