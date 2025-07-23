# Talk++ Runtime Container
FROM rust:1.77-alpine AS builder

# Install dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Set working directory
WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY compiler/ compiler/
COPY runtime/ runtime/
COPY auth/ auth/
COPY executor/ executor/
COPY simulator/ simulator/
COPY wrappers/ wrappers/
COPY cli/ cli/
COPY api-server/ api-server/
COPY frontend-server/ frontend-server/

# Build release binaries
RUN cargo build --release

# Runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    python3 \
    py3-pip \
    nodejs \
    npm \
    bash \
    curl \
    jq

# Create app user
RUN addgroup -g 1001 -S talkpp && \
    adduser -S talkpp -u 1001 -G talkpp

# Create directories
RUN mkdir -p /app/bin /app/functions /app/logs /app/config && \
    chown -R talkpp:talkpp /app

# Copy binaries from builder
COPY --from=builder /app/target/release/talkppc /app/bin/
COPY --from=builder /app/target/release/talkpprun /app/bin/
COPY --from=builder /app/target/release/talkpp-api-server /app/bin/
COPY --from=builder /app/target/release/talkpp-frontend-server /app/bin/

# Copy configuration
COPY config/ /app/config/
COPY examples/ /app/examples/

# Set PATH
ENV PATH="/app/bin:${PATH}"

# Switch to app user
USER talkpp
WORKDIR /app

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
EXPOSE 8080
CMD ["talkpp-api-server"] 