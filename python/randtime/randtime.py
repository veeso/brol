#!/usr/bin/env python

"""
Developed by Christian Visintin
Generate a random time in ISO8601
"""

from datetime import datetime
from random import randint
from time import time
from sys import argv, exit

def main(argc, argv):
    if argc == 0:
        print("Usage: randtime.py <day_offset>")
        return 255
    # Get offset
    offset = int(argv[0])
    # Get time
    t_now = int(time())
    # Days to second
    offset *= 86400
    # Generate random
    days = randint(0, offset)
    # Subtract days from time
    t_calc = t_now - days
    # Format time
    dt = datetime.fromtimestamp(t_calc)
    print(dt.strftime("%Y-%m-%dT%H:%M:%S%z"))
    return 0


if __name__ == "__main__":
    args = argv[1:]
    exit(main(len(args), args))
