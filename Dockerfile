# Build stage
FROM rust:bookworm AS builder
 
WORKDIR /app
COPY . .

# Copy sqlx-data.json for offline mode
COPY .sqlx/ ./.sqlx/

ENV SQLX_OFFLINE true

RUN cargo build --release
 
# Final run stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /app
COPY --from=builder /app/target/release/tootodo-be /app/tootodo-be

# Set the DATABASE_URL environment variable
 ENV DATABASE_URL=postgres://koyeb-adm:J7pBgCtqy9Ve@ep-ancient-union-a1thb6qd.ap-southeast-1.pg.koyeb.app/koyebdb

CMD ["/app/tootodo-be"]