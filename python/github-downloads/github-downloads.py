#!/usr/bin/python3

"""
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

 Copyright (C) 2021 Christian Visintin

 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. You just DO WHAT THE FUCK YOU WANT TO.
"""

# Getopt
from argparse import ArgumentParser
# Requests
import requests
# Sys
from sys import argv, exit
# Tabulate
import tabulate
# Typings
from typing import List, Tuple, Optional

KNRM = "\x1B[0m"
KRED = "\x1B[31m"
KGRN = "\x1B[32m"
KYEL = "\x1B[33m"
KBLU = "\x1B[34m"
KMAG = "\x1B[35m"
KCYN = "\x1B[36m"
KWHT = "\x1B[37m"

def print_err(message: str):
    """
    Print error

    :param message: message to print
    :type message: str
    """
    print("%s%s%s" % (KRED, message, KNRM))

def parse_extensions(files: str) -> List[str]:
    """
    Parse extensions from option and convert them into a list

    :param files
    """
    return files.split(",")

def fetch_github_releases(author: str, reponame: str) -> dict:
    """
    Fetch github releases

    :param author
    :param reponame
    :returns dict
    :raises requests.RequestException
    """
    response = requests.get("https://api.github.com/repos/%s/%s/releases" % (author, reponame))
    return response.json()

def is_asset_blacklisted(name: str, blacklist: List[str]) -> bool:
    """
    Check whether an asset must be filtered

    :param name
    :param blacklist
    :returns bool
    """
    return any(map(lambda x : name.endswith(x), blacklist))
    
def collect_stats(payload: dict, blacklist: List[str]) -> List[Tuple[str, List[Tuple[str, int]]]]:
    """
    Collect stats for each release in payload

    :param payload
    :param blacklist
    :returns List[Tuple(str, List[Tuple(str, int)])]
    """
    return list(map(lambda x : collect_release_stats(x, blacklist), payload))

def collect_release_stats(release: dict, blacklist: List[str]) -> Tuple[str, List[Tuple[str, int]]]:
    """
    Collect stats for a certain release

    Returns a tuple where the first element is the tag name and the second one is a list of tuples with an association between
    artifact and download counter

    :param realse
    :param blacklist
    :returns Tuple(str, List[Tuple(str, int)])
    """
    tag_name: str = release["tag_name"]
    assets_stats = []
    for asset in release["assets"]:
        stats = collect_asset_stats(asset, blacklist)
        if stats:
            assets_stats.append(stats)
    # Return stats
    return (tag_name, assets_stats)

def collect_asset_stats(asset: dict, blacklist: List[str]) -> Optional[Tuple[str, int]]:
    """
    Collect stats for a certain asset in the release

    Return the asset name and the download count

    :param asset
    :param blacklist
    :returns Tuple(str, int) or None if filtered
    """
    asset_name: str = asset["name"]
    # Check if name is blacklisted
    if is_asset_blacklisted(asset_name, blacklist):
        return None
    else:
        download_count: int = asset["download_count"]
        return (asset_name, download_count)

def calculate_version_downloads(counters: List[int]) -> int:
    """
    Calculate version downloads
    
    :param counters
    """
    return sum(counters)

def filter_releases_by_version(releases: List[Tuple[str, List[Tuple[str, int]]]], tag: str) -> List[Tuple[str, List[Tuple[str, int]]]]:
    """
    Keep only the release with the provided tag name

    :param releases
    :param tag
    :returns list
    """
    return list(filter(lambda x : x[0] == tag, releases))

def print_stats_table(repo: str, release: Tuple[str, List[Tuple[str, int]]]):
    """
    Print stats table for a single release

    :param repo
    :param release
    """
    # Push total downloads to version
    release[1].append(("total", calculate_version_downloads(list(map(lambda x : x[1], release[1])))))
    print("%s version %s" % (repo, release[0]))
    print(tabulate.tabulate(release[1], headers=["Artifact", "Downloads"], tablefmt="github"))
    print()

def main(argc: int, argv: List[str]) -> int:
    # Get options
    parser = ArgumentParser(description="Get download stats for a certain github repo")
    parser.add_argument("-e", "--exclude", help="Specify extensions to exclude from response")
    parser.add_argument("-v", "--version", help="Fetch stats for only the provided tag name")
    parser.add_argument("REPOSITORY", help="Specify the repository to fetch (author/reponame)")
    args = parser.parse_args(argv)
    (author, repo_name) = args.REPOSITORY.split("/")
    if args.exclude:
        exclude = parse_extensions(args.exclude)
    else:
        exclude = []
    keep_version = args.version
    # Fetch github
    try:
        releases = fetch_github_releases(author, repo_name)
    except requests.RequestException as err:
        print_err("Failed to fetch releases: %s" % err)
        return 1
    stats = collect_stats(releases, exclude)
    # Filter version if required
    if keep_version:
        stats = filter_releases_by_version(stats, keep_version)
    # Print table
    list(map(lambda x : print_stats_table(repo_name, x), stats))
    # Return success
    return 0

# Entry point
if __name__ == "__main__":
    exit(main(len(argv[1:]), argv[1:]))
