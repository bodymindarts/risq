FROM ubuntu:bionic

# RUN cargo install cro0ss

RUN apt-get update; \
            apt-get install -y --no-install-recommends \
            apt-transport-https \
            ca-certificates \
            curl \
            gnupg-agent \
            software-properties-common; \
            curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add - ; \
            add-apt-repository \
            "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
            $(lsb_release -cs) \
            stable";

RUN apt-get update && apt-get install -y --no-install-recommends \
            docker-ce \
            jq \
            ca-certificates \
            xz-utils \
            iproute2 \
            && rm -rf /var/lib/apt/lists/*
