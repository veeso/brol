#!/usr/bin/env python3

from decimal import Decimal, DivisionByZero
import re
from sys import argv
from typing import List, Optional, Tuple


class Proportion(object):
    def __init__(
        self,
        arg1: Optional[Decimal],
        arg2: Optional[Decimal],
        arg3: Optional[Decimal],
        arg4: Optional[Decimal],
    ) -> None:
        # check if is valid
        args = [arg1, arg2, arg3, arg4]
        if args.count(None) != 1:
            raise Exception(
                f"invalid proportion; expected one 'x', found {args.count(None)}"
            )
        self.unknown = args.index(None)
        self.factors: Tuple[Decimal, Decimal] = (
            (args[0], args[3]) if self.unknown in [1, 2] else (args[1], args[2])
        )
        self.dividend: Decimal = args[{0: 3, 1: 2, 2: 1, 3: 0}.get(self.unknown)]
        if self.dividend.is_zero():
            raise DivisionByZero

    def calc(self) -> Decimal:
        return (self.factors[0] * self.factors[1]) / self.dividend


def main(args: List[str]) -> int:
    result = re.search(
        r"(\d*\.?\d*)?(x)?:(\d*\.?\d*)?(x)?=(\d*\.?\d*)?(x)?:(\d*\.?\d*)?(x)?",
        "".join(args),
    )
    if result is None:
        print("invalid syntax")
        return 1
    arg1 = None if result.group(2) == "x" else Decimal(result.group(1))
    arg2 = None if result.group(4) == "x" else Decimal(result.group(3))
    arg3 = None if result.group(6) == "x" else Decimal(result.group(5))
    arg4 = None if result.group(8) == "x" else Decimal(result.group(7))
    try:
        proportion = Proportion(arg1, arg2, arg3, arg4)
    except Exception as e:
        print(e)
        return 1
    print(proportion.calc())
    return 0


if __name__ == "__main__":
    main(argv[1:])
