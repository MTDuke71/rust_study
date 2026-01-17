# Claude Code + Obsidian Workflow

*AI-assisted knowledge management combining Claude Code as interface and Obsidian as consumption layer.*

---

## Overview

This note captures a comprehensive workflow for using **Claude Code** and **Obsidian** together to create an AI-assisted knowledge base. The approach treats Claude Code as the primary interface for research, analysis, and note organization, while Obsidian serves as the consumption and visualization layer.

**Core Philosophy**: Claude Code = User Interface | Obsidian = Consumption Layer

---

## Why This Combination Works

### Obsidian Strengths
- Plain Markdown (portable, future-proof)
- Excellent plugin ecosystem (DataView, Advanced Tables)
- Cross-device sync capability
- Superior graph visualization
- Claude Code can generate plugin-compatible content (DataView queries, Mermaid diagrams)

### Claude Code Strengths
- Research and web synthesis
- Note organization and maintenance
- Automated tagging and linking
- Processing inbox notes
- Custom commands and agents for repetitive workflows

---

## System Architecture

### Folder Structure Pattern

```
vault/
├── .claude/              # Claude Code commands and agents (hidden from Obsidian)
│   ├── commands/         # Custom automation commands
│   └── agents/           # Specialized AI personas
├── images/               # Downloaded/referenced images
├── inbox/                # Human-written quick notes
├── research-inbox/       # Topics to research
├── output/               # Analysis outputs
├── reference/            # Processed research notes
├── projects/             # Long-running research projects
└── archive/              # Old notes (note bankruptcy approach)
```

### Note Lifecycle

```
research-inbox/ → Claude researches → reference/
inbox/ → Claude processes → appropriate folder
reference/ → time passes → archive/
```

---

## Key Workflows

### 1. Research Workflow

**Input**: Create note in `research-inbox/` with:
- What you already know
- What you want to learn
- Specific questions

**Process**: Claude Code researches using web search, synthesizes information, adds:
- Structured sections
- Code examples
- Source references
- Related topic links

**Output**: Comprehensive note in `reference/` folder

### 2. Inbox Processing

**Human-written notes** go to `inbox/`
- Quick captures
- Meeting notes
- Ideas

**Claude processes**:
- Formats and structures
- Adds tags
- Moves to appropriate folder
- Preserves original content
- NO additional research (configurable)

### 3. RSS Feed Command

**"Feed" command** compiles recent notes:
- Last 20 notes with summaries
- Cross-linked for context
- Ideal for mobile/tablet review
- Personal "RSS feed" of your own research

---

## Commands and Agents Pattern

### Commands
Reusable automation for specific tasks:
- `research` - Full research synthesis
- `inbox` - Process inbox notes
- `feed` - Generate recent notes summary
- `archive` - Move old notes to archive

### Agents (Personas)
Specialized AI behaviors:
- **Research Synthesizer** - Skeptical, comprehensive, uses multiple sources
- **Maintenance Specialist** - Precise, follows instructions exactly
- **Knowledge Architect** - Designs structure and connections
- **Note Connector** - Finds and creates bidirectional links

### Creating Commands/Agents
1. Describe desired behavior to Claude
2. Ask Claude to generate the prompt/command
3. Review and refine
4. Update prompts when behavior needs adjustment

---

## Philosophy: Note Bankruptcy

**Key Insight**: All notes are ephemeral. Knowledge changes fast.

- Don't aim for "evergreen" notes
- Accept that notes will go stale
- Build system with archival in mind
- Prefer creating new notes over editing old ones (preserves history)
- Periodically declare "note bankruptcy" and start fresh

This contrasts with traditional "digital garden" approaches but works better for fast-changing technical knowledge.

---

## Customization Patterns

### Fixing Claude Behavior
When Claude does something incorrectly:
1. Tell it to fix the specific issue
2. Ask it to update relevant prompts to prevent recurrence
3. Mark fixes as "CRITICAL" with correct/incorrect examples
4. Claude learns and applies consistently

### Adapting Structure
- Start with a README describing your system
- Let Claude infer structure from README
- Adjust based on what you actually use
- Remove unused features (MOCs, tags, etc.)
- Keep what works for YOUR workflow

---

## Setup for New Vault

### Step 1: Create README
```markdown
This is a knowledge base for [your topics].

Topics:
- Topic 1
- Topic 2

Format:
- Zettelkasten method (or your preferred method)
- Obsidian-first markdown
- [Your preferences: tables, Mermaid diagrams, etc.]

AI Assisted:
This knowledge base uses Claude Code for research,
maintenance, and organization.
```

### Step 2: Initialize Claude
- Run Claude Code in vault directory
- Let it create `.claude.md` from README
- Review and refine generated settings

### Step 3: Create Structure
- Ask Claude to create folder structure
- Seed with example notes
- Iterate based on preferences

### Step 4: Add Commands/Agents
- Research relevant community examples
- Create custom commands for your workflows
- Build agents for specialized tasks

---

## Cost Considerations

Claude Code has usage costs. Consider:
- Budget equivalent to subscription services (Evernote, Bear, etc.)
- Research tasks cost more than simple formatting
- Worth it for personalized, high-quality synthesis
- Can run multiple Claude instances for parallel work

---

## Integration with Existing Zettelkasten

This workflow complements the existing [[Zettelkasten System]]:

| Aspect | Traditional | Claude-Assisted |
|--------|-------------|-----------------|
| Note creation | Manual | AI-synthesized research |
| Linking | Manual bidirectional | AI-suggested + manual |
| Organization | Folder discipline | AI-assisted routing |
| Review | Manual weekly | "Feed" command |
| Maintenance | PowerShell scripts | Claude commands |

**Hybrid Approach**: Use Claude for research and bulk operations, manual curation for quality control.

---

## Practical Tips

1. **Permissions**: Start with approval required for each web fetch, relax later
2. **Emojis**: Configure based on preference (can disable in prompts)
3. **Front matter**: Ensure Claude uses Obsidian-compatible YAML
4. **Image references**: Obsidian uses `![[image.png]]` not standard markdown
5. **Multiple instances**: Run parallel Claude sessions for different tasks

---

## Related Resources

- [[Zettelkasten System]] - Core methodology for this workspace
- [[developer-learning-habits]] - Evidence-based learning patterns
- [[Obsidian Demo Guide]] - Getting started with Obsidian navigation
- [[Daily Workflow]] - How daily notes integrate with the system
- [[learning-plan]] - Overall learning coordination

---

*Tags: #zettelkasten #obsidian #claude-code #ai-assisted #knowledge-management #workflow #pkm*

---

*Source: Video transcript on Claude Code + Obsidian workflow for AI-assisted knowledge management. Captured insights on folder structure, commands/agents pattern, research workflows, and the "note bankruptcy" philosophy.*
