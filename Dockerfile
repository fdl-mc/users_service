####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=users_microservice
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /users_microservice

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /users_microservice

# Copy our build
COPY --from=builder /users_microservice/target/x86_64-unknown-linux-musl/release/users_microservice ./

# Use an unprivileged user.
USER users_microservice:users_microservice

CMD ["/users_microservice/users_microservice"]
