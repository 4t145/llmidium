FROM rust:1.85-slim as builder
RUN rustup target add x86_64-unknown-linux-musl
# 安装构建依赖channel --update
RUN apt-get update && apt-get install -y \
    build-essential \    
    pkg-config \    
    libssl-dev \   
    musl-tools \ 
    musl-dev \ 
    && rm -rf /var/lib/apt/lists/* 
# 设置工作目录
WORKDIR /usr/src/llmidium
# 先复制Cargo.toml和Cargo.lock（如果有）
COPY Cargo.toml ./
# 如果有Cargo.lock，取消下面的注释# 
COPY Cargo.lock ./
COPY libs/ libs/
# 创建一个临时的main.rs来构建依赖
RUN mkdir -p src && \    
    echo "fn main() {println!(\"placeholder\");}" > src/main.rs && \    
    cargo build --release && \    
    rm -rf src
# 复制实际的源代码
COPY src/ src/
# 重新构建（这次会因为缓存而快很多）
RUN cargo build --release  --target=x86_64-unknown-linux-musl

FROM nixos/nix as runtime 
ENV NIX_CONFIG="experimental-features = nix-command flakes"
COPY llmidium-system/etc/nixos /etc/nixos
RUN nix-channel --update && \
    nix-env -iA nixpkgs.nixos-install-tools
    # export PATH="/root/.nix-profile/bin:$PATH" && \
    # cd /etc/nixos && \
    # nixos-rebuild switch 
# 设置工作目录
WORKDIR /

# 从构建阶段复制二进制文件
COPY --from=builder /usr/src/llmidium/target/x86_64-unknown-linux-musl/release/llmidium /llmidium

# 设置最高权限
RUN chmod 4755 /llmidium

# 复制可能需要的配置文件或资源
# COPY config/ /app/config/
# COPY resources/ /app/resources/

# 修改所有权
# RUN chown -R llmidium:llmidium /llmidium

# 切换到非root用户
# USER llmidium

# 暴露需要的端口
EXPOSE 7777

# 设置启动命令，明确指定监听所有网络接口
CMD ["/llmidium", "--host", "0.0.0.0"]

