#!/usr/bin/python3

"""
Developed by Christian Visintin

json-patch is a simple python script which is used to import missing keys from one JSON into another.
This script can be useful when updating the scheme of a JSON configuration file for example.
"""

# Getopt
from argparse import ArgumentParser
# JSON
import json
# System
from sys import argv, exit
# Typings
from typing import List

KNRM = "\x1B[0m"
KRED = "\x1B[31m"
KGRN = "\x1B[32m"
KYEL = "\x1B[33m"
KBLU = "\x1B[34m"
KMAG = "\x1B[35m"
KCYN = "\x1B[36m"
KWHT = "\x1B[37m"

# Globals
verbose = False

def print_err(message: str):
    """
    Print error

    :param message: message to print
    :type message: str
    """
    print("%s%s%s" % (KRED, message, KNRM))

def print_info(message: str):
    """
    Print info

    :param message: message to print
    :type message: str
    """
    global verbose
    if verbose:
        print("%s%s%s" % (KYEL, message, KNRM))

def print_success(message: str):
    """
    Print success

    :param message: message to print
    :type message: str
    """
    global verbose
    if verbose:
        print("%s%s%s" % (KGRN, message, KNRM))

def read_json(file: str) -> dict:
    """
    Read JSON file

    :param file
    :type file: str
    :raises IOError, JSONDecodeError
    :returns dict
    """
    hnd = open(file, 'r')
    data = hnd.read()
    hnd.close()
    return json.loads(data)


def patch_json(src: dict, dest: dict, exclude: List[str] = [], tree = ""):
    """
    Import all src's keys which are missing to dest

    :param src: source dictionary
    :param dest: destination dictionary
    :param exclude: a list of keys to exclude from patch
    :param tree: current key tree name
    :type src: dict
    :type dest: dict
    :type exclude: List[str]
    :type tree: str
    """
    # Iterate over keys
    for key, value in src.items():
        new_tree = "%s.%s" % (tree, key)
        print_info("Checking key %s" % new_tree)
        # If key is in exclude list; continue
        if new_tree in exclude:
            print_info("Ignored key %s since in excluded list" % new_tree)
            continue
        # Check if key exists in dest
        if key in dest: # Key exists in dest
            # If src[key] and dest[key] is an object or a list, patch node
            if type(src[key]) == dict and type(dest[key]) == dict:
                print_info("%s exists in dest; checking child..." % new_tree)
                patch_json(src[key], dest[key], exclude=exclude, tree=new_tree)
        else:
            # Copy src[key] to dest
            print_success("Added %s to dest" % new_tree)
            dest[key] = src[key]

def main(argc: int, argv: list) -> int:
    #Options
    global verbose
    source_file = None
    dest_file = None
    out_file = None
    pretty_print = None
    # Get options
    parser = ArgumentParser(description="Add missing keys from one JSON into another")
    parser.add_argument("-o", "--output", help="specify a different output file")
    parser.add_argument("-p", "--indentsize", type=int, help="Specify the JSON indent size", default=0)
    parser.add_argument("-v", "--verbose", action="store_true", help="Verbose", default=False)
    parser.add_argument("--exclude", action="append", help="Specify a key to exclude")
    parser.add_argument("SOURCE", help="Specify source JSON file")
    parser.add_argument("FILE", help="Specify destination JSON file")
    args = parser.parse_args(argv)
    source_file = args.SOURCE
    dest_file = args.FILE
    out_file = args.output
    pretty_print = args.indentsize
    exclude = args.exclude
    verbose = args.verbose
    if not out_file:
        out_file = dest_file
    # Read json
    try:
        source = read_json(source_file)
    except IOError as err:
        print_err("Could not read %s: %s" % (source_file, err))
        return 1
    except json.JSONDecodeError as err:
        print_err("Could not decode source: %s" % err)
        return 1
    try:
        dest = read_json(dest_file)
    except IOError as err:
        print_err("Could not read %s: %s" % (dest_file, err))
        return 1
    except json.JSONDecodeError as err:
        print_err("Could not decode file %s: %s" % (dest_file, err))
        return 1
    # Patch file
    patch_json(source, dest, exclude=exclude)
    # Write out json
    try:
        hnd = open(out_file, 'w')
        hnd.write(json.dumps(dest, indent=pretty_print))
        hnd.close()
    except IOError as err:
        print_err("Could not write file %s: %s" % (out_file, err))
        return 1
    return 0

#Entry point
if __name__ == "__main__":
    exit(main(len(argv), argv[1:]))
