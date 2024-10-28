#!/usr/bin/env bash

set -e

# https://github.com/zed-industries/zed/issues/22098#issuecomment-2661578161
# update process:

# Rebase
# Reset+restore flake.lock and cargo.lock to upstream/main (to remove any of those conflicts)
# cargo update-lockfile; nix flake update; nix/update-pins.sh
# Amend the latest commit
# That usually works, and honestly could be scripted into a github action.

git fetch upstream main
# rebase and checkout flake.lock and cargo.lock
git rebase upstream/main
git checkout upstream/main -- flake.lock Cargo.lock
cargo update
nix flake update
# nix/update-pins.sh is a sibling script to this one
$(dirname $0)/update-pins.sh
