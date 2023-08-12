# Work on this 

FROM rust:1.71.1-alpine3.17

COPY ./target/release/backend /usr/bin/

EXPOSE 8000

CMD ["backend"]

