#!/bin/bash

set -eou pipefail

if [ -z ${USER_MANUPULSE+x} ]; then
    echo "env var USER_MANUPULSE is unset"
    exit
fi
if [ -z ${PASSWORD_MANUPULSE+x} ]; then
    echo "env var PASSWORD_MANUPULSE is unset"
    exit
fi

k6 run -e USER_MANUPULSE="$USER_MANUPULSE" -e PASSWORD_MANUPULSE="$PASSWORD_MANUPULSE" -d 2s -u 30 benchmarks/k6.js
