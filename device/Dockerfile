FROM rust:1.85-bookworm
 
RUN rustup target add thumbv7m-none-eabi
 
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    git \
    ninja-build \
    libglib2.0-dev \
    libpixman-1-dev \
    libfdt-dev \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*
 
COPY Cargo.toml Cargo.lock memory.x ./
COPY .cargo ./.cargo
COPY app ./app
COPY communication_component ./communication_component
COPY local_device_manager ./local_device_manager
COPY simulated_led_driver ./simulated_led_driver
COPY qemu-custom/build_linux/qemu-system-arm ./qemu-custom/build/qemu-system-arm-unsigned
 
ARG MY_IDS
ARG INTERESTED_IDS
ARG ROLE
ENV MY_IDS=${MY_IDS}
ENV INTERESTED_IDS=${INTERESTED_IDS}
ENV ROLE=${ROLE}

RUN cargo build --release
 
CMD ["cargo", "run", "--release"]