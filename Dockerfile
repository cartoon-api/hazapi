# Etapa 1: Build da aplicação
FROM rust:1.78 AS builder
WORKDIR /app

# Copia arquivos de dependências primeiro (para cache)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copia o restante do código e compila de verdade
COPY . .
RUN cargo build --release

# Etapa 2: Imagem final leve
FROM debian:bookworm-slim
WORKDIR /app

# Copia o binário compilado
COPY --from=builder /app/target/release/hazapi ./hazapi

# Porta que a API vai expor
EXPOSE 8080

# Comando para rodar a API
CMD ["./hazapi"]

