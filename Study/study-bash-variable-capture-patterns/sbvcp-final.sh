#!/bin/bash
set -e

# This is my 'smoking gun' to make sure that local var assignment and
#   error propagation functions so that the exit code makes it out.
#   This should always return exit code 23 with no output.

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

