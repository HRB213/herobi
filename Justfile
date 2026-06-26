set shell := ["bash", "-cu"]

default:

    @just --list

docker-build:

    docker build -t herobi -f Containerfile .

docker-run:

    docker run --rm herobi

docker-build-run:

    docker build -t herobi -f Containerfile .

    docker run --rm herobi