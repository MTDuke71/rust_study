# Generate Daily Study Redirect Pages
# Creates zettelkasten/daily-study/DayXX.md files that redirect to actual daily study content

$dayMappings = @(
    # Week 1
    @{Day=1; Week=1; Title="Vec and Basic Collections"; Topics="Vector basics, push/pop, iteration"},
    @{Day=2; Week=1; Title="HashMap Fundamentals"; Topics="Key-value storage, hash functions, Entry API"},
    @{Day=3; Week=1; Title="HashSet Operations"; Topics="Set operations, deduplication, membership testing"},
    @{Day=4; Week=1; Title="BTreeMap Sorted Storage"; Topics="Ordered maps, range queries, sorted iteration"},
    @{Day=5; Week=1; Title="Iterator Patterns"; Topics="Lazy evaluation, map/filter/fold, chaining"},
    @{Day=6; Week=1; Title="Error Handling Basics"; Topics="Result, Option, ? operator, error propagation"},
    @{Day=7; Week=1; Title="Pattern Matching Deep Dive"; Topics="Match expressions, destructuring, guards"},
    
    # Week 2
    @{Day=8; Week=2; Title="Vector Advanced Patterns"; Topics="Capacity management, slicing, zero-copy"},
    @{Day=9; Week=2; Title="HashMap Deep Dive"; Topics="Entry API, custom hashers, performance tuning"},
    @{Day=10; Week=2; Title="HashSet Advanced Operations"; Topics="Set algebra, batch operations, custom equality"},
    @{Day=11; Week=2; Title="BTreeMap Range Operations"; Topics="Range queries, split operations, ordered traversal"},
    @{Day=12; Week=2; Title="Iterator Combinators"; Topics="chain, zip, enumerate, scan"},
    @{Day=13; Week=2; Title="Iterator Consumers"; Topics="collect, fold, for_each, partition"},
    @{Day=14; Week=2; Title="Error Handling with Collections"; Topics="Result<Vec<T>>, error propagation in pipelines"},
    
    # Week 3
    @{Day=15; Week=3; Title="Trait Basics"; Topics="Defining traits, implementations, trait bounds"},
    @{Day=16; Week=3; Title="Generic Types"; Topics="Type parameters, monomorphization, generic structs"},
    @{Day=17; Week=3; Title="Lifetimes Introduction"; Topics="Lifetime annotations, borrow checker, elision"},
    @{Day=18; Week=3; Title="Advanced Traits"; Topics="Associated types, trait inheritance, marker traits"},
    @{Day=19; Week=3; Title="Trait Objects and Dynamic Dispatch"; Topics="dyn Trait, vtables, object safety"},
    @{Day=20; Week=3; Title="Advanced Generics"; Topics="Where clauses, HRTB, conditional implementations"},
    @{Day=21; Week=3; Title="Lifetime Patterns"; Topics="Multiple lifetimes, 'static, lifetime bounds"},
    
    # Week 4
    @{Day=22; Week=4; Title="2D Grid Fundamentals"; Topics="Grid representation, coordinate systems, indexing"},
    @{Day=23; Week=4; Title="Direction and Navigation"; Topics="Cardinal directions, movement vectors, rotation"},
    @{Day=24; Week=4; Title="Flood Fill Algorithms"; Topics="Recursive/iterative flood fill, connected components"},
    @{Day=25; Week=4; Title="Queue Applications"; Topics="BFS, level-order traversal, queue patterns"},
    @{Day=26; Week=4; Title="Pathfinding Basics"; Topics="Dijkstra, A*, graph search"},
    @{Day=27; Week=4; Title="Binary Search Applications"; Topics="Search variants, boundary conditions"},
    @{Day=28; Week=4; Title="Problem Solving Patterns"; Topics="AoC strategies, parsing, optimization"}
)

$zettelDir = "d:\repos\rust_study\zettelkasten\daily-study"

# Create directory if it doesn't exist
if (!(Test-Path $zettelDir)) {
    New-Item -ItemType Directory -Path $zettelDir -Force | Out-Null
    Write-Host "Created directory: $zettelDir" -ForegroundColor Green
}

foreach ($day in $dayMappings) {
    $dayNum = $day.Day
    $weekNum = $day.Week
    $title = $day.Title
    $topics = $day.Topics
    
    $dayStr = "Day{0:D2}" -f $dayNum
    $filePath = Join-Path $zettelDir "$dayStr.md"
    $weekPath = "rust_learning_week{0}_notes" -f $weekNum
    
    # Determine navigation
    $prevDay = if ($dayNum -gt 1) { "Day{0:D2}" -f ($dayNum - 1) } else { "Week 1 Overview" }
    $nextDay = if ($dayNum -lt 28) { "Day{0:D2}" -f ($dayNum + 1) } else { "Week 5 Preview" }
    
    # Determine related concepts based on topic
    $relatedNotes = ""
    switch -Wildcard ($title) {
        "*HashMap*" { $relatedNotes = @"
- [[hashmap-internals]] - Hash table implementation
- [[mission-5]] - HashMap from scratch
"@ }
        "*HashSet*" { $relatedNotes = @"
- [[hashset-operations]] - Set operations deep dive
- [[mission-5]] - HashMap and HashSet implementation
"@ }
        "*Grid*" { $relatedNotes = @"
- [[find-all-components]] - Component detection
- [[mission-6]] - Grid algorithms and pathfinding
"@ }
        "*Flood Fill*" { $relatedNotes = @"
- [[find-all-components]] - Component detection algorithm
- [[flood-fill]] - Flood fill implementations
- [[explore-component]] - DFS helper function
- [[4-connectivity]] - Grid neighbor patterns
"@ }
        "*Trait*" { $relatedNotes = @"
- [[trait-bounds]] - Generic type constraints
- [[trait-objects]] - Dynamic dispatch patterns
"@ }
        "*Lifetime*" { $relatedNotes = @"
- [[lifetime-annotations]] - Lifetime syntax
- [[rust-book-ch10]] - Generics, traits, lifetimes chapter
"@ }
        "*Iterator*" { $relatedNotes = @"
- [[iterator-patterns]] - Iterator combinators
- [[rust-book-ch13]] - Iterators and closures
"@ }
        "*Error*" { $relatedNotes = @"
- [[result-patterns]] - Result type usage
- [[error-propagation]] - The ? operator
- [[rust-book-ch9]] - Error handling chapter
"@ }
        default { $relatedNotes = "- [[zettel-index]] - Main knowledge hub" }
    }
    
    $content = @"
# $dayStr - $title

> **Redirect Page**: This is a navigation helper. See full content in the actual daily study file.

---

## 📍 **Location**

**Full Content**: [rust_learning_week${weekNum}_notes/$dayStr.md](../../daily_study/$weekPath/$dayStr.md)

**Direct Link**: [[../../daily_study/$weekPath/$dayStr]]

---

## 🎯 **Quick Summary**

**Topics**: $topics

**Week**: Week $weekNum

**Related Notes:**
$relatedNotes

---

## 🔗 **Navigation**

**Previous**: [[$prevDay]]
**Next**: [[$nextDay]]

**Week Overview**: [[../../daily_study/$weekPath/README|Week $weekNum Overview]]

**Main Index**: [[zettel-index]]

---

*Tags: #daily-study #week$weekNum #redirect*

*This redirect enables clean ``[[daily-study/$dayStr]]`` links throughout the zettelkasten*
"@

    # Write the file
    Set-Content -Path $filePath -Value $content -Encoding UTF8
    Write-Host "Created: $dayStr.md → week${weekNum}_notes/$dayStr.md" -ForegroundColor Cyan
}

Write-Host "`n✅ All redirect pages created successfully!" -ForegroundColor Green
Write-Host "Location: $zettelDir" -ForegroundColor Yellow
Write-Host "Total files: $($dayMappings.Count)" -ForegroundColor Yellow
