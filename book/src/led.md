# LED Control

GoDice has two RGB LEDs. The `dice-rs` library provides methods for setting
colors and pulse animations.

## LedColor

`LedColor` is an RGB color with channels 0–255:

```rust
use dice_rs::LedColor;

let red = LedColor::RED;
let custom = LedColor::new(128, 64, 255);
let from_hex = LedColor::from_hex(0xFF8800);
let off = LedColor::OFF;
```

## Set LEDs

```rust,no_run
use dice_rs::{DiceManager, LedColor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    // Set both LEDs to the same color
    dice.set_led(LedColor::GREEN).await?;

    // Set each LED independently
    dice.set_leds(LedColor::RED, LedColor::BLUE).await?;

    // Turn off
    dice.turn_off_leds().await?;

    Ok(())
}
```

## Debouncing

`set_leds()` uses a debounce mechanism: rapid successive calls within a 30ms
window are coalesced into a single BLE write. Only the most recent colors
are written. This prevents BlueZ/DBus socket buffer overflow when an
application fires many color changes in quick succession.

For one-shot commands where coalescing is undesirable, use
`set_leds_immediate()`:

```rust,no_run
use dice_rs::{DiceManager, LedColor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    // Write immediately, bypassing debounce
    dice.set_leds_immediate(LedColor::RED, LedColor::RED).await?;

    Ok(())
}
```

## Pulse LEDs

Pulse animations blink the LEDs with configurable timing:

```rust,no_run
use dice_rs::{DiceManager, LedColor};
use dice_rs::model::led::PulseBlinkMode;
use dice_rs::model::led::PulseLeds;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    // Pulse 3 times: 500ms on, 200ms off, green, solid color, both LEDs
    dice.pulse_leds(3, 50, 20, LedColor::GREEN, PulseBlinkMode::Color, PulseLeds::Both).await?;

    // Convenience: single pulse
    dice.pulse_once(50, 20, LedColor::RED).await?;

    Ok(())
}
```

`on_time` and `off_time` are in units of 10ms. Maximum value is 255 (2.55s).

### PulseBlinkMode

| Mode | Description |
|------|-------------|
| `Color` | Solid color blink |
| `Rainbow` | Rainbow color cycle |

### PulseLeds

| Value | Description |
|-------|-------------|
| `Both` | Both LEDs pulse |
| `Led1` | Only LED 1 pulses |
| `Led2` | Only LED 2 pulses |
