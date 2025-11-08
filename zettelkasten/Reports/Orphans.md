# 📊 Orphaned Files Report

> **Navigation**: [[zettel-index]] | [[Collections MOC]] | [[Missions Overview]]

This dashboard shows files that need integration into the knowledge graph.

---

## 🚨 True Orphans (No Links At All)

Files with **no incoming or outgoing links** - completely disconnected from the knowledge graph:

```dataview
TABLE 
  file.folder as "📁 Location",
  file.size as "📏 Size",
  dateformat(file.mtime, "yyyy-MM-dd HH:mm") as "📅 Last Modified"
WHERE length(file.inlinks) = 0 AND length(file.outlinks) = 0
AND !contains(file.path, ".obsidian")
AND !contains(file.path, "target")
AND !contains(file.path, ".git")
AND !contains(file.path, "advent_of_code/aoc2024/2024py")
AND file.name != "Orphans"
SORT file.mtime DESC
LIMIT 100
```

---

## 🔗 No Incoming Links (May Need More References)

Files that link out but have **no incoming links** - they reference other notes but aren't referenced themselves:

```dataview
TABLE 
  length(file.outlinks) as "🔗 Outgoing",
  file.folder as "📁 Location",
  dateformat(file.mtime, "yyyy-MM-dd") as "📅 Modified"
WHERE length(file.inlinks) = 0
AND length(file.outlinks) > 0
AND !contains(file.path, ".obsidian")
AND !contains(file.path, "target")
AND !contains(file.path, ".git")
AND !contains(file.path, "advent_of_code/aoc2024/2024py")
AND file.name != "Orphans"
SORT length(file.outlinks) DESC
LIMIT 75
```

---

## 📝 No Outgoing Links (May Need More Context)

Files with incoming links but **no outgoing links** - they're referenced but don't reference anything:

```dataview
TABLE 
  length(file.inlinks) as "🔗 Incoming",
  file.folder as "📁 Location",
  dateformat(file.mtime, "yyyy-MM-dd") as "📅 Modified"
WHERE length(file.outlinks) = 0
AND length(file.inlinks) > 0
AND !contains(file.path, ".obsidian")
AND !contains(file.path, "target")
AND !contains(file.path, ".git")
AND !contains(file.path, "advent_of_code/aoc2024/2024py")
AND file.name != "Orphans"
SORT length(file.inlinks) DESC
LIMIT 50
```

---

## 📊 Statistics

```dataview
TABLE WITHOUT ID
  choice(length(file.inlinks) = 0 AND length(file.outlinks) = 0, "✅", "") as "True Orphan",
  choice(length(file.inlinks) = 0 AND length(file.outlinks) > 0, "✅", "") as "No Incoming",
  choice(length(file.outlinks) = 0 AND length(file.inlinks) > 0, "✅", "") as "No Outgoing",
  choice(length(file.inlinks) > 0 AND length(file.outlinks) > 0, "✅", "") as "Well Connected"
FROM ""
WHERE !contains(file.path, ".obsidian")
AND !contains(file.path, "target")
AND !contains(file.path, ".git")
AND !contains(file.path, "advent_of_code/aoc2024/2024py")
GROUP BY 
  choice(length(file.inlinks) = 0 AND length(file.outlinks) = 0, "True Orphan",
  choice(length(file.inlinks) = 0 AND length(file.outlinks) > 0, "No Incoming",
  choice(length(file.outlinks) = 0 AND length(file.inlinks) > 0, "No Outgoing", "Well Connected")))
```

---

## 🎯 Integration Priority

**High Priority** (True Orphans in important directories):
- Tutorials that aren't linked
- Mission documentation missing references
- Learning notes without connections

**Medium Priority** (No incoming links):
- Concept notes that should be referenced from MOCs
- Examples that should be linked from tutorials
- Documentation that needs discoverability

**Low Priority** (No outgoing links):
- Terminal nodes (FAQ answers, definitions)
- Standalone reference materials
- Completed standalone exercises

---

## 🛠️ How to Use This Dashboard

1. **Review True Orphans first** - These need immediate integration
2. **Check "No Incoming"** - Add references in MOCs or related notes
3. **Optionally check "No Outgoing"** - Add context links if valuable
4. **Refresh** - This updates automatically as you add links!

---

*Tags: #meta #dashboard #knowledge-management #graph-analysis #orphans*

*Links: [[zettel-index]] | [[Collections MOC]] | [[Missions Overview]]*
