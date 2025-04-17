# docker rmi 33f6595dceff

# docker build -t mystu_server:latest .

# docker run --network host mystu_server
# 构建阶段
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 运行阶段
FROM ubuntu:24.04
WORKDIR /app
COPY --from=builder /app/target/release/mystu_server /app/
COPY ./public /app/public
CMD ["./mystu_server"]