FROM node:14-alpine as webBuild

WORKDIR /webapp

COPY ./web .
RUN npm i
RUN npm run build

FROM rust:latest as backendBuild

WORKDIR /usr/src/backend

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential git clang llvm-dev libclang-dev libssl-dev pkg-config libpq-dev musl-tools brotli

COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine:3 as final
WORKDIR /app
COPY --from=webBuild /webapp/build ./build
COPY --from=backendBuild /usr/local/cargo/bin/PaintShare .

COPY ./migrations ./migrations
COPY ./sqlx-data.json .
COPY ./data/default_profile_picture.jpg ./data/default_profile_picture.jpg
RUN mkdir ./data/posts
RUN mkdir ./data/profile_pictures


EXPOSE 8080
CMD ["./PaintShare"]