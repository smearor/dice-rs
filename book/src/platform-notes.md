# Platform Notes

## Linux (BlueZ)

`dice-rs` uses [btleplug](https://github.com/deviceplug/btleplug) which
communicates with BlueZ via DBus on Linux. This is the primary and
currently only supported platform.

### Requirements

- BlueZ 5.x
- `bluetoothd` daemon running
- DBus session bus access
- A Bluetooth adapter with BLE support

### Setup

```sh
# Check BlueZ is running
systemctl status bluetooth

# Start if needed
sudo systemctl start bluetooth

# Ensure the user has Bluetooth access
sudo usermod -aG bluetooth $USER
# Log out and back in for group changes to take effect
```

### Permissions

The user running `dice-rs` needs DBus access to the Bluetooth adapter. In
most desktop environments this is granted automatically. On headless
servers or containers, you may need to:

1. Start a DBus session: `export DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/$(id -u)/bus`
2. Ensure `bluetoothd` is running with `--experimental` flag if using
   extended BLE features.

### Troubleshooting

**"No Bluetooth adapter found"**

Ensure `bluetoothd` is running and an adapter is present:

```sh
hciconfig -a
```

**"Device not found during scan"**

- Make sure the GoDice is awake (tap or roll to wake)
- Check that the dice is not already connected to another host
- Try disconnecting stale connections: `dice-rs disconnect-all`
- BlueZ does not deliver RSSI advertisement updates for connected devices.
  Disconnect before scanning.

**"Connection failed after 3 retries"**

- The dice may be charging from 0% battery and not yet accepting connections.
  Wait a few minutes and retry.
- Move closer to the Bluetooth adapter.
- Check for interference from other BLE devices.

**"BlueZ/DBus socket buffer overflow"**

This can happen when sending many LED commands in rapid succession. The
`set_leds()` method includes a 30ms debounce to prevent this. If using
`set_leds_immediate()`, throttle your calls.

## macOS

`dice-rs` supports macOS via btleplug's CoreBluetooth backend. The library
(`dice-rs`), CLI (`dice-rs-cli`), and WebSocket server (`dice-rs-ws`) are
fully supported. The GTK4 controller (`dice-rs-controller`) is Linux-only.

### Requirements

- macOS 12 (Monterey) or later
- A Bluetooth adapter with BLE support (built-in on all modern Macs)
- Xcode Command Line Tools: `xcode-select --install`

### Permissions

macOS requires Bluetooth permission for the process. When running the CLI or
WebSocket server for the first time, the OS will prompt for Bluetooth access.
Grant the permission to your terminal or application.

### Troubleshooting

**"No Bluetooth adapter found"**

Ensure Bluetooth is enabled in System Settings > Bluetooth.

**"Device not found during scan"**

- Make sure the GoDice is awake (tap or roll to wake)
- Check that the dice is not already connected to another host
- CoreBluetooth may cache device state — try toggling Bluetooth off/on

## Windows

`dice-rs` supports Windows via btleplug's WinRT backend. The library
(`dice-rs`), CLI (`dice-rs-cli`), and WebSocket server (`dice-rs-ws`) are
fully supported. The GTK4 controller (`dice-rs-controller`) is Linux-only.

### Requirements

- Windows 10 (build 19041) or later
- A Bluetooth adapter with BLE support
- [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Permissions

Windows requires Bluetooth access for the process. No explicit permission
prompt is shown, but the Bluetooth radio must be enabled in Settings >
Bluetooth & devices.

### Troubleshooting

**"No Bluetooth adapter found"**

Ensure Bluetooth is enabled in Settings > Bluetooth & devices and that a
compatible adapter is present.

**"Device not found during scan"**

- Make sure the GoDice is awake (tap or roll to wake)
- Check that the dice is not already connected to another host
- WinRT may not deliver RSSI updates for all devices — the library falls
  back to cached properties when `read_rssi()` fails

**"Connection failed after 3 retries"**

- The dice may be charging from 0% battery and not yet accepting connections.
  Wait a few minutes and retry.
- Move closer to the Bluetooth adapter.
- Check for interference from other BLE devices.

## Controller (Linux-only)

The `dice-rs-controller` GTK4 desktop application is Linux-only. It depends
on GTK4, OpenGL (`glow`), and BlueZ-specific behaviors. There are no plans
to port it to Windows or macOS. For cross-platform UI access, use the
`dice-rs-ws` WebSocket server with a web-based or native client.
