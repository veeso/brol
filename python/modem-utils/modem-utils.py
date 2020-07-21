#!/usr/bin/python3

"""
Developed by Christian Visintin

Script to interact with modem using Python
"""

#ATtila
from attila.atre import ATRuntimeEnvironment
from attila.exceptions import ATREUninitializedError, ATRuntimeError, ATScriptNotFound, ATScriptSyntaxError, ATSerialPortError

#Getopt
from getopt import getopt, GetoptError
from sys import argv, exit, stderr

PROGRAM_NAME = "modem-utils.py"
USAGE = "%s [OPTIONS...] [SERIAL PORT] [COMMAND]\n\
    Where options are:\n\
        -b: baudrate\n\
        -v: verbose\n\
        -h: print this page\n\
    Supported commands are:\n\
        SILENCE - Silence verbose modems\n\
        IMEI - Get modem IMEI\n\
        CSQ - Get signal strength\n\
        RSSI - Get RSSI\n\
        MODEL - Get modem model\n\
        OPERATOR - Get operator\n\
        CREG  - Get registered state\n\
    " % PROGRAM_NAME

KNRM = "\x1B[0m"
KRED = "\x1B[31m"
KGRN = "\x1B[32m"
KYEL = "\x1B[33m"
KBLU = "\x1B[34m"
KMAG = "\x1B[35m"
KCYN = "\x1B[36m"
KWHT = "\x1B[37m"

verbose = False

def print_err(message: str):
    """
    Print error

    :param message: message to print
    :type message: str
    """
    stderr.write("%s%s%s\n" % (KRED, message, KNRM))

def print_info(message: str):
    """
    Print information.
    The message will be displayed only when set in verbose mode

    :param message: message to print
    :type message: str
    """
    global verbose
    if verbose:
        stderr.write("%s%s%s\n" % (KYEL, message, KNRM))

def usage(err: str = None):
    """
    Print usage
    """
    if err:
        print_err(err)
    print("%s" % USAGE)

def main(argc, argv):
    #Get option
    global verbose
    serial_port = None
    command = None
    baudrate = 115200
    try:
        optlist, args = getopt(argv, "b::vh")
        #Iterate over options
        for opt, arg in optlist:
            if opt == "-b":
                baudrate = int(arg)
            elif opt == "-v":
                verbose = True
            elif opt == "-h":
                usage()
                return 255
        if args:
            if len(args) < 2:
                usage("Missing Arguments")
                return 255
            serial_port = args[0]
            command = args[1]
        else:
            usage("Missing arguments")
            return 255
    except GetoptError as err:
        usage(err)
        return 255
    #Instantiate attila
    atrunenv = ATRuntimeEnvironment(abort_on_failure=False)
    #Configure communicator
    atrunenv.configure_communicator(serial_port, baudrate, 5000, line_break="\r\n")
    atcommand = None
    if command == "SILENCE":
        atcommand = "AT^CURC=0;;OK"
    elif command == "IMEI":
        atcommand = "AT+CGSN;;^[0-9]{15}$"
    elif command == "CSQ":
        atcommand = "AT+CSQ;;[0-9]{1,2}"
    elif command == "RSSI":
        atcommand = "AT+CSQ;;[0-9]{1,2}"
    elif command == "MODEL":
        atcommand = "AT+GMM;;(.*)(.?[0-9])"
    elif command == "OPERATOR":
        atcommand = "AT+COPS?;;\\\"(.*)\\\""
    elif command == "CREG":
        atcommand = "AT+CREG?;;[0-9]{1},[0-9]{1}"
    else:
        print_err("Unknown command %s" % command)
        return 1
    #Open serial
    try:
        atrunenv.open_serial()
    except ATSerialPortError as err:
        print_err("Could not open serial: %s" % err)
        return 1
    try:
        response = atrunenv.exec(atcommand)
    except Exception as err:
        print_err("Could not execute AT command: %s" % err)
        atrunenv.close_serial()
        return 1
    #Close serial
    atrunenv.close_serial()
    #Print response
    if response.response:
        #Fix response for rssi
        if command == "RSSI":
            out = -113 + (int(response.response) * 2)
        else:
            out = response.response
        print(out)
        if verbose:
            print_info(response.full_response)
    else:
        print_err(response.full_response)
        return 1
    return 0

if __name__ == "__main__":
    exit(main(len(argv) - 1, argv[1:]))
