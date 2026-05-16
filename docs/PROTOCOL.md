# RustDesk Protocol Specification

**Version:** 1.4.6
**Last updated:** 2026-05-14

## Overview

RustDesk uses a custom protocol for remote desktop communication. The protocol is defined in protobuf format and supports:

- Peer registration and discovery
- NAT traversal (UDP hole punching, relay fallback)
- Video/audio streaming
- Input events (mouse, keyboard, clipboard)
- File transfer
- Port forwarding
- Terminal service

## Minimal Library v1 Scope

The validated minimal remote-core library v1 intentionally exposes only the protocol capabilities needed to build a basic custom remote desktop client:
- session connect / disconnect / state
- frame pull (raw RGBA)
- keyboard / mouse control
- text clipboard send
- lifecycle/event observation

Capabilities such as file transfer, terminal, port forward, printer, recording, and advanced display management remain outside the minimal v1 boundary.

## Message Types

### Connection Messages

Defined in `libs/hbb_common/protos/rendezvous.proto`

#### RegisterPeer

```
message RegisterPeer {
  string id = 1;        // Peer ID (9-digit number)
  int32 serial = 2;     // Registration serial number
}
```

**Purpose:** Register peer with rendezvous server.

#### PunchHoleRequest

```
message PunchHoleRequest {
  string id = 1;              // Target peer ID
  NatType nat_type = 2;       // NAT type (ASYMMETRIC/SYMMETRIC)
  string licence_key = 3;     // License key (optional)
  ConnType conn_type = 4;     // Connection type
  string token = 5;           // Auth token
  string version = 6;         // Client version
  int32 udp_port = 7;         // UDP port
  bool force_relay = 8;       // Force relay connection
}
```

**Purpose:** Request direct connection to peer (NAT traversal).

#### PunchHole

```
message PunchHole {
  bytes socket_addr = 1;              // Peer socket address
  string relay_server = 2;            // Relay server if needed
  NatType nat_type = 3;               // Peer's NAT type
  ControlPermissions control_permissions = 8;  // Allowed permissions
}
```

**Purpose:** Response from server with peer address for hole punching.

### Session Messages

Defined in `libs/hbb_common/protos/message.proto`

#### LoginRequest

```
message LoginRequest {
  string username = 1;         // Username (peer ID)
  bytes password = 2;          // Password hash
  string my_id = 4;            // Client's peer ID
  string my_name = 5;          // Client's name
  OptionMessage option = 6;    // Session options
  oneof union {
    FileTransfer file_transfer = 7;
    PortForward port_forward = 8;
    ViewCamera view_camera = 15;
    Terminal terminal = 16;
  }
  bool video_ack_required = 9;
  uint64 session_id = 10;
  string version = 11;
}
```

**Purpose:** Authenticate and establish session.

#### LoginResponse

```
message LoginResponse {
  oneof union {
    string error = 1;          // Error message
    PeerInfo peer_info = 2;    // Remote peer information
  }
}
```

**Success:** Returns PeerInfo with displays, supported codecs.

**Failure:** Returns error string (e.g., "Wrong Password", "Offline").

### Video Messages

#### VideoFrame

```
message VideoFrame {
  oneof union {
    EncodedVideoFrames vp9s = 6;   // VP9 frames
    EncodedVideoFrames h264s = 10; // H264 frames
    EncodedVideoFrames h265s = 11; // H265 frames
    EncodedVideoFrames vp8s = 12;  // VP8 frames
    EncodedVideoFrames av1s = 13;  // AV1 frames
    RGB rgb = 7;                   // Raw RGB (legacy)
    YUV yuv = 8;                   // Raw YUV (legacy)
  }
  int32 display = 14;              // Display index
}
```

#### EncodedVideoFrame

```
message EncodedVideoFrame {
  bytes data = 1;   // Encoded frame data
  bool key = 2;     // Is key frame
  int64 pts = 3;    // Presentation timestamp
}
```

### Input Messages

#### MouseEvent (custom binary)

Mouse events are sent as binary data with format:

| Field | Size | Description |
|-------|------|-------------|
| type | 4 bytes | Event type (move/down/up/wheel) |
| x | 4 bytes | X coordinate |
| y | 4 bytes | Y coordinate |
| buttons | 4 bytes | Button mask |

#### KeyEvent (custom binary)

Keyboard events:

| Field | Size | Description |
|-------|------|-------------|
| keycode | 4 bytes | Hardware keycode |
| down | 1 byte | True if pressed |
| modifiers | 4 bytes | Shift/Ctrl/Alt mask |

## Codec Negotiation

### SupportedCodec Discovery

Client sends `SupportedEncoding` in LoginRequest.option:

```
message SupportedEncoding {
  bool h264 = 1;
  bool h265 = 2;
  bool vp8 = 3;
  bool av1 = 4;
  CodecAbility i444 = 5;  // I444 chroma support
}
```

Server responds with `PeerInfo.encoding` indicating available codecs.

### Codec Selection Order

1. **Hardware codec preference:** If both support hwcodec, use hardware encoder
2. **AV1:** Highest quality, best compression (if supported)
3. **VP9:** Default choice, good quality/compression balance
4. **H264/H265:** Hardware acceleration available on most platforms
5. **VP8:** Legacy fallback

### Codec Configuration

Quality levels controlled via `OptionMessage.image_quality`:
- **Best:** Maximum bitrate, highest quality
- **Balanced:** Default, 50% quality/bitrate
- **Low:** Minimal bitrate, basic quality
- **Custom:** User-defined bitrate ratio

## Connection Lifecycle

### 1. Registration Phase

```
Client → Server: RegisterPeer(id, serial)
Server → Client: RegisterPeerResponse(request_pk)
```

If `request_pk=true`, client sends public key:

```
Client → Server: RegisterPk(id, uuid, pk)
Server → Client: RegisterPkResponse(OK)
```

### 2. Connection Phase

```
Client → Server: PunchHoleRequest(target_id, nat_type, ...)
Server → Client: PunchHole(socket_addr, relay_server, ...)
```

**Direct connection attempt:**
- Client sends UDP packets to `socket_addr`
- Peer responds with UDP packets (hole punching)

**Relay fallback:**
- If direct fails, connect to `relay_server`
- TCP relay for symmetric NAT

### 3. Authentication Phase

```
Client → Peer: LoginRequest(username, password, option)
Peer → Client: LoginResponse(peer_info)
```

### 4. Session Phase

- Video frames streamed continuously
- Input events sent as needed
- Control messages (display change, quality adjust)

### 5. Termination Phase

```
Client → Peer: Close connection
Peer → Client: Acknowledge close
```

## Wire Format

### Protobuf Encoding

All messages use standard protobuf encoding:
- Field number + wire type (varint prefix)
- Payload length (varint)
- Payload data

### Binary Frame Transport

Video frames may be sent via:
- **TCP:** Reliable, ordered delivery
- **UDP:** Faster, may drop frames

For UDP, each packet has:
- Sequence number (4 bytes)
- Frame type (1 byte)
- Payload

## Security

### Password Hashing

Password is hashed before transmission:
- PBKDF2 with SHA256
- Salt derived from peer ID
- 10,000 iterations

### Public Key Exchange

Each peer has RSA key pair:
- Private key stored locally
- Public key registered with server
- Used for secure handshake

### Permission Control

Server sends `ControlPermissions` bitmask:
- keyboard: Allow keyboard input
- clipboard: Allow clipboard sync
- file: Allow file transfer
- terminal: Allow terminal service
- etc.

Client must respect permissions - violations terminate session.

## References

- Protobuf definitions: `libs/hbb_common/protos/*.proto`
- Implementation: `src/client.rs`, `src/server.rs`
- NAT traversal: `src/rendezvous_mediator.rs`

---
*Protocol version matches RustDesk 1.4.6 release*