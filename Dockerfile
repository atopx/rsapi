FROM rust:1.93.0-bookworm AS builder

# 配置 Cargo 使用阿里云镜像源 (Sparse Index)
RUN mkdir -p $CARGO_HOME \
    && echo '[source.crates-io]' > $CARGO_HOME/config.toml \
    && echo 'replace-with = "aliyun"' >> $CARGO_HOME/config.toml \
    && echo '[source.aliyun]' >> $CARGO_HOME/config.toml \
    && echo 'registry = "sparse+https://mirrors.aliyun.com/crates.io-index/"' >> $CARGO_HOME/config.toml

WORKDIR /app

# 复制依赖定义文件
COPY Cargo.toml Cargo.lock ./

# 创建 Dummy 主文件以预构建依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖
RUN cargo build --release

# 删除 Dummy 文件
RUN rm -rf src

# 复制实际源代码
COPY src src

# 更新 main.rs 时间戳，确保重新编译
RUN touch src/main.rs

# 构建正式二进制文件
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

# 配置 apt 使用阿里云镜像源
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources

# 安装运行时依赖
# ca-certificates: 用于 HTTPS 请求
# libssl-dev/openssl: 数据库连接等可能需要
# RUN apt-get update \
#     && apt-get install -y --no-install-recommends ca-certificates \
#     libssl-dev \
#     penssl \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从 builder 阶段复制编译好的二进制文件
COPY --from=builder /app/target/release/api /app/server

# 运行应用
CMD ["./server"]
