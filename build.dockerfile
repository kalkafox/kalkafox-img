FROM alpine

WORKDIR /bin

COPY target/release/kalkafox-img /bin/kalkafox-img

COPY frontend/dist /frontend

CMD ["/bin/kalkafox-img"]