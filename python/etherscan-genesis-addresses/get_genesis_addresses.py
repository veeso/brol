#!/usr/bin/env python3

from selenium import webdriver
from typing import List, Tuple
from bs4 import BeautifulSoup
import json

MAX_PAGE = 178


def get_page(page: int) -> BeautifulSoup:
    driver = webdriver.Firefox()
    driver.get(f"https://etherscan.io/txs?block=0&p={page}")
    soup = BeautifulSoup(driver.page_source, "html.parser")
    driver.close()

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
        to_address_col = cols[9]
        to_address_col = to_address_col.find("a")
        to_address = to_address_col["href"].replace("/address/", "")

        # get value
        value_col = cols[10]
        value_col = value_col.find("span")
        value = value_col["data-bs-title"].split(" ")[0]
        value = value.replace(",", "")
        eth = float(value)
        wei = str(int(eth * 1_000_000_000_000_000_000))

        genesis_addresses.append((to_address, wei))

    return genesis_addresses


if __name__ == "__main__":
    genesis_addresses = []
    for page in (0, MAX_PAGE + 1):
        soup = get_page(page)
        genesis_addresses += get_genesis_addresses(soup)

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
