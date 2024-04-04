## Builder Stage
FROM rust:latest AS builder
ENV SQLX_OFFLINE=true

# Create a new Rust project
WORKDIR /app

# Copy the source code and build the application
COPY . .
RUN cargo build --release --locked

## Production Stage
FROM debian:bookworm-slim AS runner

ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /app/target/release/tootodo-be ${APP}/tootodo-be

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./tootodo-be"]
EXPOSE 8000

# # Build stage
# FROM rust:bookworm AS builder
 
# WORKDIR /app
# COPY . .

# # Copy sqlx-data.json for offline mode
# COPY .sqlx/ ./.sqlx/

# ENV SQLX_OFFLINE true

# RUN cargo build --release
 
# # Final run stage
# FROM debian:bookworm-slim AS runner
 
# WORKDIR /app
# COPY --from=builder /app/target/release/tootodo-be /app/tootodo-be

# # Set the DATABASE_URL environment variable
#  ENV DATABASE_URL=postgresql://tootodo_owner:35GozmHTEYQu@ep-tight-salad-a18zqnr9.ap-southeast-1.aws.neon.tech/tootodo?sslmode=require

# CMD ["/app/tootodo"]

# ## Builder Stage
# FROM rust:latest AS builder
# ENV SQLX_OFFLINE=true

# # Create a new Rust project
# RUN USER=root cargo new --bin tootodo-be
# WORKDIR /tootodo-be

# # Copy and build dependencies
# COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release --locked
# RUN rm src/*.rs

# # Copy the source code and build the application
# COPY . .
# RUN cargo build --release --locked

# ## Production Stage
# FROM debian:bookworm-slim AS runner

# ARG APP=/usr/src/app
# RUN apt-get update \
#     && apt-get install -y ca-certificates tzdata \
#     && rm -rf /var/lib/apt/lists/*

# ENV TZ=Etc/UTC \
#     APP_USER=appuser

# RUN groupadd $APP_USER \
#     && useradd -g $APP_USER $APP_USER \
#     && mkdir -p ${APP}

# COPY --from=builder /tootodo-be/target/release/tootodo-be ${APP}/tootodo-be

# RUN chown -R $APP_USER:$APP_USER ${APP}
# USER $APP_USER
# WORKDIR ${APP}

# ENTRYPOINT ["./tootodo-be"]
# EXPOSE 8000
