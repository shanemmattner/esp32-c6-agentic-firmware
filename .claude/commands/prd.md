# Generate Product Requirements Document (PRD)

Create a comprehensive PRD for a new firmware feature using our standard template.

## Ask User

1. **Feature Name**: {What is this feature called?}
2. **Purpose**: {Why are we building this?}
3. **Use Case**: {Who will use it and how?}
4. **Peripherals**: {What hardware is involved?}
5. **Success Criteria**: {How do we know it works?}

## PRD Template

```markdown
# PRD: {Feature Name}

## Overview

**Feature**: {Feature Name}
**Author**: {Auto-fill from git}
**Date**: {Auto-fill current date}
**Status**: Draft → Review → Approved → Implemented

## Problem Statement

### Current Situation
{Describe the current state}

### Pain Points
- {What problems exist?}
- {What limitations are there?}
- {What user needs are unmet?}

## Proposed Solution

### High-Level Description
{1-2 paragraph overview of the solution}

### Key Features
1. **{Feature 1}**: {Description}
2. **{Feature 2}**: {Description}
3. **{Feature 3}**: {Description}

## Technical Requirements

### Hardware Requirements
- **MCU**: ESP32-C6 (RISC-V)
- **Peripherals**:
  - {Peripheral 1}: {Purpose}
  - {Peripheral 2}: {Purpose}
- **Pins**:
  - GPIO{X}: {Function}
  - GPIO{Y}: {Function}

### Software Requirements
- **esp-hal**: 1.0.0 with features {list}
- **Libraries**:
  - {Library 1}: {Version} - {Purpose}
  - {Library 2}: {Version} - {Purpose}
- **Optional**:
  - Embassy (if async needed)
  - State machine library (if FSM needed)

### Performance Requirements
- **Timing**: {Response time, update rate, etc.}
- **Power**: {Power consumption targets}
- **Memory**: {Flash/RAM budget}

## Architecture

### Component Diagram
```
┌─────────────────┐
│   Application   │
├─────────────────┤
│  State Machine  │  ← (if applicable)
├─────────────────┤
│    Drivers      │  ← esp-hal 1.0.0
├─────────────────┤
│   Hardware      │  ← ESP32-C6
└─────────────────┘
```

### Data Flow
1. {Step 1}
2. {Step 2}
3. {Step 3}

### State Machine (if applicable)
```
[State 1] --event--> [State 2] --event--> [State 3]
```

## Implementation Plan

### Phase 1: Basic Functionality
- [ ] Initialize peripheral
- [ ] Implement core logic
- [ ] Add basic logging
- [ ] Test on hardware

### Phase 2: Enhanced Features
- [ ] Add error handling
- [ ] Implement state machine
- [ ] Add async support (if needed)
- [ ] Comprehensive logging

### Phase 3: Testing & Validation
- [ ] Unit tests
- [ ] Integration tests
- [ ] Hardware-in-loop tests
- [ ] Performance validation

## Success Criteria

### Functional Requirements
✅ {Requirement 1}
✅ {Requirement 2}
✅ {Requirement 3}

### Non-Functional Requirements
✅ Code compiles without warnings
✅ All tests pass
✅ Logging shows expected behavior
✅ Performance meets requirements

### Acceptance Tests
```rust
#[test]
fn test_feature_works() {
    // Given: {Initial state}
    // When: {Action taken}
    // Then: {Expected result}
}
```

## Logging Strategy

### Key Log Points
- **INFO**: Feature initialization, state changes, milestones
- **DEBUG**: Detailed operation steps, variable values
- **WARN**: Recoverable issues, edge cases
- **ERROR**: Failures, critical issues

### Example Log Output
```
INFO: Initializing {feature}...
DEBUG: Configuring peripheral with {config}
INFO: {Feature} ready
DEBUG: Processing input: {value}
INFO: Operation completed successfully
```

## Testing Plan

### Unit Tests
- Test individual functions
- Mock hardware interactions
- Verify edge cases

### Integration Tests
- Test feature with real peripherals
- Verify communication protocols
- Test error scenarios

### Manual Test Cases
1. **Test Case 1**: {Description}
   - Setup: {Steps}
   - Expected: {Result}
2. **Test Case 2**: {Description}
   - Setup: {Steps}
   - Expected: {Result}

## Dependencies

### Crate Dependencies
```toml
[dependencies]
esp-hal = { version = "1.0.0", features = ["esp32c6", "rt", "unstable"] }
{additional dependencies}
```

### Hardware Dependencies
- ESP32-C6 DevKit
- {Additional hardware}

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| {Risk 1} | High/Med/Low | {How to mitigate} |
| {Risk 2} | High/Med/Low | {How to mitigate} |

## Timeline

- **Week 1**: Phase 1 - Basic implementation
- **Week 2**: Phase 2 - Enhanced features
- **Week 3**: Phase 3 - Testing & validation
- **Week 4**: Documentation & polish

## Future Enhancements

- {Enhancement 1}
- {Enhancement 2}
- {Enhancement 3}

## References

- [esp-hal Documentation](https://docs.esp-rs.org/esp-hal/)
- [Datasheet](link)
- [Related PRDs](links)

---

**Approval**:
- [ ] Technical Review
- [ ] Implementation Ready
```

## After PRD Creation

1. Save PRD as `docs/prd/{feature-name}.md`
2. Create tracking issue on GitHub
3. Break down into implementation tasks
4. Start with test-first development
5. Reference PRD in commits
