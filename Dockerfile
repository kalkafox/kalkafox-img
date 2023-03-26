FROM alpine

RUN apk add --no-cache curl bash nodejs npm build-base

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir /app

WORKDIR /app

COPY . .

WORKDIR /app/frontend

RUN npm install

RUN npx vite build

WORKDIR /app

RUN cargo build --release

CMD ["/app/target/release/kalkafox-img"]
