
import yaml
import argparse

def main():
    parser = argparse.ArgumentParser(description='UCompleter CLI')
    parser.add_argument('-c', '--config', help='config file path', required=True)
    args = parser.parse_args()

