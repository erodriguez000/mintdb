FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc:latest

COPY --from=builder /app/target/release/mint /

COPY --from=builder /app/src/ui /src/ui

EXPOSE 8000

CMD ["./mint"]