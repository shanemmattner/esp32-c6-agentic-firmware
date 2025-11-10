# CLAUDE.md - Guidelines for Claude Code Assistant

## Model Selection

**DEFAULT: Use Haiku Model**
- Unless explicitly told otherwise, use Claude Haiku (fastest, most cost-effective)
- Only use Sonnet or Opus if the task requires complex reasoning
- Current model: claude-haiku-4-5-20251001

**How to specify model:**
```
/model sonnet    # Switch to Sonnet
/model opus      # Switch to Opus
/model haiku     # Back to Haiku (default)
```

---

## File Operations

### ❌ Task() CANNOT Create Files
- `Task()` launches a subprocess agent for complex work
- **Agents cannot create files** - they can only read and report back
- **Don't use Task()** for file generation

### ✅ Use These Tools Instead
- `Write()` - Create new files or overwrite existing
- `Edit()` - Modify specific parts of existing files
- `Bash` - Create files via shell commands
- `Read()` - Read files before editing

### Rule of Thumb
**If you need to create/modify files → Use Write/Edit/Bash directly, NOT Task()**

---

## When to Use Task()

Task() is useful for:
- ✅ Complex research/exploration (general-purpose agent)
- ✅ Finding patterns in large codebases (Explore agent)
- ✅ Multi-step analysis and reporting
- ❌ **NOT for file creation/modification**

---

## Lean Lessons Approach

These lessons should be **simple and practical**:
- Focus on working code, not massive documentation
- Minimal READMEs (just basics)
- One main .rs file per lesson (~100-150 lines)
- Test on hardware immediately
- Keep it type-able for YouTube videos

---

## Project Conventions

### Directory Structure
```
lessons/{NN}-{name}/
├── src/
│   ├── bin/
│   │   └── main.rs          # Main firmware
│   └── lib.rs               # (optional library code)
├── .cargo/
│   └── config.toml          # Build config
├── Cargo.toml               # Dependencies
├── rust-toolchain.toml      # Toolchain
├── build.rs                 # Build script
└── README.md                # Simple docs (keep short!)
```

### Cargo.toml
- Always include `[[bin]]` section pointing to `src/bin/main.rs`
- Keep dependencies minimal
- Use esp-hal 1.0.0

### Documentation
- README.md: Keep it short (< 300 lines)
- Focus on: wiring, expected output, troubleshooting
- Skip lengthy theory - put that in code comments

---

## Testing Approach

1. **Build:** `cargo build --release`
2. **Flash:** `cargo run --release`
3. **Test:** Manual hardware validation
4. **Iterate:** Fix issues, re-test

No massive test plans until code works on hardware.

---

## Git Workflow

- Commit after each working lesson
- Keep commit messages clear and concise
- Format: `feat(lesson-{NN}): {brief description}`

---

## Common Mistakes to Avoid

1. ❌ Using Task() to generate files
2. ❌ Over-engineering lessons (keep them simple!)
3. ❌ Massive documentation before working code
4. ❌ Not testing on hardware
5. ❌ Using expensive models (Sonnet/Opus) by default

---

## Quick Reference

| Task | Tool | Time |
|------|------|------|
| Create lesson code | Write() + Bash | 5-10 min |
| Modify file | Edit() | 2-5 min |
| Create README | Write() | 3-5 min |
| Test on hardware | Manual | 10-20 min |
| **Avoid: Massive planning** | ~~Task()~~ | ⏱️ Don't |

---

**Last Updated:** 2025-11-09
**Next:** Implement Lesson 03: DHT22 sensor (simple version!)
