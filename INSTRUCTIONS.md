# Running the Experiment

To run the *ParrotProject* experiment, you do not need to clone this repository
to run the experiment: all the required images are published on the registry at 
https://hub.docker.com/repositories/gianlucabresolin, so
it is enough to download the `docker-compose.yaml` file and run it. 

## Prerequisites

- Docker Engine (or Docker Desktop) installed
- Docker Compose (bundled with recent Docker versions as `docker compose`)

## Setup

Download the `docker-compose.yaml` file to an empty folder on your machine.

## Running the scenario

From the folder containing `docker-compose.yaml`, run one of the following:

```bash
docker compose --profile default up
```

```bash
docker compose --profile debug-state up
```

```bash
docker compose --profile debug-verbose up
```

This will runs the experiment with the regular images: the Virtual CAN Bus, the
victim ECU, the attacker ECU (spoofing the victim's CAN ID), and the led-device
reacting to frames on that ID. 

Both the commands will run the same scenario, but the `--profile debug` variant
uses the images that additionally print detailed internal logs (e.g. bit-level
transmission/reception events, Parrot Defense state transitions, ECU's CAN Bus
state), useful for inspecting the experiment scenario step by step rather than
just the application-level output.

Only one profile should be run at a time.   
To stop the scenario, press `Ctrl+C` or run 
`docker compose --profile <default|debug> down` in the same folder.  