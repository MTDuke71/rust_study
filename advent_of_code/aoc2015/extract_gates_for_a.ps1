#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Extract and display all gates leading to wire 'a' in Day 7 AoC circuit
.DESCRIPTION
    This script analyzes the debug output to extract all the logic gates
    and their connections that contribute to computing wire 'a'.
.PARAMETER OutputFormat
    Format for output: 'list', 'tree', 'graph', or 'verilog'
.PARAMETER SaveToFile
    Save the gate analysis to a file
.PARAMETER ShowStats
    Show statistics about the gate network
.EXAMPLE
    .\extract_gates_for_a.ps1
    .\extract_gates_for_a.ps1 -OutputFormat "tree"
    .\extract_gates_for_a.ps1 -SaveToFile -ShowStats
#>

param(
    [string]$OutputFormat = "list",
    [switch]$SaveToFile = $false,
    [switch]$ShowStats = $false
)

Write-Host "🔧 Gate Extraction Tool for Wire 'a'" -ForegroundColor Cyan
Write-Host "=" * 50 -ForegroundColor Gray

# Load input file to get all gate definitions
$inputFile = "inputs\day07_example.txt"
if (-not (Test-Path $inputFile)) {
    Write-Host "❌ Input file not found: $inputFile" -ForegroundColor Red
    exit 1
}

$allGates = Get-Content $inputFile
Write-Host "📁 Loaded $($allGates.Count) total gate definitions" -ForegroundColor Yellow

# Extract evaluated wires from debug output  
$debugFile = "day07_debug_output_2025-09-30_19-33-06.txt"
if (-not (Test-Path $debugFile)) {
    Write-Host "❌ Debug file not found: $debugFile" -ForegroundColor Red
    exit 1
}

Write-Host "🔍 Analyzing debug output for wire evaluation..." -ForegroundColor Green

# Get all wires that were evaluated for 'a' (Part 1 only)
$evalLines = Select-String "Evaluating wire '(\w+)'" $debugFile | Where-Object { $_.LineNumber -lt 118940 }
$evaluatedWires = $evalLines | ForEach-Object { 
    if ($_.Line -match "Evaluating wire '(\w+)'") { $matches[1] } 
} | Sort-Object -Unique

Write-Host "⚡ Found $($evaluatedWires.Count) wires evaluated for 'a'" -ForegroundColor Cyan

# Extract only the gates for evaluated wires
$relevantGates = @()
foreach ($wire in $evaluatedWires) {
    $gateDefinition = $allGates | Where-Object { $_ -match "-> $wire$" }
    if ($gateDefinition) {
        $relevantGates += [PSCustomObject]@{
            Wire = $wire
            Definition = $gateDefinition
            Inputs = @()
            GateType = ""
        }
    }
}

# Parse gate types and inputs
foreach ($gate in $relevantGates) {
    $def = $gate.Definition
    
    if ($def -match "^(\w+) -> ") {
        # Direct assignment: x -> wire
        $gate.GateType = "ASSIGN"
        $gate.Inputs = @($matches[1])
    }
    elseif ($def -match "^(\w+) AND (\w+) -> ") {
        $gate.GateType = "AND" 
        $gate.Inputs = @($matches[1], $matches[2])
    }
    elseif ($def -match "^(\w+) OR (\w+) -> ") {
        $gate.GateType = "OR"
        $gate.Inputs = @($matches[1], $matches[2])
    }
    elseif ($def -match "^(\w+) LSHIFT (\d+) -> ") {
        $gate.GateType = "LSHIFT"
        $gate.Inputs = @($matches[1], $matches[2])
    }
    elseif ($def -match "^(\w+) RSHIFT (\d+) -> ") {
        $gate.GateType = "RSHIFT" 
        $gate.Inputs = @($matches[1], $matches[2])
    }
    elseif ($def -match "^NOT (\w+) -> ") {
        $gate.GateType = "NOT"
        $gate.Inputs = @($matches[1])
    }
    elseif ($def -match "^(\d+) -> ") {
        $gate.GateType = "CONST"
        $gate.Inputs = @($matches[1])
    }
    else {
        $gate.GateType = "UNKNOWN"
        $gate.Inputs = @()
    }
}

# Display results based on format
switch ($OutputFormat.ToLower()) {
    "list" {
        Write-Host "`n📋 Gate List Format:" -ForegroundColor Magenta
        Write-Host "=" * 60 -ForegroundColor Gray
        
        $relevantGates | Sort-Object Wire | ForEach-Object {
            $inputStr = $_.Inputs -join ", "
            Write-Host "🔗 $($_.Wire) = $($_.GateType)($inputStr)" -ForegroundColor White
            Write-Host "   Definition: $($_.Definition)" -ForegroundColor Gray
            Write-Host ""
        }
    }
    
    "tree" {
        Write-Host "`n🌳 Tree Format (first 50 gates):" -ForegroundColor Magenta
        Write-Host "=" * 60 -ForegroundColor Gray
        
        # Show tree starting from 'a'
        function Show-GateTree {
            param($wireName, $level = 0)
            
            $indent = "  " * $level
            $gate = $relevantGates | Where-Object { $_.Wire -eq $wireName }
            
            if ($gate) {
                $color = switch ($gate.GateType) {
                    "AND" { "Green" }
                    "OR" { "Blue" } 
                    "NOT" { "Red" }
                    "LSHIFT" { "Yellow" }
                    "RSHIFT" { "Yellow" }
                    "ASSIGN" { "White" }
                    "CONST" { "Cyan" }
                    default { "Gray" }
                }
                
                Write-Host "$indent$($gate.Wire) [$($gate.GateType)]" -ForegroundColor $color
                
                # Recursively show inputs (limit depth to prevent overflow)
                if ($level -lt 5) {
                    foreach ($gateInput in $gate.Inputs) {
                        if ($gateInput -match "^\w+$") {  # Only follow wire names, not constants
                            Show-GateTree $gateInput ($level + 1)
                        } else {
                            Write-Host "$indent  $gateInput [CONST]" -ForegroundColor Cyan
                        }
                    }
                }
            }
        }
        
        Show-GateTree "a"
    }
    
    "verilog" {
        Write-Host "`n⚡ Verilog-style Format:" -ForegroundColor Magenta  
        Write-Host "=" * 60 -ForegroundColor Gray
        
        Write-Host "// Generated gate netlist for wire 'a'" -ForegroundColor Green
        Write-Host "module day7_circuit_for_a();" -ForegroundColor Yellow
        Write-Host ""
        
        $relevantGates | Sort-Object Wire | ForEach-Object {
            $wire = $_.Wire
            $type = $_.GateType
            $inputs = $_.Inputs
            
            switch ($type) {
                "AND" { Write-Host "  assign $wire = $($inputs[0]) & $($inputs[1]); // $($_.Definition)" -ForegroundColor White }
                "OR"  { Write-Host "  assign $wire = $($inputs[0]) | $($inputs[1]); // $($_.Definition)" -ForegroundColor White }
                "NOT" { Write-Host "  assign $wire = ~$($inputs[0]); // $($_.Definition)" -ForegroundColor White }
                "LSHIFT" { Write-Host "  assign $wire = $($inputs[0]) << $($inputs[1]); // $($_.Definition)" -ForegroundColor White }
                "RSHIFT" { Write-Host "  assign $wire = $($inputs[0]) >> $($inputs[1]); // $($_.Definition)" -ForegroundColor White }
                "ASSIGN" { Write-Host "  assign $wire = $($inputs[0]); // $($_.Definition)" -ForegroundColor White }
                "CONST" { Write-Host "  assign $wire = $($inputs[0]); // $($_.Definition)" -ForegroundColor White }
                default { Write-Host "  // UNKNOWN: $($_.Definition)" -ForegroundColor Red }
            }
        }
        
        Write-Host ""
        Write-Host "endmodule" -ForegroundColor Yellow
    }
    
    default {
        Write-Host "❌ Unknown output format: $OutputFormat" -ForegroundColor Red
        Write-Host "Available: list, tree, verilog" -ForegroundColor Yellow
        exit 1
    }
}

# Show statistics if requested
if ($ShowStats) {
    Write-Host "`n📊 Gate Statistics:" -ForegroundColor Magenta
    Write-Host "=" * 40 -ForegroundColor Gray
    
    $gateTypeCounts = $relevantGates | Group-Object GateType | Sort-Object Count -Descending
    
    Write-Host "Total gates for wire 'a': $($relevantGates.Count)" -ForegroundColor Cyan
    Write-Host ""
    
    foreach ($group in $gateTypeCounts) {
        $percentage = [math]::Round(($group.Count / $relevantGates.Count) * 100, 1)
        Write-Host "  $($group.Name): $($group.Count) gates ($percentage%)" -ForegroundColor Yellow
    }
    
    Write-Host ""
    Write-Host "Wire 'a' dependency depth: $(($evaluatedWires | Measure-Object -Maximum | Select-Object -ExpandProperty Maximum).Length) character max" -ForegroundColor Green
    Write-Host "Evaluation efficiency: $($relevantGates.Count)/$($allGates.Count) gates used ($([math]::Round(($relevantGates.Count / $allGates.Count) * 100, 1))%)" -ForegroundColor Green
}

# Save to file if requested
if ($SaveToFile) {
    $timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
    $outputFile = "gates_for_wire_a_$timestamp.txt"
    
    $output = @()
    $output += "# Gates Leading to Wire 'a' - Generated $(Get-Date)"
    $output += "# Total gates: $($relevantGates.Count)"
    $output += ""
    
    $relevantGates | Sort-Object Wire | ForEach-Object {
        $output += "$($_.Wire) = $($_.GateType)($($_.Inputs -join ', '))"
        $output += "  // $($_.Definition)"
        $output += ""
    }
    
    $output | Out-File -FilePath $outputFile -Encoding UTF8
    Write-Host "`n💾 Gate list saved to: $outputFile" -ForegroundColor Green
}

Write-Host "`n🎉 Gate extraction complete!" -ForegroundColor Green