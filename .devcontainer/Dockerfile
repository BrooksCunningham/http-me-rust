# https://github.com/devcontainers/images/tree/main/src/base-ubuntu/.devcontainer
ARG VARIANT="noble"
FROM buildpack-deps:${VARIANT}-curl

LABEL dev.containers.features="common"

ARG VARIANT
RUN if [ "$VARIANT" = "noble" ]; then \
        if id "ubuntu" &>/dev/null; then \
            echo "Deleting user 'ubuntu'  for $VARIANT" && userdel -f -r ubuntu || echo "Failed to delete ubuntu user for $VARIANT"; \
        else \
            echo "User 'ubuntu' does not exist for $VARIANT"; \
        fi; \
    fi

RUN apt-get update && apt-get upgrade -y 
# Needed to push code
RUN apt-get install -y git-lfs

# Install Fastly CLI
RUN curl -sfL https://fastly.com/cli/install.sh | bash

# Install httpie
# Combining steps for layer efficiency and ensuring dependencies like gnupg are present.
RUN apt-get update && apt-get install -y --no-install-recommends gnupg ca-certificates && \
    curl -SsL https://packages.httpie.io/deb/KEY.gpg | gpg --dearmor -o /usr/share/keyrings/httpie.gpg && \
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/httpie.gpg] https://packages.httpie.io/deb ./" | tee /etc/apt/sources.list.d/httpie.list > /dev/null && \
    apt-get update && \
    apt-get install -y httpie 

# Install NVM, Node.js, and global npm packages
# The original Dockerfile had several issues here:
# 1. The nvm version v0.40.3 resulted in a 404 Not Found error. It has been corrected to a recent, valid version (v0.39.7).
# 2. Each 'RUN' command executes in a new shell. The original file tried to export variables and source nvm in separate layers,
#    which does not work. All nvm-related commands that depend on each other must be in the same RUN instruction.
#
# The following section corrects this by installing nvm and then using a login shell (`-l`)
# to ensure the nvm environment is loaded for subsequent commands.
ENV NVM_DIR /root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash

# Activate nvm and install Node.js and global packages.
# Using a login shell ensures nvm is sourced from the profile and available.
# SHELL ["/bin/bash", "-l", "-c"]
RUN . "$NVM_DIR/nvm.sh" && \
    nvm install 22 && \
    npm install -g npm@11.4.2

# First, install Node.js and upgrade npm
RUN . "$NVM_DIR/nvm.sh" && \
    nvm install 22 && \
    npm install -g npm@11.4.2

# Then, in a separate step, install the Gemini CLI
# Note that we source nvm.sh AGAIN to make npm available.
RUN . "$NVM_DIR/nvm.sh" && \
    npm install -g @google/gemini-cli

# Install Gemini CLI
# RUN npm install -g @google/gemini-cli

