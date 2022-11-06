#! /usr/bin/env python3

from __future__ import annotations

import json
import logging
import os
import re
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

GITHUB_OUTPUT_FILE = os.getenv("GITHUB_OUTPUT")
assert GITHUB_OUTPUT_FILE is not None


def main():
    logging.basicConfig(format="%(levelname)s: %(message)s", level=logging.INFO)
    assemble()


def assemble():
    package = PackageMetadata.load()
    cargo_install(package)
    archive_path = create_archive(package)

    set_output_parameter("archive-path", str(archive_path))
    set_output_parameter("archive-name", archive_path.name)


@dataclass
class PackageMetadata:
    name: str
    version: str
    target_dir: Path

    @staticmethod
    def load() -> PackageMetadata:
        logging.info("loading package metadata...")
        result = run_output(
            "cargo", "metadata", "--no-deps", "--format-version=1", "--locked"
        )
        result_json = json.loads(result)
        metadata = PackageMetadata(
            result_json["packages"][0]["name"],
            result_json["packages"][0]["version"],
            Path(result_json["target_directory"]),
        )
        logging.info("package metadata = %s", metadata)
        return metadata

    @property
    def install_dir(self) -> Path:
        return self.target_dir / "install"

    @property
    def install_bin_dir(self) -> Path:
        return self.install_dir / "bin"


def cargo_install(package: PackageMetadata):
    logging.info("installing the package to %s...", package.install_dir)
    subprocess.run(
        [
            "cargo",
            "install",
            "--path=.",
            f"--root={package.install_dir}",
            "--force",
            "--no-track",
            "--locked",
            "--verbose",
        ],
        check=True,
    )


def create_archive(package: PackageMetadata) -> Path:
    logging.info("creating archive...")

    # create .tar.gz files on Linux and macOS, .zip files on Windows
    if sys.platform.startswith("linux") or sys.platform == "darwin":
        format = "gztar"
    elif sys.platform == "win32":
        format = "zip"
    else:
        raise RuntimeError("unsupported operating system")

    target_triple = get_target_triple()
    base_name = f"{package.name}-{package.version}-{target_triple}"
    base_path = package.install_dir / base_name  # without extension
    logging.info("archive basename = %s, format = %s", base_name, format)

    path = shutil.make_archive(str(base_path), format, package.install_bin_dir)
    path = Path(path)
    logging.info("archive = %s", path)

    return path


def get_target_triple() -> str:
    output = run_output("rustc", "--version", "--verbose")
    match = re.search(r"host: ([\w-]+)", output)
    assert match is not None

    target_triple = match.group(1)
    return target_triple


def set_output_parameter(name: str, value: str):
    with open(GITHUB_OUTPUT_FILE, "a", encoding="utf-8") as f:
        f.write(f"{name}={value}\n")


def run(*command_line: str):
    assert len(command_line) > 0
    subprocess.run(command_line, check=True)


def run_output(*command_line: str) -> str:
    assert len(command_line) > 0
    result = subprocess.run(
        command_line,
        capture_output=True,
        check=True,
        encoding="utf-8",
    )
    return result.stdout


if __name__ == "__main__":
    main()
