#!/bin/bash

# Get the base name of the script
script_name=$(basename "$0")

# Extract the parameter from the script name
# It's the text between the first and second '.' characters
parameter=$(echo "$script_name" | cut -d'.' -f2)

# Change to the parent directory
cd "$(dirname "$0")/.." || exit 1

echo parameter is $parameter

# Call the inner script with the extracted parameter
Tools/tabtarget-dispatch.sh 1 "$script_name" "MBSR_ARG_MONIKER=$parameter"d