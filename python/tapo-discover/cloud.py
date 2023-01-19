import asyncio
import base64
from pprint import pprint
from tplinkcloud import TPLinkDeviceManager

import json
from sys import argv, exit

username = argv[1]
password = argv[2]


async def run():
    device_manager = TPLinkDeviceManager(username, password, verbose=True)
    devices = await device_manager.get_devices()
    if devices:
        print(f"Found {len(devices)} devices")
        for device in devices:
            device_name = base64.b64decode(device.get_alias()).decode("utf-8")
            print(f"{device.model_type.name} device called {device_name}")
            device = await device_manager.find_device(device.get_alias())
            if device:
                pprint(device.device_info)


loop = asyncio.get_event_loop()
loop.run_until_complete(run())
loop.stop()


exit(0)
