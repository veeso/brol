#!/usr/bin/env python3

import json
from sys import argv

if __name__ == "__main__":
    json_file = argv[1]
    output_file = argv[2]

    with open(json_file, "r") as f:
        records = json.loads(f.read())
        f.close()

    genesis_accounts = records["genesisAddresses"]

    with open(output_file, "w") as f:
        for account in genesis_accounts:
            address = account["address"]
            tokens = account["tokens"]
            f.write(f'record {{ 0= "{address}"; 1= opt "{hex(int(tokens))}"}};\n')
        f.close()
