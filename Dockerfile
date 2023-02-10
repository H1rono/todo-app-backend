FROM rust:1.66-buster

WORKDIR /usr/src/todo-app-backend
COPY . .

ENV DOCKERIZE_VERSION=v0.6.1
RUN apt-get update && apt-get install -y wget \
    && wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

RUN cargo build

ENTRYPOINT [ "dockerize", "-wait", "tcp://todo-database:3306" ]
CMD [ "cargo", "run" ]
