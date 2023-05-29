FROM rust:1.67 as builder
WORKDIR /usr/src/gip
COPY ./gip .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/gip /usr/local/bin/gip
ENTRYPOINT [ "/usr/local/bin/gip" ]