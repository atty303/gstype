FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:latest

RUN mkdir /tmp/deleteme \
 && cd /tmp/deleteme \
 && cargo init \
 && cargo add serde \
 && rm -rf /tmp/deleteme
