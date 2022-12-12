#!/bin/sh
if [ "$#" != "1" ]; then
    exit 1
fi

cargo new "$1"
cp template/src/main.rs $1/src
cp template/Cargo.toml $1
cp template/ex.txt $1
git add $1/src/main.rs
git add $1/Cargo.toml
git add $1/ex.txt
git commit -m"initialized $1"


