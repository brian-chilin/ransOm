# build
FROM rust:1.82-bullseye AS builder
RUN apt-get update && apt-get install -y curl && \
    curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
COPY . .
RUN wasm-pack build --target web --release


# run
FROM nginx:stable-bullseye AS runtime
COPY --from=builder /workspace/ /usr/share/nginx/html/
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
