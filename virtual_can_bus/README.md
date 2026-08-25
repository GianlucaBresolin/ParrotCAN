# *ParrotProject: Virtual CAN Bus*

This module implements a virtual CAN bus in Rust to simulate communication
between ECUs in a containerized environment.

## What it does

The service accepts TCP connections from devices, registers the controllers, and
synchronizes the bus state. Each device sends one bit (`0` or `1`), and the
server determines the global bus value according to CAN wired-AND logic:

- if at least one device transmits `0`, the bus is considered `0` (dominant bit)
- if all devices transmit `1`, the bus value is `1` (recessive bit)

This allows simulating bus behavior without real hardware.

## How it works

The server:

1. opens a TCP listener on `0.0.0.0:8080`
2. accepts controller connections
3. stores the registered clients
4. collects one bit from each device
5. waits for the end of the bus round
6. computes the bus state
7. sends that state back to all connected clients

The cycle is managed with `tokio` and a `Notify` to synchronize when all
controller bits have been received. 

## Listening port

The service listens on TCP: `0.0.0.0:8080`

The port is exposed in the `Dockerfile` with:

```dockerfile
EXPOSE 8080
```

## Container build

To build the virtual bus image:

```bash
docker build -t virtual-can-bus ./virtual_can_bus
```

To start the full project setup, at the root of the project run:

```bash
docker compose up --build
```
