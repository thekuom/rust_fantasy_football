FROM rust:1.40.0

RUN apt-get update && apt-get install -y postgresql-client

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app
COPY . .

CMD ["cargo", "run"]
