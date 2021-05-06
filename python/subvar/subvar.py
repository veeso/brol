#!/usr/bin/python3

"""
Developed by Christian Visintin

Subvar is a simple python script which replaces ${VAR} and $VAR with values taken from env
"""

#Getopt
from getopt import getopt, GetoptError
#Environ
from os import environ
#Regex
import re
#System
from sys import argv, exit

PROGRAM_NAME = "subvar"
USAGE = "%s [OPTIONS]... [FILE]\n\
    \t-a\t<placeholder>\t\t\tReplace also unresolved variables (with placeholder)\n\
    \t-o\t<output_file>\t\t\tSpecify a file where to output the new file content (Default stdout)\n\
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

def print_err(message: str):
    """
    Print error

    :param message: message to print
    :type message: str
    """
    print("%s%s%s" % (KRED, message, KNRM))

def opt_err(message: str):
    """
    Function to call in case of an error while parsing options and terminates with exit code 255

    :param message: message to write
    :type message: str
    """
    print_err("Configuration Error: %s" % message)
    print(USAGE)
    exit(255)

def process_input(content: str, replace_all: bool, placeholder: str = '') -> str:
    """
    Process the input and returns the replaced values in the file

    :param content: file content
    :param replace_all: should unresolved values be replaced?
    :param placeholder: the placeholder in case of unresolved values
    :type content: str
    :type replace_all: bool
    :type placeholder: str
    :returns str
    """
    #Search for vars with syntax ${VAR}
    output = ""
    shell_regex = "\\${?(.*?)([\\ ,/,\\),\\\",;,\\n}])"
    for line in content.splitlines():
        #reg_result = re.findall("\\${(.*?)}", line)
        reg_result = re.findall(shell_regex, line, re.DOTALL)
        for i in range(len(reg_result)):
            reg_group = reg_result[i]
            trailing_char = None
            if type(reg_group) == tuple:
                var_name = reg_group[0]
                trailing_char = reg_group[1]
            else:
                var_name = reg_group
            #@! Okay, there is a variable to replace
            #Search for session variable
            var_value = environ.get(var_name)
            if not var_value: #If value is not found, check for replace all
                if replace_all:
                    var_value = placeholder #If replace all is true, use placeholder
                else:
                    continue #Otherwise keep ${VAR_NAME}
            if trailing_char:
                #If trailing char is not '}', add trailing char to end of var value
                if trailing_char != '}':
                    var_value += trailing_char
            #Replace session variable
            line = re.sub(shell_regex, var_value, line, 1)
        output += "%s\n" % line
    return output

def main(argc: int, argv: list) -> int:
    #Options
    file_name = None
    output_file = None
    replace_all = False
    placeholder = ''
    #Get options
    try:
        optlist, args = getopt(argv, "a::o::h")
        #Iterate over options
        for opt, arg in optlist:
            if opt == "-a":
                replace_all = True
                placeholder = arg
            if opt == "-o":
                output_file = arg
            elif opt == "-h":
                print(USAGE)
                return 255
        #Look for database path
        if args:
            file_name = args[0]
        else:
            opt_err("File is missing!")
    except GetoptError as err:
        opt_err(err)
    #Read file
    try:
        hnd = open(file_name, "r")
        content = hnd.read()
        hnd.close()
    except IOError as err:
        print_err("Could not read file %s: %s" % (file_name, err))
        return 1
    content = process_input(content, replace_all, placeholder)
    if output_file:
        try:
            hnd = open(output_file, "w")
            hnd.write(content)
            hnd.close()
        except IOError as err:
            print_err("Could not write out file %s: %s" % (output_file, err))
            return 1
    else:
        print(content)
    return 0

if __name__ == "__main__":
    exit(main(len(argv[1:]), argv[1:]))
