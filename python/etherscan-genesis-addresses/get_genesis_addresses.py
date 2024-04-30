#!/usr/bin/env python3

from selenium import webdriver
from typing import List, Tuple
from bs4 import BeautifulSoup
from getopt import getopt
import json

MAX_PAGE = 178
NETWORK_GOERLI = "goerli"
NETWORK_MAINNET = "mainnet"
NETWORK_SEPOLIA = "sepolia"
NETWORKS = [
    NETWORK_GOERLI,
    NETWORK_MAINNET,
    NETWORK_SEPOLIA,
]
NETWORKS_URL = {
    NETWORK_GOERLI: "https://goerli.etherscan.io",
    NETWORK_MAINNET: "https://etherscan.io",
    NETWORK_SEPOLIA: "https://sepolia.etherscan.io",
}


def get_page(network: str, page: int, driver: webdriver.Firefox) -> BeautifulSoup:
    driver.get(f"{NETWORKS_URL[network]}/txs?block=0&p={page}")
    soup = BeautifulSoup(driver.page_source, "html.parser")

    return soup


def get_genesis_addresses(soup: BeautifulSoup) -> List[Tuple[str, str]]:
    table_container = soup.find(id="ContentPlaceHolder1_divTransactions")

    if table_container is None:
        raise "Could not find 'ContentPlaceHolder1_divTransactions'"
    table_body = table_container.find("tbody")
    if table_body is None:
        raise "Could not find 'tbody'"

    genesis_addresses = []
    # iter over table rows
    table_rows = table_body.find_all("tr")

    for row in table_rows:
        cols = row.find_all("td")
        if len(cols) < 11:
            continue
        to_address_col = cols[9]
        to_address_col = to_address_col.find("a")
        to_address = to_address_col["href"].replace("/address/", "")

        # get value
        value_col = cols[10]
        value_col = value_col.find("span")
        value = value_col["data-bs-title"].split(" ")[0]
        value = value.replace(",", "")
        eth = int(value)
        wei = str(int(eth * 1_000_000_000_000_000_000))

        genesis_addresses.append((to_address, wei))

    return genesis_addresses


def usage():
    print(f"Usage: {argv[0]} [-n <network>] [-o <outputfile>] [-h]")
    print(
        "\t-n <network>: network to get genesis addresses from (mainnet, sepolia, goerli)"
    )
    print("\t-o <outputfile>: output file")
    print("\t-h: show this help message")


if __name__ == "__main__":
    # getopts
    from sys import argv

    try:
        optlist, args = getopt(argv[1:], "n:o:h")
    except:
        usage()
        exit(1)

    network = NETWORK_MAINNET
    outputfile = "output.json"
    for opt, arg in optlist:
        if opt == "-n":
            if arg not in NETWORKS:
                usage()
                exit(1)
            network = str(arg)
        elif opt == "-o":
            outputfile = str(arg)
        elif opt == "-h":
            usage()
            exit(1)

    if not network in NETWORKS:
        usage()
        exit(1)

    genesis_addresses = []
    driver = webdriver.Firefox()
    for page in (0, MAX_PAGE + 1):
        soup = get_page(network, page, driver)
        genesis_addresses += get_genesis_addresses(soup)

    driver.close()

    # prepare output
    genesis_addresses_list = []
    for address, wei in genesis_addresses:
        genesis_addresses_list.append(
            {
                "address": address,
                "tokens": wei,
            }
        )
    output = {"genesisAddresses": genesis_addresses_list}
    # write output file
    with open("output.json", "w") as f:
        f.write(json.dumps(output))
        f.close()
