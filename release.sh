#!/bin/sh

cargo build

mkdir ./release
mkdir ./release/archive
mkdir ./release/templates

cp ./target/debug/nota ./release

zip -r nota.zip ./release

cp -R release/ ~/Documents/nota
