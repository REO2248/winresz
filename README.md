# winresz

A simple window resize utility for Windows.

## How to use it

- Change Resolution by window title
  ```
  winresz --title-contains "Windowed Projector" 1280x720
  ```
- Change Resolution by executable path
  ```
  winresz --path-endswith "notepad.exe" 1280x720
  ```

## Short hand!

- Change Resolution by window title
  ```
  winresz -t "Windowed Projector" 1280x720
  ```
- Change Resolution by executable path
  ```
  winresz -p "notepad.exe" 1280x720
  ```

## Resolution Formats

You can specify resolution in multiple formats:

### 1. Standard format (WIDTHxHEIGHT)
```
winresz -t "notepad" 1920x1080
winresz -t "notepad" 1280x800
```

### 2. Resolution constants
```
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
```
winresz -t "notepad" 720p     # Auto-calculated to 1280x720
winresz -t "notepad" 1080p    # Auto-calculated to 1920x1080
winresz -t "notepad" 1440p    # Auto-calculated to 2560x1440
winresz -t "notepad" 2160p    # Auto-calculated to 3840x2160
winresz -t "notepad" 480p     # Auto-calculated to 853x480
```
