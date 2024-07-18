ARG RUST_VERSION=1.79.0
ARG APP_NAME=api-rs

FROM tonistiigi/xx:1.3.0 AS xx
FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app
COPY --from=xx / /
RUN apk add --no-cache clang file git lld musl-dev upx
ARG TARGETPLATFORM
RUN xx-apk add --no-cache musl-dev gcc
ADD . .
RUN xx-cargo build --bin ${APP_NAME} --locked --release --target-dir ./target && \
    upx --best ./target/$(xx-cargo --print-target-triple)/release/${APP_NAME} && \
    cp ./target/$(xx-cargo --print-target-triple)/release/${APP_NAME} /bin/server && \
    xx-verify /bin/server

FROM scratch
ADD assets /assets
ADD data /data
ADD templates /templates
COPY --from=build /bin/server /bin/
EXPOSE 8080
ENTRYPOINT [ "/server" ]
