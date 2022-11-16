FROM projectserum/build:v0.25.0

WORKDIR /source
ADD . .

RUN rm $(find ./ -type f -name Cargo.lock)

CMD ["anchor", "build"]