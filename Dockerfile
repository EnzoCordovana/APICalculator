# Dockerfile optimisé pour le développement
FROM rust:1.83

# Installer cargo-watch pour hot reload
RUN cargo install --force cargo-watch

WORKDIR /app

COPY app/ /app/

# Pré-télécharger les dépendances (cache Docker)
RUN cargo build

EXPOSE 8080

# Commande par défaut avec hot reload
CMD ["cargo", "watch", "-x", "run"]