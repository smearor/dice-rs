# WebSocket Server

The `dice-rs-ws` crate provides a WebSocket and REST server for exposing
GoDice events over a network API.

## Running

```sh
cargo run -p dice-rs-ws
```

The server starts on `0.0.0.0:3000`.

## REST API

### Scan

```
POST /api/scan
```

Returns a JSON array of discovered devices:

```json
[
  {
    "address": "AA:BB:CC:DD:EE:FF",
    "name": "GoDice_AAABBCC_O_v04",
    "rssi": -45
  }
]
```

### Connect

```
POST /api/connect
Content-Type: application/json

{
  "address": "AA:BB:CC:DD:EE:FF",
  "dice_type": "d6"
}
```

Returns a session ID:

```json
{
  "session_id": "abc123"
}
```

`dice_type` is optional. Accepts: `d6`, `d20`, `d10`, `d10x`, `d4`, `d8`, `d12`.

### Disconnect

```
POST /api/disconnect
Content-Type: application/json

{
  "session_id": "abc123"
}
```

### LED Control

```
POST /api/led
Content-Type: application/json

{
  "session_id": "abc123",
  "led1": "FF0000",
  "led2": "00FF00"
}
```

### Battery

```
GET /api/battery/:session_id
```

### Status

```
GET /api/status/:session_id
```

### Calibration

```
POST /api/calibrate
Content-Type: application/json

{
  "session_id": "abc123"
}
```

## WebSocket API

Connect to `ws://localhost:3000/ws` to receive a real-time stream of
`DiceEvent`s as JSON:

```json
{
  "kind": "Stable",
  "face": 6,
  "acceleration": { "x": 64, "y": 0, "z": 0 }
}
```

```json
{
  "kind": "RollStart"
}
```

```json
{
  "kind": "Disconnected"
}
```

## Deployment

The server uses `axum` with tokio. For production deployment, consider
running behind a reverse proxy (e.g. nginx) with TLS termination.
