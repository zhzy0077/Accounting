FROM rust:1.31 as builder

WORKDIR /usr/src/accounting
COPY . .

RUN cargo --version
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cd client && wasm-pack build --target web --release
RUN cd server && cargo build --release

FROM debian:latest

WORKDIR /accounting

RUN apt update
RUN apt install -y sqlite3 libsqlite3-dev
COPY --from=builder /usr/src/accounting/db /accounting/db
COPY --from=builder /usr/src/accounting/target/release/server /accounting/server
COPY --from=builder /usr/src/accounting/client/www/ /accounting/static/
COPY --from=builder /usr/src/accounting/client/pkg/client.js /accounting/static/client.js
COPY --from=builder /usr/src/accounting/client/pkg/client_bg.wasm /accounting/static/client_bg.wasm

CMD ["./server"]