#!/bin/bash
set -e

failing_cmd() {
    echo "output_from_failing_cmd"
    return 1
}

test_function() {
    local var
    var=$(failing_cmd) || exit 23
    echo "This line should never execute: var='$var'"
}

test_function
echo "This line should never execute either"

