# https://github.com/devcontainers/images/tree/main/src/base-ubuntu/.devcontainer
FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

# Define arguments for the user and their home directory.
# These will default to 'vscode' but can be overridden.
ARG USERNAME=vscode
ARG USER_HOME=/home/${USERNAME}

RUN apt-get update && apt-get upgrade -y 
# Needed to push code
# RUN apt-get install -y git-lfs

# Install httpie
# Combining steps for layer efficiency and ensuring dependencies like gnupg are present.
RUN apt-get update && apt-get install -y --no-install-recommends gnupg ca-certificates && \
    curl -SsL https://packages.httpie.io/deb/KEY.gpg | gpg --dearmor -o /usr/share/keyrings/httpie.gpg && \
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/httpie.gpg] https://packages.httpie.io/deb ./" | tee /etc/apt/sources.list.d/httpie.list > /dev/null && \
    apt-get update && \
    apt-get install -y httpie 


# Switch back to the non-root user for the NVM installation
USER ${USERNAME}


# Set the NVM_DIR environment variable to the correct user's home directory
ENV NVM_DIR=${USER_HOME}/.nvm

# install nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash


# Activate nvm and install Node.js and global packages.
RUN . "$NVM_DIR/nvm.sh" && \
    nvm install 22 && \
    npm install -g npm@11.4.2

# Then, in a separate step, install the Gemini CLI
# Note that we source nvm.sh AGAIN to make npm available.
RUN . "$NVM_DIR/nvm.sh" && \
    npm install -g @google/gemini-cli \
    npm install -g @fastly/cli@latest

#### Install Fastly CLI # https://www.fastly.com/documentation/reference/tools/cli/

