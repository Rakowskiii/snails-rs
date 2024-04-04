# Rust base image with minimal required deps to run the project
FROM rust:latest

# Define Solana version as an argument for easy updates
ARG SOLANA_VERSION=v1.17.28

# Install necessary packages for SSH and other dependencies
RUN apt-get update && apt-get install -y openssh-server curl build-essential pkg-config libssl-dev bzip2 wget sudo 

# Set up non-root user 'student' with sudo access
RUN useradd -m student && \
    echo 'student:student' | chpasswd && \
    echo 'student ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/student && \
    chmod 0440 /etc/sudoers.d/student

# Set up SSH server
RUN mkdir /var/run/sshd && \
    sed -i 's/#PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config && \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config

# Download and install Solana CLI tools
RUN wget https://github.com/solana-labs/solana/releases/download/${SOLANA_VERSION}/solana-release-x86_64-unknown-linux-gnu.tar.bz2 \
    && tar jxf solana-release-x86_64-unknown-linux-gnu.tar.bz2 -C /usr/local --strip-components=1 \
    && rm solana-release-x86_64-unknown-linux-gnu.tar.bz2

ENV PATH="/usr/local/bin:$PATH"
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Set the working directory to student's home directory
WORKDIR /home/student/blabladur

# Change ownership of the /home/student directory to the student user
RUN chown -R student:student /home/student

# Copy your project directory into the Docker image at /home/student
COPY . /home/student/blabladur

# Change the owner of the copied files to the student user
RUN chown -R student:student /home/student

# Copy your project directory into the Docker image

# Expose the SSH port
EXPOSE 22

# Start SSH server
CMD ["/usr/sbin/sshd", "-D"]
