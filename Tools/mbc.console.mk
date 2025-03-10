## ©️ 2023 Scale Invariant, Inc.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> December 2023


# Set below variable to add a localization context to pretty lines
MBC_ARG__CTXT ?= mbu-c

# No quotes since value is integer, not a string
MBC_CONSOLEPARAM__COLS  := $(shell tput cols)
MBC_CONSOLEPARAM__LINES := $(shell tput lines)

zMBC_TPUT_RESET  := $(shell tput sgr0)
zMBC_TPUT_BOLD   := $(shell tput bold)
zMBC_TPUT_YELLOW := $(shell tput setaf 3)$(shell tput bold)
zMBC_TPUT_RED    := $(shell tput setaf 1)$(shell tput bold)
zMBC_TPUT_GREEN  := $(shell tput setaf 2)$(shell tput bold)

MBC_TERMINAL_SETTINGS := TERM=xterm-256color COLUMNS=$(MBC_CONSOLEPARAM__COLS) LINES=$(MBC_CONSOLEPARAM__LINES) 

# This tolerates extra spaces at end of lines when only a few params used
MBC_SHOW_NORMAL := @printf "$(MBC_ARG__CTXT): %s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"
MBC_SHOW_WHITE  := @printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_BOLD)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"
MBC_SHOW_YELLOW := @printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_YELLOW)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"
MBC_SHOW_RED    := @printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_RED)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"
MBC_SHOW_GREEN  := @printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_GREEN)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"

MBC_RAW_YELLOW := @printf "$(zMBC_TPUT_YELLOW)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"

MBC_START := $(MBC_SHOW_WHITE)
MBC_STEP  := $(MBC_SHOW_WHITE)
MBC_PASS  := $(MBC_SHOW_GREEN)
MBC_FAIL  := (printf $(zMBC_TPUT_RESET)$(zMBC_TPUT_RED)$(MBC_ARG__CTXT)' FAILED\n'$(zMBC_TPUT_RESET) && exit 1)

# For use in compound statements.
MBC_SEE_NORMAL := printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_RESET)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"
MBC_SEE_RED    := printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_RED)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"
MBC_SEE_YELLOW := printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_YELLOW)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"
MBC_SEE_GREEN  := printf "$(MBC_ARG__CTXT): $(zMBC_TPUT_GREEN)%s %s %s %s %s %s %s %s %s$(zMBC_TPUT_RESET)\n"


# EOF
