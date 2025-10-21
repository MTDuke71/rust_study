# Add Generic Zettelkasten Footers to _github Documentation Files
# This script adds outgoing links to integrate documentation files into the knowledge graph

$files = @(
    "RUST_DOCUMENTATION_STANDARDS.md",
    "RUST_TEST_DOCUMENTATION_STANDARDS.md", 
    "BEFORE_AFTER_COMPARISON.md",
    "COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md",
    "COMPLETION_CHECKLIST.md",
    "DAILY_STUDY_CREATION_GUIDE.md",
    "CREATION_WORKFLOW_CLARIFICATION.md",
    "COMPLETION_REPORT.md",
    "DAILY_STUDY_STRUCTURE_UPDATE.md",
    "DOCUMENTATION_UPDATE_SUMMARY.md",
    "DOCUMENTATION_WORKFLOW_CLARIFICATION_SUMMARY.md",
    "FILE_MIGRATION_SUMMARY.md",
    "NAMING_CONVENTIONS.md",
    "QUICK_REFERENCE_TWO_FILE_STRUCTURE.md",
    "SESSION_FINAL_VERIFICATION.md",
    "QUICK_DECISION_GUIDE.md", 
    "SESSION_SUMMARY.md",
    "WORKFLOW_DOCUMENTATION_INDEX.md",
    "copilot-zettelkasten-tags.md",
    "DOCUMENTATION_INDEX.md",
    "DOCUMENTATION_WORKFLOW_UPDATE.md",
    "ISSUE_TEMPLATE\bug_report.md",
    "MISSION5_CASE_STUDY.md",
    "tutorial.engineer.md"
)

$footer = @"

---

## 🔗 Related Documentation

*This document is part of the comprehensive Rust Study documentation system.*

### **Core Documentation**
- **[[../zettelkasten/Documentation Standards]]** - Complete documentation standards and guidelines
- **[[../zettelkasten/Project Management and Session Reports]]** - Project tracking and session summaries
- **[[../zettelkasten/API Design Patterns]]** - Code interface design principles
- **[[../zettelkasten/Quality Assurance]]** - Testing and quality standards

### **Learning System Integration**
- **[[../zettelkasten/Rust Concepts MOC]]** - Core Rust language concepts
- **[[../zettelkasten/Daily Study MOC]]** - Daily learning progression
- **[[../zettelkasten/Missions Overview]]** - Hands-on project implementations  
- **[[../zettelkasten/V-Cycle Methodology]]** - Requirements-driven development approach

### **Workspace Navigation**
- **[[../zettelkasten/zettel-index]]** - Master index of all zettelkasten notes
- **[[WORKFLOW_DOCUMENTATION_INDEX]]** - Complete workflow documentation index

---

*Tags: #documentation #standards #workflow #rust-study #project-management*
*Part of: [[../zettelkasten/Documentation Standards]] | [[../zettelkasten/Project Management and Session Reports]]*
"@

$githubDir = "d:\repos\rust_study\_github"

foreach ($file in $files) {
    $filePath = Join-Path $githubDir $file
    
    if (Test-Path $filePath) {
        Write-Host "Processing: $file" -ForegroundColor Green
        
        # Read current content
        $content = Get-Content $filePath -Raw
        
        # Check if footer already exists
        if ($content -notlike "*## 🔗 Related Documentation*") {
            # Add footer
            $newContent = $content + $footer
            Set-Content $filePath $newContent -NoNewline
            Write-Host "  ✅ Footer added" -ForegroundColor Cyan
        } else {
            Write-Host "  ⏭️  Footer already exists" -ForegroundColor Yellow
        }
    } else {
        Write-Host "  ❌ File not found: $filePath" -ForegroundColor Red
    }
}

Write-Host "`n🎉 Zettelkasten footer addition complete!" -ForegroundColor Green
Write-Host "All files now have outgoing links to integrate with the knowledge graph." -ForegroundColor Cyan