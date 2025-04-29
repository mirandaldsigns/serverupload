FROM rust:latest

WORKDIR /app
COPY upload-server ./upload-server
WORKDIR /app/upload-server

RUN cargo build --release

EXPOSE 3000
CMD ["./target/release/upload-server"]
