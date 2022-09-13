FROM rust:1.63.0 AS server_builder
WORKDIR /poker_rooms

COPY src src
RUN cargo install --bin poker_rooms --path . --debug

FROM ubuntu:focal AS runner

WORKDIR /poker_rooms

COPY --from=server_builder /usr/local/cargo/bin/poker_rooms /usr/local/bin/poker_rooms

CMD ["poker_rooms"]