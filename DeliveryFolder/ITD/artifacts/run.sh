#!/bin/bash
docker image load -i clup-frontend.tgz
docker image load -i clup-backend.tgz
docker-compose up