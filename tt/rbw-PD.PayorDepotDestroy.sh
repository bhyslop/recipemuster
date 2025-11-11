#!/bin/bash
cd "${0%/*}/.." && BUD_NO_LOG=1 Tools/buk/bud_dispatch.sh "${0##*/}" "$@"