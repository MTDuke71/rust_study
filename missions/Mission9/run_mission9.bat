@echo off
setlocal enabledelayedexpansion

:MAIN_MENU
cls
echo.
echo ================================================================================
echo                        Mission 9 - Pathfinding Algorithms
echo ================================================================================
echo.
echo Available Options:
echo.
echo [1] Graph-based CLI (mission9)
echo [2] Grid-based CLI (mission9-grid) 
echo [3] Simple Demo (example)
echo [4] Performance Demo (example)
echo [5] Run Benchmarks
echo [6] Show Help for Graph CLI
echo [7] Show Help for Grid CLI
echo [8] Quick Examples Menu
echo [0] Exit
echo.
set /p choice="Enter your choice (0-8): "

if "%choice%"=="1" goto GRAPH_CLI
if "%choice%"=="2" goto GRID_CLI
if "%choice%"=="3" goto SIMPLE_DEMO
if "%choice%"=="4" goto PERFORMANCE_DEMO
if "%choice%"=="5" goto BENCHMARKS
if "%choice%"=="6" goto GRAPH_HELP
if "%choice%"=="7" goto GRID_HELP
if "%choice%"=="8" goto EXAMPLES_MENU
if "%choice%"=="0" goto EXIT
goto MAIN_MENU

:GRAPH_CLI
cls
echo Running Graph-based CLI...
echo.
echo Available subcommands:
echo - find-path: Find path between two nodes
echo - batch: Process multiple path queries
echo - info: Show graph information
echo - benchmark: Performance benchmarking
echo - generate: Generate test graphs
echo.
echo Examples:
echo   mission9 find-path -f graph.json -s 0 -g 5 --algorithm astar
echo   mission9 batch graph.json queries.csv --output results.csv
echo   mission9 generate grid 100 --file test.json
echo.
set /p subcmd="Enter subcommand (or 'menu' to return): "
if "%subcmd%"=="menu" goto MAIN_MENU

cargo run --bin mission9 -- %subcmd%
echo.
pause
goto MAIN_MENU

:GRID_CLI
cls
echo Running Grid-based CLI...
echo.
echo Available subcommands:
echo - solve: Solve pathfinding on a grid
echo - generate: Generate grid scenarios
echo - visualize: Create visual output
echo.
echo Examples:
echo   mission9-grid solve --width 20 --height 15 --start 0,0 --goal 19,14
echo   mission9-grid generate maze --width 30 --height 20 --output maze.txt
echo.
set /p subcmd="Enter subcommand and options (or 'menu' to return): "
if "%subcmd%"=="menu" goto MAIN_MENU

cargo run --bin mission9-grid -- %subcmd%
echo.
pause
goto MAIN_MENU

:SIMPLE_DEMO
cls
echo Running Simple Demo...
echo.
cargo run --example simple_demo
echo.
pause
goto MAIN_MENU

:PERFORMANCE_DEMO
cls
echo Running Performance Demo...
echo.
cargo run --example step4_performance_demo
echo.
pause
goto MAIN_MENU

:BENCHMARKS
cls
echo Running Benchmarks...
echo.
cargo bench
echo.
pause
goto MAIN_MENU

:GRAPH_HELP
cls
echo Graph CLI Help:
echo.
cargo run --bin mission9 -- --help
echo.
pause
goto MAIN_MENU

:GRID_HELP
cls
echo Grid CLI Help:
echo.
cargo run --bin mission9-grid -- --help
echo.
pause
goto MAIN_MENU

:EXAMPLES_MENU
cls
echo.
echo ================================================================================
echo                              Quick Examples
echo ================================================================================
echo.
echo [1] Generate test graph and find path
echo [2] Grid pathfinding with obstacles
echo [3] Batch processing example
echo [4] Algorithm comparison
echo [5] Performance benchmark
echo [6] Visualization example
echo [0] Back to main menu
echo.
set /p example="Enter example choice (0-6): "

if "%example%"=="1" goto EX_GENERATE_FIND
if "%example%"=="2" goto EX_GRID_OBSTACLES
if "%example%"=="3" goto EX_BATCH
if "%example%"=="4" goto EX_ALGORITHM_COMPARE
if "%example%"=="5" goto EX_PERFORMANCE
if "%example%"=="6" goto EX_VISUALIZATION
if "%example%"=="0" goto MAIN_MENU
goto EXAMPLES_MENU

:EX_GENERATE_FIND
cls
echo Example 1: Generate test graph and find path
echo.
echo Step 1: Generating a 10x10 grid graph...
cargo run --bin mission9 -- generate --graph-type grid --nodes 100 --file example_grid.json --format json
echo.
echo Step 2: Finding path from node 0 to node 99 using A*...
cargo run --bin mission9 -- find-path -f example_grid.json -s 0 -g 99 --algorithm astar --heuristic euclidean --verbose
echo.
echo Step 3: Finding same path using Dijkstra for comparison...
cargo run --bin mission9 -- find-path -f example_grid.json -s 0 -g 99 --algorithm dijkstra --verbose
echo.
pause
goto EXAMPLES_MENU

:EX_GRID_OBSTACLES
cls
echo Example 2: Grid pathfinding with obstacles
echo.
echo Solving pathfinding on a 15x15 grid with maze obstacles...
cargo run --bin mission9-grid -- find-path --width 15 --height 15 --start 1,1 --goal 13,13 --algorithm bidirectional-astar
echo.
pause
goto EXAMPLES_MENU

:EX_BATCH
cls
echo Example 3: Batch processing example
echo.
echo Step 1: Generating test graph...
cargo run --bin mission9 -- generate --graph-type random --nodes 50 --file batch_test.json --format json
echo.
echo Step 2: Creating query file...
echo 0,49> batch_queries.csv
echo 5,45>> batch_queries.csv
echo 10,40>> batch_queries.csv
echo 15,35>> batch_queries.csv
echo.
echo Step 3: Processing batch queries...
cargo run --bin mission9 -- batch -f batch_test.json --queries batch_queries.csv --output batch_results.csv --algorithm astar --verbose
echo.
echo Results saved to batch_results.csv
pause
goto EXAMPLES_MENU

:EX_ALGORITHM_COMPARE
cls
echo Example 4: Algorithm comparison
echo.
echo Comparing Dijkstra vs A* performance...
echo.
echo Generating 20x20 grid for testing...
cargo run --bin mission9 -- generate --graph-type grid --nodes 400 --file compare_test.json --format json
echo.
echo Running benchmark comparing both algorithms...
cargo run --bin mission9 -- benchmark -f compare_test.json -s 0 -g 399 --iterations 100
echo.
pause
goto EXAMPLES_MENU

:EX_PERFORMANCE
cls
echo Example 5: Performance benchmark
echo.
echo Running comprehensive performance analysis...
cargo run --example step4_performance_demo
echo.
pause
goto EXAMPLES_MENU

:EX_VISUALIZATION
cls
echo Example 6: Visualization example
echo.
echo Creating visualized pathfinding solution...
echo Generating small grid and finding path with visualization...
cargo run --bin mission9 -- generate --graph-type grid --nodes 25 --file viz_test.json --format json
echo.
echo Finding path and saving visualization...
cargo run --bin mission9 -- find-path -f viz_test.json -s 0 -g 24 --algorithm astar -z pathfinding_visual.dot --verbose
echo.
echo DOT visualization saved to pathfinding_visual.dot
echo You can render it with: dot -Tpng pathfinding_visual.dot -o output.png
pause
goto EXAMPLES_MENU

:EXIT
echo.
echo Thank you for using Mission 9 Pathfinding Algorithms!
echo.
pause
exit /b 0

:ERROR
echo.
echo An error occurred. Please check your input and try again.
echo.
pause
goto MAIN_MENU