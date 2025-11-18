# 🔗 Broken Links Report

This page tracks all **non-existent pages** (broken wiki links) in the zettelkasten, showing which concepts are referenced but not yet created.

> **💡 Tip**: Pages listed here are frequently referenced concepts that should be created to complete the knowledge network!

---


## 🚨 Most Referenced Missing Pages

**Priority: Create these concepts first!**

```dataview
TABLE WITHOUT ID
	link as "Missing Page",
	length(rows.file.link) as "Referenced By Count",
	rows.file.link as "Referencing Files"
FROM "zettelkasten" OR "missions" OR "advanced_examples" OR "advent_of_code" OR "rust_book" OR "daily_study" OR "tutorials"
FLATTEN file.outlinks as link
WHERE !link.file AND link != [[]]
GROUP BY link
SORT length(rows.file.link) DESC
LIMIT 50
```

---

## 📋 All Broken Links by Frequency

**Complete list of all missing concepts:**

```dataview
TABLE WITHOUT ID
	link as "Missing Page",
	length(rows.file.link) as "Count"
FROM "zettelkasten" OR "missions" OR "advanced_examples" OR "advent_of_code" OR "rust_book" OR "daily_study" OR "tutorials"
FLATTEN file.outlinks as link
WHERE !link.file AND link != [[]]
GROUP BY link
SORT length(rows.file.link) DESC
```

---

## 🗂️ Broken Links by Source Directory

### Mission Projects
```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	filter(file.outlinks, (l) => !l.file) as "Broken Links",
	length(filter(file.outlinks, (l) => !l.file)) as "Count"
FROM "missions"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
```

### Zettelkasten Core
```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	filter(file.outlinks, (l) => !l.file) as "Broken Links",
	length(filter(file.outlinks, (l) => !l.file)) as "Count"
FROM "zettelkasten"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
```

### Advanced Examples (Brackets)
```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	filter(file.outlinks, (l) => !l.file) as "Broken Links",
	length(filter(file.outlinks, (l) => !l.file)) as "Count"
FROM "advanced_examples"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
```

### Advent of Code
```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	filter(file.outlinks, (l) => !l.file) as "Broken Links",
	length(filter(file.outlinks, (l) => !l.file)) as "Count"
FROM "advent_of_code"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
```

### Rust Book
```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	filter(file.outlinks, (l) => !l.file) as "Broken Links",
	length(filter(file.outlinks, (l) => !l.file)) as "Count"
FROM "rust_book"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
```

### Daily Study Notes
```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	filter(file.outlinks, (l) => !l.file) as "Broken Links",
	length(filter(file.outlinks, (l) => !l.file)) as "Count"
FROM "daily_study"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
```

### Tutorials
```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	filter(file.outlinks, (l) => !l.file) as "Broken Links",
	length(filter(file.outlinks, (l) => !l.file)) as "Count"
FROM "tutorials"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
```

---

## 📈 Files with Most Broken Links

**Files that need the most attention:**

```dataview
TABLE WITHOUT ID
	file.link as "Source File",
	length(filter(file.outlinks, (l) => !l.file)) as "Broken Link Count",
	round((length(filter(file.outlinks, (l) => !l.file)) / length(file.outlinks)) * 100, 1) + "%" as "Broken %"
FROM "zettelkasten" OR "missions" OR "advanced_examples" OR "advent_of_code" OR "rust_book" OR "daily_study" OR "tutorials"
WHERE length(filter(file.outlinks, (l) => !l.file)) > 0
SORT length(filter(file.outlinks, (l) => !l.file)) DESC
LIMIT 20
```

---

## 🎯 Action Items

### High Priority (5+ references)
Create these concept pages first - they're referenced frequently across multiple files.

### Medium Priority (2-4 references)
These concepts appear in multiple contexts and would strengthen knowledge connections.

### Low Priority (1 reference)
Single-reference links - consider whether they should be:
- Created as standalone concepts
- Merged into related existing pages
- Removed if they were linking mistakes

---

## 🔧 Maintenance Tips

**How to use this report:**

1. **Identify High-Priority Missing Pages**: Check "Most Referenced Missing Pages" section
2. **Create Concept Pages**: Use the MOC templates to create missing concepts
3. **Verify Link Format**: Ensure links use `[[Page Name]]` format (no `.md` extension)
4. **Check Capitalization**: Obsidian links are case-sensitive
5. **Regular Review**: Update this report weekly to track progress

**Creating missing pages:**
```markdown
# [Concept Name]

Brief description of the concept...

## Key Points

- Point 1
- Point 2

## Related Concepts

- [[Related Concept 1]]
- [[Related Concept 2]]

## Examples

...

---

*Links: [[zettel-index]] | [[Relevant MOC]]*

*Tags: #relevant #tags #here*
```

---

## 📊 Progress Tracking

**Goal**: Reduce broken links to < 10% of total links

**How to track progress:**
1. Note the "Total Broken Links" count from Summary Statistics
2. Create high-priority pages each week
3. Re-run queries to see improvement
4. Celebrate when broken link percentage drops!

---

## 🔗 Related Pages

- [[zettel-index]] - Main zettelkasten index
- [[Missions Overview]] - Mission overview
- [[rust-concepts-MOC]] - Core Rust concepts
- [[Daily Study MOC]] - Learning progression
- [[AoC 2015 MOC]] - Advent of Code problems

---

*Tags: #meta #maintenance #broken-links #knowledge-graph #obsidian-queries #dataview*
