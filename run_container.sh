#!/bin/bash

# 默认超时时间（秒）
DEFAULT_TIMEOUT=300

# 获取命令行参数
TIMEOUT=${1:-$DEFAULT_TIMEOUT}

# 确保宿主机上的目录存在
mkdir -p ./nixos_config

# 运行容器，并映射宿主机目录到容器的/nixos
CONTAINER_ID=$(docker run -d \
  --name llmidium \
  -p 7777:7777 \
  -v "$(pwd)/nixos_config:/nixos" \
  -e TIMEOUT=$TIMEOUT \
  llmidium:latest)

echo "容器已启动，Nix配置将保存在 $(pwd)/nixos_config 目录"
echo "Nix已配置启用实验性功能: nix-command flakes"
echo "MCP超时时间设置为: $TIMEOUT 秒"
echo "可以通过以下命令进入容器: docker exec -it $CONTAINER_ID bash"
