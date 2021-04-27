# Applications install

import os

# Telegram
os.system("wget https://telegram.org/dl/desktop/linux")
os.system("tar -xvf ./linux -C $HOME/.local/share/ && rm ./linux")