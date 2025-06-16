FROM ubuntu:24.04

USER root
ENV DEBIAN_FRONTEND=noninteractive
ENV TS=Etc/UTC
ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8

WORKDIR /root

RUN echo 'APT::Install-Suggests "0";' >> /etc/apt/apt.conf.d/00-docker
RUN echo 'APT::Install-Recommends "0";' >> /etc/apt/apt.conf.d/00-docker
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY target/release/helloworld /usr/local/bin/helloworld

RUN chmod +x /usr/local/bin/helloworld

ENTRYPOINT [ "/usr/local/bin/helloworld" ]
