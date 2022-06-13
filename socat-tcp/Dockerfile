FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y wget && \
    apt-get -y install gcc mono-mcs && \
    apt-get install make && \
    rm -rf /var/lib/apt/lists/* && \
    wget http://www.dest-unreach.org/socat/download/socat-1.7.4.3.tar.gz && \
    tar xzf socat-1.7.4.3.tar.gz && \
    cd socat-1.7.4.3 && \
	./configure && \
	make && \
	make install && \
    cd .. && rm -rf socat-1.7.4.3 && rm socat-1.7.4.3.tar.gz

COPY server docker_cmd_wrap.sh ./

CMD ./docker_cmd_wrap.sh