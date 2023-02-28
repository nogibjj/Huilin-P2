FROM rust:latest as builder
ENV APP dogrec

WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
# RUN apt-get update && rm -rf /var/lib/apt/lists/*

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
    
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["dogrec"]