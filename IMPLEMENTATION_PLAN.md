# Floater Implementation Plan

## Project Overview
Floater is a macOS desktop application that creates a small floating window (200x120px) that can contain text content and widgets. Built with Tauri + React.

## Implementation Priorities (High to Low)

### 1. ✅ **COMPLETED** - Basic Window Configuration
- [x] Configure window to be 200x120px
- [x] Make window draggable
- [x] Make window non-resizable
- [x] Apply green on white theme

### 2. **IN PROGRESS** - CLI Interface Foundation
- [ ] Create CLI binary (`floatercli`)
- [ ] Implement `floatercli show "text"` command
- [ ] Implement `floatercli hide` command
- [ ] Setup IPC between CLI and Tauri app

### 3. **PENDING** - Timer Widget
- [ ] Create timer widget UI component
- [ ] Implement stopwatch functionality (counts up)
- [ ] Add `floatercli show "text" --timer` support
- [ ] Handle widget state management

### 4. **PENDING** - Content Display System
- [ ] Dynamic content rendering in floater window
- [ ] Text content display
- [ ] Widget container system
- [ ] State persistence between show/hide

### 5. **PENDING** - Packaging & Distribution
- [ ] Configure Tauri for macOS DMG
- [ ] Bundle CLI binary with the app
- [ ] Setup app to be launchable from Applications folder
- [ ] Create installer workflow

## Technical Notes

### Current Status
- ✅ Basic Tauri + React project scaffolded
- ✅ Window configured to spec (200x120px, draggable, non-resizable)
- ✅ Green on white theme applied
- 🔄 Working on CLI interface

### Architecture Decisions
- **Frontend**: React with TypeScript
- **Backend**: Tauri (Rust)
- **CLI**: Node.js/Bun binary bundled with app
- **IPC**: Tauri's built-in command system
- **State Management**: React state for UI, Tauri commands for persistence

### Development Workflow
1. Start with window configuration (DONE)
2. Build CLI interface next (IN PROGRESS)
3. Add widget system
4. Setup packaging

## Next Steps
Currently implementing CLI interface foundation - this is the highest priority remaining item.