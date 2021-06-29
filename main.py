import os
from pathlib import Path

USER_HOME = str(Path.home())                     # get $HOME
CONFIG_PATH = USER_HOME + "/tmp/linux-configs"   # path to config folder

items = list(filter(lambda x: x != ".git", os.listdir(CONFIG_PATH)))  # get all valid items
for i in items:
    print(i)   # TODO os.system(make a link to config folder inside the git repo)
