# Tahap 1: Setup Cargo Chef untuk caching dependency
FROM rust:1.85 AS chef
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Tahap 2: Build dependencies sekali saja (cached)
FROM rust:1.85 AS builder
WORKDIR /app
COPY --from=chef /app/recipe.json recipe.json
RUN cargo install cargo-chef
RUN cargo chef cook --release --recipe-path recipe.json

# Tahap 3: Build aplikasi utama (tanpa build ulang dependency)
COPY . .
RUN cargo build --release

# Tahap 4: Gunakan runtime yang lebih ringan
FROM debian:bookworm-slim
WORKDIR /app

# Install OpenSSL 3 dan sertifikat SSL di runtime stage
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/crypto-fun /app/

# Expose port Actix Web
EXPOSE 8080

# Jalankan aplikasi
CMD ["/app/crypto-fun"]
