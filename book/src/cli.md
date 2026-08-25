# CLI Tool

The `dice-rs-cli` crate provides a command-line tool (`dice-rs`) for
interacting with GoDice devices without writing code.

## Installation

```sh
cargo install dice-rs-cli
```

## Commands

### Scan

Scan for nearby GoDice devices:

```sh
dice-rs scan 5
```

The argument is the scan duration in seconds.

### Listen

Connect to a dice and print events:

```sh
dice-rs listen AA:BB:CC:DD:EE:FF d6
```

The first argument is the MAC address (or a partial prefix). The second is
the dice type (`d6`, `d20`, `d10`, `d10x`, `d4`, `d8`, `d12`).

### Battery

Query battery level:

```sh
dice-rs battery AA:BB:CC:DD:EE:FF
```

### LED Control

Set LED colors:

```sh
# Set both LEDs to red
dice-rs led AA:BB:CC:DD:EE:FF set red

# Set each LED independently
dice-rs led AA:BB:CC:DD:EE:FF set-dual red blue

# Pulse animation
dice-rs led AA:BB:CC:DD:EE:FF pulse green

# Turn off
dice-rs led AA:BB:CC:DD:EE:FF off
```

### Tap Interrupts

Enable or disable tap notifications:

```sh
dice-rs tap AA:BB:CC:DD:EE:FF true
dice-rs double-tap AA:BB:CC:DD:EE:FF true
```

### Calibration

```sh
dice-rs calibrate AA:BB:CC:DD:EE:FF
```

### Status

Get comprehensive status:

```sh
dice-rs status AA:BB:CC:DD:EE:FF
```

### Color

Query the dice shell color:

```sh
dice-rs color AA:BB:CC:DD:EE:FF
```

### Charging

Query charging state:

```sh
dice-rs charging AA:BB:CC:DD:EE:FF
```

### Disconnect

```sh
# Disconnect a specific dice
dice-rs disconnect AA:BB:CC:DD:EE:FF

# Disconnect all connected dice
dice-rs disconnect-all
```

### Interactive Mode

Start a REPL for interactive exploration:

```sh
dice-rs interactive
```

Available commands in interactive mode: `scan`, `connect <address>`,
`disconnect`, `battery`, `color`, `charging`, `led <color>`, `status`,
`calibrate`, `quit`.
