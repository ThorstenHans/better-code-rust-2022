FROM rust:1.65 AS build
WORKDIR /opt/build
RUN rustup target add wasm32-wasi 
COPY . .
RUN cargo build --target wasm32-wasi --release
RUN rm spin.toml && mv spin.container.toml spin.toml

FROM scratch
COPY --from=build /opt/build/target/wasm32-wasi/release/redurce_product_price.wasm .
COPY --from=build /opt/build/spin.toml .

