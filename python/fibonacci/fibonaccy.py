#!/usr/bin/python3

"""
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

 Copyright (C) 2021 Christian Visintin

 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. You just DO WHAT THE FUCK YOU WANT TO.
"""

# Sys
from sys import argv, exit
# Typings
from typing import List

def fibonacci(n: int) -> int:
    if n == 0:
        return 0
    elif n < 3:
        return 1
    return fibonacci(n - 1) + fibonacci(n - 2)

def curious_fibonacci(n: int) -> int:
    if n == 0:
        return 0
    elif n < 3:
        return 1
    return ((curious_fibonacci(n - 2) * 2) + curious_fibonacci(n - 3))

def main(argc: int, argv: List[str]) -> int:
    # Get options
    if argc < 1:
        print("Usage: fibonaccy.py <n>")
        return 255
    recursions = int(argv[0])
    if recursions < 3:
        print("n must be 3 at least")
        return 1
    list(map(lambda x : print("%d" % fibonacci(x)), list(range(recursions))))
    print("-- Curious fibonacci --")
    list(map(lambda x : print("%d" % curious_fibonacci(x)), list(range(recursions))))
    # Return success
    return 0

# Entry point
if __name__ == "__main__":
    exit(main(len(argv[1:]), argv[1:]))
