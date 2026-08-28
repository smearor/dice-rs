# BLE Specifications

BLE specification of the GoDice dice.

The dice are powered by the [Nordic nRF52805](https://www.nordicsemi.com/Products/nRF52805)
SoC - a Bluetooth 5.2 System-on-Chip with a 64 MHz ARM Cortex-M4 processor,
192 KB Flash, and 24 KB RAM. The nRF52805 is optimized for small two-layer PCB
designs in a 2.48 x 2.46 mm WLCSP package. The SoC's 64 MHz Cortex-M4 processes
the dice's 3D sensor data to calculate roll results and detect movement,
tilting, free fall, and taps.

[Particula selected the nRF52805](https://www.nordicsemi.com/Nordic-news/2022/08/Particulas-GoDice-employs-Nordics-nRF52805-SoC)
for its reliability and low power consumption. GoDice uses supercapacitor
technology for ultra-fast battery-free charging - thanks in part to
the ultra-low power characteristics of the Nordic SoC (4.6 mA in TX at 0 dBm,
4.6 mA in RX, and 0.3 uA in System OFF).

The dice use the [Nordic UART Service (NUS)](https://docs.nordicsemi.com/bundle/ncs-3.2.1/page/nrf/libraries/bluetooth/services/nus.html)
profile internally. NUS is a custom GATT service that emulates a serial port
over BLE, originally designed by Nordic for UART-to-BLE bridging. GoDice
repurposes it as a raw byte transport: the application protocol (opcodes and
events) is layered on top of the NUS RX/TX characteristics.

The nRF52805 runs a [SoftDevice S112 or S113](https://www.nordicsemi.com/Products/nRF52805/Download)
- a memory-optimized Peripheral-only Bluetooth LE protocol stack suited to the
SoC's 24 KB RAM. Both stacks support up to 4 concurrent Peripheral connections
with a Broadcaster, Bluetooth 5.1 qualification, 2 Mbps high-throughput, and
Channel Selection Algorithm #2.

The full protocol was reverse-engineered from the official
[JavaScript API](https://github.com/ParticulaCode/GoDiceJavaScriptAPI/blob/main/godice.js),
[Python API](https://github.com/ParticulaCode/GoDicePythonAPI/blob/main/godice/dice.py),
and [C API](https://github.com/ParticulaCode/GoDiceAndroid_iOS_API/blob/main/common/godiceapi.c)
source code.

## Device Properties

| Property              | Description                        | Value                                |
|-----------------------|------------------------------------|--------------------------------------|
| Device name           | Prefix                             | GoDice_                              |
| Service UUID          | NUS Service (16-bit offset 0x0001) | 6e400001-b5a3-f393-e0a9-e50e24dcca9e |
| Write Characteristic  | NUS RX - host writes commands      | 6e400002-b5a3-f393-e0a9-e50e24dcca9e |
| Notify Characteristic | NUS TX - dice sends notifications  | 6e400003-b5a3-f393-e0a9-e50e24dcca9e |

### NUS Transport Details

- **Write type**: Both Write Request (with response) and Write Command (without
  response) are supported on the RX characteristic
- **Notifications**: The dice sends all data via Handle Value Notifications on
  the TX characteristic; the host must enable notifications by writing to the
  CCCD (Client Characteristic Configuration Descriptor, value `0x0001`)
- **Security**: All permissions are open (SEC_OPEN) - no pairing or bonding
  required
- **Max payload**: `MTU_SIZE - 3` bytes (20 bytes with the default 23-byte ATT
  MTU)
- **No encryption**: Communication is unencrypted; the dice accept connections
  from any central

## Byte Commands (Host → Dice)

All commands are written as byte arrays to the Write Characteristic.
The first byte is always the opcode.

| Opcode | Decimal | Command              | Payload Bytes                                                                                                        | Description                                                                                   |
|--------|---------|----------------------|----------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------|
| 0x03   | 3       | Get Battery Level    | (none)                                                                                                               | Response: `Bat` + level byte                                                                  |
| 0x08   | 8       | Set LEDs             | `[R1, G1, B1, R2, G2, B2]` (6 bytes, 0–255)                                                                          | Sets both RGB LEDs; `[0,0,0,0,0,0]` turns off                                                 |
| 0x10   | 16      | Pulse LEDs           | `[pulseCount, onTime, offTime, R, G, B, blinkMode, leds]`                                                            | `onTime`/`offTime` in units of 10 ms; max 255. `blinkMode` and `leds` select which LEDs blink |
| 0x14   | 20      | Stop Pulse LEDs      | (none)                                                                                                               | Stops any active pulse LED animation                                                          |
| 0x17   | 23      | Get Dice Color       | (none)                                                                                                               | Response: `Col` + color byte                                                                  |
| 0x19   | 25      | Init                 | `[sensitivity, pulseCount, onTime, offTime, R, G, B, blinkMode, leds]` (9 bytes)                                     | Initializes dice with sensitivity and LED configuration                                       |
| 0x31   | 49      | Set Tap Interrupt    | `[enable]` (1 byte, 0=disable, 1=enable)                                                                             | Enables/disables single tap event notifications. Disabled by default.                         |
| 0x32   | 50      | Set Double Tap Interrupt | `[enable]` (1 byte, 0=disable, 1=enable)                                                                         | Enables/disables double tap event notifications. Disabled by default.                         |
| 0x65   | 101     | Detection Settings   | `[samplesCount, movementCount, faceCount, minFlatDeg, maxFlatDeg, weakStable, movementDeg, rollThreshold]` (8 bytes) | Updates roll detection sensitivity parameters                                                 |

## Receiving Events (Dice → Host)

The dice sends byte packets on state changes as notifications on the
Notify Characteristic. The first byte determines the event type. Some
events use ASCII prefixes for identification.

| First Byte(s)       | ASCII  | Event        | Payload                                | Description                                                        |
|---------------------|--------|--------------|----------------------------------------|--------------------------------------------------------------------|
| 0x52                | `R`    | RollStart    | (none)                                 | Dice is currently rolling                                          |
| 0x53                | `S`    | Stable       | `[X, Y, Z]` (3 signed bytes, offset 1) | Dice is stable and flat; face derived from XYZ                     |
| 0x46 0x53           | `FS`   | FakeStable   | `[X, Y, Z]` (3 signed bytes, offset 2) | Stable after a "fake" roll; face derived from XYZ                  |
| 0x54 0x53           | `TS`   | TiltStable   | `[X, Y, Z]` (3 signed bytes, offset 2) | Stable but tilted (not flat); face derived from XYZ                |
| 0x4D 0x53           | `MS`   | MoveStable   | `[X, Y, Z]` (3 signed bytes, offset 2) | Stable after small movement (face rotation); face derived from XYZ |
| 0x42 0x61 0x74      | `Bat`  | BatteryLevel | `[level]` (1 byte, offset 3)           | Battery level response (0–100 percent)                             |
| 0x43 0x6F 0x6C      | `Col`  | DiceColor    | `[color]` (1 byte, offset 3)           | Dice color response                                                |
| 0x43 0x68 0x61 0x72 | `Char` | Charging     | `[charging]` (1 byte, offset 4)        | Charging status (0 = not charging, 1 = charging)                   |
| 0x54 0x61 0x70      | `Tap`  | Tap          | (none)                                 | Single tap detected (no payload)                                   |
| 0x44 0x54 0x61 0x70 | `DTap` | DoubleTap    | (none)                                 | Double tap detected (no payload)                                   |

## Dice Colors

| Value | Color  |
|-------|--------|
| 0     | Black  |
| 1     | Red    |
| 2     | Green  |
| 3     | Blue   |
| 4     | Yellow |
| 5     | Orange |

## Dice Types (Shells)

| Value | Type  | Vector Table |
|-------|-------|--------------|
| 0     | D6    | d6Vectors    |
| 1     | D20   | d20Vectors   |
| 2     | D10   | d20Vectors → d10Transform   |
| 3     | D10X  | d20Vectors → d10XTransform  |
| 4     | D4    | d24Vectors → d4Transform    |
| 5     | D8    | d24Vectors → d8Transform    |
| 6     | D12   | d24Vectors → d12Transform   |

> **Note**: D10X is also referred to as **D100** (percentile) in the C API.
> The transform maps the D20 vector index to a D10 face value multiplied by 10
> (i.e. `d10_transform(roll) * 10`), yielding values 0, 10, 20, …, 90.

`setDieType` is a client-side setting - no command is sent to the dice.
Instead, it selects which vector table and transform to use when interpreting
the XYZ accelerometer data to determine the face value.

## Face Value Determination

The dice does not send the rolled number directly. Instead, it sends raw
XYZ accelerometer data (3 signed 8-bit integers). The client determines
the upper face by finding the closest vector in a pre-defined table:

1. Extract `[x, y, z]` from the notification payload.
2. Look up the vector table for the current `DiceType`.
3. For each entry `(face_value, reference_vector)`, compute the
   Euclidean distance: `sqrt((x - rx)² + (y - ry)² + (z - rz)²)`.
   (The squared distance without `sqrt` is functionally equivalent for
   finding the minimum, since `sqrt` is monotonically increasing.)
4. Return the face value with the smallest distance.
5. If a shell transform applies (D10, D10X, D4, D8, D12), map the
   intermediate value through the transform table.

### D6 Vector Table

![D6 Vectors](./assets/d6_vectors.svg)

| Face | X    | Y    | Z    |
|------|------|------|------|
| 1    | -64  | 0    | 0    |
| 2    | 0    | 0    | 64   |
| 3    | 0    | 64   | 0    |
| 4    | 0    | -64  | 0    |
| 5    | 0    | 0    | -64  |
| 6    | 64   | 0    | 0    |

### D20 Vector Table

![D20 Vectors](./assets/d20_vectors.svg)

| Face |  X   |  Y   |  Z   |
|------|------|------|------|
|  1   | -64  |  0   | -22  |
|  2   |  42  | -42  |  40  |
|  3   |  0   |  22  | -64  |
|  4   |  0   |  22  |  64  |
|  5   | -42  | -42  |  42  |
|  6   |  22  |  64  |  0   |
|  7   | -42  | -42  | -42  |
|  8   |  64  |  0   | -22  |
|  9   | -22  |  64  |  0   |
| 10   |  42  | -42  | -42  |
| 11   | -42  |  42  |  42  |
| 12   |  22  | -64  |  0   |
| 13   | -64  |  0   |  22  |
| 14   |  42  |  42  |  42  |
| 15   | -22  | -64  |  0   |
| 16   |  42  |  42  | -42  |
| 17   |  0   | -22  | -64  |
| 18   |  0   | -22  |  64  |
| 19   | -42  |  42  | -42  |
| 20   |  64  |  0   |  22  |

### D24 Vector Table

![D24 Vectors](./assets/d24_vectors.svg)

| Face |  X   |  Y   |  Z   |
|------|------|------|------|
|  1   |  20  | -60  | -20  |
|  2   |  20  |  0   |  60  |
|  3   | -40  | -40  |  40  |
|  4   | -60  |  0   |  20  |
|  5   |  40  |  20  |  40  |
|  6   | -20  | -60  | -20  |
|  7   |  20  |  60  |  20  |
|  8   | -40  |  20  | -40  |
|  9   | -40  |  40  |  40  |
| 10   | -20  |  0   |  60  |
| 11   | -20  | -60  |  20  |
| 12   |  60  |  0   |  20  |
| 13   | -60  |  0   | -20  |
| 14   |  20  |  60  | -20  |
| 15   |  20  |  0   | -60  |
| 16   |  40  | -20  | -40  |
| 17   | -20  |  60  | -20  |
| 18   | -40  | -40  | -40  |
| 19   |  40  | -20  |  40  |
| 20   |  20  | -60  |  20  |
| 21   |  60  |  0   | -20  |
| 22   |  40  |  20  | -40  |
| 23   | -20  |  0   | -60  |
| 24   | -20  |  60  |  20  |

### Shell Transform Tables

Each transform maps the vector table index (1-based) to the final face value.
D6 and D20 use identity (no transform). D10X multiplies the D10 transform by 10.

#### D4 Transform (D24 → D4)

| Index | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 |
|-------|----|----|----|----|----|----|----|----|
| Face  | 3  | 1  | 4  | 1  | 4  | 4  | 1  | 4  |

| Index | 09 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
|-------|----|----|----|----|----|----|----|----|
| Face  | 2  |  3 |  1 |  1 |  1 |  4 |  2 |  3 |


| Index | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 |
|-------|----|----|----|----|----|----|----|----|
| Face  |  3 |  2 |  2 |  2 |  4 |  1 |  3 |  2 |

#### D8 Transform (D24 → D8)

| Index | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 |
|-------|----|----|----|----|----|----|----|----|
| Face  | 3  | 3  | 6  | 1  | 2  | 8  | 1  | 1  |

| Index | 09 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
|-------|----|----|----|----|----|----|----|----|
| Face  | 4  |  7 |  5 |  5 |  4 |  4 |  2 |  5 |

| Index | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 |
|-------|----|----|----|----|----|----|----|----|
| Face  |  7 |  7 |  8 |  2 |  8 |  3 |  6 |  6 |

#### D10 Transform (D20 → D10)

| Index | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 | 10 |
|-------|----|----|----|----|----|----|----|----|----|----|
| Face  | 8  | 2  | 6  | 1  | 4  | 3  | 9  | 0  | 7  |  5 |

| Index | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
|-------|----|----|----|----|----|----|----|----|----|----|
| Face  |  5 |  7 |  0 |  9 |  3 |  4 |  1 |  6 |  2 |  8 |

#### D10X Transform (D20 → D10X)

| Index | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 | 10 |
|-------|----|----|----|----|----|----|----|----|----|----|
| Face  | 80 | 20 | 60 | 10 | 40 | 30 | 90 | 0  | 70 | 50 |

| Index | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
|-------|----|----|----|----|----|----|----|----|----|----|
| Face  | 50 | 70 |  0 | 90 | 30 | 40 | 10 | 60 | 20 | 80 |

#### D12 Transform (D24 → D12)

| Index | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 |
|-------|----|----|----|----|----|----|----|----|
| Face  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  |

| Index | 09 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
|-------|----|----|----|----|----|----|----|----|
| Face  | 9  | 10 | 11 | 12 |  1 |  2 |  3 |  4 |

| Index | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 |
|-------|----|----|----|----|----|----|----|----|
| Face  |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 |
