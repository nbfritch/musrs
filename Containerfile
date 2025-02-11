FROM docker.io/oven/bun:1-slim as uibuilder
WORKDIR /app
COPY ui ui/
WORKDIR /app/ui
RUN bun install
RUN bun run build
RUN ls --recursive .

FROM docker.io/rustlang/rust:nightly-slim as backendbuilder
WORKDIR /app
COPY --from=uibuilder /app/ui/dist/ ./ui/dist
COPY .sqlx .sqlx/
COPY migrations/ ./migrations
COPY src/ ./src
COPY build.rs .
COPY Cargo.toml .
COPY Cargo.lock .
ENV IS_CONTAINER_BUILD="true"
RUN ls -a --recursive .
ENV SQLX_OFFLINE="true"
RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim as runner
WORKDIR /app
COPY --from=backendbuilder /app/target/release/musrs .
ENV WEB_ADDR=0.0.0.0
ENV WEB_PORT=8080
ENV MUS_DIR=/app/mus_dir
ENV DATABASE_URL=sqlite://library.db
EXPOSE 8080
ENTRYPOINT ["./musrs"]
