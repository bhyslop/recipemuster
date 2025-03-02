# podman-5.2.5 Machine Commands

*This document contains Machine commands from the Podman documentation.*

## Table of Contents

- [podman-machine-info - Display machine host info](#podman-machine-info)
- [podman-machine-init - Initialize a new virtual machine](#podman-machine-init)
- [podman-machine-inspect - Inspect one or more virtual machines](#podman-machine-inspect)
- [podman-machine-list - List virtual machines](#podman-machine-list)
- [podman-machine-os - Manage a Podman virtual machine's OS](#podman-machine-os)
- [podman-machine-reset - Reset Podman machines and environment](#podman-machine-reset)
- [podman-machine-rm - Remove a virtual machine](#podman-machine-rm)
- [podman-machine-set - Set a virtual machine setting](#podman-machine-set)
- [podman-machine-ssh - SSH into a virtual machine](#podman-machine-ssh)
- [podman-machine-start - Start a virtual machine](#podman-machine-start)
- [podman-machine-stop - Stop a virtual machine](#podman-machine-stop)

<a id='podman-machine-info'></a>

## podman-machine-info - Display machine host info

##  NAME

podman-machine-info - Display machine host info

##  SYNOPSIS

**podman machine info**

##  DESCRIPTION

Display information pertaining to the machine host. Rootless only, as
all `podman machine` commands can be only be used with rootless Podman.

##  OPTIONS

#### **\--format**, **-f**=*format*

Change output format to \"json\" or a Go template.

  **Placeholder**   **Description**
  ----------------- ------------------------------------
  .Host \...        Host information for local machine
  .Version \...     Version of the machine

#### **\--help**

Print usage statement.

##  EXAMPLES

Display default Podman machine info.

    $ podman machine info
    Host:
      Arch: amd64
      CurrentMachine: ""
      DefaultMachine: ""
      EventsDir: /run/user/3267/podman
      MachineConfigDir: /home/myusername/.config/containers/podman/machine/qemu
      MachineImageDir: /home/myusername/.local/share/containers/podman/machine/qemu
      MachineState: ""
      NumberOfMachines: 0
      OS: linux
      VMType: qemu
    Version:
      APIVersion: 4.4.0
      Built: 1677097848
      BuiltTime: Wed Feb 22 15:30:48 2023
      GitCommit: aa196c0d5c9abd5800edf9e27587c60343a26c2b-dirty
      GoVersion: go1.20
      Os: linux
      OsArch: linux/amd64
      Version: 4.4.0

Display default Podman machine info formatted as json.

    $ podman machine info --format json
    {
      "Host": {
        "Arch": "amd64",
        "CurrentMachine": "",
        "DefaultMachine": "",
        "EventsDir": "/run/user/3267/podman",
        "MachineConfigDir": "/home/myusername/.config/containers/podman/machine/qemu",
        "MachineImageDir": "/home/myusername/.local/share/containers/podman/machine/qemu",
        "MachineState": "",
        "NumberOfMachines": 0,
        "OS": "linux",
        "VMType": "qemu"
      },
      "Version": {
        "APIVersion": "4.4.0",
        "Version": "4.4.0",
        "GoVersion": "go1.20",
        "GitCommit": "aa196c0d5c9abd5800edf9e27587c60343a26c2b-dirty",
        "BuiltTime": "Wed Feb 22 15:30:48 2023",
        "Built": 1677097848,
        "OsArch": "linux/amd64",
        "Os": "linux"
      }
    }

Display default Podman machine Host.Arch field.

    $ podman machine info --format "{{ .Host.Arch }}"
    amd64

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

June 2022, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-machine-init'></a>

## podman-machine-init - Initialize a new virtual machine

##  NAME

podman-machine-init - Initialize a new virtual machine

##  SYNOPSIS

**podman machine init** \[*options*\] \[*name*\]

##  DESCRIPTION

Initialize a new virtual machine for Podman.

The default machine name is `podman-machine-default`. If a machine name
is not specified as an argument, then the new machine will be named
`podman-machine-default`.

Rootless only.

Podman on MacOS and Windows requires a virtual machine. This is because
containers are Linux - containers do not run on any other OS because
containers\' core functionality are tied to the Linux kernel. Podman
machine must be used to manage MacOS and Windows machines, but can be
optionally used on Linux.

**podman machine init** initializes a new Linux virtual machine where
containers are run. SSH keys are automatically generated to access the
VM, and system connections to the root account and a user account inside
the VM are added.

By default, the VM distribution is [Fedora
CoreOS](https://getfedora.org/en/coreos?stream=testing) except for WSL
which is based on a custom Fedora image. While Fedora CoreOS upgrades
come out every 14 days, the automatic update mechanism Zincata is
disabled by Podman machine.

To check if there is an upgrade available for your machine os, you can
run the following command:

    $ podman machine ssh 'sudo rpm-ostree upgrade --check'

If an update is available, you can rerun the above command and remove
the `--check` and your operating system will be updated. After updating,
you must stop and start your machine with
`podman machine stop && podman machine start` for it to take effect.

Note: Updating as described above can result in version mismatches
between Podman on the host and Podman in the machine. Executing
`podman info` should reveal versions of both. A configuration where the
Podman host and machine mismatch are unsupported.

For more information on updates and advanced configuration, see the
Fedora CoreOS documentation about
[auto-updates](https://docs.fedoraproject.org/en-US/fedora-coreos/auto-updates/)
and [update
strategies](https://coreos.github.io/zincati/usage/updates-strategy/).

Fedora CoreOS upgrades come out every 14 days and are detected and
installed automatically. The VM is rebooted during the upgrade. For more
information on updates and advanced configuration, see the Fedora CoreOS
documentation about
[auto-updates](https://docs.fedoraproject.org/en-US/fedora-coreos/auto-updates/)
and [update
strategies](https://coreos.github.io/zincati/usage/updates-strategy/).

##  OPTIONS

#### **\--cpus**=*number*

Number of CPUs.

#### **\--disk-size**=*number*

Size of the disk for the guest VM in GiB.

#### **\--help**

Print usage statement.

#### **\--ignition-path**

Fully qualified path of the ignition file.

If an ignition file is provided, the file is copied into the user\'s
CONF_DIR and renamed. Additionally, no SSH keys are generated, nor are
any system connections made. It is assumed that the user does these
things manually or handled otherwise.

#### **\--image**

Fully qualified registry, path, or URL to a VM image. Registry target
must be in the form of `docker://registry/repo/image:version`.

#### **\--memory**, **-m**=*number*

Memory (in MiB). Note: 1024MiB = 1GiB.

#### **\--now**

Start the virtual machine immediately after it has been initialized.

#### **\--rootful**

Whether this machine prefers rootful (`true`) or rootless (`false`)
container execution. This option determines the remote connection
default if there is no existing remote connection configurations.

API forwarding, if available, follows this setting.

#### **\--timezone**

Set the timezone for the machine and containers. Valid values are
`local` or a `timezone` such as `America/Chicago`. A value of `local`,
which is the default, means to use the timezone of the machine host.

The timezone setting is not used with WSL. WSL automatically sets the
timezone to the same as the host Windows operating system.

#### **\--usb**=*bus=number,devnum=number* or *vendor=hexadecimal,product=hexadecimal*

Assign a USB device from the host to the VM via USB passthrough. Only
supported for QEMU Machines.

The device needs to have proper permissions in order to be passed to the
machine. This means the device needs to be under your user group.

Note that using bus and device number are simpler but the values can
change every boot or when the device is unplugged.

When specifying a USB using vendor and product ID\'s, if more than one
device has the same vendor and product ID, the first available device is
assigned.

#### **\--user-mode-networking**

Indicates that this machine relays traffic from the guest through a
user-space process running on the host. In some VPN configurations the
VPN may drop traffic from alternate network interfaces, including VM
network devices. By enabling user-mode networking (a setting of `true`),
VPNs observe all podman machine traffic as coming from the host,
bypassing the problem.

When the qemu backend is used (Linux, Mac), user-mode networking is
mandatory and the only allowed value is `true`. In contrast, The
Windows/WSL backend defaults to `false`, and follows the standard WSL
network setup. Changing this setting to `true` on Windows/WSL informs
Podman to replace the WSL networking setup on start of this machine
instance with a user-mode networking distribution. Since WSL shares the
same kernel across distributions, all other running distributions reuses
this network. Likewise, when the last machine instance with a `true`
setting stops, the original networking setup is restored.

#### **\--username**

Username to use for executing commands in remote VM. Default value is
`core` for FCOS and `user` for Fedora (default on Windows hosts). Should
match the one used inside the resulting VM image.

#### **\--volume**, **-v**=*source:target\[:options\]*

Mounts a volume from source to target.

Create a mount. If /host-dir:/machine-dir is specified as the
`*source:target*`, Podman mounts *host-dir* in the host to *machine-dir*
in the Podman machine.

Additional options may be specified as a comma-separated string.
Recognized options are: \* **ro**: mount volume read-only \* **rw**:
mount volume read/write (default) \* **security_model=\[model\]**:
specify 9p security model (see below)

The 9p security model \[determines\]
https://wiki.qemu.org/Documentation/9psetup#Starting_the_Guest_directly
if and how the 9p filesystem translates some filesystem operations
before actual storage on the host.

In order to allow symlinks to work, on MacOS the default security model
is *none*.

The value of *mapped-xattr* specifies that 9p store symlinks and some
file attributes as extended attributes on the host. This is suitable
when the host and the guest do not need to interoperate on the shared
filesystem, but has caveats for actual shared access; notably, symlinks
on the host are not usable on the guest and vice versa. If
interoperability is required, then choose *none* instead, but keep in
mind that the guest is not able to do things that the user running the
virtual machine cannot do, e.g. create files owned by another user.
Using *none* is almost certainly the best choice for read-only volumes.

Example: `-v "$HOME/git:$HOME/git:ro,security_model=none"`

Default volume mounts are defined in *containers.conf*. Unless changed,
the default values is `$HOME:$HOME`.

##  EXAMPLES

Initialize the default Podman machine, pulling the content from the
internet.

    $ podman machine init

Initialize a Podman machine for the specified name pulling the content
from the internet.

    $ podman machine init myvm

Initialize the default Podman machine pulling the content from the
internet defaulting to rootful mode. The default is rootless.

    $ podman machine init --rootful

Initialize the default Podman machine overriding its disk size override,
pulling the content from the internet.

    $ podman machine init --disk-size 50

Initialize the specified Podman machine overriding its memory size,
pulling the content from the internet.

    $ podman machine init --memory=1024 myvm

Initialize the default Podman machine with the host directory `/Users`
mounted into the VM at `/mnt/Users`.

    $ podman machine init -v /Users:/mnt/Users

Initialize the default Podman machine with a usb device passthrough
specified with options. Only supported for QEMU Machines.

    $ podman machine init --usb vendor=13d3,product=5406

Initialize the default Podman machine with a usb device passthrough with
specified with options. Only supported for QEMU Machines.

    $ podman machine init --usb bus=1,devnum=3

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

March 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-machine-inspect'></a>

## podman-machine-inspect - Inspect one or more virtual machines

##  NAME

podman-machine-inspect - Inspect one or more virtual machines

##  SYNOPSIS

**podman machine inspect** \[*options*\] \[*name*\] \...

##  DESCRIPTION

Inspect one or more virtual machines

Obtain greater detail about Podman virtual machines. More than one
virtual machine can be inspected at once.

The default machine name is `podman-machine-default`. If a machine name
is not specified as an argument, then `podman-machine-default` will be
inspected.

Rootless only.

##  OPTIONS

#### **\--format**

Print results with a Go template.

  -----------------------------------------------------------------------------
  **Placeholder**       **Description**
  --------------------- -------------------------------------------------------
  .ConfigDir \...       Machine configuration directory location

  .ConnectionInfo \...  Machine connection information

  .Created \...         Machine creation time (string, ISO3601)

  .LastUp \...          Time when machine was last booted

  .Name                 Name of the machine

  .Resources \...       Resources used by the machine

  .Rootful              Whether the machine prefers rootful or rootless
                        container execution

  .Rosetta              Whether this machine uses Rosetta

  .SSHConfig \...       SSH configuration info for communicating with machine

  .State                Machine state

  .UserModeNetworking   Whether this machine uses user-mode networking
  -----------------------------------------------------------------------------

#### **\--help**

Print usage statement.

##  EXAMPLES

Inspect the specified Podman machine.

    $ podman machine inspect podman-machine-default

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

April 2022, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-machine-list'></a>

## podman-machine-list - List virtual machines

##  NAME

podman-machine-list - List virtual machines

##  SYNOPSIS

**podman machine list** \[*options*\]

**podman machine ls** \[*options*\]

##  DESCRIPTION

List Podman managed virtual machines.

Podman on MacOS and Windows requires a virtual machine. This is because
containers are Linux - containers do not run on any other OS because
containers\' core functionality are tied to the Linux kernel. Podman
machine must be used to manage MacOS and Windows machines, but can be
optionally used on Linux.

Rootless only.

NOTE: The podman-machine configuration file is managed under the
`$XDG_CONFIG_HOME/containers/podman/machine/` directory. Changing the
`$XDG_CONFIG_HOME` environment variable while the machines are running
can lead to unexpected behavior. (see [podman(1)](podman.html))

##  OPTIONS

#### **\--format**=*format*

Change the default output format. This can be of a supported type like
\'json\' or a Go template. Valid placeholders for the Go template are
listed below:

  **Placeholder**       **Description**
  --------------------- -------------------------------------------
  .CPUs                 Number of CPUs
  .Created              Time since VM creation
  .Default              Is default machine
  .DiskSize             Disk size of machine
  .IdentityPath         Path to ssh identity file
  .LastUp               Time since the VM was last run
  .Memory               Allocated memory for machine
  .Name                 VM name
  .Port                 SSH Port to use to connect to VM
  .RemoteUsername       VM Username for rootless Podman
  .Running              Is machine running
  .Stream               Stream name
  .UserModeNetworking   Whether machine uses user-mode networking
  .VMType               VM type

#### **\--help**

Print usage statement.

#### **\--noheading**, **-n**

Omit the table headings from the listing.

#### **\--quiet**, **-q**

Only print the name of the machine. This also implies no table heading
is printed.

##  EXAMPLES

List all Podman machines.

    $ podman machine list
    NAME                    VM TYPE     CREATED      LAST UP      CPUS        MEMORY      DISK SIZE
    podman-machine-default  qemu        2 weeks ago  2 weeks ago  1           2.147GB     10.74GB

List all Podman machines using the specified table format.

    $ podman machine ls --format "table {{.Name}}\t{{.VMType}}\t{{.Created}}\t{{.LastUp}}"
    NAME                    VM TYPE     CREATED      LAST UP
    podman-machine-default  qemu        2 weeks ago  2 weeks ago

List all Podman machines in json format.

    $ podman machine ls --format json
    [
        {
            "Name": "podman-machine-default",
            "Default": false,
            "Created": "2021-12-27T10:36:14.373347492-05:00",
            "Running": false,
            "LastUp": "2021-12-27T11:22:50.17333371-05:00",
            "Stream": "default",
            "VMType": "qemu",
            "CPUs": 1,
            "Memory": "2147483648",
            "DiskSize": "10737418240"
        }
    ]

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

March 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-machine-os'></a>

## podman-machine-os - Manage a Podman virtual machine's OS

##  NAME

podman-machine-os - Manage a Podman virtual machine\'s OS

##  SYNOPSIS

**podman machine os** *subcommand*

##  DESCRIPTION

`podman machine os` is a set of subcommands that manage a Podman virtual
machine\'s operating system.

##  SUBCOMMANDS

  --------------------------------------------------------------------------------------------------
  Command   Man Page                                                     Description
  --------- ------------------------------------------------------------ ---------------------------
  apply     [podman-machine-os-apply(1)](podman-machine-os-apply.html)   Apply an OCI image to a
                                                                         Podman Machine\'s OS

  --------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**,
**[podman-machine-os-apply(1)](podman-machine-os-apply.html)**

##  HISTORY

February 2023, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-machine-reset'></a>

## podman-machine-reset - Reset Podman machines and environment

##  NAME

podman-machine-reset - Reset Podman machines and environment

##  SYNOPSIS

**podman machine reset** \[*options*\]

##  DESCRIPTION

Reset your Podman machine environment. This command stops any running
machines and then removes them. Configuration and data files are then
removed. Data files would include machine disk images and any previously
pulled cache images. When this command is run, all of your Podman
machines will have been deleted.

##  OPTIONS

#### **\--force**, **-f**

Reset without confirmation.

#### **\--help**

Print usage statement.

##  EXAMPLES

    $ podman machine reset
    Warning: this command will delete all existing podman machines
    and all of the configuration and data directories for Podman machines

    The following machine(s) will be deleted:

    dev
    podman-machine-default

    Are you sure you want to continue? [y/N] y
    $

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

Feb 2024, Originally compiled by Brent Baude<bbaude@redhat.com>


---

<a id='podman-machine-rm'></a>

## podman-machine-rm - Remove a virtual machine

##  NAME

podman-machine-rm - Remove a virtual machine

##  SYNOPSIS

**podman machine rm** \[*options*\] \[*name*\]

##  DESCRIPTION

Remove a virtual machine and its related files. What is actually deleted
depends on the virtual machine type. For all virtual machines, the
generated podman system connections are deleted. The ignition files
generated for that VM are also removed as is its image file on the
filesystem.

Users get a display of what is deleted and are required to confirm
unless the option `--force` is used.

The default machine name is `podman-machine-default`. If a machine name
is not specified as an argument, then `podman-machine-default` will be
removed.

Rootless only.

##  OPTIONS

#### **\--force**, **-f**

Stop and delete without confirmation.

#### **\--help**

Print usage statement.

#### **\--save-ignition**

Do not delete the generated ignition file.

#### **\--save-image**

Do not delete the VM image.

##  EXAMPLES

Remove the specified Podman machine.

    $ podman machine rm test1

    The following files will be deleted:

    /home/user/.config/containers/podman/machine/qemu/test1.ign
    /home/user/.local/share/containers/podman/machine/qemu/test1_fedora-coreos-33.20210315.1.0-qemu.x86_64.qcow2
    /home/user/.config/containers/podman/machine/qemu/test1.json

    Are you sure you want to continue? [y/N] y

Remove the specified Podman machine even if it is running.

    $ podman machine rm -f test1
    $

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

March 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-machine-set'></a>

## podman-machine-set - Set a virtual machine setting

##  NAME

podman-machine-set - Set a virtual machine setting

##  SYNOPSIS

**podman machine set** \[*options*\] \[*name*\]

##  DESCRIPTION

Change a machine setting.

The default machine name is `podman-machine-default`. If a machine name
is not specified as an argument, then the settings will be applied to
`podman-machine-default`.

Rootless only.

##  OPTIONS

#### **\--cpus**=*number*

Number of CPUs. Only supported for QEMU machines.

#### **\--disk-size**=*number*

Size of the disk for the guest VM in GB. Can only be increased. Only
supported for QEMU machines.

#### **\--help**

Print usage statement.

#### **\--memory**, **-m**=*number*

Memory (in MB). Only supported for QEMU machines.

#### **\--rootful**

Whether this machine prefers rootful (`true`) or rootless (`false`)
container execution. This option updates the current podman remote
connection default if it is currently pointing at the specified machine
name (or `podman-machine-default` if no name is specified).

Unlike [**podman system connection
default**](podman-system-connection-default.html) this option makes the
API socket, if available, forward to the rootful/rootless socket in the
VM.

Note that changing this option means that all the existing
containers/images/volumes, etc\... are no longer visible with the
default connection/socket. This is because the root and rootless users
in the VM are completely separated and do not share any storage. The
data however is not lost and you can always change this option back or
use the other connection to access it.

#### **\--usb**=*bus=number,devnum=number* or *vendor=hexadecimal,product=hexadecimal* or *\"\"*

Assign a USB device from the host to the VM. Only supported for QEMU
Machines.

The device needs to be present when the VM starts. The device needs to
have proper permissions in order to be assign to podman machine.

Use an empty string to remove all previously set USB devices.

Note that using bus and device number are simpler but the values can
change every boot or when the device is unplugged. Using vendor and
product might lead to collision in the case of multiple devices with the
same vendor product value, the first available device is assigned.

#### **\--user-mode-networking**

Indicates that this machine relays traffic from the guest through a
user-space process running on the host. In some VPN configurations the
VPN may drop traffic from alternate network interfaces, including VM
network devices. By enabling user-mode networking (a setting of `true`),
VPNs observe all podman machine traffic as coming from the host,
bypassing the problem.

When the qemu backend is used (Linux, Mac), user-mode networking is
mandatory and the only allowed value is `true`. In contrast, The
Windows/WSL backend defaults to `false`, and follows the standard WSL
network setup. Changing this setting to `true` on Windows/WSL informs
Podman to replace the WSL networking setup on start of this machine
instance with a user-mode networking distribution. Since WSL shares the
same kernel across distributions, all other running distributions reuses
this network. Likewise, when the last machine instance with a `true`
setting stops, the original networking setup is restored.

##  EXAMPLES

To switch the default Podman machine from rootless to rootful:

    $ podman machine set --rootful

or more explicitly set with value true.

    $ podman machine set --rootful=true

Switch the default Podman machine from rootful to rootless.

    $ podman machine set --rootful=false

Switch the specified Podman machine from rootless to rootful.

    $ podman machine set --rootful myvm

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

February 2022, Originally compiled by Jason Greene
<jason.greene@redhat.com>


---

<a id='podman-machine-ssh'></a>

## podman-machine-ssh - SSH into a virtual machine

##  NAME

podman-machine-ssh - SSH into a virtual machine

##  SYNOPSIS

**podman machine ssh** \[*options*\] \[*name*\] \[*command* \[*arg*
\...\]\]

##  DESCRIPTION

SSH into a Podman-managed virtual machine and optionally execute a
command on the virtual machine. Unless using the default virtual
machine, the first argument must be the virtual machine name. The
optional command to execute can then follow. If no command is provided,
an interactive session with the virtual machine is established.

The exit code from ssh command is forwarded to the podman machine ssh
caller, see [Exit Codes](#Exit-Codes).

The default machine name is `podman-machine-default`. If a machine name
is not specified as an argument, then `podman-machine-default` will be
SSH\'d into.

Rootless only.

##  OPTIONS

#### **\--help**

Print usage statement.

#### **\--username**=*name*

Username to use when SSH-ing into the VM.

##  Exit Codes

The exit code from `podman machine ssh` gives information about why the
command failed. When `podman machine ssh` commands exit with a non-zero
code, the exit codes follow the `chroot` standard, see below:

**125** The error is with podman ***itself***

    $ podman machine ssh --foo; echo $?
    Error: unknown flag: --foo
    125

**126** Executing a *contained command* and the *command* cannot be
invoked

    $ podman machine ssh /etc; echo $?
    Error: fork/exec /etc: permission denied
    126

**127** Executing a *contained command* and the *command* cannot be
found

    $ podman machine ssh foo; echo $?
    Error: fork/exec /usr/bin/bogus: no such file or directory
    127

**Exit code** *contained command* exit code

    $ podman machine ssh /bin/sh -c 'exit 3'; echo $?
    3

##  EXAMPLES

To get an interactive session with the default Podman machine:

SSH into the default Podman machine.

    $ podman machine ssh

Run command inside the default Podman machine via ssh.

    $ podman machine ssh myvm

Run command inside the specified Podman machine via ssh.

    $ podman machine ssh myvm rpm -q podman

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

March 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-machine-start'></a>

## podman-machine-start - Start a virtual machine

##  NAME

podman-machine-start - Start a virtual machine

##  SYNOPSIS

**podman machine start** \[*name*\]

##  DESCRIPTION

Starts a virtual machine for Podman.

Rootless only.

Podman on MacOS and Windows requires a virtual machine. This is because
containers are Linux - containers do not run on any other OS because
containers\' core functionality are tied to the Linux kernel. Podman
machine must be used to manage MacOS and Windows machines, but can be
optionally used on Linux.

The default machine name is `podman-machine-default`. If a machine name
is not specified as an argument, then `podman-machine-default` will be
started.

Only one Podman managed VM can be active at a time. If a VM is already
running, `podman machine start` returns an error.

**podman machine start** starts a Linux virtual machine where containers
are run.

##  OPTIONS

#### **\--help**

Print usage statement.

#### **\--no-info**

Suppress informational tips.

#### **\--quiet**, **-q**

Suppress machine starting status output.

##  EXAMPLES

Start the specified podman machine.

    $ podman machine start myvm

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

March 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-machine-stop'></a>

## podman-machine-stop - Stop a virtual machine

##  NAME

podman-machine-stop - Stop a virtual machine

##  SYNOPSIS

**podman machine stop** \[*name*\]

##  DESCRIPTION

Stops a virtual machine.

The default machine name is `podman-machine-default`. If a machine name
is not specified as an argument, then `podman-machine-default` will be
stopped.

Rootless only.

Podman on MacOS and Windows requires a virtual machine. This is because
containers are Linux - containers do not run on any other OS because
containers\' core functionality are tied to the Linux kernel. Podman
machine must be used to manage MacOS and Windows machines, but can be
optionally used on Linux.

**podman machine stop** stops a Linux virtual machine where containers
are run.

##  OPTIONS

#### **\--help**

Print usage statement.

##  EXAMPLES

Stop a podman machine named myvm.

    $ podman machine stop myvm

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine(1)](podman-machine.html)**

##  HISTORY

March 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

