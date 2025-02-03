#!/bin/bash

# downloads repo
dl_repo() {
  wget -nv -O- "https://github.com/$1/archive/refs/heads/main.zip" ||
  wget -nv -O- "https://github.com/$1/archive/refs/heads/master.zip"
}

export -f dl_repo

# downloads and extracts repo
get_repo() {
  dl_repo $1 | bsdtar -xf - -C repos/
}

export -f get_repo

mkdir -p repos

# run in parallel
head -n 500 ./repos.lst | parallel -j 8 get_repo
