## ©️ 2023 Scale Invariant, Inc.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> December 2023


# Set below variable to add a localization context to pretty lines
MBC_ARG__CONTEXT_STRING ?= mbu-c

# Parameters extracted from tabtarget text delimited by '.'
MBC_TTPARAM__FIRST  = $(word 1, $(subst ., ,$@))
MBC_TTPARAM__SECOND = $(word 2, $(subst ., ,$@))
MBC_TTPARAM__THIRD  = $(word 3, $(subst ., ,$@))
MBC_TTPARAM__FOURTH = $(word 4, $(subst ., ,$@))
MBC_TTPARAM__FIFTH  = $(word 5, $(subst ., ,$@))

# No quotes since value is integer, not a string
MBC_CONSOLEPARAM__COLS  := $(shell tput cols)
MBC_CONSOLEPARAM__LINES := $(shell tput lines)

zMBC_TPUT_RESET  := '$(shell tput sgr0)'
zMBC_TPUT_BOLD   := '$(shell tput bold)'
zMBC_TPUT_YELLOW := '$(shell tput setaf 3;tput bold)'
zMBC_TPUT_RED    := '$(shell tput setaf 1;tput bold)'
zMBC_TPUT_GREEN  := '$(shell tput setaf 2;tput bold)'

# This tolerates extra spaces at end of lines when only a few params used
MBC_SHOW_NORMAL := @printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_RESET)
MBC_SHOW_WHITE  := @printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_BOLD)
MBC_SHOW_YELLOW := @printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_YELLOW)
MBC_SHOW_RED    := @printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_RED)
MBC_SHOW_GREEN  := @printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_GREEN)

MBC_NOW := $(shell date +'%Y%m%d__%H%M%S%3N')

MBC_PASS := $(MBC_SHOW_GREEN)
MBC_FAIL := (printf $(zMBC_TPUT_RESET)$(zMBC_TPUT_RED)$(MBC_ARG__CONTEXT_STRING)' FAILED\n'$(zMBC_TPUT_RESET) && exit 1)

# Experiment to use in compound statements.
MBC_SEE_RED    := printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_RED)


#################################
# EXECUTION ENVIRONMENT CUSTOMIZATION
# 
zMBC_EEC_UNAME_S := $(shell uname -s)

zMBC_EEC_TYPE_MAC      := 0
zMBC_EEC_TYPE_LINUX    := 0
zMBC_EEC_TYPE_CYGWIN   := 0
zMBC_EEC_LABEL_TYPE    := __bad_label__
ifneq   (,$(findstring CYGWIN_NT-10.0, $(zMBC_EEC_UNAME_S)))
  zMBC_EEC_TYPE_CYGWIN := 1
  zMBC_EEC_LABEL_TYPE  := cygwin_x86
else ifeq   ($(zMBC_EEC_UNAME_S),Darwin)
  zMBC_EEC_TYPE_MAC    := 1
  zMBC_EEC_LABEL_TYPE  := mac_m1
else ifeq   ($(zMBC_EEC_UNAME_S),Linux)
  zMBC_EEC_TYPE_LINUX  := 1
  zMBC_EEC_LABEL_TYPE  := linux_x86
endif

# On everything but MAC, the below pattern involving expanding a variable to select
#   another variable that is then expanded works:
MBC_ROOT_ABSDIR               := __unknown_execution_operating_system__
MBC_ROOT_ABSDIR.cygwin_x86    := $(shell cygpath -ma .)
MBC_ROOT_ABSDIR.linux_x86     := $(shell pwd)
MBC_ROOT_ABSDIR.mac_m1        := $(shell pwd)
MBC_ROOT_ABSDIR.__bad_label__ := __error_station_file_must_specify_operating_system__
MBC_ROOT_ABSDIR               := $(MBC_ROOT_ABSDIR.$(zMBC_EEC_LABEL_TYPE))

# If MACs are needed, prefer the following.  Needs testing
# ITCH_EphemDockerScrub: switch to below because make complains under linux of no cygpath app
ifeq      ($(zMBC_EEC_TYPE_CYGWIN),1)
MBC_ALTROOT_ABSDIR  = $(MBC_ROOT_ABSDIR.cygwin_x86)
else ifeq ($(zMBC_EEC_TYPE_MAC),1)
MBC_ALTROOT_ABSDIR  = $(MBC_ROOT_ABSDIR.mac_m1)
else ifeq ($(zMBC_EEC_TYPE_LINUX),1)
MBC_ALTROOT_ABSDIR  = $(MBC_ROOT_ABSDIR.linux_x86)
else 
MBC_ALTROOT_ABSDIR := __unknown_execution_operating_system__
endif




# EOF
