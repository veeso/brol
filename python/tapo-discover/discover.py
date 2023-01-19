from pprint import pprint
import requests
from sys import argv
from typing import List, Dict, Any
from uuid import uuid4

BASE_URL = "https://eu-wap.tplinkcloud.com/"


def tapocloud_login(email: str, password: str) -> str:
    terminal_uuid = str(uuid4())

    login_request = {
        "method": "login",
        "params": {
            "appType": "Tapo_Android",
            "cloudPassword": password,
            "cloudUserName": email,
            "terminalUUID": terminal_uuid,
        },
    }

    response = requests.post(BASE_URL, json=login_request)
    if response.status_code != 200:
        raise Exception(f"expected 200, got {response.status_code}")

    data = response.json()
    if data["error_code"] != 0:
        raise Exception(f"Login failed")

    return data["result"]["token"]


def tapocloud_discover(cloud_token: str) -> Dict[Any, Any]:
    discover_request = {"method": "getDeviceList"}
    response = requests.post(f"{BASE_URL}?token={cloud_token}", json=discover_request)
    if response.status_code != 200:
        raise Exception(f"expected 200, got {response.status_code}")

    return response.json()["result"]["deviceList"]


def tidy_mac(mac: str) -> str:
    return mac.replace(r"/:/g", "").upper()


def main(args: List[str]) -> int:
    if len(args) < 2:
        print("Usage: <email> <password>")
        return 255

    email = args[0]
    password = args[1]

    try:
        cloud_token = tapocloud_login(email, password)
    except Exception as e:
        print(f"Login failed: {e}")
        return 1

    # discover devices
    try:
        devices = tapocloud_discover(cloud_token)
    except Exception as e:
        print(f"Failed to discover devices: {e}")
        return 1

    pprint(devices)

    return 0


if __name__ == "__main__":
    main(argv[1:])
