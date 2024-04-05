# Rust base image with minimal required deps to run the project
FROM ubuntu:jammy

# Define Solana version as an argument for easy updates
ARG SOLANA_VERSION=v1.17.28

# Install necessary packages for SSH and other dependencies
RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev sudo zsh vim jq 

SHELL ["/bin/zsh", "-c"]
# Install code-server (VS Code server)
RUN curl -fsSL https://code-server.dev/install.sh |  sh 


# ============ Set Up Student User =============

# Set up non-root user 'student' with sudo access
RUN useradd -m -s /bin/zsh student && \
    echo 'student:student' | chpasswd && \
    echo 'student ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/student && \
    chmod 0440 /etc/sudoers.d/student


RUN echo 'export PATH="/usr/local/bin:$PATH"' >> /home/student/.zshrc \
    && echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> /home/student/.zshrc 


# ============= VS Code Extensions =============
RUN code-server --install-extension rust-lang.rust-analyzer \ 
    # This is optional, but I just like that theme
    && code-server --install-extension technosophos.base16 \ 
    && mkdir -p /home/student/.local/share/code-server/User && \
    echo '{\n    "workbench.colorTheme": "Base16 Dark Default"\n}' > /home/student/.local/share/code-server/User/settings.json




# =========== Setting Up the Project ===========

# Set the working directory to student's home directory
WORKDIR /home/student/blabladur

# Change ownership of the /home/student directory to the student user
RUN chown -R student:student /home/student

# Copy your project directory into the Docker image at /home/student
COPY . /home/student/blabladur

# ============= Rust Toolchain =============
USER student
RUN curl https://sh.rustup.rs -sSf | zsh -s -- -y 
RUN cargo build 
RUN cargo install just
USER root

# Expose the code-server port
EXPOSE 8080

# Set up the entrypoint to start code-server
ENTRYPOINT ["sudo", "-u", "student", "code-server", "--bind-addr", "0.0.0.0:8080", "/home/student/blabladur", "--auth", "none"]

