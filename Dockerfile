FROM rust:1.63.0 AS server_builder
WORKDIR /poker_rooms

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo install --bin poker_rooms --path . --debug \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src src
RUN cargo install --bin poker_rooms --path . --debug

FROM ubuntu:focal AS runner

WORKDIR /poker_rooms

COPY index.html .
COPY --from=server_builder /usr/local/cargo/bin/poker_rooms /usr/local/bin/poker_rooms

CMD ["poker_rooms"]