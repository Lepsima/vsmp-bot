FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /bot

COPY target/release/discord-bot .

CMD ["./discord-bot"]