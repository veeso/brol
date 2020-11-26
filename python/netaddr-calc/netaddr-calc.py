#!/usr/bin/python3

"""
Developed by Christian Visintin
netaddr-calc is a script which allows to calculate different network values

    - netmask to CIDR (24 => 255.255.255.0)
    - CIDR to netmask (255.255.0.0 => 16)
    - ip/netmask to network address (192.168.0.1/24 => 192.168.0.0)
    - ip/netmask to broadcast address (192.168.0.1/24 => 192.168.0.255)
"""

from math import sqrt, log2
#System
from sys import argv, exit

PROGRAM_NAME = "netaddr-calc"
USAGE = "%s [COMMAND] [ARGS...]\n\
    where commands are:\n\
        N2C\t<netmask>\t\t\tnetmask to CIDR\n\
        C2N\t<CIDR>\t\t\t\tCIDR to netmask\n\
        IP2NET\t<address> <netmask>\t\tip to network address\n\
        IP2BRD\t<address> <netmask>\t\tip to broadcast address\n\
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

def netmask_to_cidr(netmask: int) -> list:
    """
    Calculate CIDR from netmask

    :param netmask
    :type netmask int
    :returns list
    :raises Exception
    """
    if netmask > 32 or netmask < 1:
        raise Exception("Invalid netmask %d" % netmask)
    mask = pow(2, netmask) - 1
    shift = 32 - netmask
    mask = mask << shift
    # Print netmask
    return [((mask >> 24) & 0xFF), ((mask >> 16) & 0xFF), ((mask >> 8) & 0xFF), (mask & 0xFF)]

def cidr_to_netmask(cidr: str) -> int:
    """
    Calculate netmask from CIDR

    :param cidr
    :type cidr str
    :returns int
    :raises Exception
    """
    # Split cidr
    blocks = cidr.split(".")
    if len(blocks) != 4:
        raise Exception("Invalid CIDR: %s" % cidr)
    # Validate cidr
    for i in range(4):
        blocks[i] = int(blocks[i])
        valid_value = False
        for j in range(9):
            if (256 - pow(2, j)) == blocks[i]:
                valid_value = True
                break
        if not valid_value:
            raise Exception("Invalid netmask %s" % cidr)
    # Get netmask
    netmask = 0
    netmask += log2(256 - blocks[0])
    netmask += log2(256 - blocks[1])
    netmask += log2(256 - blocks[2])
    netmask += log2(256 - blocks[3])
    netmask = int(32 - netmask)
    return netmask

def calc_network_addr(addr: str, netmask: int) -> list:
    """
    Calculate network address from ip address and netmask

    :param addr
    :param netmask
    :type addr str
    :type netmask int
    :returns list
    :raises Exception
    """
    # Calc netmask cidr
    cidr = netmask_to_cidr(netmask)
    # Split address
    addr = addr.split(".")
    if len(addr) != 4:
        raise Exception("Invalid ip address: %s" % addr)
    # Verify address
    for i in range(4):
        addr[i] = int(addr[i])
        if addr[i] < 0 or addr[i] > 255:
            raise Exception("Invalid ip address: %s" % addr)
    # Network address
    network_addr = [
        addr[0] & cidr[0],
        addr[1] & cidr[1],
        addr[2] & cidr[2],
        addr[3] & cidr[3]
    ]
    return [network_addr[0], network_addr[1], network_addr[2], network_addr[3]]

def calc_broadcast_addr(addr: str, netmask: int) -> list:
    """
    Calculate broadcast address from ip address and netmask

    :param addr
    :param netmask
    :type addr str
    :type netmask int
    :returns list
    :raises Exception
    """
    # Calc network address
    network_address = calc_network_addr(addr, netmask)
    joined_address = []
    #for i in range(4):
    #    for j in range(8):
    #        joined_address.append((network_address[i]) >> j)
    or_mask = pow(2, 32 - netmask) - 1
    joined_address = (network_address[0] << 24) + (network_address[1] << 16) + (network_address[2] << 8) + network_address[3]
    joined_address = joined_address | or_mask
    return [((joined_address >> 24) & 0xFF), ((joined_address >> 16) & 0xFF), ((joined_address >> 8) & 0xFF), (joined_address & 0xFF)]

def main(argc: int, argv: list) -> int:
    command = None
    args = argv[1:]
    if len(args) == 0:
        print_err("Missing command")
        print_info(USAGE)
        return 255
    # Get command
    command = args[0]
    command = command.upper()
    # Switch on command
    if command == "N2C":
        if len(args) < 2:
            print_err("Missing <netmask> argument")
            print_info(USAGE)
            return 255
        netmask = args[1]
        try:
            cidr = netmask_to_cidr(int(netmask))
            print("%d.%d.%d.%d" % (cidr[0], cidr[1], cidr[2], cidr[3]))
            return 0
        except Exception as err:
            print_err(err)
            return 1
    elif command == "C2N":
        if len(args) < 2:
            print_err("Missing <cidr> argument")
            print_info(USAGE)
            return 255
        try:
            print(cidr_to_netmask(args[1]))
            return 0
        except Exception as err:
            print_err(err)
            return 1
    elif command == "IP2NET":
        if len(args) < 3:
            print_err("Missing <address> <netmask> arguments")
            print_info(USAGE)
            return 255
        try:
            net_addr = calc_network_addr(args[1], int(args[2]))
            print("%d.%d.%d.%d" % (net_addr[0], net_addr[1], net_addr[2], net_addr[3]))
            return 0
        except Exception as err:
            print_err(err)
            return 1
    elif command == "IP2BRD":
        if len(args) < 3:
            print_err("Missing <address> <netmask> arguments")
            print_info(USAGE)
            return 255
        try:
            broadcast_addr = calc_broadcast_addr(args[1], int(args[2]))
            print("%d.%d.%d.%d" % (broadcast_addr[0], broadcast_addr[1], broadcast_addr[2], broadcast_addr[3]))
            return 0
        except Exception as err:
            print_err(err)
            return 1
    else:
        print_err("Unknown command '%s'" % command)
        print_info(USAGE)
        return 255
    return 0

if __name__ == "__main__":
    exit(main(len(argv), argv))
