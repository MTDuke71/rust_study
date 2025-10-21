# Fix Zettelkasten Footer Links - Use Absolute Paths
# Update the footer format to use paths that Obsidian will recognize as outgoing links

$files = @(
    "BEFORE_AFTER_COMPARISON.md",
    "COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md", 
    "tutorial.engineer.md"
)

$newFooter = @"

---

## 🔗 Related Documentation

*This document is part of the comprehensive Rust Study documentation system.*

### **Core Documentation**
- **[[zettelkasten/Documentation Standards]]** - Complete documentation standards and guidelines
- **[[zettelkasten/Project Management and Session Reports]]** - Project tracking and session summaries
- **[[zettelkasten/API Design Patterns]]** - Code interface design principles
- **[[zettelkasten/Quality Assurance]]** - Testing and quality standards

### **Learning System Integration**
- **[[zettelkasten/Rust Concepts MOC]]** - Core Rust language concepts
- **[[zettelkasten/Daily Study MOC]]** - Daily learning progression
- **[[zettelkasten/Missions Overview]]** - Hands-on project implementations  
- **[[zettelkasten/V-Cycle Methodology]]** - Requirements-driven development approach

### **Workspace Navigation**
- **[[zettelkasten/zettel-index]]** - Master index of all zettelkasten notes
- **[[WORKFLOW_DOCUMENTATION_INDEX]]** - Complete workflow documentation index

---

*Tags: #documentation #standards #workflow #rust-study #project-management*
*Part of: [[zettelkasten/Documentation Standards]] | [[zettelkasten/Project Management and Session Reports]]*
"@

$githubDir = "d:\repos\rust_study\_github"

foreach ($file in $files) {
    $filePath = Join-Path $githubDir $file
    
    if (Test-Path $filePath) {
        Write-Host "Processing: $file" -ForegroundColor Green
        
        # Read current content
        $content = Get-Content $filePath -Raw
        
        # Remove existing footer if present
        if ($content -like "*## 🔗 Related Documentation*") {
            # Find the position of the footer and remove it
            $footerStart = $content.IndexOf("## 🔗 Related Documentation")
            if ($footerStart -ge 0) {
                $content = $content.Substring(0, $footerStart).TrimEnd()
            }
        }
        
        # Add new footer
        $newContent = $content + $newFooter
        Set-Content $filePath $newContent -NoNewline
        Write-Host "  ✅ Footer updated with absolute paths" -ForegroundColor Cyan
    } else {
        Write-Host "  ❌ File not found: $filePath" -ForegroundColor Red
    }
}

Write-Host "`n🎉 Footer link fixes complete!" -ForegroundColor Green
Write-Host "Updated to use absolute paths for better Obsidian link recognition." -ForegroundColor Cyan
"@