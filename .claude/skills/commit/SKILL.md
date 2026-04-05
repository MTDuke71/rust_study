---
name: commit
description: Commit all uncommitted files with an appropriate message and push to GitHub
---

# Commit & Push Skill

Commit all uncommitted changes with a well-formed commit message following this repo's conventions, then push to GitHub.

## Steps

1. Run `git status` (never use `-uall`) and `git diff HEAD` and `git log --oneline -5` in parallel
2. Analyze all changes (staged, unstaged, untracked) and draft a commit message:
   - Follow existing commit style: `feat(scope): description (timing)` for AoC, or appropriate conventional commit prefix
   - Scope examples: `aoc2017`, `mission5`, `zettelkasten`, `rfr` (Rust for Rustaceans)
   - Include timing in parens for AoC days
   - Keep the first line under 72 characters
   - Do NOT commit files that look like secrets (.env, credentials, tokens)
3. Stage all relevant files with `git add` (list files explicitly, do not use `git add -A` or `git add .`)
4. Commit with the message, always ending with:
   ```
   Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
   ```
   Use a HEREDOC for the commit message to preserve formatting.
5. Push to origin: `git push`
6. Report the commit hash and push status

## Rules

- If there are no changes to commit, say so and stop
- If a pre-commit hook fails, fix the issue and create a NEW commit (never amend)
- Never force push
- Never skip hooks (no `--no-verify`)
