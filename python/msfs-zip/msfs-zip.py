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

from argparse import ArgumentParser
import os
import rarfile
from sys import argv, exit
import zipfile
from typing import List, Optional


class ArchiveSimObjects(object):

    def __init__(self, file_path: str, inner_path: str) -> None:
        self.file_path = file_path
        self.inner_path = inner_path

        # get parent
        norm_inner_path = os.path.normpath(inner_path)
        parts = norm_inner_path.split(os.sep)

        self.sim_objects_parent = parts[-1]


def find_simobjects_root(namelist: List[str]):
    for item in namelist:
        # Verifica se "SimObjects/" è una sottocartella
        if "SimObjects/" in item:
            # Ottieni il percorso della cartella padre di "SimObjects"
            return os.path.split(item.split("SimObjects/")[0])[0]
        # Sceneries have `ContentInfo` instead of `SimObjects`
        elif "ContentInfo/" in item:
            return os.path.split(item.split("ContentInfo/")[0])[0]
    return None


def get_output_path(path: str, inner_path: str) -> str:
    # Normalize the path to avoid issues with trailing slashes
    path = os.path.normpath(path)
    # Split the path into components
    parts = path.split(os.sep)
    # Find the index of "SimObjects"
    if inner_path in parts:
        sim_idx = parts.index(inner_path)
        # Return the parent and "SimObjects" directories
        out_path = os.sep.join(parts[sim_idx:])
        return out_path

    raise ValueError(f"{inner_path} directory not found in path")


def get_simobjects_zip(file_name: str) -> Optional[ArchiveSimObjects]:
    with zipfile.ZipFile(file_name, "r") as zip_ref:
        simobjects_parent_dir = find_simobjects_root(zip_ref.namelist())
        if simobjects_parent_dir:
            return ArchiveSimObjects(file_name, simobjects_parent_dir)


def get_simbojects_rar(file_name: str) -> Optional[ArchiveSimObjects]:
    with rarfile.RarFile(file_name, "r") as rar_ref:
        simobjects_parent_dir = find_simobjects_root(rar_ref.namelist())
        if simobjects_parent_dir:
            return ArchiveSimObjects(file_name, simobjects_parent_dir)


def get_simbojects_archive(file_name: str) -> Optional[ArchiveSimObjects]:
    if file_name.endswith(".zip"):
        return get_simobjects_zip(file_name)
    elif file_name.endswith(".rar"):
        return get_simbojects_rar(file_name)
    else:
        return None


def get_simbojects_archives(dir: str) -> List[ArchiveSimObjects]:
    dir_files = list(
        map(
            lambda file: os.path.join(dir, file),
            filter(lambda x: x.endswith(".zip") or x.endswith(".rar"), os.listdir(dir)),
        )
    )

    maybe_simbojects_archives = map(
        lambda file: get_simbojects_archive(file), dir_files
    )
    simbojects_archives = filter(lambda x: x is not None, maybe_simbojects_archives)

    filtered: List[ArchiveSimObjects] = list(simbojects_archives)  # type: ignore

    return filtered


def merge_archives(archives: List[ArchiveSimObjects], output: str) -> None:
    with zipfile.ZipFile(output, "w") as zip_ref:
        for archive in archives:
            print("Adding archive", archive.file_path)
            if archive.file_path.endswith(".rar"):
                with rarfile.RarFile(archive.file_path, "r") as archive_ref:
                    for file in archive_ref.namelist():
                        if file.startswith(archive.inner_path):
                            # check if file is a directory, if so, skip
                            if file.endswith("/"):
                                continue
                            print(file)

                            # read file and write it to the zip
                            data = archive_ref.read(file)
                            output_file = get_output_path(
                                file, archive.sim_objects_parent
                            )
                            zip_ref.writestr(output_file, data)

            elif archive.file_path.endswith(".zip"):
                with zipfile.ZipFile(archive.file_path, "r") as archive_ref:
                    for file in archive_ref.namelist():
                        if file.startswith(archive.inner_path):
                            # check if file is a directory, if so, skip
                            if file.endswith("/"):
                                continue
                            # read file and write it to the zip
                            print(file)
                            data = archive_ref.read(file)
                            output_file = get_output_path(
                                file, archive.sim_objects_parent
                            )
                            zip_ref.writestr(output_file, data)


def main(args: List[str]) -> int:
    # Get options
    parser = ArgumentParser(
        description="Zip different msfs downloads in a single zip to extract quickly in the community folder"
    )
    parser.add_argument("-o", "--output", help="Specify the output file path")
    parser.add_argument(
        "ZIP_DIR", help="Specify the directory containing the zip files"
    )
    cli_args = parser.parse_args(args)
    zip_dir = cli_args.ZIP_DIR
    output = cli_args.output

    archives_to_add = get_simbojects_archives(zip_dir)

    if len(archives_to_add) == 0:
        print("No simobjects archives found")
        return 1

    # Merge archives
    merge_archives(archives_to_add, output)

    # Return success
    return 0


# Entry point
if __name__ == "__main__":
    exit(main(argv[1:]))
