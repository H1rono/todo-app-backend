FROM mariadb:10.9.4

ENV LANG=ja_JP.UTF-8

COPY ./migrations /docker-entrypoint-initdb.d

HEALTHCHECK --interval=10s --timeout=5s --start-period=5s --retries=5 CMD [ "mysqladmin", "ping" ]
