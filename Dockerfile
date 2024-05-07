# Use the main official rust docker image
ARG RUST_VERSION=alpine
FROM rust:${RUST_VERSION} AS builder

#Set the work directory
WORKDIR /negrdo

# Copy the app into the docker image
COPY . /negrdo

#Build the app
RUN apk add --no-cache musl-dev  \
    && chmod -R 755 /usr/local/bin \
    && cargo build --release \
    && cp ./target/release/rust-tera /usr/local/bin \
    && rm -rf /negrdo/Cargo.toml /negrdo/Cargo.lock /negrdo/src /negrdo/target

EXPOSE 8080

#Start the application
CMD ["/usr/local/bin/rust-tera"]

# docker build -f Dockerfile .
# docker run -d -p 8080:8080 --name rust-tera rust-tera