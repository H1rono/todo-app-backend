FROM rust:1.66-buster

WORKDIR /usr/src/todo-app-backend
COPY . .

RUN cargo install --path .

CMD [ "todo-app-backend" ]
