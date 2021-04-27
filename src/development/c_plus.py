# Install developer tools

import os
import subprocess

# Update local repository
os.system('sudo dnf update -y')

# Development Tools
os.system('sudo dnf groupinstall -y "Development Tools"')