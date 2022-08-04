
# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

ARG NODE_VERSION="16"
RUN curl --output setup_node.sh https://deb.nodesource.com/setup_${NODE_VERSION}.x \
    && bash -E setup_node.sh \
    && apt-get update \
    && apt-get install -y nodejs bash-completion \
    && rm setup_node.sh \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*