set shell := ["bash", "-uc"]
set positional-arguments


default:
  just --list

build:
    ./buildRustImage.sh

deploy:
    flyctl deploy --image proto-rust:latest --local-only --strategy immediate

buildDeploy:
    just build
    just deploy


@deployProb problem_num:
    flyctl deploy --image proto-rust:latest --local-only --strategy immediate --env PROBLEM=$1


@buildDeployProb problem_num:
    just build
    just deploy $1