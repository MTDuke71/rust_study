# Mission 9 - Pathfinding Algorithms Runner
# PowerShell version with parameter support

param(
    [Parameter(Position=0)]
    [ValidateSet("graph", "grid", "demo", "perf", "bench", "help", "examples")]
    [string]$Mode = "",
    
    [Parameter(Position=1, ValueFromRemainingArguments=$true)]
    [string[]]$Arguments = @()
)

function Show-Menu {
    Clear-Host
    Write-Host "================================================================================" -ForegroundColor Cyan
    Write-Host "                        Mission 9 - Pathfinding Algorithms" -ForegroundColor Yellow
    Write-Host "================================================================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\run_mission9.ps1 [mode] [arguments...]" -ForegroundColor Green
    Write-Host ""
    Write-Host "Available Modes:" -ForegroundColor White
    Write-Host "  graph       - Graph-based CLI (mission9)" -ForegroundColor Gray
    Write-Host "  grid        - Grid-based CLI (mission9-grid)" -ForegroundColor Gray
    Write-Host "  demo        - Simple demonstration" -ForegroundColor Gray
    Write-Host "  perf        - Performance analysis demo" -ForegroundColor Gray
    Write-Host "  bench       - Run benchmarks" -ForegroundColor Gray
    Write-Host "  examples    - Show example commands" -ForegroundColor Gray
    Write-Host "  help        - Show detailed help" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\run_mission9.ps1 graph find-path -f graph.json -s 0 -g 5 --algorithm astar" -ForegroundColor Cyan
    Write-Host "  .\run_mission9.ps1 grid solve --width 20 --height 15 --start 0,0 --goal 19,14" -ForegroundColor Cyan
    Write-Host "  .\run_mission9.ps1 demo" -ForegroundColor Cyan
    Write-Host "  .\run_mission9.ps1 bench" -ForegroundColor Cyan
    Write-Host ""
}

function Show-Examples {
    Write-Host "================================================================================" -ForegroundColor Cyan
    Write-Host "                              Example Commands" -ForegroundColor Yellow
    Write-Host "================================================================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "Graph CLI Examples:" -ForegroundColor Yellow
    Write-Host "  Generate and solve:" -ForegroundColor White
    Write-Host "    cargo run --bin mission9 -- generate --graph-type grid --nodes 100 --file test.json" -ForegroundColor Gray
    Write-Host "    cargo run --bin mission9 -- find-path -f test.json -s 0 -g 99 --algorithm astar --verbose" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  Batch processing:" -ForegroundColor White
    Write-Host "    cargo run --bin mission9 -- batch -f graph.json --queries queries.csv --output results.csv" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  Benchmarking:" -ForegroundColor White
    Write-Host "    cargo run --bin mission9 -- benchmark -f graph.json -s 0 -g 50 --iterations 1000" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "Grid CLI Examples:" -ForegroundColor Yellow
    Write-Host "  Basic pathfinding:" -ForegroundColor White
    Write-Host "    cargo run --bin mission9-grid -- find-path --width 15 --height 15 --start 0,0 --goal 14,14" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  With obstacles:" -ForegroundColor White
    Write-Host "    cargo run --bin mission9-grid -- find-path --width 20 --height 15 --start 0,0 --goal 19,14 --algorithm bidirectional-astar" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  Generate maze:" -ForegroundColor White
    Write-Host "    cargo run --bin mission9-grid -- benchmark --width 30 --height 20 --start 0,0 --goal 29,19 --iterations 100" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "Performance Analysis:" -ForegroundColor Yellow
    Write-Host "  cargo run --example simple_demo" -ForegroundColor Gray
    Write-Host "  cargo run --example step4_performance_demo" -ForegroundColor Gray
    Write-Host "  cargo bench" -ForegroundColor Gray
    Write-Host ""
}

function Invoke-GraphCLI {
    param([string[]]$CommandArgs)
    
    Write-Host "Running Graph-based CLI..." -ForegroundColor Green
    if ($CommandArgs.Count -eq 0) {
        cargo run --bin mission9 -- --help
    } else {
        $command = "cargo run --bin mission9 -- " + ($CommandArgs -join " ")
        Write-Host "Executing: $command" -ForegroundColor Cyan
        Invoke-Expression $command
    }
}

function Invoke-GridCLI {
    param([string[]]$CommandArgs)
    
    Write-Host "Running Grid-based CLI..." -ForegroundColor Green
    if ($CommandArgs.Count -eq 0) {
        cargo run --bin mission9-grid -- --help
    } else {
        $command = "cargo run --bin mission9-grid -- " + ($CommandArgs -join " ")
        Write-Host "Executing: $command" -ForegroundColor Cyan
        Invoke-Expression $command
    }
}

function Start-Demo {
    Write-Host "Running Simple Demo..." -ForegroundColor Green
    cargo run --example simple_demo
}

function Start-Performance {
    Write-Host "Running Performance Demo..." -ForegroundColor Green
    cargo run --example step4_performance_demo
}

function Start-Benchmarks {
    Write-Host "Running Benchmarks..." -ForegroundColor Green
    cargo bench
}

function Show-Help {
    Write-Host "================================================================================" -ForegroundColor Cyan
    Write-Host "                              Detailed Help" -ForegroundColor Yellow
    Write-Host "================================================================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "Graph CLI Help:" -ForegroundColor Yellow
    cargo run --bin mission9 -- --help
    Write-Host ""
    
    Write-Host "Grid CLI Help:" -ForegroundColor Yellow
    cargo run --bin mission9-grid -- --help
    Write-Host ""
}

# Main execution logic
switch ($Mode.ToLower()) {
    "graph" { Invoke-GraphCLI -CommandArgs $Arguments }
    "grid" { Invoke-GridCLI -CommandArgs $Arguments }
    "demo" { Start-Demo }
    "perf" { Start-Performance }
    "bench" { Start-Benchmarks }
    "examples" { Show-Examples }
    "help" { Show-Help }
    default { Show-Menu }
}

if ($Mode -eq "") {
    Write-Host "Press any key to continue..."
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}