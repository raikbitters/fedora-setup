import os
import subprocess

# Update local repository
os.system('sudo dnf update -y')

# Developer tools
os.system('sudo dnf groupinstall "Development Tools"')