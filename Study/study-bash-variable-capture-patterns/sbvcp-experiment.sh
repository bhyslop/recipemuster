#!/bin/bash

# RESULTS SUMMARY:
# ================
# FAILS - no error detection or propagation:
# - local var=$(cmd) || handler                   # || applies to 'local', not cmd; $?=0 on failure
# - read var < <(cmd) || handler                  # read always succeeds; $?=0 on cmd failure  
# - read var < <(cmd || handler)                  # handler runs in subshell, no propagation
# - read var < <(cmd || exit 1)                   # exit only affects subshell, no propagation
#
# WORKS - detects failures and propagates error codes:
# - if var=$(cmd); then...else...fi               # conditional catches failure; $?=handler_exit_code
# - local var; var=$(cmd) || handler              # separate declaration works; $?=handler_exit_code
# - var=$(cmd); [[ $? -ne 0 ]] && handler         # explicit check works; $?=handler_exit_code
# - exec N< <(cmd); read var <&N; wait $!         # wait catches background exit; $?=handler_exit_code
#
# WORKS - manual status checking (no automatic propagation):
# - { read var; read status; } < <(cmd; echo $?)  # captures exit code for manual checking; $?=0
#
# KEY INSIGHT: Most process substitution patterns mask command failures completely.
# The popular one-liner local var=$(cmd) || handler is broken - use separate declaration instead.


good_cmd() {
    echo "success_output"
    return 0
}

bad_cmd() {
    echo "failure_output"
    return 11
}

error_caught() {
    echo "ERROR_CAUGHT"
    return 17
}

success() {
    echo "SUCCESS"
    return 23
}

run_tests() {
    echo "Testing with good_cmd (returns 0)"
    echo "================================="

    echo "Single line: local declaration with command substitution and OR operator"
    local var1=$(good_cmd) || error_caught
    echo "var1='$var1' \$?=$?"
    echo "---"

    echo "Conditional assignment: if statement with assignment in condition"
    local var2; if var2=$(good_cmd); then success; else error_caught; fi
    echo "var2='$var2' \$?=$?"
    echo "---"

    echo "Two line: separate local declaration, then assignment with OR operator"
    local var3; var3=$(good_cmd) || error_caught
    echo "var3='$var3' \$?=$?"
    echo "---"

    echo "Exit code check: assignment then test \$? variable"
    local var4; var4=$(good_cmd); [[ $? -ne 0 ]] && error_caught
    echo "var4='$var4' \$?=$?"
    echo "---"

    echo "Process substitution: read from process substitution with OR operator"
    local var5; read var5 < <(good_cmd) || error_caught
    echo "var5='$var5' \$?=$?"
    echo "---"

    echo "Process substitution with exit code: capture both output and status"
    local var6 status6; { read var6; read status6; } < <(good_cmd; echo $?)
    echo "var6='$var6' status6='$status6' \$?=$?"
    echo "---"

    echo
    echo "Testing with bad_cmd (returns 1)"
    echo "================================"

    echo "Single line: local declaration with command substitution and OR operator"
    local var7=$(bad_cmd) || error_caught
    echo "var7='$var7' \$?=$?"
    echo "---"

    echo "Conditional assignment: if statement with assignment in condition"
    local var8; if var8=$(bad_cmd); then success; else error_caught; fi
    echo "var8='$var8' \$?=$?"
    echo "---"

    echo "Two line: separate local declaration, then assignment with OR operator"
    local var9; var9=$(bad_cmd) || error_caught
    echo "var9='$var9' \$?=$?"
    echo "---"

    echo "Exit code check: assignment then test \$? variable"
    local var10; var10=$(bad_cmd); [[ $? -ne 0 ]] && error_caught
    echo "var10='$var10' \$?=$?"
    echo "---"

    echo "Process substitution: read from process substitution with OR operator"
    local var11; read var11 < <(bad_cmd) || error_caught
    echo "var11='$var11' \$?=$?"
    echo "---"

    echo "Process substitution with exit code: capture both output and status"
    local var12 status12; { read var12; read status12; } < <(bad_cmd; echo $?)
    echo "var12='$var12' status12='$status12' \$?=$?"
    echo "---"

    echo "Process substitution with internal error handler"
    local var15; read var15 < <(bad_cmd || error_caught)
    echo "var15='$var15' \$?=$?"
    echo "---"

    echo "Process substitution with internal exit on failure"
    local var16; read var16 < <(bad_cmd || exit 1)
    echo "var16='$var16' \$?=$?"
    echo "---"

    echo
    echo "File descriptor tests"
    echo "===================="

    echo "File descriptor with good_cmd: exec, read, wait background process"
    local var13; exec 3< <(good_cmd); read var13 <&3; wait $! || error_caught
    echo "var13='$var13' \$?=$?"
    echo "---"

    echo "File descriptor with bad_cmd: exec, read, wait background process"
    local var14; exec 4< <(bad_cmd); read var14 <&4; wait $! || error_caught
    echo "var14='$var14' \$?=$?"
    echo "---"
}

run_tests
