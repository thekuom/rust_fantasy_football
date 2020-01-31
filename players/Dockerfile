FROM rust:1.40.0

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app
COPY . .

CMD ["cargo", "run"]
