FROM rust:1.65.0

WORKDIR /app

COPY . .

RUN cargo install --path .

CMD ["dev-environment-cli"]