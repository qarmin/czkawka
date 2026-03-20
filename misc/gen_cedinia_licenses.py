#!/usr/bin/env python3
"""
Generate THIRD_PARTY_LICENSES.txt for Cedinia from `cargo metadata` JSON output.
Usage: cargo metadata --format-version 1 | python3 misc/gen_cedinia_licenses.py
"""

import json
import sys
from collections import deque

data = json.load(sys.stdin)

# Find cedinia's package id
cedinia_id = None
for pkg in data["packages"]:
    if pkg["name"] == "cedinia":
        cedinia_id = pkg["id"]
        break

if cedinia_id is None:
    print("ERROR: cedinia package not found in metadata", file=sys.stderr)
    sys.exit(1)

# Build resolve map: id -> list of dep ids
resolve_map: dict[str, list[str]] = {}
for node in data["resolve"]["nodes"]:
    resolve_map[node["id"]] = [dep["pkg"] for dep in node.get("deps", [])]

# BFS to collect all transitive dependencies of cedinia
visited: set[str] = set()
queue: deque[str] = deque([cedinia_id])
while queue:
    pkg_id = queue.popleft()
    if pkg_id in visited:
        continue
    visited.add(pkg_id)
    for dep_id in resolve_map.get(pkg_id, []):
        if dep_id not in visited:
            queue.append(dep_id)

# Exclude cedinia itself — we only want third-party deps
visited.discard(cedinia_id)

pkg_map = {pkg["id"]: pkg for pkg in data["packages"]}

out = []
for pkg_id in sorted(visited, key=lambda i: pkg_map[i]["name"].lower()):
    if pkg_id not in pkg_map:
        continue
    pkg = pkg_map[pkg_id]
    name = pkg.get("name", "?")
    version = pkg.get("version", "")
    spdx = pkg.get("license") or "UNKNOWN"
    authors = ", ".join(pkg.get("authors") or [])
    repo = pkg.get("repository") or ""

    line = f"{name} {version}  [{spdx}]"
    if authors:
        line += f"  — {authors}"
    if repo:
        line += f"\n    {repo}"
    out.append(line)

print("\n".join(out))
