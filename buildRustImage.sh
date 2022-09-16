#!/usr/bin/env bash
set -e; set -o pipefail;
nix build '.#rust-docker'
image=$((docker load < result) | sed -n '$s/^Loaded image: //p')
docker image tag "$image" proto-rust:latest