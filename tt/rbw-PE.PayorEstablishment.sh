#!/bin/bash
cd "${0%/*}/.." && BUD_NO_LOG=1 Tools/bud_BashDispatchUtility.sh "${0##*/}" "$@"