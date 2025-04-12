FROM rust:1-alpine3.20 AS rust-builder

RUN sed -i 's/https:\/\/dl-cdn.alpinelinux.org/http:\/\/mirrors.ustc.edu.cn/g' /etc/apk/repositories && \
    apk add --no-cache musl-dev openssl-dev

RUN mkdir ~/.cargo/ && touch ~/.cargo/config \
    && echo '[source.crates-io]' > ~/.cargo/config \
    && echo 'registry = "https://github.com/rust-lang/crates.io-index"'  >> ~/.cargo/config \
    && echo "replace-with = 'tuna'"  >> ~/.cargo/config \
    && echo '[source.tuna]'   >> ~/.cargo/config \
    && echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"'  >> ~/.cargo/config \
    && echo '[net]'   >> ~/.cargo/config \
    && echo 'git-fetch-with-cli = true'   >> ~/.cargo/config \
    && echo '' >> ~/.cargo/config

ENV RUSTFLAGS="-C target-feature=-crt-static -C target-cpu=native"

WORKDIR /app
COPY ./ /app
RUN cargo fetch

RUN cargo build --release && \
    ldd /app/target/release/rsapi && \
    strip /app/target/release/rsapi

FROM alpine:3.20
ENV TZ=Asia/Shanghai
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN sed -i 's/https:\/\/dl-cdn.alpinelinux.org/http:\/\/mirrors.ustc.edu.cn/g' /etc/apk/repositories && \
    apk add --no-cache libgcc tzdata && \
    cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
    && echo "Asia/Shanghai" > /etc/timezone \
    && apk del tzdata

COPY --from=rust-builder /app/target/release/rsapi .
ENTRYPOINT ["/rsapi"]
