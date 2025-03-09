#!/bin/bash

CONTAINER_ID=$(docker run -d \
  --name llmidium \
  -p 7777:7777 \
  -v "$(pwd)/llmidium-system/mcp-server":"/nmt/mcp-server" \
  llmidium:latest)


echo "可以通过以下命令进入容器: docker exec -it $CONTAINER_ID bash"
