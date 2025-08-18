# winresz

A simple window resize utility for Windows with DWM window corner customization support.

## Features

- **Window Resizing**: Resize windows by title or executable path
- **Corner Customization**: Control window corner rounding (Windows 11+)
- **Multiple Resolution Formats**: Support for various resolution formats and presets
- **Flexible Filtering**: Filter windows by title content or executable path

## Basic Usage

### Resize Windows

- Change Resolution by window title
  ```
  winresz --title-contains "Windowed Projector" 1280x720
  ```
- Change Resolution by executable path
  ```
  winresz --path-endswith "notepad.exe" 1280x720
  ```

### Window Corner Control (Windows 11+)

Control window corner rounding using the `--corner` or `-c` option:

```bash
# Disable corner rounding
winresz -t "notepad" -c DONOTROUND

# Enable corner rounding
winresz -t "notepad" -c ROUND

# Small corner rounding
winresz -t "notepad" -c ROUNDSMALL

# System default (let Windows decide)
winresz -t "notepad" -c DEFAULT
```

### Combined Usage

Resize and set corner preference simultaneously:

```bash
# Resize to 1920x1080 and disable corner rounding
winresz -t "Windowed Projector" -c DONOTROUND 1920x1080

# Resize to 4K and enable small corner rounding
winresz -p "vlc.exe" -c ROUNDSMALL 4k
```

## Short Options

- `-t` = `--title-contains`
- `-p` = `--path-endswith`
- `-c` = `--corner`
- `-o` = `--offset`

```bash
winresz -t "notepad" -c ROUND 1920x1080
winresz -p "notepad.exe" -c DONOTROUND fhd
```

## Corner Options

| Option | Value | Description |
|--------|-------|-------------|
| `DEFAULT` | 0 | Let Windows decide when to round corners |
| `DONOTROUND` | 1 | Never round window corners |
| `ROUND` | 2 | Round corners when appropriate |
| `ROUNDSMALL` | 3 | Round corners with smaller radius |

You can use either the name or numeric value:
```bash
winresz -t "notepad" -c ROUND      # Same as -c 2
winresz -t "notepad" -c 1          # Same as -c DONOTROUND
```

## Resolution Formats

### 1. Standard format (WIDTHxHEIGHT)
```bash
winresz -t "notepad" 1920x1080
winresz -t "notepad" 1280x800
```

### 2. Resolution constants
```bash
# 4:3 aspect ratio
winresz -t "notepad" vga      # 640x480
winresz -t "notepad" svga     # 800x600
winresz -t "notepad" xga      # 1024x768
winresz -t "notepad" sxga     # 1280x1024
winresz -t "notepad" uxga     # 1600x1200

# 16:9 aspect ratio
winresz -t "notepad" hd       # 1280x720
winresz -t "notepad" fhd      # 1920x1080
winresz -t "notepad" wqhd     # 2560x1440
winresz -t "notepad" 4k       # 3840x2160
winresz -t "notepad" uhd      # 3840x2160
winresz -t "notepad" 8k       # 7680x4320

# 16:10 aspect ratio
winresz -t "notepad" wxga     # 1280x800
winresz -t "notepad" wsxga    # 1680x1050
winresz -t "notepad" wuxga    # 1920x1200
winresz -t "notepad" wqxga    # 2560x1600
winresz -t "notepad" wquxga   # 3840x2400

# 21:9 aspect ratio (Ultrawide)
winresz -t "notepad" uwfhd    # 2560x1080
winresz -t "notepad" ultrawide # 2560x1080
winresz -t "notepad" uwqhd    # 3440x1440
winresz -t "notepad" uw4k     # 5120x2160

# Other formats
winresz -t "notepad" dci4k    # 4096x2160 (DCI 4K)
```

### 3. Auto-calculated 16:9 format (HEIGHTp)
Specify height with 'p' suffix for automatic 16:9 width calculation:
```bash
winresz -t "notepad" 720p     # Auto-calculated to 1280x720
winresz -t "notepad" 1080p    # Auto-calculated to 1920x1080
winresz -t "notepad" 1440p    # Auto-calculated to 2560x1440
winresz -t "notepad" 2160p    # Auto-calculated to 3840x2160
winresz -t "notepad" 480p     # Auto-calculated to 853x480
```

## Query Mode

Run without a size parameter to query current window dimensions:

```bash
# Show current size of windows matching criteria
winresz -t "notepad"
winresz -p "vlc.exe"
```

## Requirements

- Windows 10/11
- Corner customization requires Windows 11 Build 22000 or later

## Building

```bash
cargo build --target x86_64-pc-windows-msvc --release
```
