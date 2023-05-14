#!/bin/sh
docker buildx bake --set "*.platform=linux/arm64/v8,linux/amd64" --push
