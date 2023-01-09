#!/usr/bin/python3

from random import choice
import requests
from typing import List
import urllib.parse
from sys import argv, exit


def get_name() -> str:
    gender = choice(["male", "female"])
    body = requests.get(f"https://api.namefake.com/italian-italy/{gender}/").json()
    return body["name"]


def subscribe(name: str, email: str) -> bool:
    name = urllib.parse.quote(name, safe="")
    email = urllib.parse.quote(email, safe="")
    data = f"action=bloom_subscribe&subscribe_data_array=%7B%22list_id%22%3A1441913%2C%22account_name%22%3A%22SpazioGrigio%22%2C%22service%22%3A%22convertkit%22%2C%22name%22%3A%22{name}%22%2C%22email%22%3A%22{email}%22%2C%22page_id%22%3A84%2C%22optin_id%22%3A%22optin_5%22%2C%22last_name%22%3A%22%22%2C%22ip_address%22%3A%22true%22%7D&subscribe_nonce=d8731c3a48"
    status = requests.post(
        "https://spaziogrigio.com/wp-admin/admin-ajax.php",
        data=data,
        headers={
            "content-type": "application/x-www-form-urlencoded; charset=UTF-8",
        },
    ).status_code
    return status == 200


def main(args: List[str]) -> int:
    if len(args) < 1:
        print("usage: <email>")
        return 255
    email = args[0]
    name = get_name()
    print(f"subscribing {name} with email {email}")
    if subscribe(name, email):
        print(f"Congratulations! {email} is now subscribed to spaziogrigio.com")
        return 0
    else:
        print("Could not subscribe to spaziogrigio.com")
        return 1


if __name__ == "__main__":
    exit(main(argv[1:]))
