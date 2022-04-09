#!/usr/bin/env bash

set -e

changelog_file="CHANGELOG.md"
version_suffix="# managed by release.sh"
commit_prefix="chore(release): prepare for "
tag_prefix="Release "
tagger_name="systeroid project"
tagger_email="systeroid@protonmail.com"
signing_key="03598A29AA9D297B8D6D94F1EBEB8A69FDA3720E"

usage() { printf "usage: %s <tag>\n" "${BASH_SOURCE[0]##*/}"; exit 0; }

bail() { printf "error: %s\n" "$1" "${@:2}"; exit 1; }

[ -z "$1" ] && usage
[[ "$1" != v* ]] && bail "tag name should start with 'v'"

changelog=$(git diff -U0 "$changelog_file" | grep '^[+][^+]' | sed 's/^[+]//;s/^###\s*//')
[ -z "$changelog" ] && bail "$changelog_file is not updated"

sed "s/^version = \".*\" $version_suffix$/version = \"${1#v}\" $version_suffix/g" \
    -i -- */Cargo.toml

gawk -i inplace \
    -v date="\"$(date +%Y-%m-%d)\"" \
    '/\.TH\s.*+"8".*"System Administration"/{ $4 = date } 1' man*/*

git add -A
git commit -m "$commit_prefix$1"
git show

git -c user.name="$tagger_name" \
    -c user.email="$tagger_email" \
    -c user.signingkey="$signing_key" \
    tag -s -a "$1" -m "$tag_prefix$1" -m "$changelog"
git tag -v "$1"
