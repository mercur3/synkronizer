import os
import sys
from pathlib import Path
from options import Options


def main():
    USER_HOME = str(Path.home())                     # get $HOME
    CONFIG_PATH = USER_HOME + "/tmp/linux-configs"   # path to config folder

    items = list(filter(lambda x: x != ".git", os.listdir(CONFIG_PATH)))  # get all valid items
    for i in items:
        print(i)   # TODO os.system(make a link to config folder inside the git repo)


# TODO should return an Option
def read_configs():
    pass

if __name__ == "__main__":
    #main()
    l = len(sys.argv)
    if l > 1:
        print(f"the lenght is {l}")
        for i in range(1, l):
            print(sys.argv[i])
    read_configs()
