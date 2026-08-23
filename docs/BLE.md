# BLE Specifications

BLE specification of the GoDice dice.

The dice use the Nordic UART Service (NUS) profile internally.
The full protocol was reverse-engineered from the official
[JavaScript API](https://github.com/ParticulaCode/GoDiceJavaScriptAPI/blob/main/godice.js)
and [Python API](https://github.com/ParticulaCode/GoDicePythonAPI/blob/main/godice/dice.py)
source code.

## Device Properties

| Property              | Description                    | Value                                |
|-----------------------|--------------------------------|--------------------------------------|
| Device name           | Prefix                         | GoDice_                              |
| Service UUID          |                                | 6e400001-b5a3-f393-e0a9-e50e24dcca9e |
| Write Characteristic  | Send commands                  | 6e400002-b5a3-f393-e0a9-e50e24dcca9e |
| Notify Characteristic | Receive events / results       | 6e400003-b5a3-f393-e0a9-e50e24dcca9e |

## Byte Commands (Host → Dice)

All commands are written as byte arrays to the Write Characteristic.
The first byte is always the opcode.

| Opcode | Decimal | Command           | Payload Bytes                              | Description |
|--------|---------|-------------------|--------------------------------------------|-------------|
| 0x03   | 3       | Get Battery Level | (none)                                     | Response: `Bat` + level byte |
| 0x08   | 8       | Set LEDs          | `[R1, G1, B1, R2, G2, B2]` (6 bytes, 0–255) | Sets both RGB LEDs; `[0,0,0,0,0,0]` turns off |
| 0x10   | 16      | Pulse LEDs        | `[pulseCount, onTime, offTime, R, G, B, 1, 0]` | `onTime`/`offTime` in units of 10 ms; max 255 |
| 0x17   | 23      | Get Dice Color    | (none)                                     | Response: `Col` + color byte |

## Receiving Events (Dice → Host)

The dice sends byte packets on state changes as notifications on the
Notify Characteristic. The first byte determines the event type. Some
events use ASCII prefixes for identification.

| First Byte(s)    | ASCII | Event          | Payload                                      | Description |
|------------------|-------|----------------|----------------------------------------------|-------------|
| 0x52             | `R`   | RollStart      | (none)                                       | Dice is currently rolling |
| 0x53             | `S`   | Stable         | `[X, Y, Z]` (3 signed bytes, offset 1)       | Dice is stable and flat; face derived from XYZ |
| 0x46 0x53        | `FS`  | FakeStable     | `[X, Y, Z]` (3 signed bytes, offset 2)       | Stable after a "fake" roll; face derived from XYZ |
| 0x54 0x53        | `TS`  | TiltStable     | `[X, Y, Z]` (3 signed bytes, offset 2)       | Stable but tilted (not flat); face derived from XYZ |
| 0x4D 0x53        | `MS`  | MoveStable     | `[X, Y, Z]` (3 signed bytes, offset 2)       | Stable after small movement (face rotation); face derived from XYZ |
| 0x42 0x61 0x74   | `Bat` | BatteryLevel   | `[level]` (1 byte, offset 3)                 | Battery level response (0–100 percent) |
| 0x43 0x6F 0x6C   | `Col` | DiceColor      | `[color]` (1 byte, offset 3)                 | Dice color response |

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

`setDieType` is a client-side setting — no command is sent to the dice.
Instead, it selects which vector table and transform to use when interpreting
the XYZ accelerometer data to determine the face value.

## Face Value Determination

The dice does not send the rolled number directly. Instead, it sends raw
XYZ accelerometer data (3 signed 8-bit integers). The client determines
the upper face by finding the closest vector in a pre-defined table:

1. Extract `[x, y, z]` from the notification payload.
2. Look up the vector table for the current `DiceType`.
3. For each entry `(face_value, reference_vector)`, compute the squared
   Euclidean distance: `(x - rx)² + (y - ry)² + (z - rz)²`.
4. Return the face value with the smallest distance.
5. If a shell transform applies (D10, D10X, D4, D8, D12), map the
   intermediate value through the transform table.

### D6 Vector Table

| Face | X    | Y    | Z    |
|------|------|------|------|
| 1    | -64  | 0    | 0    |
| 2    | 0    | 0    | 64   |
| 3    | 0    | 64   | 0    |
| 4    | 0    | -64  | 0    |
| 5    | 0    | 0    | -64  |
| 6    | 64   | 0    | 0    |

The D20 and D24 vector tables (20 and 24 entries respectively) and the
shell transform tables are defined in the official API source code.
