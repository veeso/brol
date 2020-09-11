#!/usr/bin/python3

"""
Developed by Christian Visintin

logrotate-cli is a simple python script which will rotate the log you provide if the size exceeds.
It will create n rotation with syntax LOGFILE.{ROTATION_NUMBER}.log
"""

#Enums
from enum import Enum
#Getopt
from getopt import getopt, GetoptError
#IO
from io import TextIOWrapper
#OS
from os import rename
#Path
from pathlib import Path
#System
from sys import argv, exit

PROGRAM_NAME = "logrotate-cli"
USAGE = "%s [OPTIONS]... [LOGFILE]\n\
    \t-d\t[directory]\t\t\tWrite rotated log to this directory (default: same as logfile)\n\
    \t-r\t<rotations\t\t\tAmount of rotations\n\
    \t-s\t<size>\t\t\tMax log size (Syntax like 5M; units (B, K, M, G))\n\
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

def print_info(message: str):
    """
    Print info

    :param message: message to print
    :type message: str
    """
    print("%s%s%s" % (KYEL, message, KNRM))

def print_success(message: str):
    """
    Print success

    :param message: message to print
    :type message: str
    """
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

#Units
class ByteSize(Enum):
    """
    This class represents byte sizes
    """
    BYTES = 'B'
    KILO_BYTES = 'K'
    MEGA_BYTES = 'M'
    GIGA_BYTES = 'G'

    @staticmethod
    def str_to_size(unit: str):
        """
        Convert string to ByteSize

        :param unit: string unit
        :type unit: str
        :returns ByteSize or None
        """
        if unit == "B":
            return ByteSize.BYTES
        elif unit == "K":
            return ByteSize.KILO_BYTES
        elif unit == "M":
            return ByteSize.MEGA_BYTES
        elif unit == "G":
            return ByteSize.GIGA_BYTES
        else:
            return None

    @staticmethod
    def to_bytes(units: int, size) -> int:
        """
        Convert bytes unit to bytes

        :param units: units amount
        :param size: size used
        :type units: int
        :type size: ByteSize
        :returns int
        """
        if size is ByteSize.BYTES:
            return units
        elif size is ByteSize.KILO_BYTES:
            return units * 1024
        elif size is ByteSize.MEGA_BYTES:
            return units * 1048576
        elif size is ByteSize.GIGA_BYTES:
            return units * 1073741824
        else:
            return 0

def rotate_previous_logs(max_rotations: int, destdir: str, logfile: str, logname: str) -> bool:
    """
    Rotate previously rotated logs

    :param max_rotations
    :param destdir
    :param logfile
    :param logname
    :type max_rotations: int
    :type destdir: str
    :type logfile: str
    :type logname: str
    :returns bool
    """
    for i in reversed(range(2, max_rotations + 1)): #From n to 2
        #Move thisRotation - 1 to thisRotation
        src_file = "%s/%s.%d.log" % (destdir, logname, i - 1)
        dest_file = "%s/%s.%d.log" % (destdir, logname, i)
        src_path = Path(src_file)
        if src_path.exists():
            #Move file
            try:
                rename(src_file, dest_file)
            except OSError as err:
                print_err("Could not move %s to %s: %s" % (src_file, dest_file, err))
                return False
    return True

def split_file(src: str, dest: str, max_size: int) -> bool:
    """
    Split a file into two;
    src will be read for max_size bytes; these bytes will be copied to dest; the last n bytes will be preserved in src

    :param src
    :param dest
    :param max_size
    :type src: str
    :type dest: str
    :type max_size: int
    :returns bool
    """
    try:
        dest_stream = open(dest, "w", errors="replace")
    except IOError as err:
        print_err("Could not open file %s: %s" % (dest, err))
        return False
    try:
        src_stream = open(src, "r+", errors="replace")
    except IOError as err:
        print_err("Could not open file %s: %s" % (src, err))
        dest_stream.close()
        return False
    curr_size = 0
    #Iterate over src_lines
    for line in src_stream.readlines():
        line_size = len(line)
        #If current size + line_size exceed max_size break
        if curr_size + line_size > max_size:
            break
        #Write row to dest
        dest_stream.write(line)
        curr_size += line_size
    #Close dest stream
    dest_stream.close()
    struncate(src_stream, curr_size)
    src_stream.close()
    return True

def struncate(file: TextIOWrapper, amount: int):
    """
    Truncate the first n bytes from the beginning of file

    :param file
    :param amount: amount of bytes to remove from start
    :type file: TextIOWrapper
    :type amount: int
    """
    #Get file size
    file.seek(0, 2)
    file_size = file.tell()
    #Go to the beginning of file
    file_offset = amount
    file.seek(0, 0)
    bytes_to_write = file_size - amount
    bytes_written = 0
    while bytes_written < bytes_to_write:
        #Move to offset + bytes_written
        file.seek(file_offset + bytes_written, 0)
        #Get bytes to rewrite
        block_size = 1024
        if bytes_to_write - bytes_written < block_size:
            block_size = bytes_to_write - bytes_written
        #Read block
        block_data = file.read(block_size)
        #Move to the beginning of file + bytes_written
        file.seek(bytes_written, 0)
        #Write block
        bytes_written += file.write(block_data)
    #Then truncate
    file.flush() #Flush write first
    file.seek(bytes_written)
    file.truncate()


def main(argc: int, argv: list) -> int:
    #Options
    destdir = None
    logfile = None
    max_rotations = None
    max_size = None
    #Get options
    try:
        optlist, args = getopt(argv, "d::r:s:h")
        #Iterate over options
        for opt, arg in optlist:
            if opt == "-d":
                destdir = arg
            elif opt == "-r":
                max_rotations = int(arg)
            elif opt == "-s":
                unit_size = arg[-1:]
                units = int(arg[:-1])
                byte_unit = ByteSize.str_to_size(unit_size)
                if byte_unit:
                    max_size = ByteSize.to_bytes(units, byte_unit)
                else:
                    opt_err("Unknown unit '%s'" % unit_size)
            elif opt == "-h":
                print(USAGE)
                return 255
        #Look for logfile
        if args:
            logfile = args[0]
        else:
            opt_err("Logfile is missing!")
    except GetoptError as err:
        opt_err(err)
    if not max_rotations or not max_size:
        opt_err("Missing mandatory options")
    #Get logfile params (basename and directory)
    log_path = Path(logfile)
    if not log_path.exists():
        print_err("Logfile %s doesn't exist" % logfile)
        return 1
    #If not destdir; destdir is log_path directory
    if not destdir:
        destdir = "%s/" % str(log_path.parent.absolute())
    #Check log size
    if max_size < log_path.stat().st_size:
        #Rotate log
        if not rotate_previous_logs(max_rotations, destdir, logfile, log_path.stem):
            print_err("Something bad happened while rotating logs; aborting...")
            return 1
        #Split current file then
        if not split_file(logfile, "%s/%s.1.log" % (destdir, log_path.stem), max_size):
            print_err("Something bad happened while rotating logs; aborting...")
            return 1
        print_info("File rotated")
    else:
        print_info("Current size %d is less than maximum size %d; nothing to do." % (log_path.stat().st_size, max_size))
    return 0

#Entry point
if __name__ == "__main__":
    exit(main(len(argv), argv[1:]))
