default:
  just --list


build:
    ./buildRustImage.sh


deploy:
    flyctl deploy --image proto-rust:latest --local-only 