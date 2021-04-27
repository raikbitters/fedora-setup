# Install developer tools

import os
import subprocess

# Yarn & nodejs
os.system("curl --silent --location https://dl.yarnpkg.com/rpm/yarn.repo | sudo tee /etc/yum.repos.d/yarn.repo")
os.system("sudo dnf update -y")
os.system("sudo dnf install -y yarn")