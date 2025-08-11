#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Recipe Bottle Kludge - Routes commands to appropriate CLI scripts

set -euo pipefail

# Get script directory
RBK_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Verbose output if BDU_VERBOSE is set
rbk_show() {
    test "${BDU_VERBOSE:-0}" != "1" || echo "RBKSHOW: $*"
}

# Simple routing function
rbk_route() {
    local z_command="$1"
    shift
    local z_args="$*"
    
    rbk_show "Routing command: $z_command with args: $z_args"
    
    # Verify BDU environment variables are present
    if [ -z "${BDU_TEMP_DIR:-}" ]; then
        echo "ERROR: BDU_TEMP_DIR not set - must be called from BDU" >&2
        exit 1
    fi
    
    if [ -z "${BDU_NOW_STAMP:-}" ]; then
        echo "ERROR: BDU_NOW_STAMP not set - must be called from BDU" >&2
        exit 1
    fi
    
    rbk_show "BDU environment verified: TEMP_DIR=$BDU_TEMP_DIR, NOW_STAMP=$BDU_NOW_STAMP"
    
    # Route based on command prefix
    case "$z_command" in
        # Image management commands (rbim)
        rbw-l)
            rbk_show "Routing to rbim_cli.sh rbim_list"
            exec "$RBK_SCRIPT_DIR/rbim_cli.sh" rbim_list $z_args
            ;;
        rbw-II)
            rbk_show "Routing to rbim_cli.sh rbim_image_info"
            exec "$RBK_SCRIPT_DIR/rbim_cli.sh" rbim_image_info $z_args
            ;;
        rbw-r)
            rbk_show "Routing to rbim_cli.sh rbim_retrieve"
            exec "$RBK_SCRIPT_DIR/rbim_cli.sh" rbim_retrieve $z_args
            ;;
        rbw-b)
            rbk_show "Routing to rbim_cli.sh rbim_build"
            exec "$RBK_SCRIPT_DIR/rbim_cli.sh" rbim_build $z_args
            ;;
        rbw-d)
            rbk_show "Routing to rbim_cli.sh rbim_delete"
            exec "$RBK_SCRIPT_DIR/rbim_cli.sh" rbim_delete $z_args
            ;;
            
        # Google admin commands (rbga)
        rbw-ag)
            rbk_show "Routing to rbga_cli.sh rbga_show_setup"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_show_setup $z_args
            ;;
        rbw-ac)
            rbk_show "Routing to rbga_cli.sh rbga_convert_admin_json"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_convert_admin_json $z_args
            ;;
        rbw-al)
            rbk_show "Routing to rbga_cli.sh rbga_list_service_accounts"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_list_service_accounts $z_args
            ;;
        rbw-ar)
            rbk_show "Routing to rbga_cli.sh rbga_create_gar_reader"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_create_gar_reader $z_args
            ;;
        rbw-as)
            rbk_show "Routing to rbga_cli.sh rbga_create_gcb_submitter"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_create_gcb_submitter $z_args
            ;;
        rbw-ad)
            rbk_show "Routing to rbga_cli.sh rbga_delete_service_account"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_delete_service_account $z_args
            ;;
        rbw-ax)
            rbk_show "Routing to rbga_cli.sh rbga_cleanup_subordinates"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" rbga_cleanup_subordinates $z_args
            ;;
            
        # Help/documentation commands
        rbw-hi)
            rbk_show "Routing to rbim_cli.sh (help)"
            exec "$RBK_SCRIPT_DIR/rbim_cli.sh" $z_args
            ;;
        rbw-ha)
            rbk_show "Routing to rbga_cli.sh (help)"
            exec "$RBK_SCRIPT_DIR/rbga_cli.sh" $z_args
            ;;
            
        # Unknown command
        *)
            echo "ERROR: Unknown command: $z_command" >&2
            echo "Available command prefixes:" >&2
            echo "  Image Management (rbim):" >&2
            echo "    rbw-l   - List images" >&2
            echo "    rbw-II  - Image info" >&2
            echo "    rbw-r   - Retrieve image" >&2
            echo "    rbw-b   - Build image" >&2
            echo "    rbw-d   - Delete image" >&2
            echo "    rbw-hi  - Image management help" >&2
            echo "" >&2
            echo "  Google Admin (rbga):" >&2
            echo "    rbw-ag  - Show setup procedure" >&2
            echo "    rbw-ac  - Convert admin JSON" >&2
            echo "    rbw-al  - List service accounts" >&2
            echo "    rbw-ar  - Create GAR reader" >&2
            echo "    rbw-as  - Create GCB submitter" >&2
            echo "    rbw-ad  - Delete service account" >&2
            echo "    rbw-ax  - Cleanup subordinates" >&2
            echo "    rbw-ha  - Admin help" >&2
            exit 1
            ;;
    esac
}

# Main execution
main() {
    local z_command="${1:-}"
    shift || true
    
    if [ -z "$z_command" ]; then
        echo "ERROR: No command specified" >&2
        exit 1
    fi
    
    rbk_route "$z_command" "$@"
}

# Execute main
main "$@"

