#!/bin/bash 

# Development build script, for quickly launching a binary with test
# input parameters
scriptName="$(basename $(pwd))"

PROD_BUILD=0

if [[ $PROD_BUILD == 1 ]]; then 
  cargo build --release
else 
  cargo build 
fi

SCRIPT_NAME=$scriptName python client.py $PROD_BUILD

