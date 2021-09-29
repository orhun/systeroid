#!/usr/bin/env bash

set -e

metadata=$(cargo metadata --no-deps --format-version 1)
src_path=$(dirname $(jq -r '.packages[].targets[0].src_path' <<< "$metadata"))
wget -O "${src_path}/rst.pest" https://raw.githubusercontent.com/flying-sheep/rust-rst/master/parser/src/rst.pest
