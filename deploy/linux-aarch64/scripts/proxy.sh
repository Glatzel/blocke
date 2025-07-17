#!/bin/bash

if [ -z "$1" ]; then
    read -p "Enter proxy URL (leave empty to disable proxy): " PROXY
else
    PROXY="$1"
fi

if [ -z "$PROXY" ]; then
    echo "Disabling proxy..."
    unset http_proxy
    unset https_proxy
else
    echo "Setting proxy to $PROXY"
    export http_proxy="$PROXY"
    export https_proxy="$PROXY"
fi
