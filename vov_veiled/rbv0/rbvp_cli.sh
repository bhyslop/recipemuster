#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBVP CLI - the guide to the browser profile the windowed walks open on
# (veiled). Dispatched by the hierophant's own workbench (rbvw_workbench.sh)
# via its own zipper (rbvz_zipper.sh), never the shipped rbw workbench.
#
# The guide prints and does nothing more: it reads no browser's files and opens
# no window. The profile's name is printed from the one constant that spells it.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# Command Functions

# Prints the guide to the Chrome profile the windowed walks open on: making it or renaming it, its Claude extension, and its sign-ins in their order.
rbvp_catechesis() {
  buc_doc_brief "Prints the guide to the Chrome profile the windowed walks open on: making it or renaming it, its Claude extension, and its sign-ins in their order."
  buc_doc_shown || return 0
  test "$#" -eq 0 || buc_die_now "This door takes no argument; refused: $*"

  # Words used in more than one section are yawped once; the readonly
  # captures survive later buym_*_yawp calls overwriting z_buym_yelp.
  buym_cmd_yawp "${RBVP_profile}"; local -r z_cmd_name="${z_buym_yelp}"
  buym_cmd_yawp "tt/rbvw-qr.RenderQualificationRegime.sh"; local -r z_cmd_render="${z_buym_yelp}"
  buym_cmd_yawp "claude"; local -r z_cmd_claude="${z_buym_yelp}"
  buym_cmd_yawp "/status"; local -r z_cmd_status="${z_buym_yelp}"
  buym_cmd_yawp "chrome://extensions"; local -r z_cmd_extensions_page="${z_buym_yelp}"
  buym_ui_yawp "Google account"; local -r z_ui_google_line="${z_buym_yelp}"
  buym_ui_yawp "Microsoft Entra administrator"; local -r z_ui_entra_line="${z_buym_yelp}"
  buym_ui_yawp "Manage Chrome profiles"; local -r z_ui_manage_profiles="${z_buym_yelp}"
  buym_ui_yawp "Claude"; local -r z_ui_claude="${z_buym_yelp}"
  buym_ui_yawp "Extensions"; local -r z_ui_extensions_menu="${z_buym_yelp}"
  buym_href_yawp "https://console.cloud.google.com/" "https://console.cloud.google.com/"; local -r z_href_cloud="${z_buym_yelp}"

  buh_section  "Browser Profile Setup"
  buh_line     "Sets up, on this computer, the Chrome profile the windowed walks open on:"
  buh_line     "making it, or renaming one that already carries the Claude extension; the"
  buh_line     "extension and its link to Claude Code; and the sign-ins, in their order."
  buh_line     "Every step is done by your own hands, in Chrome and in Claude Code."
  buh_line     "This guide changes nothing by itself."
  buh_line     "The profile's name, spelled exactly so: ${z_cmd_name}"
  buh_e
  buh_section  "Key:"
  buym_ui_yawp "precise words you see on Chrome's screens and web pages."; local -r z_ui_key_words="${z_buym_yelp}"
  buh_line     "   Magenta text refers to ${z_ui_key_words}"
  buym_cmd_yawp "something you type or copy from here."; local -r z_cmd_key_copy="${z_buym_yelp}"
  buh_line     "   Cyan text is ${z_cmd_key_copy}"
  buym_href_yawp "https://example.com/" "EXAMPLE DOT COM"; local -r z_href_key_example="${z_buym_yelp}"
  buh_line     "   Clickable links look like ${z_href_key_example}; most terminals open one when you"
  buh_line     "   hold Ctrl (Cmd on a Mac) and click it. Otherwise copy the address into Chrome."
  buh_line     "   An address beginning chrome:// is never clickable: type it into the address bar"
  buh_line     "   of the window named, then press Enter."
  buh_line     "   A step marked UNPROVEN gives the vendor's own help words; no one has yet"
  buh_line     "   recorded carrying it out on a computer."
  buh_e

  buh_section  "1. What This Computer Needs First:"
  buh_line     "   1. Google Chrome, installed. Chrome's own menus stand at the top right of"
  buh_line     "      each of its windows: the round profile picture, and beside it the"
  buym_ui_yawp "Extensions"; local -r z_ui_puzzle="${z_buym_yelp}"
  buh_line     "      ${z_ui_puzzle} menu, drawn as a puzzle piece."
  buh_line     "   2. Claude Code, signed in by its own login. In a terminal, run ${z_cmd_claude}"
  buh_line     "      to open a Claude Code chat, then type ${z_cmd_status} there and press Enter."
  buym_ui_yawp "Login method"; local -r z_ui_login_method="${z_buym_yelp}"
  buh_line     "      Its ${z_ui_login_method} line must name a Claude account, not an API key."
  buh_line     "      That account, with the organization the same screen names, is the one the"
  buh_line     "      Claude extension signs in as below. Call it the Claude Code account."
  buh_line     "   3. The two other addresses this profile signs in as. In a terminal, from the"
  buh_line     "      folder you ran this guide from, run:"
  buh_line     "      ${z_cmd_render}"
  buh_line     "      Note two lines it prints: ${z_ui_google_line} and ${z_ui_entra_line}."
  buh_line     "      Each holds an address. If either stands blank, stop here: whoever plans"
  buh_line     "      the walks on this computer fills them first."
  buh_e

  buh_section  "2. Find Which Case You Are In:"
  buym_ui_yawp "Other Chrome profiles"; local -r z_ui_other_profiles="${z_buym_yelp}"
  buh_line     "   1. In any Chrome window, click the profile picture at the top right. Under"
  buh_line     "      ${z_ui_other_profiles}, click ${z_ui_manage_profiles}. A screen opens listing"
  buh_line     "      every profile on this computer by its name."
  buh_line     "   2. If one profile there is named ${z_cmd_name}, go to section 6."
  buh_line     "   3. Otherwise, open each listed profile in turn by clicking it, and in the"
  buh_line     "      window it opens type ${z_cmd_extensions_page} into the address bar. That page"
  buh_line     "      lists the extensions the profile carries."
  buh_line     "   4. If exactly one profile lists ${z_ui_claude} there, go to section 3 and rename"
  buh_line     "      that one. If more than one lists it, go to section 3 with the one you mean"
  buh_line     "      to keep; section 6 has the others turned off."
  buh_line     "   5. If no profile lists it, go to section 4."
  buh_e

  buh_section  "3. Rename the Profile Carrying the Extension:"
  buym_ui_yawp "Customize profile"; local -r z_ui_customize="${z_buym_yelp}"
  buh_line     "   1. In that profile's window, click the profile picture at the top right,"
  buh_line     "      then ${z_ui_customize}. Chrome's settings page for this profile opens."
  buh_line     "   2. Its name field holds the profile's present name. Replace that whole name"
  buh_line     "      with ${z_cmd_name}, exactly. Chrome saves the change by itself."
  buh_line     "   3. Click the profile picture again: the menu now shows the new name."
  buh_line     "   Then go to section 6. Sections 4, 5 and 7 stood done under the old name;"
  buh_line     "   section 6 checks each of them."
  buh_e

  buh_section  "4. Make the Profile:"
  buym_ui_yawp "Add Chrome profile"; local -r z_ui_add_profile="${z_buym_yelp}"
  buh_line     "   1. In any Chrome window, click the profile picture at the top right, then"
  buh_line     "      ${z_ui_add_profile}."
  buym_ui_yawp "Set up your new Chrome profile"; local -r z_ui_setup_screen="${z_buym_yelp}"
  buym_ui_yawp "Stay signed out"; local -r z_ui_stay_out="${z_buym_yelp}"
  buym_ui_yawp "Sign in"; local -r z_ui_sign_in_button="${z_buym_yelp}"
  buh_line     "   2. On the screen ${z_ui_setup_screen}, click ${z_ui_stay_out}."
  buh_line     "      Never click ${z_ui_sign_in_button} there: it turns on sync, and sync pours your"
  buh_line     "      other passwords, extensions and sign-ins into this profile."
  buym_ui_yawp "Add a name or label, like Work or Personal"; local -r z_ui_name_field="${z_buym_yelp}"
  buym_ui_yawp "Done"; local -r z_ui_done="${z_buym_yelp}"
  buh_line     "   3. On the next screen, type ${z_cmd_name} into the field"
  buh_line     "      ${z_ui_name_field}. Pick any colour, then click ${z_ui_done}."
  buh_line     "   A window of the new profile opens; it is the profile's window in every"
  buh_line     "   section below. Go to section 5."
  buh_e

  buh_section  "5. Add the Claude Extension and Link It to Claude Code:"
  buym_href_yawp "https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn" "https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn"; local -r z_href_store="${z_buym_yelp}"
  buh_line     "   1. In the profile's window, open:"
  buh_line     "      ${z_href_store}"
  buym_ui_yawp "claude.com"; local -r z_ui_publisher="${z_buym_yelp}"
  buh_line     "      The page shows ${z_ui_claude}, published by ${z_ui_publisher}. Leave the store's own"
  buh_line     "      ${z_ui_sign_in_button} button alone; adding the extension needs none."
  buym_ui_yawp "Add to Chrome"; local -r z_ui_add_to_chrome="${z_buym_yelp}"
  buym_ui_yawp "Add extension"; local -r z_ui_add_extension="${z_buym_yelp}"
  buh_line     "   2. Click ${z_ui_add_to_chrome}. A dialog lists what the extension may do; click"
  buh_line     "      ${z_ui_add_extension}. It may read every site this profile is signed in to,"
  buh_line     "      which is why the profile holds the walks' sign-ins and nothing else."
  buh_line     "   3. Open the ${z_ui_extensions_menu} menu (the puzzle piece at the top right) and click"
  buh_line     "      ${z_ui_claude}. Sign it in as the Claude Code account from section 1, in that"
  buh_line     "      same organization."
  buym_ui_yawp "Authorize"; local -r z_ui_authorize="${z_buym_yelp}"
  buym_ui_yawp "Claude for Chrome would like to connect to your Claude chat account"; local -r z_ui_consent="${z_buym_yelp}"
  buh_line     "      The sign-in ends on a page reading"
  buh_line     "      ${z_ui_consent};"
  buh_line     "      click ${z_ui_authorize} there."
  buh_line     "      UNPROVEN: this sign-in has been recorded only in a profile that also held a"
  buh_line     "      Google sign-in; made in a profile holding none, its pages before that"
  buh_line     "      consent page are unread. Claude Code's own help words what matters: the"
  buh_line     "      extension must be signed in to the same claude.ai account and"
  buh_line     "      organization as Claude Code."
  buym_cmd_yawp "/chrome"; local -r z_cmd_chrome="${z_buym_yelp}"
  buym_ui_yawp "Enabled by default"; local -r z_ui_enabled_default="${z_buym_yelp}"
  buh_line     "   4. In a Claude Code chat on this computer, type ${z_cmd_chrome} and press Enter."
  buh_line     "      If its menu reads ${z_ui_enabled_default} as No, choose that entry to turn it on."
  buh_line     "      If it already reads Yes, change nothing."
  buh_line     "      Only chats opened after this step reach the browser: close the chat with"
  buh_line     "      the Claude Code command /exit, and open a fresh one with ${z_cmd_claude}."
  buym_ui_yawp "Status: Enabled"; local -r z_ui_status_enabled="${z_buym_yelp}"
  buym_ui_yawp "Extension: Installed"; local -r z_ui_ext_installed="${z_buym_yelp}"
  buh_line     "   5. In that fresh chat, type ${z_cmd_chrome} again. It reads ${z_ui_status_enabled} and"
  buh_line     "      ${z_ui_ext_installed}."
  buh_line     "   Then go to section 6."
  buh_e

  buh_section  "6. Check the Profile and Its Extension:"
  buh_line     "   Each check is by your own eyes, on the Chrome screen named."
  buh_line     "   1. Click the profile picture at the top right of any Chrome window, then"
  buh_line     "      ${z_ui_manage_profiles}. Exactly one profile is named ${z_cmd_name}."
  buh_line     "   2. In that profile's window, type ${z_cmd_extensions_page} into the address bar."
  buh_line     "      The page lists ${z_ui_claude}, with its switch on. If it is missing, do section 5."
  buh_line     "   3. In every other profile on this computer, type ${z_cmd_extensions_page} into the"
  buh_line     "      address bar of its window. Where ${z_ui_claude} is listed, turn its switch off,"
  buh_line     "      so only one browser connects to Claude Code."
  buym_ui_yawp "Sign in to Chrome"; local -r z_ui_sign_in_chrome="${z_buym_yelp}"
  buh_line     "   4. Back in the profile's window, click the profile picture at the top right."
  buh_line     "      The menu offers ${z_ui_sign_in_chrome}: Chrome itself is signed out on this"
  buh_line     "      profile, as it must stay. If the menu instead names a Google account as"
  buh_line     "      signed in to Chrome or syncing, stop here and tell whoever plans the walks"
  buh_line     "      on this computer: a profile Chrome itself is signed in on cannot serve."
  buh_line     "      UNPROVEN: the words of this menu are Google's help's; no one has recorded"
  buh_line     "      reading them on a computer."
  buh_line     "   If you made the profile in section 4, go to section 7."
  buh_line     "   If you came here from section 2 or 3, take step 3 of section 7 alone, then"
  buh_line     "   go to section 8; where a check there fails, it names the step that mends it."
  buh_e

  buh_section  "7. The Sign-Ins, in Their Order:"
  buh_line     "   The order matters: the first Google account signed in is the one a plain"
  buh_line     "   Google Cloud address opens as."
  buh_line     "   1. The Claude extension, as section 5 signs it in. It comes first."
  buh_line     "   2. One Google account: the address on the ${z_ui_google_line} line from section 1."
  buh_line     "      In the profile's window, open ${z_href_cloud}"
  buh_line     "      and sign in as that address."
  buym_ui_yawp "Verify it's you"; local -r z_ui_verify="${z_buym_yelp}"
  buh_line     "      If Google shows ${z_ui_verify} and asks for the password, type it; that"
  buh_line     "      check returns on Google's own timer, and is always met the same way."
  buh_line     "      If Chrome asks whether to sign in to Chrome as well, or to turn on sync,"
  buh_line     "      decline. UNPROVEN: the words of Chrome's own prompts at a web sign-in are"
  buh_line     "      unread; Google's help says only that you may be asked if you also want to"
  buh_line     "      sign in to Chrome, and that it is optional."
  buym_ui_yawp "Google services settings"; local -r z_ui_services="${z_buym_yelp}"
  buym_ui_yawp "Other Google services"; local -r z_ui_other_services="${z_buym_yelp}"
  buym_ui_yawp "Don't sign in"; local -r z_ui_dont_sign_in="${z_buym_yelp}"
  buh_line     "   3. Chrome's choice for web sign-ins, once that Google account is signed in."
  buh_line     "      Click the profile picture at the top right, then ${z_ui_services}."
  buh_line     "      Under ${z_ui_other_services}, Chrome offers three choices; choose"
  buh_line     "      ${z_ui_dont_sign_in}. Chrome keeps this choice for each Google account, so it"
  buh_line     "      is made after the account is signed in."
  buym_href_yawp "https://entra.microsoft.com/" "https://entra.microsoft.com/"; local -r z_href_entra="${z_buym_yelp}"
  buh_line     "   4. The Microsoft sign-in: the address on the ${z_ui_entra_line} line from"
  buh_line     "      section 1. In the profile's window, open ${z_href_entra}"
  buh_line     "      and sign in as that address, by whatever means that account uses: a"
  buh_line     "      passkey, a password, or a code sent to it. If the sign-in hands you to a"
  buh_line     "      Google sign-in page, stop and sign in nowhere there: it would put a second"
  buh_line     "      Google account into this profile."
  buh_line     "   Never add another Google account to this profile. Your other accounts each"
  buh_line     "   stay in a Chrome profile of their own."
  buh_line     "   Then go to section 8."
  buh_e

  buh_section  "8. Check the Sign-Ins:"
  buym_ui_yawp "Account:"; local -r z_ui_account_tooltip="${z_buym_yelp}"
  buh_line     "   1. In the profile's window, open ${z_href_cloud}"
  buh_line     "      with nothing after the address. Rest the pointer on the round account"
  buh_line     "      picture at the top right of the page. The text that appears opens"
  buh_line     "      ${z_ui_account_tooltip} and holds the address on the ${z_ui_google_line} line."
  buh_line     "      That page opens as the first Google account signed in, so this check"
  buh_line     "      also says the order is right. If no account is signed in there, take"
  buh_line     "      step 2 of section 7."
  buh_line     "   2. Click that account picture. The menu that opens lists any other Google"
  buh_line     "      account this profile is signed in to; it must list none."
  buh_line     "      UNPROVEN: no one has recorded this menu's listing of other accounts."
  buh_line     "   Where check 1 shows another address, or check 2 lists another account,"
  buh_line     "   change which Google account is first, or drop one: in that same menu,"
  buym_ui_yawp "Sign out"; local -r z_ui_sign_out="${z_buym_yelp}"
  buym_ui_yawp "Remove an account"; local -r z_ui_remove_account="${z_buym_yelp}"
  buh_line     "   click ${z_ui_sign_out}, which signs out every Google account of the profile and"
  buh_line     "   opens Google's list of accounts. On that list, click ${z_ui_remove_account}, then"
  buh_line     "   the round minus sign at the right of each unwanted account's row. Then sign"
  buh_line     "   in the wanted account by step 2 of section 7."
  buh_line     "   3. In the profile's window, open ${z_href_entra}"
  buh_line     "      Click the account button at the top right of the page. The panel that"
  buh_line     "      opens names the account signed in: it is the address on the"
  buh_line     "      ${z_ui_entra_line} line. If no one is signed in, or someone else is,"
  buh_line     "      take step 4 of section 7."
  buh_line     "      UNPROVEN as your eyes see it: what was recorded is the page's own text"
  buh_line     "      naming that address, not how the panel looks."
  buh_e

  buh_section  "9. When a Walk's Session Lists No Browser:"
  buh_line     "   Take this step only when a walk's session says it lists no browser and asks"
  buh_line     "   you for it. Never take it as a step before a walk: a freshly opened window"
  buh_line     "   needs no such check."
  buh_line     "   1. In the profile's window, open the ${z_ui_extensions_menu} menu (the puzzle piece at the"
  buh_line     "      top right) and click ${z_ui_claude}. See that it is signed in as the Claude Code"
  buh_line     "      account: the account ${z_cmd_status} names in a Claude Code chat on this computer."
  buh_e
}

######################################################################
# Furnish and Main

zrbvp_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local -r z_dir="${BASH_SOURCE[0]%/*}"

  source "${BURD_BUK_DIR}/buh_handbook.sh" || buc_die_now "Failed to source buh_handbook.sh"
  source "${z_dir}/rbvp_mystagogue.sh" || buc_die_now "Failed to source the profile's name"

  zburd_enforce
}

buc_execute rbvp_ "Browser profile guide (rbvp)" zrbvp_furnish "$@"

# eof
