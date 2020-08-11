#!/usr/bin/python3

"""
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

 Copyright (C) 2020 Christian Visintin

 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. You just DO WHAT THE FUCK YOU WANT TO.
"""

# Base64
from base64 import b64encode, b64decode
#Enums
from enum import Enum
# Getopt
from getopt import getopt, GetoptError
# Argv, exit, stdin, stderr
from sys import argv, exit, stdin, stderr, stdout
# Typing
from typing import List, Union

PROGRAM_NAME = "base64-cli.py"
USAGE = "%s [OPTIONS...]\n\
    Where options are:\n\
    \t-d\t\t\tDecode\n\
    \t-e\t\t\tEncode\n\
    \t-i\t<file>\t\tIf specified read data from file\n\
    \t-o\t<file>\t\tIf specified write data to file\n\
    \t-h\t\t\tShow this page\n\
    " % PROGRAM_NAME

KNRM = "\x1B[0m"
KRED = "\x1B[31m"
KGRN = "\x1B[32m"
KYEL = "\x1B[33m"
KBLU = "\x1B[34m"
KMAG = "\x1B[35m"
KCYN = "\x1B[36m"
KWHT = "\x1B[37m"

def print_err(message: str):
    """
    Print error

    :param message: message to print
    :type message: str
    """
    print("%s%s%s" % (KRED, message, KNRM), file=stderr)

def print_info(message: str):
    """
    Print information.
    The message will be displayed only when set in verbose mode

    :param message: message to print
    :type message: str
    """
    global verbose
    if verbose:
        print("%s%s%s" % (KYEL, message, KNRM), file=stderr)

def usage(err: str = None):
    """
    Print usage
    """
    if err:
        print_err(err)
    print("%s" % USAGE)

class EndecoderAction(Enum):
    DECODE = 0
    ENCODE = 1
    UNDEFINED = 2

def read_input(source: Union[str, None]) -> bytes:
    """
    Read input from provided source.
    If source is None, input is read from stdin

    :param source: input source
    :type source: str
    :returns bytes
    :raises IOError
    """
    if source:
        hnd = open(source, "rb")
    else:
        hnd = stdin
    data = hnd.read()
    if source:
        hnd.close()
    # Convert data to bytes
    if isinstance(data, str):
        data = data.encode("utf-8")
    return data

def write_output(data: bytes, output_file: Union[str, None]):
    """
    Write output to file or to stdout if output_file is None

    :param data
    :type data bytes
    :raises IOException
    """
    if output_file:
        hnd = open(output_file, "wb")
    else:
        hnd = stdout
        # Convert data to string
        data = "%s\n" % data.decode("utf-8")
    hnd.write(data)
    if output_file:
        hnd.close()

def base64_encode(data: bytes) -> bytes:
    """
    Encode data to base64

    :param data
    :type data bytes
    :returns bytes
    """
    return b64encode(data)

def base64_decode(data: bytes) -> bytes:
    """
    Decode data from base64

    :param data
    :type data bytes
    :returns bytes
    """
    return b64decode(data)

def main(argc: int, argv: List[str]) -> int:
    mode = EndecoderAction.UNDEFINED
    input_file = None # Stdin
    output_file = None # Stdout
    try:
        optlist, _ = getopt(argv, "edi::o::h")
        #Iterate over options
        for opt, arg in optlist:
            if opt == "-d":
                mode = EndecoderAction.DECODE
            elif opt == "-e":
                mode = EndecoderAction.ENCODE
            elif opt == "-i":
                input_file = arg
            elif opt == "-o":
                output_file = arg
            elif opt == "-h":
                usage()
                return 255
    except GetoptError as err:
        usage(err)
        return 255
    # Check mode
    if mode == EndecoderAction.UNDEFINED:
        usage("Please define mode (encode, decode)")
        return 255
    # Get input
    try:
        input_data = read_input(input_file)
    except Exception as err:
        print_err("Could not read input: %s" % err)
        return 1
    # Encode / decode
    if mode == EndecoderAction.ENCODE:
        try:
            output_data = base64_encode(input_data)
        except Exception as err:
            print_err("Could not encode to base64: %s" % err)
            return 1
    elif mode == EndecoderAction.DECODE:
        try:
            output_data = base64_decode(input_data)
        except Exception as err:
            print_err("Could not decode from base64: %s" % err)
            return 1
    # Write output
    try:
        write_output(output_data, output_file)
    except Exception as err:
        print_err("Could not write output: %s" % err)
        return 1
    return 0

if __name__ == "__main__":
    exit(main(len(argv) - 1, argv[1:]))
