#!/usr/bin/env bash

set -eux

docs_dir="$(pwd)/linux-docs"
kernel_repo="https://github.com/torvalds/linux"
sparse_path=('Documentation/admin-guide/sysctl' 'Documentation/networking')

mkdir "$docs_dir" && cd "$docs_dir"
git init
git remote add origin "$kernel_repo"
git config core.sparseCheckout true
for path in "${sparse_path[@]}"; do
    echo "$path" >> .git/info/sparse-checkout
done
git pull --depth 1 origin master
mv $docs_dir/Documentation/* "$docs_dir"
cd "$docs_dir" && rmdir Documentation && rm -rf .git
