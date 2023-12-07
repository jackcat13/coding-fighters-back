FROM rust:slim-buster as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/coding-fighters-back /usr/local/bin/coding-fighters-back
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["coding-fighters-back"]