from math import pi
from sys import argv, exit
from typing import List

def circumference(diameter: int) -> float:
    return diameter * pi

def calc_your_dose(recipe_diameter: int, your_diameter: int, dose: int) -> int:
    c1 = circumference(recipe_diameter)
    c2 = circumference(your_diameter)

    # c1 : dose = c2 : x
    return round((dose * c2) / c1)

def main(args: List[str]) -> int:
    if len(args) < 3:
         print("Usage: <recipe-diameter> <your-diameter> <dose>")
         return 255

    recipe_diameter = int(args[0])
    your_diameter = int(args[1])
    dose = int(args[2])

    your_dose = calc_your_dose(recipe_diameter, your_diameter, dose)
    print(f"Dose: {your_dose}")

    return 0

if __name__ == "__main__":
    exit(main(argv[1:]))

