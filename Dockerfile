FROM ekidd/rust-musl-builder:nightly-2021-02-13 as cargo-build
COPY . .
RUN cargo build --release


FROM gcr.io/distroless/static:nonroot
COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/holdmybackup .
ENTRYPOINT ["./holdmybackup"]
