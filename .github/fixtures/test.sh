#!/usr/bin/env bash

set -e

while IFS='=' read -r variable value; do
    if [[ -n "$value" ]]; then
        echo "[+] $variable=$value"
        test "$(sysctl -b ${variable/ /})" = "${value/ /}"
    fi
done < "sysctl.conf"
