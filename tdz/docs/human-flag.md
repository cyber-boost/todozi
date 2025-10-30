# Quick Reference: --human Flag

## One-Liner
Generate markdown checklists for human review alongside machine-readable output.

## Commands

```bash
# Extract with human checklist
todozi extract --file notes.txt --human

# Strategy with human checklist
todozi strategy "Your goal here" --human

# Combine with output formats
todozi extract content --output json --human
todozi strategy --file plan.txt --output csv --human
```

## What You Get

### Without --human
```bash
$ todozi extract "Fix auth bug" --output json
{"tasks":[{"action":"Fix auth bug","time":"ASAP",...}],...}
```

### With --human  
```bash
$ todozi extract "Fix auth bug" --output json --human
{"tasks":[{"action":"Fix auth bug","time":"ASAP",...}],...}
📋 Human checklist saved to: todozi_checklist_plan_20251027_143000.md
```

**Generated file** (`todozi_checklist_plan_20251027_143000.md`):
```markdown
# 📋 Todozi Human Checklist

## 📝 Tasks
- [ ] **Fix auth bug**
  - 📁 Project: `general`
  - ⏱️ Time: `ASAP`
  - 🎯 Priority: `high`
  ...
```

## Content Types in Checklist

| Type | Icon | Example |
|------|------|---------|
| Tasks | 📝 | `- [ ] **Review authentication system**` |
| Memories | 🧠 | `- [ ] **Client prefers iterative development**` |
| Ideas | 💡 | `- [ ] **Use microservices architecture**` |
| Errors | ❌ | `- [ ] **Database connection pool exhaustion**` |
| Training | 🎓 | `- [ ] **How to implement rate limiting**` |

## File Naming

```
todozi_checklist_{endpoint}_{timestamp}.md

endpoint = "plan" or "strategic"
timestamp = YYYYMMdd_HHMMSS format
```

## Why Use It?

✅ Easy to read and share with team  
✅ Track progress with checkboxes  
✅ Works in GitHub/GitLab/Obsidian  
✅ Print-friendly for meetings  
✅ Git-friendly for version control  

## Tips

1. **Dual output**: `--output json --human` for both formats
2. **Commit to git**: Track your planning over time
3. **Share**: Send checklist to team, keep JSON for automation
4. **Review**: Use checklist in standup meetings

## See Full Docs

- `HUMAN_CHECKLIST_USAGE.md` - Complete usage guide
- `HUMAN_CHECKLIST_EXAMPLE.md` - Full example output
- `FEATURE_HUMAN_CHECKLIST.md` - Implementation details
