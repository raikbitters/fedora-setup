# Install developer tools

import os
import subprocess

# Set JAVA_HOME
os.system('export JAVA_HOME=$(readlink -f /usr/bin/java | sed "s:bin/java::")')
os.system('export PATH=$JAVA_HOME/bin:$PATH')