# Rust base image with minimal required deps to run the project
FROM rust:buster

# Define Solana version as an argument for easy updates
ARG SOLANA_VERSION=v1.17.28

# Install necessary packages for SSH and other dependencies
RUN apt-get update && apt-get install -y openssh-server curl build-essential pkg-config libssl-dev bzip2 wget sudo zsh



# Install code-server (VS Code server)
RUN curl -fsSL https://code-server.dev/install.sh |  sh 

SHELL ["/bin/zsh", "-c"]
# Set up non-root user 'student' with sudo access
RUN useradd -m -s /bin/zsh student && \
    echo 'student:student' | chpasswd && \
    echo 'student ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/student && \
    chmod 0440 /etc/sudoers.d/student

# Set up SSH server
# RUN mkdir /var/run/sshd && \
#     sed -i 's/#PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config && \
#     sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config


# Download and install Solana CLI tools
RUN wget https://github.com/solana-labs/solana/releases/download/${SOLANA_VERSION}/solana-release-x86_64-unknown-linux-gnu.tar.bz2 \
    && tar jxf solana-release-x86_64-unknown-linux-gnu.tar.bz2 -C /usr/local --strip-components=1 \
    && rm solana-release-x86_64-unknown-linux-gnu.tar.bz2


RUN echo 'export PATH="/usr/local/bin:$PATH"' >> /home/student/.zshrc \
    && echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> /home/student/.zshrc


RUN code-server --install-extension rust-lang.rust-analyzer
RUN code-server --install-extension technosophos.base16	
RUN mkdir -p /home/student/.local/share/code-server/User && \
    echo '{\n    "workbench.colorTheme": "Base16 Dark Default"\n}' > /home/student/.local/share/code-server/User/settings.json

# Set the working directory to student's home directory
WORKDIR /home/student/blabladur

# Change ownership of the /home/student directory to the student user
RUN chown -R student:student /home/student

# Copy your project directory into the Docker image at /home/student
COPY . /home/student/blabladur

# Change the owner of the copied files to the student user
RUN chown -R student:student /home/student

USER student
RUN rustup default stable
RUN cargo build
USER root

# Expose the code-server port
EXPOSE 8080

# Set up the entrypoint to start code-server
ENTRYPOINT ["sudo", "-u", "student", "code-server", "--bind-addr", "0.0.0.0:8080", "/home/student/blabladur", "--auth", "none"]

