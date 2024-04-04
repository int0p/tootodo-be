## Builder Stage
FROM rust:latest AS builder

ENV SQLX_OFFLINE=true

WORKDIR /app

# Copy Cargo.toml and Cargo.lock first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs and build the dependencies
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release --locked

# Copy the source code and build the application
COPY . .
RUN cargo build --release --locked

## Production Stage
FROM ubuntu:latest AS runner

ARG APP=/usr/src/app

RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER && \
    useradd -g $APP_USER $APP_USER && \
    mkdir -p ${APP}

COPY --from=builder /app/target/release/tootodo-be ${APP}/tootodo-be

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./tootodo-be"]
EXPOSE 8000