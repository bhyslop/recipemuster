#!/bin/bash
cd "${0%/*}/.." && BDU_NO_LOG=1 Tools/bdu_BashDispatchUtility.sh "${0##*/}" "$@"

