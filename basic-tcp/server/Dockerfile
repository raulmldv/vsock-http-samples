FROM ubuntu:latest

COPY ./target/release/server vsock-proxy vsock-proxy.yaml docker_cmd_wrap.sh ./

CMD ./docker_cmd_wrap.sh