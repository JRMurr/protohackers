default:
  just --list


build:
    ./buildRustImage.sh

deploy:
    flyctl deploy --image proto-rust:latest --local-only --strategy immediate

buildDeploy:
    just build
    just deploy