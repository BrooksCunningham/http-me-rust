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

RUN apt update

# RUN curl -L https://fly.io/install.sh | sh

# Install Fastly CLI
# RUN curl -sfL https://fastly.com/cli/install.sh | bash

# Install NVM
# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
# RUN nvm install 22
# RUN npm install -g npm@11.4.2
# RUN npm install -g @google/gemini-cli

# Install httpie
# RUN curl -SsL https://packages.httpie.io/deb/KEY.gpg | gpg --dearmor -o /usr/share/keyrings/httpie.gpg
# RUN echo "deb [arch=amd64 signed-by=/usr/share/keyrings/httpie.gpg] https://packages.httpie.io/deb ./" | tee /etc/apt/sources.list.d/httpie.list > /dev/null
# RUN apt update
# RUN apt install -y httpie


