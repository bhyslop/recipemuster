#!/bin/bash

# This shows that exporting makes variable available to children, whereas without
#   the var is not propagated there.

ZRBV_SCRIPT_DIR="no_export"
export ZRBV_EXPORTED_DIR="yes_export"

echo "== In parent script =="
echo "ZRBV_SCRIPT_DIR: $ZRBV_SCRIPT_DIR"
echo "ZRBV_EXPORTED_DIR: $ZRBV_EXPORTED_DIR"

echo "== In child process =="
bash -c 'echo "ZRBV_SCRIPT_DIR: $ZRBV_SCRIPT_DIR"; echo "ZRBV_EXPORTED_DIR: $ZRBV_EXPORTED_DIR"'

