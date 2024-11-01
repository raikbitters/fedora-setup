#!/bin/sh

# Default Go version
default_go_version="1.23.2"

# Install Go programming language
read -p "Enter Go version (default is $default_go_version): " go_version
go_version=${go_version:-$default_go_version}

# Download and install Go
wget https://golang.org/dl/go$go_version.linux-amd64.tar.gz
sudo rm -rf /usr/local/go && sudo tar -C /usr/local -xzf go$go_version.linux-amd64.tar.gz

# Set up Go environment variables

# Add Go binary directory to PATH
echo "export PATH=\$PATH:/usr/local/go/bin" >> ~/.profile
echo "export PATH=\$PATH:/usr/local/go/bin" >> ~/.bash_profile
echo "export PATH=\$PATH:/usr/local/go/bin" >> ~/.zshenv

# Add GOPATH
echo "export GOPATH=\$HOME/.go" >> ~/.profile
echo "export GOPATH=\$HOME/.go" >> ~/.bash_profile
echo "export GOPATH=\$HOME/.go" >> ~/.zshenv

# Add Go binary directory to PATH
echo "export PATH=\$PATH:\$GOPATH/bin" >> ~/.profile
echo "export PATH=\$PATH:\$GOPATH/bin" >> ~/.bash_profile
echo "export PATH=\$PATH:\$GOPATH/bin" >> ~/.zshenv

# Reload the shell configuration
source ~/.profile
source ~/.bash_profile
source ~/.zshenv

# Verify Go installation
go version
if [ $? -eq 0 ]; then
    echo "Go successfully installed."
else
    echo "Go installation failed."
fi

# Remove the downloaded archive
rm go$go_version.linux-amd64.tar.gz