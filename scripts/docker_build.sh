#/bin/bash

set -e

echo Building image
docker build -t portfolio:latest .
