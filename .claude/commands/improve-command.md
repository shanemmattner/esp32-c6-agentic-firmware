---
description: Analyze conversation logs to improve slash commands based on real usage patterns
argument-hint: <command_name> <conversation_log_path_or_paste>
---

# Improve Slash Command from Conversation Logs

You are analyzing a conversation log to identify improvements for the slash command: **{{argument}}**

## Your Task

Analyze the conversation log and extract actionable improvements for the slash command.

## Analysis Framework

Look for these specific patterns in the conversation:

### 1. LLM Struggle Patterns

**Indicators:**
- Same question asked multiple times before getting answer
- Multiple failed attempts followed by success (try A → fail → try B → fail → try C → success)
- Long search sequences to find information
- Trial-and-error with tools or commands
- Confusion about parameters or prerequisites

**What to extract:**
- The information that was eventually found
- The successful approach after failures
- Missing context that caused confusion

**Action:** Add explicit guidance, pre-checks, or default values to command

**Example finding:**
```markdown
## Struggle: Probe selection (3 attempts)
- Tried: `--probe 1` → Failed (invalid format error)
- Tried: `--probe 303a:1001` → Failed (exclusive access)
- Success: Auto-detect from `probe-rs list | grep "esp.*jtag"`

**Recommendation:** Add probe auto-detection in Step 0:
```bash
ESP_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '^\[([0-9]+)\]' | tr -d '[]' | head -1)
PROBE_ARG="--probe $ESP_PROBE"
```
```

### 2. Knowledge Discovery

**Indicators:**
- New file locations found during exploration
- Undocumented APIs or patterns discovered
- Workarounds for common issues found
- Useful command combinations discovered
- Better tool choices identified

**What to extract:**
- New information that should be documented
- Better approaches than what's in the command
- Tools or files that should be checked

**Action:** Update command with new knowledge, or create new slash command if scope is different

**Example finding:**
```markdown
## Discovery: espflash vs probe-rs run
- Found: `probe-rs run` causes exclusive lock issues
- Found: `espflash` for flashing + `probe-rs attach` for debugging works better
- Current command uses: `probe-rs run` (problematic)

**Recommendation:** Change strategy:
```bash
# OLD (causes locks):
probe-rs run --chip esp32c6 target/.../main

# NEW (better):
espflash flash --port $USB_CDC_PORT target/.../main
probe-rs attach --chip esp32c6 $PROBE_ARG target/.../main
```
```

### 3. Anti-Patterns and Pitfalls

**Indicators:**
- Same error appearing multiple times
- Incorrect assumptions that led to failures
- Tools used incorrectly
- Hardware state conflicts
- Commands that don't work as expected

**What to extract:**
- Common errors and their causes
- Incorrect approaches to avoid
- State conflicts to prevent

**Action:** Add warnings, pre-checks, or cleanup steps

**Example finding:**
```markdown
## Anti-Pattern: Hardcoded USB ports
- Error: `/dev/cu.usbmodem2101` not found after replug
- Cause: Port numbers change when device is unplugged/replugged
- Occurred: 2 times in conversation

**Recommendation:** Use dynamic detection:
```bash
USB_CDC_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$USB_CDC_PORT" ]; then
    echo "ERROR: No USB CDC port found"
    exit 1
fi
```
```

### 4. Unnecessary Steps

**Indicators:**
- Tasks performed that didn't contribute to solution
- Over-complicated approaches
- Redundant verification
- Steps that were skipped without issue

**What to extract:**
- Steps that can be removed
- Simpler alternatives
- More efficient sequences

**Action:** Streamline command execution flow

**Example finding:**
```markdown
## Unnecessary: Multiple GDB path checks
- Command checks for both GDB and probe-rs
- User only has probe-rs
- GDB checks waste time and add confusion

**Recommendation:** Remove GDB path entirely, focus on probe-rs only
```

### 5. Missing Context

**Indicators:**
- LLM had to search for information repeatedly
- Constants or variables that weren't pre-defined
- Dependencies not mentioned in prerequisites
- Assumptions that turned out wrong

**What to extract:**
- Information that should be in command context
- Prerequisites that should be checked upfront
- Variables that should be pre-defined

**Action:** Add to prerequisites or context section

**Example finding:**
```markdown
## Missing Context: Process cleanup needed
- Error: "exclusive access" when attaching probe-rs
- Cause: Previous probe-rs session still running
- Not mentioned in prerequisites

**Recommendation:** Add cleanup step at beginning:
```bash
# Clean up any existing debug sessions
pkill -f "probe-rs" || true
pkill -f "openocd" || true
sleep 1
```
```

### 6. Successful Patterns

**Indicators:**
- Sequences that worked well first try
- Efficient problem-solving approaches
- Good use of parallelization
- Effective error handling

**What to extract:**
- Patterns to reinforce or expand
- Good practices to codify
- Efficient workflows to preserve

**Action:** Document as best practices, ensure command preserves them

---

## Analysis Process

### Step 1: Read the Conversation Log

If argument contains a file path:
```bash
cat <conversation_log_path>
```

If argument is pasted text, analyze it directly.

### Step 2: Identify the Command Being Improved

Extract the command name from the first argument (before the log path/text).

Find the current command file:
```bash
ls .claude/commands/<command_name>.md
```

Read the current command to understand what it does now.

### Step 3: Scan for Patterns

Go through the conversation chronologically and mark:
- 🔴 Struggles (multiple attempts, errors, confusion)
- 🟢 Discoveries (new knowledge, better approaches)
- 🟡 Anti-patterns (repeated errors, incorrect approaches)
- 🔵 Unnecessary steps (wasted effort)
- 🟣 Missing context (information gaps)
- ✅ Successful patterns (what worked well)

### Step 4: Extract Specific Examples

For each pattern found, document:
1. **What happened** (quote relevant parts of log)
2. **Why it happened** (root cause)
3. **Impact** (how severe, how often)
4. **Recommendation** (specific fix with code if applicable)

### Step 5: Prioritize Improvements

Rank by impact:
- **High:** Blocking issues, repeated errors, major time waste
- **Medium:** Confusion, inefficiency, missing documentation
- **Low:** Nice-to-haves, minor optimizations

### Step 6: Generate Improvement Report

Create a structured markdown report:

```markdown
# Command Improvement Analysis: <command_name>

**Analysis Date:** YYYY-MM-DD
**Conversation Date:** YYYY-MM-DD (if available)
**Conversation Length:** X messages

---

## Executive Summary

- **Total Issues Found:** X
- **High Priority:** X
- **Medium Priority:** X
- **Low Priority:** X
- **New Discoveries:** X
- **Successful Patterns:** X

**Overall Assessment:** [1-2 sentence summary]

---

## High Priority Issues

### Issue 1: [Short Description]

**Category:** Struggle / Anti-Pattern / Missing Context
**Severity:** High
**Occurrences:** X times

**What Happened:**
[Quote from log or description]

**Root Cause:**
[Why this happened]

**Current Command Behavior:**
```bash
[Current problematic code from command]
```

**Recommended Fix:**
```bash
[Improved code]
```

**Impact if Fixed:**
[Benefit of applying this fix]

---

[Repeat for each high priority issue]

---

## Medium Priority Issues

[Same format as above]

---

## Low Priority Issues

[Same format as above]

---

## Knowledge Discoveries

### Discovery 1: [What was learned]

**How Discovered:**
[Context from conversation]

**Current Command:**
[Does it use this knowledge? Yes/No]

**Recommendation:**
[How to incorporate this discovery]

---

## Successful Patterns to Preserve

### Pattern 1: [What worked well]

**Why It Worked:**
[Analysis]

**Current Command:**
[Does it include this? Yes/No]

**Recommendation:**
[Ensure this pattern is preserved/expanded]

---

## Recommended Changes Summary

### Changes to Make:

1. **[Change description]**
   - File: `.claude/commands/<command_name>.md`
   - Section: [which section]
   - Type: Add / Modify / Remove
   - Priority: High / Medium / Low

2. [...]

### New Commands to Create:

1. **/<new_command_name>**
   - Purpose: [what it does]
   - Reason: [why separate from existing command]
   - Scope: [what it should cover]

### Documentation Updates:

1. **IDEAS.md**
   - Add: [new idea discovered]
   - Update: [existing idea refined]

2. **README.md** (if applicable)
   - Add: [new documentation needed]

---

## Implementation Plan

1. [ ] Review recommendations with user
2. [ ] Apply high priority fixes to command
3. [ ] Test updated command on hardware
4. [ ] Apply medium priority fixes
5. [ ] Create new commands if needed
6. [ ] Update documentation
7. [ ] Mark this analysis as complete

---

## Notes

- [Any additional observations]
- [Patterns that need more data]
- [Questions for user]
```

---

## Output Format

Present your analysis as the markdown report above, with specific code examples and clear recommendations.

## Important Guidelines

1. **Be specific:** Quote actual log content, show actual code changes
2. **Be actionable:** Every recommendation should be implementable immediately
3. **Prioritize ruthlessly:** Not all issues are equal
4. **Look for patterns:** Single occurrences may not warrant changes
5. **Preserve what works:** Don't break successful patterns
6. **Consider scope:** Some discoveries may warrant new commands instead of modifying existing ones

---

## After Analysis

Ask the user:
1. Do you want me to apply the high priority fixes now?
2. Should I create any new commands based on discoveries?
3. Are there specific recommendations you want me to skip?

Then proceed based on their response.
