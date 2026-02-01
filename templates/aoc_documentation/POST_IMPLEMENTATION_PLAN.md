# AoC Documentation Plan - Post-Implementation Workflow

**Quick Links**: [← Templates](../README.md) | [AoC Main](../../advent_of_code/README.md) | [Stats Dashboard Template](stats_dashboard_template.md) | [Function Guide Template](function_guide_template.md)

---

Master plan for creating comprehensive documentation after solving AoC problems.

**Note**: This describes the **2-file documentation system** (summary + function guides) introduced in AoC 2022. The old 4-file system (comprehensive guides + dashboard + patterns + algorithms) proved redundant and is deprecated.

---

## 🎯 Goals

1. **Single Source of Truth**: Each day has one comprehensive guide (no redundant summaries)
2. **Visual Progress Tracking**: Charts showing cumulative runtime, pattern usage, mission integration
3. **Reusable Patterns**: Extract cross-day patterns into catalog
4. **Performance Insights**: Document optimization wins and learnings
5. **Zettelkasten Integration**: Link implementations to mathematical/algorithmic concepts

---

## 📁 Documentation Structure (2-File System)

```
advent_of_code/aoc20XX/Problem_Statements/
├── summary_20XX.md                 # Stats dashboard - quick overview (START HERE)
│                                   # - Progress tracking
│                                   # - Performance table
│                                   # - Algorithms used
│                                   # - Patterns catalog
│                                   # - Learning highlights
│
└── days/
    ├── README.md                   # Navigation index for all days
    ├── day01.md                    # Problem statement (from AoC website)
    ├── day01_function_guide.md     # Comprehensive implementation guide
    ├── day02.md
    ├── day02_function_guide.md
    ├── ...
    ├── day25.md
    └── day25_function_guide.md
```

**Why 2 Files?**
- **summary_20XX.md**: Quick reference, navigation, stats dashboard
- **dayXX_function_guide.md**: Deep dive into implementation, algorithms, performance

**What Happened to 4-File System?**
- ❌ **Old**: Separate `patterns_catalog.md`, `algorithms_reference.md`, `performance_analysis.md`
- ✅ **New**: All integrated into `summary_20XX.md` (less duplication, easier maintenance)

**Templates**: Stored in `templates/aoc_documentation/`
- `stats_dashboard_template.md` → becomes `summary_20XX.md`
- `function_guide_template.md` → becomes `dayXX_function_guide.md`

---

## 📋 Post-Implementation Workflow

### Phase 1: Daily Documentation (During Problem Solving)

**After solving EACH day**:

1. ✅ **Run Benchmarks**
   ```bash
   cargo bench --bench benchmarks dayXX
   ```
   - Capture Part 1, Part 2, Combined runtime
   - Note any optimization wins

2. ✅ **Create Function Guide** (from template)
   - Copy `templates/aoc_documentation/function_guide_template.md`
   - Rename to `Problem_Statements/days/dayXX_function_guide.md`
   - Fill in sections:
     - Performance benchmarks (from step 1)
     - Problem overview
     - Type definitions
     - Core implementation
     - Algorithm analysis
     - Key insights
     - Tests & validation
     - Zettelkasten links

3. ✅ **Update Summary Dashboard**
   - `summary_20XX.md`: Add day to performance table
   - Update algorithms used section
   - Update patterns catalog if new pattern
   - Update learning highlights

**Time Estimate**: ~30-45 minutes per day (while fresh in mind)

---

### Phase 2: Pattern Updates (As Needed)

**When you notice a repeated pattern**:

1. ✅ **Add to Summary Patterns Section**
   - Update `summary_20XX.md` patterns catalog
   - Format:
     ```markdown
     - **[Pattern Name]**: Used in Day X, Y, Z - [Brief description]
     ```

2. ✅ **Cross-Reference from Function Guides**
   - Link from dayXX_function_guide.md to summary patterns section

**No separate phase needed** - integrate into daily workflow.

**Time Estimate**: ~5-10 minutes when pattern identified

---

### Phase 3: Final Review (After Day 25)

**After completing all 25 days**:

1. ✅ **Finalize Summary Dashboard**
   - Complete progress stats (should be 25/25)
   - Calculate final total runtime
   - Verify performance table accurate
   - Complete algorithms used summary
   - Complete patterns catalog
   - Write learning highlights section

2. ✅ **Polish Function Guides**
   - Ensure all cross-links work
   - Verify zettelkasten links current
   - Check code examples accurate
   - Consistent formatting

3. ✅ **Quality Check**
   - Verify all 25 days documented
   - Check all navigation links work
   - Ensure consistent formatting
   - Run spell check

**Time Estimate**: ~2-3 hours for final polish

---

## 📊 Documentation Checklist

### Per-Day Requirements
- [ ] `dayXX_function_guide.md` created from template
- [ ] Performance benchmarks section complete
- [ ] Problem overview clear
- [ ] Type definitions with rationale
- [ ] Core implementation documented
- [ ] Algorithm analysis included (complexity, approach)
- [ ] Key insights documented
- [ ] Tests documented
- [ ] Links to zettelkasten concepts
- [ ] Navigation links (to summary, problem, prev/next days)

### Summary Dashboard Updates
- [ ] Day added to performance table
- [ ] Algorithms used updated
- [ ] Patterns catalog updated (if new pattern)
- [ ] Learning highlights updated (if key insight)

### Final Requirements
- [ ] All 25 function guides complete
- [ ] Summary dashboard complete
- [ ] All navigation links verified
- [ ] Consistent formatting throughout
- [ ] Zettelkasten cross-links validated

---

## 🎯 Quality Standards

### Stats Section
- ✓ Runtime benchmarks with Criterion (not manual timing)
- ✓ Big-O complexity analysis
- ✓ Mission integration explicitly stated
- ✓ Patterns used listed

### Implementation Details
- ✓ Type definitions with design rationale
- ✓ Core functions documented
- ✓ Algorithm complexity for each function
- ✓ Example traces for clarity

### Performance Analysis
- ✓ Before/after for optimizations (if applicable)
- ✓ Speedup quantified (e.g., "100x faster")
- ✓ Code comparison for optimizations
- ✓ Validation that results match

### Tests
- ✓ Part 1 example test
- ✓ Part 2 example test
- ✓ Helper function tests (if complex)
- ✓ Edge case tests (if applicable)

### Links
- ✓ Zettelkasten notes for algorithms
- ✓ Pattern catalog cross-references
- ✓ Related day cross-references

---

## 🔧 Tools & Scripts

### Benchmark Command
```bash
# Single day
cargo bench --bench benchmarks day01

# All days
cargo bench --bench benchmarks

# Export to JSON
cargo bench --bench benchmarks -- --save-baseline baseline_YYYYMMDD
```

### Update Scripts (To Create)
```bash
# scripts/update_stats_dashboard.ps1
# - Reads all dayXX_comprehensive.md files
# - Extracts runtime from stats sections
# - Updates stats_dashboard.md tables
# - Updates cumulative runtime chart

# scripts/validate_documentation.ps1
# - Checks all required files exist
# - Verifies all links work
# - Ensures consistent formatting
# - Reports missing sections
```

### Template Usage
```bash
# Initial setup - copy summary template
cp templates/aoc_documentation/stats_dashboard_template.md \
   advent_of_code/aoc20XX/Problem_Statements/summary_20XX.md

# For each day - copy function guide template
cp templates/aoc_documentation/function_guide_template.md \
   advent_of_code/aoc20XX/Problem_Statements/days/day05_function_guide.md

# Then fill in sections while solving
```

---

## 📈 Progress Tracking

### Milestones
- [ ] **Day 5**: First pattern extraction checkpoint
- [ ] **Day 10**: Second pattern extraction checkpoint
- [ ] **Day 15**: Third pattern extraction checkpoint
- [ ] **Day 20**: Fourth pattern extraction checkpoint
- [ ] **Day 25**: Final documentation complete
- [ ] **Post-25**: Final review and polish complete

### Time Budget
| Phase | Estimated Time |
|-------|----------------|
| Daily docs (25 days × 30min) | 12.5 hours |
| Pattern updates (as needed) | 1 hour |
| Final review | 2.5 hours |
| **Total** | **~16 hours** |

**Spread over 25 days**: ~40 minutes per day average

**Savings vs Old System**: ~5 hours (no separate patterns/algorithms/performance files)

---

## 🎓 Documentation Philosophy

### Principles
1. **Document While Fresh**: Write day guide immediately after solving
2. **Show, Don't Tell**: Code examples > prose explanations
3. **Explain Why**: Design decisions and trade-offs matter
4. **Link Everything**: Cross-reference patterns, algorithms, zettelkasten
5. **Quantify Performance**: Benchmarks > hand-waving

### Anti-Patterns to Avoid
- ❌ Waiting until end to document everything (forget details)
- ❌ **Duplicating information between summary and function guides** (NEW)
- ❌ Vague performance claims ("pretty fast")
- ❌ Orphaned patterns (used once, documented as "pattern")
- ❌ Missing complexity analysis
- ❌ **Creating separate files for patterns/algorithms** (integrate into summary)

### Target Audience
- **Future You**: Reviewing solutions 6 months later
- **Learning Record**: Documenting growth and techniques learned
- **Reference Material**: Quick lookup for similar problems

---

## 🔄 Integration with Existing Workflow

### Daily Workflow
```
1. Read problem → 2. Solve Part 1 → 3. Solve Part 2
                            ↓
4. Run benchmarks → 5. Write tests → 6. Create function guide (30min)
                            ↓
7. Update summary dashboard → 8. Commit
```

### Summary Updates (Continuous)
```
Solve problem → Update performance table → Note patterns/algorithms
```

### Zettelkasten Integration
```
Solve problem → Identify algorithm → Create/update zettelkasten note
                            ↓
Link from day guide → Link from algorithms_reference.md
```

---

## 📝 Next Steps

### Before Starting New AoC Year
1. ✅ Review these templates
2. ✅ Adjust templates if needed
3. [ ] Create initial structure:
   ```bash
   mkdir -p advent_of_code/aoc20XX/Problem_Statements/days
   cp templates/aoc_documentation/stats_dashboard_template.md \
      advent_of_code/aoc20XX/Problem_Statements/summary_20XX.md
   ```
4. [ ] Test workflow with Day 1

### During AoC
- Follow daily workflow consistently
- Update summary dashboard after each day
- Keep zettelkasten updated alongside

### After AoC
- Final review and polish
- Validate all links
- Celebrate completion! 🎉

---

**Navigation**: [← Templates](../README.md) | [AoC Main](../../advent_of_code/README.md)

**See Also**:
- [Stats Dashboard Template](stats_dashboard_template.md) - Summary template
- [Function Guide Template](function_guide_template.md) - Day guide template
- [AoC 2022 Summary](../../advent_of_code/aoc2022/Problem_Statements/summary_2022.md) - Live example of 2-file system

---

**Created**: 2026-01-20  
**Updated**: 2026-02-01 - Converted to 2-file system  
**For**: All AoC years (2022+)  
**Status**: Active - 2-file system validated in AoC 2022
