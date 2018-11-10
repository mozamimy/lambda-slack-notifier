FROM lambci/lambda:go1.x

USER root

ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH

RUN yum -y update
RUN rpm --rebuilddb && yum -y groupinstall "Development Tools"

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

WORKDIR /workspace

ENTRYPOINT []
