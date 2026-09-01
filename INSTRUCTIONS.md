# Running the Experiment

To run the *ParrotProject* experiment, you do not need to clone this repository
to run the experiment: all the required images are published on the registry at 
https://hub.docker.com/repositories/gianlucabresolin, so
it is enough to download the `docker-compose.yaml` file and run it. 

## Prerequisites

- Docker Engine (or Docker Desktop) installed
- Docker Compose (bundled with recent Docker versions as `docker compose`)

Before you continue, please make sure that you have Docker and Compose
installed. Please refer to https://docs.docker.com/get-docker/ for
documentation on how to install Docker. 

## Setup

Download the `docker-compose.yaml` file to an empty folder on your machine.

## Running the scenario

From the folder containing `docker-compose.yaml`, to enable cross-platform CPU
architecture emulation, run the following command 
```bash
docker run --privileged --rm tonistiigi/binfmt --install all 
```

Finally, one of the following:

```bash
docker compose --profile default up
```
This will runs the experiment with the regular images: the Virtual CAN Bus, the
victim ECU, the attacker ECU (spoofing the victim's CAN ID), and the led-device
reacting to frames on that ID. 

```bash
docker compose --profile debug-state up
```
This variant additionally prints the ECU's CAN Bus state together with the TEC
and REC values, every time they are updated. 

```bash
docker compose --profile debug-verbose up
```
This variant additionally prints the transmission errors, the tx bits
(specifying whether they are part of the IFS or of an error  delimiter), and the
bit transmitted on the virtual CAN bus.

Only one profile should be run at a time.  
To stop the scenario, press `Ctrl+C` or run 
`docker compose --profile <default|debug> down` in the same folder.  

Note that the testing scenario requires around 1 min to complete (2815 ticks,
with a virtual can bus that outputs a bit every 20ms to ensure clean logs).