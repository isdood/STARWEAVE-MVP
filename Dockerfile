# #FFD700 Agent Deployment
FROM rust:1.78-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# #9400D3 Minimal runtime
COPY --from=builder /app/target/release/starweave-mvp /usr/local/bin/starweave-mvp
CMD ["starweave-mvp"]
