# Builder Stage
FROM rust:latest AS builder
ENV SQLX_OFFLINE=true
WORKDIR /app

# Only copy over the Cargo manifest files
COPY ./Cargo.toml ./Cargo.lock ./

# This trick will cache the dependencies as a separate layer
RUN mkdir src/ \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -f target/release/deps/tootodo_be*

COPY . .

# Build the actual application
RUN cargo build --release --locked
RUN cargo install sqlx-cli --no-default-features --features postgres

# Production Stage
FROM ubuntu:latest AS runner
ARG APP=/usr/src/app
ENV TZ=Etc/UTC APP_USER=appuser

# Install only the runtime dependencies
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/tootodo-be ${APP}/tootodo-be

# Ensure the user owns the application files
RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./tootodo-be"]
EXPOSE 8000