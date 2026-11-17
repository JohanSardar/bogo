# bogo-relay, built from this repository. It serves plain HTTP on port 8080 for
# HTTPS in front of it: compose.yaml's Caddy, a tunnel, or a cloud service's
# own HTTPS. Clients only ever speak HTTPS to the relay. Set BOGO_RELAY_SERVICE
# to the host:port clients use (compose.yaml does this from DOMAIN).
#   docker build -f deploy/relay/Dockerfile -t bogo-relay .
FROM rust:1.95-slim-trixie AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p bogo-relay

FROM debian:trixie-slim
COPY --from=build /src/target/release/bogo-relay /usr/local/bin/bogo-relay
RUN useradd --system --uid 10001 --home-dir /data bogo-relay \
    && mkdir -p /data && chown bogo-relay /data && chmod 700 /data
USER bogo-relay
VOLUME /data
EXPOSE 8080
ENTRYPOINT ["bogo-relay", "--listen", "0.0.0.0:8080", "--database", "/data/relay.sqlite"]
