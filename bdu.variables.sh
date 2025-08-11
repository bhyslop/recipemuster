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

##########################################
# Makefile Bash Variables File
#
# This is a reference variables file defined when mapping the RBM
# makefile system into a repo.  The stock dispatch script
# uses this to set up the dispatch environment.
#
# This file is bilingual: it is interpreted by both bash and make.
# Therefore: no spaces are allowed around `=`.  Also, only declare
# variables here.  If you define any variables in terms of other
# ones, use ${xxx} to expand.

MBV_STATION_FILE=../station-files/MBS.STATION.sh

MBV_TABTARGET_DIR=tt

MBV_TABTARGET_DELIMITER=.

MBV_TOOLS_DIR=Tools

MBV_CONSOLE_MAKEFILE=${MBV_TOOLS_DIR}/rbw.workbench.mk

MBV_TEMP_ROOT_DIR=../temp-${MBV_CONSOLE_MAKEFILE}

MBV_LOG_LAST=last

MBV_LOG_EXT=txt


