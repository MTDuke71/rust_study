# Rust Study Dashboard

**Tags:** #dashboard #learning-hub #overview #progress-tracking #zettelkasten-system  
**Created:** 2025-10-22  
**Related:** [[Daily Study MOC]], [[Missions Overview]], [[Progress Tracking]], [[Quality Assurance]], [[Rust Concepts MOC]], [[3-Track Integration]]

*Your central hub for Rust learning and development*

---

## 📚 Learning Tracks

### Core Rust Book
```dataview
LIST 
FROM "zettelkasten" 
WHERE contains(tags, "rust-book") AND contains(tags, "chapter-overview")
SORT file.name ASC
```

### Daily Study Progression  
```dataview
LIST
FROM "daily_study" 
WHERE contains(tags, "daily-study")
SORT file.name ASC
LIMIT 10
```

### Mission Projects
```dataview
TABLE tags, file.mtime as "Last Modified"
FROM "missions" OR "tutorials"
WHERE contains(tags, "mission") OR contains(file.path, "Mission")
SORT file.mtime DESC
LIMIT 8
```

---

## 🗺️ Knowledge Maps (MOCs)

| Area | Link | Description |
|------|------|-------------|
| **All Concepts** | [[Rust Concepts MOC]] | Complete concept overview |
| **Collections** | [[Collections MOC]] | Data structures and containers |
| **AoC Patterns** | [[AoC Patterns MOC]] | Algorithm patterns from Advent of Code |
| **Missions** | [[Missions Overview]] | Project-based learning track |
| **3-Track System** | [[3-Track Integration]] | Integrated learning approach |

---

## 🔗 Quick Links

### Recent Activity
```dataview  
LIST file.mtime as "Modified"
FROM "zettelkasten"
SORT file.mtime DESC
LIMIT 5
```

### Most Linked Notes
```dataview
TABLE length(file.inlinks) as "Backlinks"
FROM "zettelkasten" 
SORT length(file.inlinks) DESC
LIMIT 10
```

---

## 📊 Study Statistics

### By Category
- **Concepts:** `= length(filter(file.tags, (t) => contains(t, "concept")))`
- **Daily Notes:** `= length(filter(file.tags, (t) => contains(t, "daily-study")))`  
- **Mission Projects:** `= length(filter(file.tags, (t) => contains(t, "mission")))`
- **Chapter Overviews:** `= length(filter(file.tags, (t) => contains(t, "chapter-overview")))`

### Current Streak
- **Days studied:** [Track manually or use plugin]
- **Last study session:** `= date(now)`

---

## 🎮 Adventures in Code

### Advent of Code Progress
```dataview
TABLE tags, file.mtime as "Last Updated"
FROM "advent_of_code"
WHERE contains(file.name, "day") OR contains(file.name, "Day")
SORT file.mtime DESC
LIMIT 5
```

### Advanced Examples  
```dataview
LIST
FROM "advanced_examples"
SORT file.mtime DESC
LIMIT 5
```

---

## 🛠️ Development Tools

### Recent Scripts & Utilities
- [[Recent Activity Report]] - Track file changes
- [[Clippy Automation]] - Code quality workflows
- [[Quality Pipeline]] - Comprehensive analysis

### Build & Test Status
- **Last Clippy Run:** [Check GitHub Actions]
- **Last Quality Report:** [[Quality Report]]
- **Coverage Status:** [View latest report]

---

## 📝 Quick Capture

### Today's Learning Log
- **Key Insight:** 
- **Challenge:** 
- **Next Step:** 

### Ideas & TODOs
- [ ] 
- [ ] 
- [ ] 

---

## 🔍 Search & Discovery

### Recently Modified
```dataview
LIST file.mtime
FROM ""
WHERE file.mtime > date(today) - dur(3 days)
SORT file.mtime DESC
LIMIT 8
```

### Orphaned Notes (Needs Linking)
```dataview
LIST
FROM "zettelkasten"
WHERE length(file.inlinks) = 0 AND length(file.outlinks) = 0
LIMIT 5
```

---

## 🎨 Visual Knowledge Map

![[Graph View of Learning Network]]

**Tags to Explore:**
- #rust #collections #data-structures #algorithms
- #daily-study #mission #chapter-overview
- #aoc #competitive #patterns

---

*Last updated: `= date(now)`*