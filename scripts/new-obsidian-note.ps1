# Obsidian Integration for Rust Study
# Creates new notes with proper templates and linking

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("Daily", "Chapter", "Concept", "Mission", "MOC")]
    [string]$Type,
    
    [Parameter(Mandatory=$true)]
    [string]$Title,
    
    [string]$Tags = "",
    [string]$RelatedNotes = ""
)

$obsidianPath = ".obsidian"
$zettelPath = "zettelkasten"
$templatePath = "$obsidianPath\templates"

# Ensure zettelkasten directory exists
if (-not (Test-Path $zettelPath)) {
    New-Item -ItemType Directory -Path $zettelPath -Force
}

# Generate filename
$cleanTitle = $Title -replace '[^\w\s-]', '' -replace '\s+', ' '
$fileName = "$cleanTitle.md"
$filePath = Join-Path $zettelPath $fileName

if (Test-Path $filePath) {
    Write-Host "❌ File already exists: $filePath" -ForegroundColor Red
    exit 1
}

# Load appropriate template
$templateFile = switch ($Type) {
    "Daily" { "$templatePath\Daily Study Template.md" }
    "Chapter" { "$templatePath\Chapter Overview Template.md" }  
    "Concept" { "$templatePath\Concept Template.md" }
    "Mission" { "$templatePath\Mission Template.md" }
    "MOC" { "$templatePath\Concept Template.md" }  # Use concept as base for MOCs
}

if (-not (Test-Path $templateFile)) {
    Write-Host "❌ Template not found: $templateFile" -ForegroundColor Red
    exit 1
}

# Read template content
$content = Get-Content $templateFile -Raw

# Replace template variables
$content = $content -replace '\{\{title\}\}', $Title
$content = $content -replace '\{\{date\}\}', (Get-Date -Format "yyyy-MM-dd")

# Add custom tags if provided
if ($Tags) {
    $content = $content -replace '#rust', "#rust #$Tags"
}

# Add related notes if provided  
if ($RelatedNotes) {
    $relatedLinks = ($RelatedNotes -split ',') | ForEach-Object { "- [[$($_.Trim())]]" }
    $content = $content -replace '- \[\[\]\]', ($relatedLinks -join "`n")
}

# Create the file
$content | Out-File -FilePath $filePath -Encoding UTF8

Write-Host "✅ Created $Type note: $filePath" -ForegroundColor Green

# Optional: Open in Obsidian (if installed)
if (Get-Command "obsidian" -ErrorAction SilentlyContinue) {
    Write-Host "🔗 Opening in Obsidian..." -ForegroundColor Cyan
    & obsidian "obsidian://open?path=$($PWD.Path -replace '\\', '/')/$filePath"
} else {
    Write-Host "💡 Tip: Install Obsidian CLI to auto-open files" -ForegroundColor Yellow
}

# Show quick stats
$totalNotes = (Get-ChildItem $zettelPath -Filter "*.md").Count
Write-Host "📊 Total notes in zettelkasten: $totalNotes" -ForegroundColor Cyan