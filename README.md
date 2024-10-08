# i3 Switch Tabs
Switch top-level tabs in i3-wm

## Notes

Requires i3-wm version 4.8 or newer.

## Usage

```
Usage: i3-switch-tabs <DIRECTION>

Arguments:
  <DIRECTION>  Direction to switch to [possible values: left, right, down, up]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Example

Add this to your i3-wm config:

```
bindsym $mod+Tab exec i3-switch-tabs right
bindsym $mod+Shift+Tab exec i3-switch-tabs left
```
