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

Not supported in the initial release. The `BleTransport` trait allows a
future CoreBluetooth backend.

## Windows

Not supported in the initial release. The `BleTransport` trait allows a
future WinRT backend.
