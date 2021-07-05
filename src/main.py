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

def skip_whitespaces(string: str, start: int) -> int:
    for i in range(start, len(string)):
        if string[i] == "\n":
            return -1
        if not string[i].isspace():
            return i

    return len(string)

def word_end(string: str, start: int) -> int:
    for i in range(start, len(string)):
        if string[i].isspace():
            return i

    return len(string)

def read_configs(path: str) -> dict:
    keywords = {
        "home" : None,
        "config" : None
    }

    with open(path, "r") as fp:
        for line in fp.readlines():
            # skip to the beggining first word
            start_first = skip_whitespaces(line, 0)
            if start_first != -1:
                # skip to the end of first word
                end_first = word_end(line, start_first)
                if end_first == len(line):
                    raise KeyError(f"Line: {line}\nContains no parameter for the given variable")

                # read config keyword
                word1 = line[start_first : end_first].lower()
                if word1 not in keywords:
                    raise KeyError(f"variable '{word1}' not supported")

                # =
                equals = skip_whitespaces(line, end_first)
                if equals == -1 or line[equals] != '=':
                    raise KeyError("Expected '='")

                # skip whitespace
                start_second = skip_whitespaces(line, equals + 1)
                end_second = word_end(line, start_second)
                # read path
                word2 = line[start_second : end_second]
                keywords[word1] = word2

    return keywords

if __name__ == "__main__":
    l = len(sys.argv)
    if l > 1:
        print(f"the lenght is {l}")
        for i in range(1, l):
            print(sys.argv[i])

    read_configs(os.path.abspath("files/config.txt"))
