#!/usr/bin/bash
mkdir -p ./nginx_ssl/private
mkdir -p ./nginx_ssl/certs
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout ./nginx_ssl/private/localhost.key -out ./nginx_ssl/certs/localhost.crt