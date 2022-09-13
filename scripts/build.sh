#!/bin/bash

git pull

docker compose up -d --build --force-recreate --remove-orphans