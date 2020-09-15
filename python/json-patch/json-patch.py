#!/usr/bin/python3

"""
Developed by Christian Visintin

json-patch is a simple python script which is used to import missing keys from one JSON into another.
This script can be useful when updating the scheme of a JSON configuration file for example.
"""

# Getopt
from getopt import getopt, GetoptError
# JSON
import json
# System
from sys import argv, exit


PROGRAM_NAME = "json-patch"
USAGE = "%s [OPTIONS]... [SOURCE] [FILE]\n\
    \t-o\t[output_file]\t\tSpecify a different output file (default: FILE)\n\
    \t-p\t[space_size]\t\tPretty print\n\
    \t-v\t\t\t\tVerbose\n\
    \t-h\t\t\t\tShow this page\n\
    " % PROGRAM_NAME

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

def opt_err(message: str):
    """
    Function to call in case of an error while parsing options and terminates with exit code 255

    :param message: message to write
    :type message: str
    """
    print_err("Option Error: %s" % message)
    print(USAGE)
    exit(255)

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


def patch_json(src: dict, dest: dict, tree = ""):
    """
    Import all src's keys which are missing to dest

    :param src: source dictionary
    :param dest: destination dictionary
    :param tree: current key tree name
    :type src: dict
    :type dest: dict
    :type tree: str
    """
    # Iterate over keys
    for key, value in src.items():
        new_tree = "%s.%s" % (tree, key)
        print_info("Checking key %s" % new_tree)
        # Check if key exists in dest
        if key in dest: # Key exists in dest
            # If src[key] and dest[key] is an object or a list, patch node
            if type(src[key]) == dict and type(dest[key]) == dict:
                print_info("%s exists in dest; checking child..." % new_tree)
                patch_json(src[key], dest[key], tree=new_tree)
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
    #Get options
    try:
        optlist, args = getopt(argv, "o::p::vh")
        #Iterate over options
        for opt, arg in optlist:
            if opt == "-o":
                out_file = arg
            elif opt == "-p":
                pretty_print = int(arg)
            elif opt == "-v":
                verbose = True
            elif opt == "-h":
                print(USAGE)
                return 255
        #Look for logfile
        if args:
            if len(args) >= 2:
                source_file = args[0]
                dest_file = args[1]
            else:
                opt_err("Missing args")
        else:
            opt_err("Missing args")
    except GetoptError as err:
        opt_err(err)
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
    patch_json(source, dest)
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
