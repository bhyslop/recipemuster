# Copyright 2024 Scale Invariant, Inc.
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

SHELL := /bin/bash -eo pipefail

# Get the master configuration
include mbv.variables.sh

# Submake config: What console tool will put in prefix of each line
MBC_ARG__CTXT = $(MBV_CONSOLE_MAKEFILE)

# Submake config: Select bottle service from a token in the rule (parsed by dispatch)
RBM_MONIKER = $(MBD_PARAMETER_2)

# Submake config: Where to find derived nameplate and test files
RBM_NAMEPLATE_FILE = $(RBRR_NAMEPLATE_PATH)/nameplate.$(RBM_MONIKER).mk
RBM_TEST_FILE      = RBM-tests/rbt.test.$(RBM_MONIKER).mk

# OUCH do better here: is ../station-files well known?
include ../station-files/RBRS.STATION.mk
include rbrr.repo.mk
-include $(RBM_NAMEPLATE_FILE)
-include $(RBM_TEST_FILE)
include $(RBV_GITHUB_PAT_ENV)
include $(MBV_TOOLS_DIR)/mbc.console.mk
include $(MBV_TOOLS_DIR)/rbg.github.mk
include $(MBV_TOOLS_DIR)/rbrr.mapper.mk
include $(MBV_TOOLS_DIR)/rbrn.nameplate.mk
include $(MBV_TOOLS_DIR)/rbp.podman.mk


RBW_RECIPES_DIR  = RBM-recipes

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(MBV_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)


#######################################
#  Podman automation
#
# These rules are designed to allow the pattern match to parameterize
# the operation via $(RBM_MONIKER).

rbw-a.%: zrbw_prestart_rule rbp_podman_machine_start_rule rbg_container_registry_login_rule rbp_check_connection
	$(MBC_PASS) "Podman started and logged into container registry."

rbw-z.%: zrbw_prestop_rule rbp_podman_machine_stop_rule
	$(MBC_PASS) "Podman stopped."

rbw-Z.%: zrbw_prenuke_rule rbp_podman_machine_nuke_rule
	$(MBC_PASS) "Nuke completed."

rbw-vc.%: rbp_stash_check_rule
	$(MBC_PASS) "VM image check complete."

rbw-vu.%: rbp_stash_update_rule
	$(MBC_PASS) "VM Image update complete."

rbw-S.%: rbp_connect_sentry_rule
	$(MBC_PASS) "No errors."

rbw-B.%: rbp_connect_bottle_rule
	$(MBC_PASS) "No errors."

rbw-o.%: rbp_observe_networks_rule
	$(MBC_PASS) "No errors."

rbw-s.%: rbp_check_connection rbp_start_service_rule
	$(MBC_STEP) "Completed delegate."

rbw-v.%: zrbp_validate_regimes_rule
	$(MBC_PASS) "No errors."

zrbw_prestart_rule:
	$(MBC_START) "Starting podman and logging in to container registry..."

zrbw_prestop_rule:
	$(MBC_START) "Stopping podman..."

zrbw_prenuke_rule:
	$(MBC_START) "Nuking podman..."


#######################################
#  Test Targets
#

RBT_TESTS_DIR            = RBM-tests
MBT_PODMAN_BASE          = podman --connection $(RBM_MACHINE)
MBT_PODMAN_EXEC_SENTRY   = $(MBT_PODMAN_BASE)                         exec    $(RBM_SENTRY_CONTAINER)
MBT_PODMAN_EXEC_BOTTLE   = $(MBT_PODMAN_BASE) machine ssh sudo podman exec    $(RBM_BOTTLE_CONTAINER)
MBT_PODMAN_EXEC_BOTTLE_I = $(MBT_PODMAN_BASE) machine ssh sudo podman exec -i $(RBM_BOTTLE_CONTAINER)

# Each test defines same rule
rbw-to.%:  rbt_test_bottle_service_rule
	$(MBC_PASS) "No errors."

zRBC_RESTART_SERVICE_CMD  = $(MAKE) -f $(MBV_CONSOLE_MAKEFILE) rbp_start_service_rule
zRBC_RUN_SERVICE_TEST_CMD = $(MAKE) -f $(MBV_CONSOLE_MAKEFILE) rbt_test_bottle_service_rule RBM_TEMP_DIR=$(MBD_TEMP_DIR) -j $(MBD_JOB_PROFILE)

rbw-tb.%:
	$(MBC_START) "For each well known nameplate, and threads:$(MBD_JOB_PROFILE)"
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=nsproto
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=nsproto
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=srjcl
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=srjcl
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=pluml
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=pluml
	$(MBC_PASS) "No errors."

zRBC_TEST_RECIPE = test_busybox.recipe

zRBC_FQIN_FILE     = $(MBD_TEMP_DIR)/fqin.txt
zRBC_FQIN_CONTENTS = $$(cat $(zRBC_FQIN_FILE))

rbw-tg.%:
	$(MBC_START) "Test github action build, retrieval, use"
	$(MBC_STEP) "Validate list before..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(MBC_STEP) "Validate build..."
	tt/rbg-b.BuildWithRecipe.sh $(RBW_RECIPES_DIR)/$(zRBC_TEST_RECIPE) $(zRBC_FQIN_FILE)
	$(MBC_STEP) "Validate list during..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(MBC_STEP) "Validate retrieval..."
	tt/rbg-r.RetrieveImage.sh $(zRBC_FQIN_CONTENTS)
	$(MBC_STEP) "Validate deletion..."
	tt/rbg-d.DeleteImageFromRegistry.sh $(zRBC_FQIN_CONTENTS) RBG_ARG_SKIP_DELETE_CONFIRMATION=SKIP
	$(MBC_STEP) "Validate list after..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(MBC_PASS) "No errors."

rbw-tf.%:
	$(MBC_START) "Fast test..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=pluml
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=pluml
	$(MBC_PASS) "No errors."

rbw-ta.%:
	$(MBC_START) "RUN REPOWIDE TESTS"
	$(MBC_STEP) "Github tests..."
	tt/rbw-tg.TestGithubWorkflow.sh
	$(MBC_STEP) "Bottle service tests..."
	tt/rbw-tb.TestBottles.parallel.sh
	$(MBC_PASS) "TEST ALL PASSED WITHOUT ERRORS."


#######################################
#  TabTarget Maintenance TabTargets
#
#  Helps you create default form tabtargets in right place.

# Parameter from the tabtarget: what is the full name of the new tabtarget, no directory 
RBC_TABTARGET_NAME   = 

zRBC_TABTARGET_FILE  = $(MBV_TABTARGET_DIR)/$(RBC_TABTARGET_NAME)
zRBC_DISPATCH_SCRIPT = $(MBV_TOOLS_DIR)/mbd.dispatch.sh
zRBC_TABTARGET_CMD   = 'cd "$$(dirname "$$0")/.." &&  $(zRBC_DISPATCH_SCRIPT) jp_single om_line "$$(basename "$$0")"'

ttc.CreateTabtarget.sh:
	@test -n "$(RBC_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh'           >  $(zRBC_TABTARGET_FILE)
	@echo $(zRBC_TABTARGET_CMD) >> $(zRBC_TABTARGET_FILE)
	@chmod +x                      $(zRBC_TABTARGET_FILE)
	git add                        $(zRBC_TABTARGET_FILE)
	git update-index --chmod=+x    $(zRBC_TABTARGET_FILE)
	$(MBC_PASS) "No errors."

ttx.FixTabtargetExecutability.sh:
	$(MBC_START) "Repair windows proclivity to goof up executable privileges"
	git update-index --chmod=+x $(MBV_TABTARGET_DIR)/*
	$(MBC_PASS) "No errors."


#######################################
#  Bash File TabTargets
#

bff-s.%:
	$(MBC_START) "Fetch: Placing file batch on clipboard..."
	$(MBV_TOOLS_DIR)/bff.fetch.sh $(MBD_CLI_ARGS) | clip
	$(MBC_PASS) "No errors."


#######################################
#  Visual SlickEdit TabTargets
#


vsp-g.%:
	$(MBC_START) "Regenerating slickedit project..."
	$(MBV_TOOLS_DIR)/vswb.generate-rbm.sh
	$(MBC_PASS) "No errors."


#######################################
#  Cerebro Setup TabTargets
#


csu-hi.%:
	$(MBC_START) "Help for setting up Cerebro:"
	@echo
	$(MBC_STEP)        "1. Acquire Ubuntu Server live ubuntu-24.04.2-live-server-amd64.iso"
	$(MBC_STEP)        "2. Burn it using rufus-4.7_x86.exe to a newer thumb drive."
	$(MBC_STEP)        "3. Download new MSI bios, current version is 2025/02/13"
	$(MBC_STEP)        "4. Place new MSI bios at root of thumb drive renamed ??? MSI.ROM"
	$(MBC_STEP)        "5. Boot into Cerebro bios using Del Key."
	$(MBC_RAW_YELLOW)  "                                     Del"
	$(MBC_STEP)        "    a. Find icon for 'flash mode'."
	$(MBC_STEP)        "    b. ??? do upgrade."
	$(MBC_STEP)        "6. Reboot into Cerebro bios."
	$(MBC_STEP)        "    a. Assure Secure boot enabled."
	$(MBC_STEP)        "    b. Select Boot from usb key or disk"
	$(MBC_STEP)        "    c. ??? Select second boot option to pick UEFI boot order."
	$(MBC_STEP)        "7. Boot into the Ubuntu Server Live image."
	$(MBC_STEP)        "    a. Select the ??? try ubuntu option."
	$(MBC_STEP)        "    b. ??? At 'Wilcommen! Bienvenue!' switch into console:"
	$(MBC_RAW_YELLOW)  "                                     Ctrl-Alt-F2"
	$(MBC_STEP)        "    c. Assure live internet access. You may need to use a USB-C Ethernet adapter:"
	@echo
	$(MBC_RAW_CYAN)    "                                     ip addr"
	@echo
	$(MBC_STEP)        "    d. Start ssh server:"
	@echo
	$(MBC_RAW_CYAN)    "                                     sudo systemctl start ssh"
	@echo
	$(MBC_STEP)        "    d. Set an ssh password:"
	@echo
	$(MBC_RAW_CYAN)    "                                     sudo passwd ubuntu-server"
	@echo
	$(MBC_STEP)        "8. On a second computer, set up env variable with Cerebro's temporary IP:"
	@echo
	$(MBC_RAW_ORANGE)  "                        export CEREBRO_IP_ADDR=xxxx"
	@echo
	$(MBC_STEP)        "9. Set up passwordless SSH access from your workstation:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh-copy-id ubuntu-server@\$$CEREBRO_IP_ADDR"
	@echo
	$(MBC_STEP)        "10. Verify connection without password:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'uname -a'"
	@echo
	$(MBC_STEP)        "11. Disable sudo password for the ubuntu-server user:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR \"echo 'ubuntu-server ALL=(ALL) NOPASSWD:ALL' | sudo tee /etc/sudoers.d/ubuntu-nopasswd && sudo chmod 440 /etc/sudoers.d/ubuntu-nopasswd\""
	@echo
	$(MBC_STEP)        "12. Verify sudo works without password by checking disk information:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo fdisk -l && echo \"Sudo works without password!\"'"
	@echo
	$(MBC_STEP)        "13. Install NVMe utilities:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo apt update && sudo apt install -y nvme-cli'"
	@echo
	$(MBC_STEP)        "14. Identify drives and their partitions:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo lsblk -o NAME,MODEL,SIZE,SERIAL,MOUNTPOINTS && echo -e \"\\nNVMe Details:\" && sudo nvme list'"
	@echo
	$(MBC_STEP)        "15. Get detailed partition information:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'echo -e \"\\nPartition Details:\" && sudo fdisk -l | grep -E \"^Disk /dev/nvme|^/dev/nvme\"'"
	@echo
	$(MBC_STEP)        "16. Verify partition status for each NVMe drive:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'test \$$(lsblk -no NAME /dev/nvme0n1 | grep -c \"nvme0n1p\") -eq 0 && echo \"PASS: nvme0n1 has no partitions as expected\" || (echo \"FAIL: nvme0n1 has partitions, should be empty\" && exit 1)'"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'test \$$(lsblk -no NAME /dev/nvme1n1 | grep -c \"nvme1n1p\") -eq 0 && echo \"PASS: nvme1n1 has no partitions as expected\" || (echo \"FAIL: nvme1n1 has partitions, should be empty\" && exit 1)'"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'test \$$(lsblk -no NAME /dev/nvme2n1 | grep -c \"nvme2n1p\") -gt 0 && echo \"PASS: nvme2n1 has Windows partitions as expected\" || (echo \"FAIL: nvme2n1 has no partitions, should have Windows\" && exit 1)'"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'test \$$(lsblk -no NAME /dev/nvme3n1 | grep -c \"nvme3n1p\") -eq 0 && echo \"PASS: nvme3n1 has no partitions as expected\" || (echo \"FAIL: nvme3n1 has partitions, should be empty\" && exit 1)'"
	@echo
	$(MBC_STEP)        "17. Create partitions on the three empty drives for RAID:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo parted /dev/nvme0n1 mklabel gpt && sudo parted -a optimal /dev/nvme0n1 mkpart primary 0% 100% && echo \"Partitioned nvme0n1 successfully\"'"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo parted /dev/nvme1n1 mklabel gpt && sudo parted -a optimal /dev/nvme1n1 mkpart primary 0% 100% && echo \"Partitioned nvme1n1 successfully\"'"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo parted /dev/nvme3n1 mklabel gpt && sudo parted -a optimal /dev/nvme3n1 mkpart primary 0% 100% && echo \"Partitioned nvme3n1 successfully\"'"
	@echo
	$(MBC_STEP)        "18. Create the RAID0 array with the three drives:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo mdadm --create /dev/md0 --level=0 --raid-devices=3 /dev/nvme0n1p1 /dev/nvme1n1p1 /dev/nvme3n1p1 && echo \"RAID0 array created successfully\"'"
	@echo
	$(MBC_STEP)        "19. Verify the RAID array was created properly:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo mdadm --detail /dev/md0 && echo \"RAID0 verification complete\"'"
	@echo
	$(MBC_STEP)        "20. Create filesystem on the RAID array:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo mkfs.ext4 -F /dev/md0 && echo \"Filesystem created on RAID0 array\"'"
	@echo
	$(MBC_STEP)        "21. Mount the RAID array to verify access:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo mkdir -p /mnt/raid && sudo mount /dev/md0 /mnt/raid && df -h /mnt/raid && echo \"RAID0 mounted successfully\"'"
	@echo
	$(MBC_STEP)        "22. Save RAID configuration for persistence after reboot:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh ubuntu-server@\$$CEREBRO_IP_ADDR 'sudo mdadm --detail --scan | sudo tee /etc/mdadm/mdadm.conf && echo \"RAID configuration saved\"'"
	@echo
	$(MBC_STEP)        "23. Switch to installer interface to complete Ubuntu installation:"
	$(MBC_RAW_YELLOW)  "                                     Ctrl-Alt-F1"
	$(MBC_RAW_YELLOW)  "                             xxx SKIPPED STEPS"
	$(MBC_STEP)        "24. In the Ubuntu installer:"
	$(MBC_STEP)        "    a. Select 'Manual' partitioning"
	$(MBC_STEP)        "    b. Find your Windows drive (nvme2n1) and select its EFI partition (nvme2n1p1)"
	$(MBC_STEP)        "    c. Set it to mount at '/boot/efi' and ensure it's formatted as vfat"
	$(MBC_STEP)        "    d. Select the RAID device (/dev/md0) and choose 'Edit'"
	$(MBC_STEP)        "    e. Select 'Format' and choose 'ext4' as the filesystem type"
	$(MBC_STEP)        "    f. After formatting options, select the mount point as '/'"
	$(MBC_STEP)        "    g. If root (/) option is greyed out, complete the format step first"
	$(MBC_STEP)        "    h. Verify configuration summary shows:"
	$(MBC_STEP)        "       - EFI partition mounted at /boot/efi"
	$(MBC_STEP)        "       - RAID array (md0) formatted as ext4 and mounted at /"
	$(MBC_STEP)        "    i. Select 'Done' to proceed with installation"
	$(MBC_STEP)        "25. Complete the installation by following the remaining Ubuntu setup prompts"
	$(MBC_STEP)        "26. Configure boot priority in MSI BIOS:"
	$(MBC_STEP)        "    a. Reboot and enter BIOS by pressing Del key during startup"
	$(MBC_STEP)        "    b. Navigate to the 'Boot' section (shown in the Click BIOS interface)"
	$(MBC_STEP)        "    c. Select 'Boot Priority' or 'Boot Option #1'"
	$(MBC_STEP)        "    d. Set 'Ubuntu (WD_BLACK_SN850)' as the first boot option"
	$(MBC_STEP)        "    e. Set 'Windows Boot Manager (WD_BLAC...)' as the second boot option"
	$(MBC_STEP)        "    f. Save changes and exit (usually F10 key)"
	$(MBC_STEP)        "    g. System will now boot to Ubuntu by default, with option to select Windows"
	$(MBC_PASS) "Successfully completed Cerebro setup with dual-boot configuration."
	$(MBC_PASS) "No errors."


csu-hg.%:
	$(MBC_START) "Help for setting up NVIDIA RTX 5090 drivers on Ubuntu:"
	@echo
	$(MBC_STEP)        "1. Boot into Ubuntu installed on the RAID array"
	$(MBC_STEP)        "2. On a second computer, set up environment variable with Cerebro's IP:"
	@echo
	$(MBC_RAW_ORANGE)  "                        export CEREBRO_IP_ADDR=xxxx"
	@echo
	$(MBC_STEP)        "2. Set up passwordless SSH access from your workstation:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh-copy-id bhyslop@\$$CEREBRO_IP_ADDR"
	@echo
	$(MBC_STEP)        "3. Disable sudo password for the bhyslop user:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh -t bhyslop@\$$CEREBRO_IP_ADDR 'echo \"bhyslop ALL=(ALL) NOPASSWD:ALL\" | sudo tee /etc/sudoers.d/bhyslop-nopasswd && sudo chmod 440 /etc/sudoers.d/bhyslop-nopasswd'"
	@echo
	$(MBC_STEP)        "3. Check if the GPU is detected remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo lspci | grep -i nvidia'"
	@echo
	$(MBC_STEP)        "4. Disable Secure Boot temporarily in BIOS (for driver installation):"
	$(MBC_RAW_YELLOW)  "                                     Del"
	$(MBC_STEP)        "5. Install required packages remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo apt update && sudo apt install -y dkms build-essential'"
	@echo
	$(MBC_STEP)        "6. Add NVIDIA repository remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo add-apt-repository ppa:graphics-drivers/ppa && sudo apt update'"
	@echo
	$(MBC_STEP)        "7. Install NVIDIA drivers remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo apt install -y nvidia-driver-570'"
	@echo
	$(MBC_STEP)        "8. Generate signing key for Secure Boot remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo mokutil --generate-self-signed-cert'"
	@echo
	$(MBC_STEP)        "9. Sign the NVIDIA modules remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo kmodsign sha512 /var/lib/shim-signed/mok/MOK.priv /var/lib/shim-signed/mok/MOK.der $(modinfo -n nvidia)'"
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo kmodsign sha512 /var/lib/shim-signed/mok/MOK.priv /var/lib/shim-signed/mok/MOK.der $(modinfo -n nvidia_uvm)'"
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo kmodsign sha512 /var/lib/shim-signed/mok/MOK.priv /var/lib/shim-signed/mok/MOK.der $(modinfo -n nvidia_drm)'"
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo kmodsign sha512 /var/lib/shim-signed/mok/MOK.priv /var/lib/shim-signed/mok/MOK.der $(modinfo -n nvidia_modeset)'"
	@echo
	$(MBC_STEP)        "10. Import the key to the MOK list remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo mokutil --import /var/lib/shim-signed/mok/MOK.der'"
	@echo
	$(MBC_STEP)        "11. Reboot the system remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo reboot'"
	@echo
	$(MBC_STEP)        "12. During boot, enroll the MOK key (must be done locally):"
	$(MBC_STEP)        "    a. Select 'Enroll MOK' from the MOK management screen"
	$(MBC_STEP)        "    b. Select 'Continue'"
	$(MBC_STEP)        "    c. Select 'Yes' to enroll the key"
	$(MBC_STEP)        "    d. Enter the password created during key generation"
	$(MBC_STEP)        "    e. Select 'Reboot'"
	$(MBC_STEP)        "13. Re-enable Secure Boot in BIOS:"
	$(MBC_RAW_YELLOW)  "                                     Del"
	$(MBC_STEP)        "14. Reconnect to Cerebro and verify driver installation:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'nvidia-smi'"
	@echo
	$(MBC_STEP)        "15. Enable PCIe ReBAR support remotely:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'sudo bash -c \"echo \\\"options nvidia NVreg_EnableResizableBAR=1\\\" > /etc/modprobe.d/nvidia-rebar.conf\"'"
	@echo
	$(MBC_STEP)        "16. Verify ReBAR status after reboot:"
	@echo
	$(MBC_RAW_ORANGE)  "                        ssh bhyslop@\$$CEREBRO_IP_ADDR 'nvidia-smi -q | grep -i rebar'"
	@echo
	$(MBC_PASS) "Successfully installed NVIDIA drivers with Secure Boot and ReBAR support."


#########################################
#  Legacy helpers
#

rbw-hw.%:
	$(MBC_START) "Helper for WSL Distribution Management:"
	$(MBC_SHOW_NORMAL) Stop wsl:
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --shutdown"
	@echo
	$(MBC_SHOW_NORMAL) "List current distributions:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl -l -v"
	@echo
	$(MBC_SHOW_NORMAL) "List available distributions:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --list --online"
	@echo
	$(MBC_SHOW_NORMAL) "Delete a distribution:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --unregister <DistroName>"
	@echo
	$(MBC_SHOW_NORMAL) "Install a distribution:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --install <DistroName>"
	@echo
	$(MBC_SHOW_NORMAL) "Install Required Podman Dependencies:"
	@echo
	$(MBC_RAW_YELLOW)  "     sudo dnf install make"
	$(MBC_RAW_YELLOW)  "     sudo dnf install ncurses   # for tput"
	$(MBC_RAW_YELLOW)  "     sudo dnf install qemu-img"
	$(MBC_RAW_YELLOW)  "     sudo dnf install qemu-system-x86"
	$(MBC_RAW_YELLOW)  "     sudo dnf install libvirt-daemon-driver-qemu"
	$(MBC_RAW_YELLOW)  "     sudo dnf install virtiofsd"
	@echo
	$(MBC_SHOW_NORMAL) "Access Windows C: drive:"
	@echo
	$(MBC_RAW_YELLOW)  "     cd /mnt/c"
	@echo
	$(MBC_SHOW_NORMAL) "Validate Podman Installation:"
	@echo
	$(MBC_RAW_YELLOW)  "     podman --version"
	$(MBC_RAW_YELLOW)  "     podman machine ls"
	@echo
	$(MBC_PASS) "No errors."


zRBW_PODMAN_INSTALL_ROOT = /cygdrive/c/podman-remote
zRBW_PODMAN_INSTALL_WIN  = $(shell cygpath -wa $(zRBW_PODMAN_INSTALL_ROOT))

zRBW_SAMPLE_HTML_ROOT = /cygdrive/c/podman-remote/podman-5.4.0/docs

zRBW_DOC_CONSOLIDATOR_PYTHON = Study/study-strip-podman-docs/spd.strip-podman-docs.py

zRBW_DOC_CONSOLIDATION_IMAGE = ghcr.io/bhyslop/recipemuster:bottle_deftextpro.20250227__172342

rbw-dph.DigestPodmanHtml.sh:
	echo path is -> $(zRBW_PODMAN_INSTALL_WIN)
	podman -c podman-machine-default run --rm \
	  -v '$(zRBW_PODMAN_INSTALL_WIN)':/podman-remote:ro \
	  -v ./Study/study-strip-podman-docs:/app/study:rw \
	  $(zRBW_DOC_CONSOLIDATION_IMAGE) \
	  python /app/study/spd.strip-podman-docs.py /podman-remote /app/study/output

oga.OpenGithubAction.sh:
	$(MBC_STEP) "Assure podman services available..."
	cygstart https://github.com/bhyslop/recipemuster/actions/


# EOF
