#!/usr/bin/python3

"""
Search and retrieve metadata for Persons from WikiData starting from search
"""

# Getopts
from getopt import getopt, GetoptError
# JSON
import json
# Locale
from locale import getdefaultlocale
# Requests
import requests
#URL
from urllib.parse import quote
#Argv
from sys import argv, exit

from typing import List

DEFAULT_LOCALE = getdefaultlocale()[0].split("_")[0]

PROGRAM_NAME = "wikidata-search.py"
USAGE = "%s [OPTIONS...] [SEARCH]\n\
    Where options are:\n\
    \t-l\t<language>\tSpecify client language (default: %s)\n\
    \t-t\t<tries>\tSpecify amount of search result to fetch\n\
    \t-D\t\t\tDebug\n\
    \t-v\t\t\tVerbose\n\
    \t-h\t\t\tShow this page\n\
    " % (PROGRAM_NAME, DEFAULT_LOCALE)

KNRM = "\x1B[0m"
KRED = "\x1B[31m"
KGRN = "\x1B[32m"
KYEL = "\x1B[33m"
KBLU = "\x1B[34m"
KMAG = "\x1B[35m"
KCYN = "\x1B[36m"
KWHT = "\x1B[37m"

# Wikidata stuff
WIKIDATA_URL = "https://www.wikidata.org/w/api.php"

# Global options
debug = False
verbose = False

def print_err(message: str):
    """
    Print error
    :param message: message to print
    :type message: str
    """
    print("%s%s%s" % (KRED, message, KNRM))

def print_info(message: str):
    """
    Print information.
    The message will be displayed only when set in verbose mode
    :param message: message to print
    :type message: str
    """
    global verbose
    if verbose:
        print("%s%s%s" % (KYEL, message, KNRM))

def print_debug(message: str):
    """
    Print debug.
    The message will be displayed only when set in debug mode
    :param message: message to print
    :type message: str
    """
    global debug
    if debug:
        print("%s%s%s" % (KCYN, message, KNRM))

def usage(err: str = None):
    """
    Print usage
    """
    if err:
        print_err(err)
    print("%s" % USAGE)

def print_end():
    print("--------------------------------------------------------------------------------------------")

def perform_search(search: str, max_records: int) -> List[str]:
    """
    Perform search on wikidata
    :param search: search target
    :param max_records: maximum amount of records to return
    :returns List[str]
    """
    results = []
    url = "%s?format=json&action=query&list=search&srlimit=%d&srsearch=%s" % (WIKIDATA_URL, max_records, quote(search))
    # Perform request
    print_debug("Sending GET %s" % url)
    response = requests.get(url)
    data = response.json()
    print_debug("%s -> %d" % (url, response.status_code))
    print_debug("%s" % response.text)
    # Get search results
    records = data["query"]["search"]
    # Iterate over records
    for record in records:
        results.append(record["title"])
    return results

def get_wbentity(id: str, language: str) -> dict:
    """
    Get wb entity from wikidata querying id
    :param id: entity id
    :param language: search language
    :type id: str
    :type language: str
    :returns dict
    """
    url = "%s?format=json&action=wbgetentities&ids=%s&languages=%s" % (WIKIDATA_URL, id, language)
    # Perform request
    print_debug("Sending GET %s" % url)
    response = requests.get(url)
    data = response.json()
    print_debug("%s -> %d" % (url, response.status_code))
    print_debug("%s" % response.text)
    return data["entities"][id]

def get_image_info(filename: str) -> str:
    """
    Return image URI from filename
    :param filename: image filename
    :type filename: str
    :returns str
    """
    url = "%s?format=json&action=query&prop=imageinfo&iiprop=url&titles=File:%s" % (WIKIDATA_URL, filename)
    # Perform request
    print_debug("Sending GET %s" % url)
    response = requests.get(url)
    data = response.json()
    print_debug("%s -> %d" % (url, response.status_code))
    print_debug("%s" % response.text)
    return data["query"]["pages"]["-1"]["imageinfo"][0]["url"]

def process_entity(entity: dict, language: str) -> bool:
    """
    Process entity and print information
    :param entity: entity dictionary
    :param language: language
    :type entity: dict
    :type language: str
    :returns bool
    """
    # Check if entity is human
    claims = entity["claims"]
    if "P31" in claims: # Instance of
        # Iterate over instancesOf
        for instance in claims["P31"]:
            mainsnak = instance["mainsnak"]
            if "datavalue" in mainsnak:
                if mainsnak["datavalue"]["type"] == "wikibase-entityid":
                    # Verify if value is Q5
                    instanceof_value = mainsnak["datavalue"]["value"]["id"]
                    if instanceof_value == "Q5":
                        print_info("Entity is a person")
                        return process_human_entity(entity, language)
                    elif instanceof_value in TOPIC_TABLE:
                        print_info("Entity is a topic")
                        return process_topic_entity(entity, language)
    print_info("Entity is not a person")
    return True

def process_human_entity(entity: dict, language: str) -> bool:
    """
    Given a human entity, gather its metadata
    :param entity
    :param language
    :type entity dict
    :type language str
    :returns bool
    """
    try:
        claims = entity["claims"]
        # Get ID
        remote_id = entity["title"]
        print("%s\t%s" % ("ID".ljust(16), remote_id))
        # Get name from label
        name = entity["labels"][language]["value"].lower()
        print("%s\t%s" % ("name".ljust(16), name))
        # Get brief
        brief = entity["descriptions"][language]["value"].lower()
        print("%s\t%s" % ("brief".ljust(16), brief))
        # Birthdate
        if "P569" in claims:
            birthdate = claims["P569"][0]["mainsnak"]["datavalue"]["value"]["time"]
            print("%s\t%s" % ("birthdate".ljust(16), birthdate))
        # Is Dead
        print("%s\t%s" % ("is dead".ljust(16), "P570" in claims))
        # Citizenship
        if "P27" in claims:
            country_id = claims["P27"][0]["mainsnak"]["datavalue"]["value"]["id"]
            # Get country entity
            country_entity = get_wbentity(country_id, language)
            if "P297" in country_entity["claims"]:
                citizenship = country_entity["claims"]["P297"][0]["mainsnak"]["datavalue"]["value"]
                print("%s\t%s" % ("citizenship".ljust(16), citizenship))
        # Birthplace
        if "P19" in claims:
            city_id = claims["P19"][0]["mainsnak"]["datavalue"]["value"]["id"]
            # Get city entity
            city_entity = get_wbentity(city_id, language)
            if "P1448" in city_entity["claims"]:
                birthplace = city_entity["claims"]["P1448"][0]["mainsnak"]["datavalue"]["value"]["text"]
            else:
                birthplace = city_entity["labels"][language]["value"]
            print("%s\t%s" % ("birthplace".ljust(16), birthplace))
        # Image
        if "P18" in claims:
            image_id = claims["P18"][0]["mainsnak"]["datavalue"]["value"]
            image_uri = get_image_info(image_id)
            print("%s\t%s" % ("image".ljust(16), image_uri))
        # Occupation
        if "P106" in claims:
            occupation_id = claims["P106"][0]["mainsnak"]["datavalue"]["value"]["id"]
            occupation_entity = get_wbentity(occupation_id, language)
            occupation = occupation_entity["labels"][language]["value"].lower()
            print("%s\t%s" % ("occupation".ljust(16), occupation))
        print_end()
    except Exception as err:
        print_err("%s error: %s" % (remote_id, err))
        return False
    return True

def main(argc: int, argv: List[str]) -> int:
    global debug
    global verbose
    search = None
    search_tries = 4 # Default
    language = DEFAULT_LOCALE
    try:
        optlist, args = getopt(argv, "l::t::Dvh")
        #Iterate over options
        for opt, arg in optlist:
            if opt == "-l":
                language = DEFAULT_LOCALE
            elif opt == "-t":
                search_tries = int(arg)
            elif opt == "-D":
                debug = True
            elif opt == "-v":
                verbose = True
            elif opt == "-h":
                usage()
                return 255
        #Look for search
        if args:
            search = " ".join(args)
        else:
            usage("Missing search target")
            return 255
    except GetoptError as err:
        usage(err)
        return 255
    # Perform search
    search_targets = perform_search(search, search_tries)
    print_info("Found targets: %s" % search_targets)
    # Iterate over targets and get entities
    for wiki_id in search_targets:
        print_info("Working on %s" % wiki_id)
        entity = get_wbentity(wiki_id, language)
        if not process_entity(entity, language):
            print_err("Error in processing entity %s" % wiki_id)
    
    return 0

if __name__ == "__main__":
    exit(main(len(argv) - 1, argv[1:]))
