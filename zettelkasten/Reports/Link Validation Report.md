# 🔗 Link Validation Report

*Automated link health monitoring for the zettelkasten*

---

## 📊 Overall Link Statistics

```dataview
TABLE WITHOUT ID
  length(file.outlinks) as "Total Outgoing Links",
  length(filter(file.outlinks, (l) => l)) as "Valid Links"
FROM ""
WHERE file.name = "zettel-index"
```

---

## ❌ Broken Links in zettel-index

**Note:** Links without existing target files are shown below.

```dataview
TABLE WITHOUT ID
  link.path as "Link Text",
  choice(meta(link).path, "✅ Exists", "❌ MISSING FILE") as "Status",
  choice(meta(link).path, meta(link).path, "No file found") as "Target Path"
FROM "zettelkasten"
WHERE file.name = "zettel-index"
FLATTEN file.outlinks as link
SORT choice(meta(link).path, 1, 0) ASC
```

### Alternative: List Only Broken Links

```dataview
LIST WITHOUT ID link.path + " → ❌ FILE MISSING"
FROM "zettelkasten"  
WHERE file.name = "zettel-index"
FLATTEN file.outlinks as link
WHERE !meta(link).path
```

---

## 🔍 All Outgoing Links from zettel-index

```dataview
TABLE
  file.outlinks as "Links"
FROM ""
WHERE file.name = "zettel-index"
```

---

## 📋 Detailed Link Analysis

```dataview
TABLE WITHOUT ID
  link as "Link Target",
  choice(meta(link).path, "✅ Valid", "❌ Broken") as "Status"
FROM "zettelkasten"
WHERE file.name = "zettel-index"
FLATTEN file.outlinks as link
```

---

## 🔗 All Files with Broken Links

```dataview
TABLE 
  length(file.outlinks) as "Total Links",
  filter(file.outlinks, (l) => !meta(l).path) as "Broken Links"
FROM "zettelkasten"
WHERE length(filter(file.outlinks, (l) => !meta(l).path)) > 0
```

---

## 📊 Workspace-Wide Link Health

```dataview
TABLE WITHOUT ID
  "Total Files" as Metric,
  length(file.name) as Count
FROM ""
GROUP BY true

UNION

TABLE WITHOUT ID
  "Total Links" as Metric,
  length(rows.file.outlinks) as Count
FROM ""
GROUP BY true

UNION

TABLE WITHOUT ID
  "Orphaned Files (No incoming links)" as Metric,
  length(rows.file) as Count
FROM ""
WHERE length(file.inlinks) = 0
GROUP BY true
```

---

## 🏷️ Files by Link Count (Top 10)

```dataview
TABLE 
  length(file.outlinks) as "Outgoing Links",
  length(file.inlinks) as "Incoming Links"
FROM "zettelkasten"
SORT length(file.outlinks) DESC
LIMIT 10
```

---

## 🔴 Orphaned Pages (No Incoming Links)

```dataview
TABLE
  file.ctime as "Created",
  length(file.outlinks) as "Outgoing Links"
FROM "zettelkasten"
WHERE length(file.inlinks) = 0 AND file.name != "zettel-index"
SORT file.ctime DESC
```

---

## 📈 Link Network Statistics

```dataview
TABLE WITHOUT ID
  choice(length(file.inlinks) > 5, "🔥 Hub", 
    choice(length(file.inlinks) > 2, "📍 Connected",
      choice(length(file.inlinks) = 0, "🏝️ Orphan", "🔗 Linked"))) as "Type",
  file.name as "File",
  length(file.inlinks) as "In",
  length(file.outlinks) as "Out"
FROM "zettelkasten"
SORT length(file.inlinks) DESC
```

---

## 🛠️ How to Use This Report

1. **Check Overall Statistics** - See total link counts for zettel-index
2. **Find Broken Links** - The "Broken Links" section shows links pointing to non-existent files
3. **Review Orphans** - Pages with no incoming links might need integration
4. **Monitor Network Health** - See which pages are central hubs vs isolated

---

## 🔧 Quick Fixes

### **For Broken Links:**
1. Click on the broken link reference
2. Either:
   - Create the missing page
   - Update the link to point to correct file
   - Remove the link if no longer needed

### **For Orphaned Pages:**
1. Review the orphaned page content
2. Add relevant links from other pages
3. Or add to zettel-index if it's an important page

---

*Last Updated: Auto-updates on each view (Dataview live queries)*

---

*Tags: #maintenance #validation #dataview #links #health-check*
*Links: [[zettel-index]] | [[Collections MOC]] | [[Missions MOC]]*
