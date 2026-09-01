# ParrotProject

ParrotProject is a cyber-physical security demonstrator developed for the course
"Cyber-Physical Systems and IoT Security" at the University of Padua. 

The project simulates a CAN-based embedded system in Docker, with a virtual CAN
bus and multiple ECUs that communicate, transmit frames, and react to traffic in
a controlled environment. It is designed to reproduce a realistic attack
scenario in which an attacker ECU spoofs a victim identifier and triggers a
local device response.

## Project goal

The objective is to study how CAN-like communication can be emulated in software
and how malicious behavior such as spoofing or frame-triggered reactions can be
observed safely in a laboratory setup. In particular, the project implements the
Parrot defense algorithm to force a malicious ECU off the bus by exploiting the
dominant-recessive behavior of CAN, thereby demonstrating a practical
software-only countermeasure against bus-level spoofing attacks.

## High-level architecture

- `virtual_can_bus/`: emulates the shared bus behavior and distributes the bus
  state to connected ECUs 
- `device/`: contains the simulated ECUs, including the victim, the attacker,
  and the LED-reactive device 
- `docker-compose.yml`: orchestrates the full experiment setup 

## How to run

From the project root:

```bash
docker run --privileged --rm tonistiigi/binfmt --install all
docker compose --profile default up
```

Available profiles:
- `default`: standard scenario
- `debug-state`: includes additional state dumps
- `debug-verbose`: includes detailed transmission diagnostics

For more details, see [Instructions.md](Instructions.md) and the dedicated
virtual bus documentation in
[virtual_can_bus/README.md](virtual_can_bus/README.md) and
[device/README.md](device/README.md).  

## Repository structure

- [virtual_can_bus/](virtual_can_bus/): virtual CAN bus implementation
- [device/](device/): simulated ECU/device components
- [docs/](docs/): Parrot paper, Project Report and some experiment's assets
- [Instructions.md](Instructions.md): execution instructions
- [docker-compose.yml](docker-compose.yml): container orchestration
