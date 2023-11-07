FROM public.ecr.aws/lambda/provided:al2 as rust-al2
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN yum install -y gcc
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none
ENV PATH $PATH:/root/.cargo/bin
RUN rustup install 1.73 && \
    cargo install cargo-chef

FROM rust-al2 as planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust-al2 as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
ENV RUST_BACKTRACE=full
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust-al2 as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --locked

FROM public.ecr.aws/lambda/provided:al2
WORKDIR /app
COPY --from=builder /app/target/release/bootstrap /var/runtime/bootstrap
CMD ["placeholder"]
