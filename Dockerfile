FROM scratch
ADD assets /assets
ADD data /data
ADD target/x86_64-unknown-linux-musl/release/api-rs /server
ENTRYPOINT [ "/server" ]
