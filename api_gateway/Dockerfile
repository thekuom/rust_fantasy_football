FROM rust:1.40.0

WORKDIR /usr/src/app
COPY . .

RUN cargo build

CMD ["cargo", "run"]
