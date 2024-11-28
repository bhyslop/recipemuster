

MBC_CHECK__IS_CIDR = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  ($(MBC_SEE_RED) "Value '$(2)' must be in valid CIDR notation" && exit 1))





