# 📝 Generate Daily Notes

*Script to automatically create daily notes from MONTHLY_CALENDAR.md*

---

## 🚀 **Automated Daily Note Creation**

### **Method 1: Templater Plugin (Recommended)**

**Setup:**
1. Install Templater plugin in Obsidian
2. Create template with current date
3. Auto-generate daily notes

**Template Code:**
```javascript
<%*
// Get today's date in YYYY-MM-DD format
const today = tp.date.now("YYYY-MM-DD");
const dayName = tp.date.now("dddd");
const monthDay = tp.date.now("MMMM d");

// Read MONTHLY_CALENDAR.md content
const calendar = app.vault.getAbstractFileByPath("MONTHLY_CALENDAR.md");
const content = await app.vault.read(calendar);

// Find today's section
const todaySection = content.match(new RegExp(`### \\*\\*${dayName}, ${monthDay}\\*\\*[\\s\\S]*?(?=###|$)`));
const section = todaySection ? todaySection[0] : "No content found for today";

// Create filename
const filename = `Daily Notes/${today}.md`;

// Template content
const template = `# ${dayName}, ${monthDay}, 2025

${section}

---

*Tags: #daily-note #${today} #${dayName.toLowerCase()}*
*Links: [[MONTHLY_CALENDAR]] | [[Daily Study MOC]] | [[Missions Overview]]*`;

// Create file
await tp.file.create_new(template, filename);
%>
```

### **Method 2: PowerShell Script**

**Create:** `scripts/generate-daily-note.ps1`
```powershell
# Get today's date
$today = Get-Date -Format "yyyy-MM-dd"
$dayName = Get-Date -Format "dddd"
$monthDay = Get-Date -Format "MMMM d"

# Read MONTHLY_CALENDAR.md
$calendar = Get-Content "MONTHLY_CALENDAR.md" -Raw

# Find today's section
$pattern = "### \*\*$dayName, $monthDay\*\*[\s\S]*?(?=###|$)"
$match = [regex]::Match($calendar, $pattern)
$content = if ($match.Success) { $match.Value } else { "No content found for today" }

# Create daily note
$filename = "zettelkasten/Daily Notes/$today.md"
$template = @"
# $dayName, $monthDay, 2025

$content

---

*Tags: #daily-note #$today #$($dayName.ToLower())*
*Links: [[MONTHLY_CALENDAR]] | [[Daily Study MOC]] | [[Missions Overview]]*
"@

$template | Out-File -FilePath $filename -Encoding UTF8
Write-Host "Created daily note: $filename"
```

---

## 🔧 **Setup Instructions**

### **Option 1: Templater Plugin**
1. Install Templater plugin
2. Copy the template code above
3. Create new file with template
4. Set up hotkey for instant daily note creation

### **Option 2: PowerShell Script**
1. Create `scripts/generate-daily-note.ps1`
2. Copy PowerShell code above
3. Run: `.\scripts\generate-daily-note.ps1`
4. Set up Windows Task Scheduler for daily automation

### **Option 3: Manual (One-time setup)**
1. Copy content from MONTHLY_CALENDAR.md
2. Create file: `zettelkasten/Daily Notes/YYYY-MM-DD.md`
3. Format according to template

---

## 📅 **Daily Workflow**

### **Automated Workflow:**
1. **Morning**: Open [[Today's Focus]] (auto-shows today's daily note)
2. **Work**: Follow daily tasks and commands
3. **Evening**: Update session tracking in Today's Focus

### **Manual Workflow:**
1. **Morning**: Run daily note generation script
2. **Open**: `zettelkasten/Daily Notes/YYYY-MM-DD.md`
3. **Work**: Follow daily tasks and commands
4. **Evening**: Update progress tracking

---

## 🎯 **Benefits**

- **Fully Automated**: No manual copying needed
- **Always Current**: Shows today's actual content
- **Consistent Format**: Same structure every day
- **Easy Navigation**: Direct links to relevant files
- **Progress Tracking**: Built-in session tracking

---

*Tags: #automation #daily-notes #templater #scripts*
*Links: [[Today's Focus]] | [[MONTHLY_CALENDAR]] | [[Daily Study MOC]]*
