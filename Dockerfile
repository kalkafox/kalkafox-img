FROM alpine

RUN apk add --no-cache cargo nodejs python3

WORKDIR /build

COPY . .

RUN cargo build --release

RUN npm install --prefix ./frontend

RUN npm run build --prefix ./frontend

WORKDIR /app

RUN cp /build/target/release/kalkafox-img .

RUN mv /build/frontend/dist ./frontend

RUN rm -fR /build

# Clean APK cache
RUN rm -rf /var/cache/apk/*

CMD ["/app/kalkafox-img"]