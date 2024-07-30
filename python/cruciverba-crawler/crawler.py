#!/usr/bin/env python3

from csv import DictReader, DictWriter
from selenium import webdriver
from selenium.webdriver import Firefox
from typing import List, Optional
from bs4 import BeautifulSoup
from getopt import getopt
from sys import argv
import os
import re

BASE_URL = "https://www.dizy.com"
ENTRYPOINTS = ["https://www.dizy.com/it/cruciverba/5431997007921152"]
# definition regex is /it/cruciverba/NUMBER
DEFINITION_RE = re.compile(r"/it/cruciverba/(\d+)")


class Word(object):

    def __init__(self, word: str, definition: str) -> None:
        self.word = word
        self.definition = definition


class CrawlResult(object):

    def __init__(self, word: Word, outpoints: List[str]) -> None:
        self.word = word
        self.outpoints = outpoints


def usage():
    print(f"Usage: {argv[0]} [-n <network>] [-o <outputfile>] [-h]")
    print("\t-l <count>: max count of words to crawl")
    print("\t-o <outputfile>: output file")
    print("\t-h: show this help message")


def get_page(url: str, driver: Firefox) -> BeautifulSoup:
    driver.get(url)
    soup = BeautifulSoup(driver.page_source, "html.parser")

    return soup


def crawl_page(soup: BeautifulSoup) -> Optional[CrawlResult]:
    definition_node = soup.find("h1")
    if not definition_node:
        return None
    definition = definition_node.text
    # get 5th span
    word_node = soup.find_all("span")[4]
    if not word_node:
        return None
    # <b> tag contains the word
    b_node = word_node.find("b")
    if not b_node:
        return None
    word = b_node.text
    if not word:
        return None
    word = Word(word, definition)
    # get outpoints
    outpoints: List[str] = []
    # get all <a> tags
    a_nodes = soup.find_all("a")
    for a_node in a_nodes:
        outpoint = a_node["href"]
        if outpoint and DEFINITION_RE.match(outpoint):
            outpoints.append(f"{BASE_URL}{outpoint}")

    return CrawlResult(word, outpoints)


def main(limit: int, output_file: str, entrypoints: List[str]) -> None:
    definitions: List[Word] = []
    url_cache: List[str] = []
    # check if file exists
    if os.path.exists(output_file):
        # first load existing words
        with open(output_file, "r") as f:
            reader = DictReader(f)
            for row in reader:
                definitions.append(Word(row["word"], row["definition"]))

    print(f"Found {len(definitions)} definitions")

    found = 0
    driver = webdriver.Firefox()
    while found < limit:
        new_entrypoints = []
        for url in entrypoints:
            if url in url_cache:
                continue
            page = get_page(url, driver)
            crawl_result = crawl_page(page)
            if crawl_result:
                url_cache.append(url)
                # check if definition is new
                definition_exists = False
                for definition in definitions:
                    if definition.definition == crawl_result.word.definition:
                        definition_exists = True
                        break
                if not definition_exists:
                    definitions.append(crawl_result.word)
                    print(crawl_result.word.word)
                    found += 1
                new_entrypoints.extend(crawl_result.outpoints)
        entrypoints = new_entrypoints

    driver.close()

    # finally write to file
    with open(output_file, "w") as f:
        writer = DictWriter(f, fieldnames=["word", "definition"])
        writer.writeheader()
        for definition in definitions:
            writer.writerow(
                {"word": definition.word, "definition": definition.definition}
            )


if __name__ == "__main__":
    try:
        optlist, args = getopt(argv[1:], "l:o:e:h")
    except:
        usage()
        exit(1)

    limit = 1_024
    output_file = "output.csv"
    entrypoints = ENTRYPOINTS
    for opt, arg in optlist:
        if opt == "-l":
            limit = int(arg)
        elif opt == "-o":
            output_file = str(arg)
        elif opt == "-e":
            entrypoints = arg.split(",")
        elif opt == "-h":
            usage()
            exit(1)

    main(limit, output_file, entrypoints)
