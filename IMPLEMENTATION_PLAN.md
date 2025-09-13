# Floater Implementation Plan

## Project Overview
Floater is a macOS desktop application that creates a small floating window (200x120px) that can contain text content and widgets. Built with Tauri + React.

## Implementation Priorities (High to Low)

### 1. ✅ **COMPLETED** - Basic Window Configuration
- [x] Configure window to be 200x120px
- [x] Make window draggable
- [x] Make window non-resizable
- [x] Apply green on white theme

### 2. ✅ **COMPLETED** - CLI Interface Foundation
- [x] Create CLI binary (`floatercli`)
- [x] Implement `floatercli show "text"` command
- [x] Implement `floatercli hide` command
- [x] Setup IPC between CLI and Tauri app

### 3. ✅ **COMPLETED** - Timer Widget
- [x] Create timer widget UI component
- [x] Implement stopwatch functionality (counts up)
- [x] Add `floatercli show "text" --timer` support
- [x] Handle widget state management

### 4. ✅ **COMPLETED** - Content Display System
- [x] Dynamic content rendering in floater window
- [x] Text content display
- [x] Widget container system
- [x] State persistence between show/hide

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
- ✅ CLI interface complete with timer support
- ✅ Timer widget functional
- ✅ Content display system working

### Architecture Decisions
- **Frontend**: React with TypeScript
- **Backend**: Tauri (Rust)
- **CLI**: Go binary with Cobra CLI framework
- **IPC**: Unix domain socket communication
- **State Management**: React state for UI, Tauri commands for persistence

### Development Workflow
1. ✅ Start with window configuration (DONE)
2. ✅ Build CLI interface (DONE)
3. ✅ Add widget system (DONE)
4. 🔄 Setup packaging (NEXT)

## Next Steps
CLI interface, timer widget, and content display systems are now complete. Next priority is packaging & distribution - creating DMG installer and bundling CLI binary with the app.