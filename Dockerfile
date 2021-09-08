FROM ekidd/rust-musl-builder:nightly-2021-02-13 as cargo-build
COPY Cargo.toml Cargo.toml
RUN mkdir -p src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f ./target/x86_64-unknown-linux-musl/release/deps/holdmybackup*
COPY . .
RUN cargo build --release


FROM gcr.io/distroless/static:nonroot
COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/holdmybackup .
ENTRYPOINT ["./saphire"]
