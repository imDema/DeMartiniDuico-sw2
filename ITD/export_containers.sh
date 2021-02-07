#!/bin/bash
docker-compose build
docker tag itd_clup-frontend clup-frontend
docker tag itd_clup-backend clup-backend
docker save clup-frontend | pigz > ../DeliveryFolder/ITD/artifacts/clup-frontend.tgz
docker save clup-backend | pigz > ../DeliveryFolder/ITD/artifacts/clup-backend.tgz
docker image rm clup-frontend
docker image rm clup-backend