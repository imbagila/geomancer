FROM rust:latest AS builder

ARG BOT_TOKEN
ARG BOT_USERNAME
ARG BOT_MESSAGE
ARG BOT_MEMBERS

ENV BOT_TOKEN=${BOT_TOKEN}
ENV BOT_USERNAME=${BOT_USERNAME}
ENV BOT_MESSAGE=${BOT_MESSAGE}
ENV BOT_MEMBERS=${BOT_MEMBERS}

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y ca-certificates openssl pkg-config libssl-dev musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=geomancer
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /geomancer

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /geomancer

# Copy our build
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /geomancer/target/x86_64-unknown-linux-musl/release/geomancer ./

# Use an unprivileged user.
USER geomancer:geomancer

CMD ["/geomancer/geomancer"]
