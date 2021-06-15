FROM rust:latest AS builder

WORKDIR /usr/app/morgaine

COPY . .

RUN cargo build --release

FROM alpine

WORKDIR /app/morgaine

COPY --from=builder /usr/app/morgaine/ /app/morgaine/ 

CMD [ "./target/release/morgaine" ]