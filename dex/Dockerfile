FROM projectserum/build:v0.18.2

WORKDIR /source
ADD . .

RUN anchor build
COPY  /target/clob /