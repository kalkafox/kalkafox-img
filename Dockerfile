FROM alpine

WORKDIR /build

COPY . .

RUN apk add --no-cache nodejs python3 cargo rust npm gcc && cargo build --release && npm --prefix frontend install && npm --prefix frontend run build && mv frontend/dist /frontend && mv target/release/kalkafox-img /bin/kalkafox-img

FROM alpine

RUN apk add --no-cache libgcc

COPY --from=0 /frontend /frontend
COPY --from=0 /bin/kalkafox-img /bin/kalkafox-img
CMD ["/bin/kalkafox-img"]
