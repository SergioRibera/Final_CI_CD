FROM alpine:latest

RUN apk add --no-cache \
    gcompat \
    libc6-compat \
    libstdc++

WORKDIR /app
COPY simple_server /app

CMD [ "/app/simple_server" ]
