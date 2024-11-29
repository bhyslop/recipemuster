## ©️ 2023 Scale Invariant, Inc.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> December 2023


# Set below variable to add a localization context to pretty lines
MBC_ARG__CONTEXT_STRING ?= mbu-c

# Timestamp good for unique temp filenames durable for a compound make process
MBC_NOW := $(shell date +'%Y%m%d__%H%M%S%3N')

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

MBC_START := $(MBC_SHOW_WHITE)
MBC_STEP  := $(MBC_SHOW_WHITE)
MBC_PASS  := $(MBC_SHOW_GREEN)
MBC_FAIL  := (printf $(zMBC_TPUT_RESET)$(zMBC_TPUT_RED)$(MBC_ARG__CONTEXT_STRING)' FAILED\n'$(zMBC_TPUT_RESET) && exit 1)

# For use in compound statements.
MBC_SEE_RED    := printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_RED)
MBC_SEE_YELLOW := printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_YELLOW)
MBC_SEE_GREEN  := printf '%s'$(MBC_ARG__CONTEXT_STRING)': %s %s %s %s %s %s %s %s %s\n'$(zMBC_TPUT_RESET) $(zMBC_TPUT_GREEN)


# Validation helpers
MBC_CHECK_EXPORTED = \
  test "$(1)" != "1" || (env | grep -q ^'$(2)'= || \
  ($(MBC_SEE_RED) "Variable '$(2)' must be exported" && exit 1))

MBC_CHECK__BOOLEAN = \
  test "$(1)" != "1" || (test '$(2)' = "0" -o '$(2)' = "1" || \
  ($(MBC_SEE_RED) "Value '$(2)' must be 0 or 1" && exit 1))

MBC_CHECK_IN_RANGE = \
  test "$(1)" != "1" || (test '$(2)' -ge '$(3)' -a '$(2)' -le '$(4)' || \
  ($(MBC_SEE_RED) "Value '$(2)' must be between '$(3)' and '$(4)'" && exit 1))

MBC_CHECK_NONEMPTY = \
  test "$(1)" != "1" || (test -n '$(2)' || \
  ($(MBC_SEE_RED) "Value '$(2)' must not be empty" && exit 1))

MBC_CHECK__MATCHES = \
  test "$(1)" != "1" || (echo $(2) | grep -E $(3) || \
  ($(MBC_SEE_RED) "Value '$(2)' does not match required pattern" && exit 1))

MBC_CHECK_STARTS_W = \
  test "$(1)" != "1" || (echo $(2) | grep -E ^$(3) || \
  ($(MBC_SEE_RED) "Value '$(2)' must start with required pattern" && exit 1))

MBC_CHECK_ENDSWITH = \
  test "$(1)" != "1" || (echo $(2) | grep -E $(3)$$ || \
  ($(MBC_SEE_RED) "Value '$(2)' must end with required pattern" && exit 1))

# This regex-based check has limitations:
#   - Accepts invalid octet values >255 
#   - Allows invalid prefix lengths beyond /32
#   - Cannot validate proper octet count/delimiting
#   - Cannot verify network address aligns with prefix length
MBC_CHECK__IS_CIDR = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  ($(MBC_SEE_RED) "Value '$(2)' must be in valid CIDR notation" && exit 1))

# This regex-based check has limitations:
#   - Allows dots at start/end despite RFC rules
#   - Cannot validate 63-char max label length
#   - Accepts consecutive dots
#   - Domain parts can contain numbers only
MBC_CHECK_ISDOMAIN = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$$' || \
  ($(MBC_SEE_RED) "Value '$(2)' must be a valid domain name" && exit 1))

# This regex-based check has limitations:
#   - Accepts invalid octet values >255
#   - Allows leading zeros in octets
#   - Cannot validate proper octet count if delimiters malformed
#   - Cannot verify IPv4 address class/scope validity
MBC_CHECK__IS_IPV4 = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$$' || \
  ($(MBC_SEE_RED) "Value '$(2)' must be a valid IPv4 address" && exit 1))


# EOF
