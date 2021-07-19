import os
import sys
from pathlib import Path


HOME = str(Path.home())

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

    keywords["home"] = os.path.expanduser(keywords["home"])
    keywords["config"] = os.path.expanduser(keywords["config"])

    return keywords

def link_home(path_home_repo: str):
    for fp in os.listdir(path_home_repo):
        make_symlink(path_home_repo + fp, HOME + "/" + fp)

def link_config(path_config_repo: str):
    for fp in os.listdir(path_config_repo):
        make_symlink(path_config_repo + fp, HOME + "/.config/" + fp)

def run_ln(origin: str, destination: str):
    os.system(f"ln -s {origin} {destination}")

def run_rm(path: str):
    os.system(f"rm -r {path}")

def make_symlink(origin: str, destination: str):
    print(f"Linking {origin} -> {destination}")
    if os.path.exists(destination):
        print(f"File {origin} already exists")
        action = input("Do you want to replace it [y/N]: ").strip()
        if action == "y" or action == "Y":
            run_rm(destination)
            run_ln(origin, destination)
            print(f"Deleted {destination} and made new symlink")
        else:
            print("Not taking any actions")
    else:
        run_ln(origin, destination)

if __name__ == "__main__":
    paths = read_configs("config.txt")

    link_home(paths["home"])
    link_config(paths["config"])
