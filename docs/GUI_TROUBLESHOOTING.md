# Zenith GUI Troubleshooting Guide

**Problem**: GUI applications run but windows don't appear  
**System**: Linux with Wayland + X11

---

## Quick Diagnosis

Run this to test:

```bash
cd /home/vantrong/Downloads/ngon_ngu_lap_trinh_moi/zenith

# Method 1: Force X11 (not Wayland)
WAYLAND_DISPLAY= cargo run --release -p zenith -- run examples/10_simple_counter.zn

# Method 2: With debug logs
RUST_LOG=debug cargo run -p zenith -- run examples/10_simple_counter.zn 2>&1 | grep -i "window\|display\|eframe"

# Method 3: Check if process runs
cargo run --release -p zenith -- run examples/12_form.zn &
PID=$!
sleep 2
ps -p $PID && echo "Running!" || echo "Crashed!"
pkill zenith
```

---

## Common Issues & Solutions

### Issue 1: Headless Environment (No Display)

**Symptoms**: `DISPLAY` is empty or GUI never shows  
**Solution**: Install X server or use VNC

```bash
# Check display
echo $DISPLAY  # Should show ":0" or ":1"

# If empty, you're in headless mode
# Option A: Install X server
sudo apt install xorg

# Option B: Use Xvfb (virtual framebuffer)
Xvfb :99 -screen 0 1024x768x24 &
export DISPLAY=:99
cargo run -p zenith -- run examples/10_simple_counter.zn
```

### Issue 2: Wayland Compatibility

**Symptoms**: Process runs but no window on Wayland  
**Solution**: Force X11 backend

```bash
# Temporarily disable Wayland
WAYLAND_DISPLAY= cargo run -p zenith -- run examples/10_simple_counter.zn

# Or permanently in your shell
echo 'export WAYLAND_DISPLAY=' >> ~/.bashrc
```

### Issue 3: Missing Dependencies

```bash
# Install required libraries
sudo apt install -y \
    libx11-dev \
    libxcb1-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libssl-dev \
    libegl1-mesa-dev \
    libgl1-mesa-dev

# Rebuild
cargo clean
cargo build --release -p zenith
```

### Issue 4: Window Off-Screen

**Symptoms**: Process runs, but window is hidden  
**Solution**: Reset window position

```bash
# Check if window exists (requires wmctrl)
wmctrl -l

# Or use xdotool
xdotool search --name "Zenith"
```

---

## Verification Steps

### Step 1: Test Console Example (Should Work)

```bash
cargo run --release -p zenith -- run examples/01_basics.zn
```

**Expected**: Output printed, exits successfully  
✅ If this works → Runtime is fine

### Step 2: Test Simple GUI

```bash
timeout 3 cargo run --release -p zenith -- run examples/10_simple_counter.zn &
sleep 2
pgrep zenith && echo "✅ Running" || echo "❌ Crashed"
pkill zenith
```

**Expected**: Process found running  
✅ If this works → eframe starts successfully

### Step 3: Look for Window

```bash
# Run GUI
cargo run --release -p zenith -- run examples/12_form.zn &
PID=$!

# Wait and check
sleep 2
xdotool search --pid $PID --name "" 2>/dev/null && echo "✅ Window found!" || echo "❌ No window"

pkill zenith
```

---

## Working Examples to Test

### Console Examples (Always Work)
```bash
./target/release/zenith run examples/01_basics.zn
./target/release/zenith run examples/02_flow_control.zn  
./target/release/zenith run examples/03_functions.zn
./target/release/zenith run examples/04_data.zn
./target/release/zenith run examples/04_oop.zn
./target/release/zenith run examples/05_file_io.zn
```

### GUI Examples (Need Display)
```bash
# Simple counter
./target/release/zenith run examples/10_simple_counter.zn

# Color picker
./target/release/zenith run examples/11_color_picker.zn

# Form
./target/release/zenith run examples/12_form.zn

# Dashboard
./target/release/zenith run examples/08_dashboard.zn

# Todo list
./target/release/zenith run examples/06_ui_todo.zn

# Chat UI
./target/release/zenith run examples/09_chat_ui.zn

# Tic-Tac-Toe
./target/release/zenith run examples/13_game_tictactoe.zn
```

---

## Alternative: Screenshot Mode

If GUI won't show, capture to image:

```bash
# Method 1: Xvfb + screenshot
Xvfb :99 -screen 0 1920x1080x24 &
export DISPLAY=:99

cargo run -p zenith -- run examples/10_simple_counter.zn &
sleep 3
import -window root screenshot.png  # or use scrot
pkill zenith

# View screenshot
xdg-open screenshot.png
```

---

## Expected Behavior

When working correctly:

1. **Console examples**: Print output and exit immediately
2. **GUI examples**: 
   - Process launches
   - Window opens showing UI
   - Responds to mouse/keyboard
   - Ctrl+C or close window to exit

---

## Get Help

If none of these work:

1. Provide output of:
   ```bash
   echo "DISPLAY=$DISPLAY"
   echo "XDG_SESSION_TYPE=$XDG_SESSION_TYPE"  
   ldd target/release/zenith | grep -i "gl\|x11\|xcb"
   RUST_LOG=debug cargo run -p zenith -- run examples/10_simple_counter.zn 2>&1 | head -50
   ```

2. Check if other GUI apps work:
   ```bash
   xeyes  # Simple X11 test
   glxgears  # OpenGL test
   ```

3. Try a minimal eframe example outside Zenith

---

**Note**: GUI examples require a graphical environment. On headless servers, use console examples or set up X11 forwarding/VNC.
