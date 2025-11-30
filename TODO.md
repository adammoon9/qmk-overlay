# QMK Keymap Overlay - TODO

A cross-platform (Linux/macOS) QMK keymap overlay that requires no firmware modifications and can be easily installed via package managers.

## Phase 1: Project Setup & Core Infrastructure

- [x] Initialize Rust project with cargo
  - [ ] Set up workspace structure
  - [x] Configure `Cargo.toml` with dependencies
  - [ ] Set up cross-platform build targets

- [x] Dependencies to add:
  - [x] `serde` and `serde_json` - JSON parsing
  - [x] `deser-hjson` - HJSON parsing for QMK data files
  - [x] `evdev` - Linux keyboard input monitoring
  - [ ] `core-foundation` + `core-graphics` - macOS keyboard monitoring
  - [x] GUI framework (choose one):
    - [x] `egui` + `eframe` (lightweight, immediate-mode)
    - [ ] `tauri` (if using web tech for UI)
    - [ ] `iced` (native Rust GUI)
  - [x] `directories` - XDG/platform-specific paths
  - [x] `toml` - Configuration file parsing
  - [x] `thiserror` - Error handling
  - [x] `log` and `env_logger` - Logging

- [ ] Create basic project structure:
  ```
  src/
  ├── main.rs
  ├── keymap_parser/
  │   ├── mod.rs
  │   └── qmk_json.rs
  ├── input_monitor/
  │   ├── mod.rs
  │   ├── linux.rs
  │   └── macos.rs
  ├── layer_tracker/
  │   ├── mod.rs
  │   └── layer_logic.rs
  ├── overlay_ui/
  │   ├── mod.rs
  │   └── renderer.rs
  └── config/
      ├── mod.rs
      └── settings.rs
  ```

## Phase 2: Keymap Parser

- [ ] Implement QMK JSON keymap parser
  - [x] Define Rust structs matching QMK JSON schema
  - [ ] Parse keyboard layout (physical key positions)
  - [ ] Parse layers array
  - [ ] Parse keycodes for each key on each layer

- [x] Load QMK keycode definitions from HJSON
  - [x] Parse `qmk_firmware/data/constants/keycodes/*.hjson` files
  - [x] Build keycode lookup table (code -> label mapping)
  - [x] Handle keycode aliases
  - [x] Support multiple keycode files (keycodes_basic.hjson, keycodes_quantum.hjson, etc.)
  - [ ] Cache parsed keycode definitions

- [ ] Implement QMK keycode understanding
  - [ ] Create enum for QMK keycodes (basic keycodes)
  - [ ] Parse layer switching keycodes:
    - [ ] `MO(layer)` - Momentary layer
    - [ ] `LT(layer, kc)` - Layer tap
    - [ ] `TG(layer)` - Toggle layer
    - [ ] `TT(layer)` - Tap toggle
    - [ ] `OSL(layer)` - One-shot layer
    - [ ] `TO(layer)` - Switch to layer
    - [ ] `DF(layer)` - Set default layer
  - [ ] Parse modifier keycodes (for proper display)
  - [ ] Handle custom keycodes gracefully
  - [ ] Use loaded keycode definitions for display labels

- [ ] Keymap file discovery
  - [ ] Search standard QMK locations (`~/qmk_firmware`)
  - [ ] Support custom path via config/CLI argument
  - [ ] List available keyboards and keymaps
  - [ ] Validate keymap.json structure
  - [ ] Locate QMK data directory for keycode definitions

## Phase 3: Layer Tracking Logic

- [ ] Implement layer state machine
  - [ ] Create `LayerTracker` struct
  - [ ] Maintain stack of active layers
  - [ ] Track held keys and their layer actions

- [ ] Implement layer switching logic
  - [ ] `MO(n)` - Add layer to stack on press, remove on release
  - [ ] `LT(n, kc)` - Track hold time, decide layer vs keycode
  - [ ] `TG(n)` - Toggle layer on/off
  - [ ] `TT(n)` - Toggle on Nth tap, momentary otherwise
  - [ ] `OSL(n)` - Activate for next keypress only
  - [ ] `TO(n)` - Clear stack and switch to layer
  - [ ] `DF(n)` - Change base layer

- [ ] Handle edge cases
  - [ ] Multiple simultaneous layer keys
  - [ ] Layer precedence (highest wins)
  - [ ] State recovery on focus loss
  - [ ] Handle transparent keys (`KC_TRNS`)

## Phase 4: Input Monitoring

### Linux (evdev)
- [ ] Implement evdev keyboard monitoring
  - [ ] Detect and enumerate keyboard devices
  - [ ] Handle multiple keyboards
  - [ ] Require proper permissions (input group)
  - [ ] Map evdev keycodes to QMK keycodes
  - [ ] Handle key press/release events
  - [ ] Deal with key repeat events

### macOS (IOKit)
- [ ] Implement IOKit keyboard monitoring
  - [ ] Set up event tap
  - [ ] Handle accessibility permissions
  - [ ] Map macOS keycodes to QMK keycodes
  - [ ] Handle key press/release events
  - [ ] Deal with secure input mode

### Platform Abstraction
- [ ] Create unified keyboard input interface
- [ ] Conditional compilation for platform-specific code
- [ ] Test on both platforms

## Phase 5: Overlay UI

- [ ] Design overlay window
  - [ ] Keyboard layout renderer
  - [ ] Key labels with current layer
  - [ ] Highlight active layer keys
  - [ ] Visual feedback for held keys

- [ ] Implement overlay features
  - [ ] Always-on-top window
  - [ ] Transparent/semi-transparent background
  - [ ] Click-through option
  - [ ] Draggable window (hold to drag)
  - [ ] Auto-hide when base layer active (optional)
  - [ ] Smooth layer transitions/animations

- [ ] Key rendering
  - [ ] Different colors for different key types
  - [ ] Show modifiers clearly
  - [ ] Handle special keycodes (media, functions, etc.)
  - [ ] Support multi-line labels for complex keys
  - [ ] Highlight currently pressed keys

- [ ] UI Controls
  - [ ] Settings menu (right-click or hotkey)
  - [ ] Layer selector/indicator
  - [ ] Opacity control
  - [ ] Size/zoom control
  - [ ] Theme selection

## Phase 6: Configuration System

- [ ] Create configuration file format (TOML)
  - [ ] Keymap path
  - [ ] Window position and size
  - [ ] Auto-hide settings
  - [ ] Theme/appearance settings
  - [ ] Keyboard device selection

- [ ] Configuration management
  - [ ] Load config from XDG/platform-specific location
  - [ ] Validate configuration
  - [ ] Hot-reload on config change
  - [ ] CLI arguments override config file
  - [ ] First-run setup wizard

## Phase 7: Polish & User Experience

- [ ] Error handling
  - [ ] Graceful failure messages
  - [ ] Help user debug issues (permissions, paths, etc.)
  - [ ] Log to file for troubleshooting

- [ ] Documentation
  - [ ] README with screenshots
  - [ ] Installation instructions
  - [ ] Usage guide
  - [ ] Configuration examples
  - [ ] Troubleshooting section
  - [ ] Comparison with original Windows tool

- [ ] Testing
  - [ ] Unit tests for keymap parser
  - [ ] Unit tests for layer logic
  - [ ] Integration tests
  - [ ] Test with various QMK keymaps
  - [ ] Test on different Linux distros
  - [ ] Test on macOS versions

## Phase 8: Distribution & Packaging

- [ ] Cargo/crates.io
  - [ ] Publish to crates.io
  - [ ] Document `cargo install` method

- [ ] Homebrew (macOS)
  - [ ] Create Homebrew formula
  - [ ] Submit to homebrew-core or create tap

- [ ] Linux packages
  - [ ] Create AUR package (Arch)
  - [ ] Consider .deb package (Debian/Ubuntu)
  - [ ] Consider .rpm package (Fedora)
  - [ ] Flatpak/Snap consideration

- [ ] GitHub Releases
  - [ ] Set up CI/CD (GitHub Actions)
  - [ ] Automated builds for Linux/macOS
  - [ ] Provide pre-built binaries
  - [ ] Automated release process

## Phase 9: Advanced Features (Future)

- [ ] Optional HID Raw protocol support
  - [ ] Define simple HID Raw protocol
  - [ ] Provide QMK firmware example code
  - [ ] Implement HID Raw listener
  - [ ] Auto-detect HID vs simulated mode

- [ ] Advanced QMK features
  - [ ] Tap dance support
  - [ ] Combo support
  - [ ] Mod-tap timing accuracy
  - [ ] Custom keycode definitions

- [ ] Additional features
  - [ ] Key press statistics/heatmap
  - [ ] Layer usage analytics
  - [ ] Multiple keyboard profiles
  - [ ] Web-based configurator
  - [ ] Training mode (quiz on key locations)

## Known Limitations (Document These)

- Layer switching logic is simulated, not 100% accurate for complex features
- Tap-dance, combos, and complex timing features may not work perfectly
- If application loses focus during key press, state may desynchronize
- Some QMK features (macros, sequences) cannot be fully simulated
- Requires QMK firmware repository for keycode definitions (or bundle them)

## Alternative Approach for Keycode Definitions

Instead of requiring the full QMK repo, consider:
- [ ] Bundle pre-parsed keycode definitions with the application
- [ ] Extract and include only the essential HJSON files in the binary
- [ ] Auto-update keycode definitions from QMK repo (optional feature)
- [ ] Fall back to bundled definitions if QMK repo not found

## Nice-to-Haves

- [ ] Animated layer transitions
- [ ] Sound effects on layer change
- [ ] Customizable color schemes
- [ ] Export overlay as image
- [ ] Multiple window layouts (compact, full, split)
- [ ] Vi-mode indicator for users with Vi-mode layers
