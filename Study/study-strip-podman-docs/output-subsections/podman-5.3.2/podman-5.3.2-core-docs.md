# podman-5.3.2 Core Commands

*This document contains Core commands from the Podman documentation.*

## Table of Contents

- [podman-attach - Attach to a running container](#podman-attach)
- [podman-build - Build a container image using a Containerfile](#podman-build)
- [podman-commit - Create new image based on the changed container](#podman-commit)
- [podman-compose - Run Compose workloads via an external compose
provider](#podman-compose)
- [podman-container - Manage containers](#podman-container)
- [podman-cp - Copy files/folders between a container and the local
filesystem](#podman-cp)
- [podman-create - Create a new container](#podman-create)
- [podman-diff - Inspect changes on a container or image's
filesystem](#podman-diff)
- [podman-events - Monitor Podman events](#podman-events)
- [podman-exec - Execute a command in a running container](#podman-exec)
- [podman-export - Export a container's filesystem contents as a tar
archive](#podman-export)
- [podman-farm - Farm out builds to machines running podman for
different architectures](#podman-farm)
- [podman-generate - Generate structured data based on containers, pods
or volumes](#podman-generate)
- [podman-healthcheck - Manage healthchecks for containers](#podman-healthcheck)
- [podman - Simple management tool for pods, containers and images](#podman-help)
- [podman-history - Show the history of an image](#podman-history)
- [podman-image - Manage images](#podman-image)
- [podman-images - List images in local storage](#podman-images)
- [podman-import - Import a tarball and save it as a filesystem
image](#podman-import)
- [podman-info - Display Podman related system information](#podman-info)
- [podman-init - Initialize one or more containers](#podman-init)
- [podman-inspect - Display a container, image, volume, network, or
pod's configuration](#podman-inspect)
- [podman-kill - Kill the main process in one or more containers](#podman-kill)
- [podman-kube - Play containers, pods or volumes based on a structured
input file](#podman-kube)
- [podman-load - Load image(s) from a tar archive into container
storage](#podman-load)
- [podman-login - Log in to a container registry](#podman-login)
- [podman-logout - Log out of a container registry](#podman-logout)
- [podman-logs - Display the logs of one or more containers](#podman-logs)
- [podman-machine - Manage Podman's virtual machine](#podman-machine)
- [podman-manifest - Create and manipulate manifest lists and image
indexes](#podman-manifest)
- [podman-network - Manage Podman networks](#podman-network)
- [podman-pause - Pause one or more containers](#podman-pause)
- [podman-pod - Management tool for groups of containers, called
pods](#podman-pod)
- [podman-port - List port mappings for a container](#podman-port)
- [podman-ps - Print out information about containers](#podman-ps)
- [podman-pull - Pull an image from a registry](#podman-pull)
- [podman-push - Push an image, manifest list or image index from local
storage to elsewhere](#podman-push)
- [podman-rename - Rename an existing container](#podman-rename)
- [podman-restart - Restart one or more containers](#podman-restart)
- [podman-rm - Remove one or more containers](#podman-rm)
- [podman-rmi - Remove one or more locally stored images](#podman-rmi)
- [podman-run - Run a command in a new container](#podman-run)
- [podman-save - Save image(s) to an archive](#podman-save)
- [podman-search - Search a registry for an image](#podman-search)
- [podman-secret - Manage podman secrets](#podman-secret)
- [podman-start - Start one or more containers](#podman-start)
- [podman-stats - Display a live stream of one or more container's
resource usage statistics](#podman-stats)
- [podman-stop - Stop one or more running containers](#podman-stop)
- [podman-system - Manage podman](#podman-system)
- [podman-tag - Add an additional name to a local image](#podman-tag)
- [podman-top - Display the running processes of a container](#podman-top)
- [podman-unpause - Unpause one or more containers](#podman-unpause)
- [podman-untag - Remove one or more names from a locally-stored
image](#podman-untag)
- [podman-update - Update the configuration of a given container](#podman-update)
- [podman-version - Display the Podman version information](#podman-version)
- [podman-volume - Simple management tool for volumes](#podman-volume)
- [podman-wait - Wait on one or more containers to stop and print their
exit codes](#podman-wait)

<a id='podman-attach'></a>

## podman-attach - Attach to a running container

##  NAME

podman-attach - Attach to a running container

##  SYNOPSIS

**podman attach** \[*options*\] *container*

**podman container attach** \[*options*\] *container*

##  DESCRIPTION

**podman attach** attaches to a running *container* using the
*container\'s name* or *ID*, to either view its ongoing output or to
control it interactively.\
The *container* can be detached from (and leave it running) using a
configurable key sequence. The default sequence is `ctrl-p,ctrl-q`.
Configure the keys sequence using the **\--detach-keys** OPTION, or
specifying it in the `containers.conf` file: see
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
for more information.

##  OPTIONS

#### **\--detach-keys**=*sequence*

Specify the key sequence for detaching a container. Format is a single
character `[a-Z]` or one or more `ctrl-<value>` characters where
`<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`. Specifying \"\"
disables this feature. The default is *ctrl-p,ctrl-q*.

This option can also be set in **containers.conf**(5) file.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--no-stdin**

Do not attach STDIN. The default is **false**.

#### **\--sig-proxy**

Proxy received signals to the container process. SIGCHLD, SIGURG,
SIGSTOP, and SIGKILL are not proxied.

The default is **true**.

##  EXAMPLES

Attach to a container called \"foobar\".

    $ podman attach foobar

Attach to the latest created container. (This option is not available
with the remote Podman client, including Mac and Windows (excluding
WSL2) machines)

    $ podman attach --latest

Attach to a container that start with the ID \"1234\".

    $ podman attach 1234

Attach to a container without attaching STDIN.

    $ podman attach --no-stdin foobar

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-exec(1)](podman-exec.html)**,
**[podman-run(1)](podman-run.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**


---

<a id='podman-build'></a>

## podman-build - Build a container image using a Containerfile

##  NAME

podman-build - Build a container image using a Containerfile

##  SYNOPSIS

**podman build** \[*options*\] \[*context*\]

**podman image build** \[*options*\] \[*context*\]

##  DESCRIPTION

**podman build** Builds an image using instructions from one or more
Containerfiles or Dockerfiles and a specified build context directory. A
Containerfile uses the same syntax as a Dockerfile internally. For this
document, a file referred to as a Containerfile can be a file named
either \'Containerfile\' or \'Dockerfile\' exclusively. Any file that
has additional extension attached will not be recognized by
`podman build .` unless a `-f` flag is used to specify the file.

The build context directory can be specified as the http(s) URL of an
archive, git repository or Containerfile.

When invoked with `-f` and a path to a Containerfile, with no explicit
CONTEXT directory, Podman uses the Containerfile\'s parent directory as
its build context.

Containerfiles ending with a \".in\" suffix are preprocessed via CPP(1).
This can be useful to decompose Containerfiles into several reusable
parts that can be used via CPP\'s **#include** directive. Containerfiles
ending in .in are restricted to no comment lines unless they are CPP
commands. Note, a Containerfile.in file can still be used by other tools
when manually preprocessing them via `cpp -E`.

When the URL is an archive, the contents of the URL is downloaded to a
temporary location and extracted before execution.

When the URL is a Containerfile, the Containerfile is downloaded to a
temporary location.

When a Git repository is set as the URL, the repository is cloned
locally and then set as the context. A URL is treated as a Git
repository if it has a `git://` prefix or a `.git` suffix.

NOTE: `podman build` uses code sourced from the `Buildah` project to
build container images. This `Buildah` code creates `Buildah` containers
for the `RUN` options in container storage. In certain situations, when
the `podman build` crashes or users kill the `podman build` process,
these external containers can be left in container storage. Use the
`podman ps --all --external` command to see these containers.

`podman buildx build` command is an alias of `podman build`. Not all
`buildx build` features are available in Podman. The `buildx build`
option is provided for scripting compatibility.

##  OPTIONS

#### **\--add-host**=*hostname\[;hostname\[;\...\]\]*:*ip*

Add a custom host-to-IP mapping to the container\'s `/etc/hosts` file.

The option takes one or multiple semicolon-separated hostnames to be
mapped to a single IPv4 or IPv6 address, separated by a colon. It can
also be used to overwrite the IP addresses of hostnames Podman adds to
`/etc/hosts` by default (also see the **\--name** and **\--hostname**
options). This option can be specified multiple times to add additional
mappings to `/etc/hosts`. It conflicts with the **\--no-hosts** option
and conflicts with *no_hosts=true* in `containers.conf`.

Instead of an IP address, the special flag *host-gateway* can be given.
This resolves to an IP address the container can use to connect to the
host. The IP address chosen depends on your network setup, thus there\'s
no guarantee that Podman can determine the *host-gateway* address
automatically, which will then cause Podman to fail with an error
message. You can overwrite this IP address using the
*host_containers_internal_ip* option in *containers.conf*.

The *host-gateway* address is also used by Podman to automatically add
the `host.containers.internal` and `host.docker.internal` hostnames to
`/etc/hosts`. You can prevent that by either giving the **\--no-hosts**
option, or by setting *host_containers_internal_ip=\"none\"* in
*containers.conf*. If no *host-gateway* address was configured manually
and Podman fails to determine the IP address automatically, Podman will
silently skip adding these internal hostnames to `/etc/hosts`. If Podman
is running in a virtual machine using `podman machine` (this includes
Mac and Windows hosts), Podman will silently skip adding the internal
hostnames to `/etc/hosts`, unless an IP address was configured manually;
the internal hostnames are resolved by the gvproxy DNS resolver instead.

Podman will use the `/etc/hosts` file of the host as a basis by default,
i.e. any hostname present in this file will also be present in the
`/etc/hosts` file of the container. A different base file can be
configured using the *base_hosts_file* config in `containers.conf`.

#### **\--all-platforms**

Instead of building for a set of platforms specified using the
**\--platform** option, inspect the build\'s base images, and build for
all of the platforms for which they are all available. Stages that use
*scratch* as a starting point can not be inspected, so at least one
non-*scratch* stage must be present for detection to work usefully.

#### **\--annotation**=*annotation=value*

Add an image *annotation* (e.g. annotation=*value*) to the image
metadata. Can be used multiple times.

Note: this information is not present in Docker image formats, so it is
discarded when writing images in Docker formats.

#### **\--arch**=*arch*

Set the architecture of the image to be built, and that of the base
image to be pulled, if the build uses one, to the provided value instead
of using the architecture of the build host. Unless overridden,
subsequent lookups of the same image in the local storage matches this
architecture, regardless of the host. (Examples: arm, arm64, 386, amd64,
ppc64le, s390x)

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--build-arg**=*arg=value*

Specifies a build argument and its value, which is interpolated in
instructions read from the Containerfiles in the same way that
environment variables are, but which are not added to environment
variable list in the resulting image\'s configuration.

#### **\--build-arg-file**=*path*

Specifies a file containing lines of build arguments of the form
`arg=value`. The suggested file name is `argfile.conf`.

Comment lines beginning with `#` are ignored, along with blank lines.
All others must be of the `arg=value` format passed to `--build-arg`.

If several arguments are provided via the `--build-arg-file` and
`--build-arg` options, the build arguments are merged across all of the
provided files and command line arguments.

Any file provided in a `--build-arg-file` option is read before the
arguments supplied via the `--build-arg` option.

When a given argument name is specified several times, the last instance
is the one that is passed to the resulting builds. This means
`--build-arg` values always override those in a `--build-arg-file`.

#### **\--build-context**=*name=value*

Specify an additional build context using its short name and its
location. Additional build contexts can be referenced in the same manner
as we access different stages in COPY instruction.

Valid values are:

-   Local directory -- e.g. \--build-context
    project2=../path/to/project2/src (This option is not available with
    the remote Podman client. On Podman machine setup (i.e macOS and
    Windows) path must exists on the machine VM)
-   HTTP URL to a tarball -- e.g. \--build-context
    src=https://example.org/releases/src.tar
-   Container image -- specified with a container-image:// prefix, e.g.
    \--build-context alpine=container-image://alpine:3.15, (also accepts
    docker://, docker-image://)

On the Containerfile side, reference the build context on all commands
that accept the "from" parameter. Here's how that might look:

::: {#cb1 .sourceCode}
``` {.sourceCode .dockerfile}
FROM [name]
COPY --from=[name] ...
RUN --mount=from=[name] …
```
:::

The value of [name](#name) is matched with the following priority order:

-   Named build context defined with \--build-context [name](#name)=..
-   Stage defined with AS [name](#name) inside Containerfile
-   Image [name](#name), either local or in a remote registry

#### **\--cache-from**=*image*

Repository to utilize as a potential cache source. When specified,
Buildah tries to look for cache images in the specified repository and
attempts to pull cache images instead of actually executing the build
steps locally. Buildah only attempts to pull previously cached images if
they are considered as valid cache hits.

Use the `--cache-to` option to populate a remote repository with cache
content.

Example

::: {#cb2 .sourceCode}
``` {.sourceCode .bash}
# populate a cache and also consult it
buildah build -t test --layers --cache-to registry/myrepo/cache --cache-from registry/myrepo/cache .
```
:::

Note: `--cache-from` option is ignored unless `--layers` is specified.

#### **\--cache-to**=*image*

Set this flag to specify a remote repository that is used to store cache
images. Buildah attempts to push newly built cache image to the remote
repository.

Note: Use the `--cache-from` option in order to use cache content in a
remote repository.

Example

::: {#cb3 .sourceCode}
``` {.sourceCode .bash}
# populate a cache and also consult it
buildah build -t test --layers --cache-to registry/myrepo/cache --cache-from registry/myrepo/cache .
```
:::

Note: `--cache-to` option is ignored unless `--layers` is specified.

#### **\--cache-ttl**

Limit the use of cached images to only consider images with created
timestamps less than *duration* ago. For example if `--cache-ttl=1h` is
specified, Buildah considers intermediate cache images which are created
under the duration of one hour, and intermediate cache images outside
this duration is ignored.

Note: Setting `--cache-ttl=0` manually is equivalent to using
`--no-cache` in the implementation since this means that the user does
not want to use cache at all.

#### **\--cap-add**=*CAP_xxx*

When executing RUN instructions, run the command specified in the
instruction with the specified capability added to its capability set.
Certain capabilities are granted by default; this option can be used to
add more.

#### **\--cap-drop**=*CAP_xxx*

When executing RUN instructions, run the command specified in the
instruction with the specified capability removed from its capability
set. The CAP_CHOWN, CAP_DAC_OVERRIDE, CAP_FOWNER, CAP_FSETID, CAP_KILL,
CAP_NET_BIND_SERVICE, CAP_SETFCAP, CAP_SETGID, CAP_SETPCAP, and
CAP_SETUID capabilities are granted by default; this option can be used
to remove them.

If a capability is specified to both the **\--cap-add** and
**\--cap-drop** options, it is dropped, regardless of the order in which
the options were given.

#### **\--cert-dir**=*path*

Use certificates at *path* (\*.crt, \*.cert, \*.key) to connect to the
registry. (Default: /etc/containers/certs.d) For details, see
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--cgroup-parent**=*path*

Path to cgroups under which the cgroup for the container is created. If
the path is not absolute, the path is considered to be relative to the
cgroups path of the init process. Cgroups are created if they do not
already exist.

#### **\--cgroupns**=*how*

Sets the configuration for cgroup namespaces when handling `RUN`
instructions. The configured value can be \"\" (the empty string) or
\"private\" to indicate that a new cgroup namespace is created, or it
can be \"host\" to indicate that the cgroup namespace in which `buildah`
itself is being run is reused.

#### **\--compat-volumes**

Handle directories marked using the VOLUME instruction (both in this
build, and those inherited from base images) such that their contents
can only be modified by ADD and COPY instructions. Any changes made in
those locations by RUN instructions will be reverted. Before the
introduction of this option, this behavior was the default, but it is
now disabled by default.

#### **\--compress**

This option is added to be aligned with other containers CLIs. Podman
doesn\'t communicate with a daemon or a remote server. Thus, compressing
the data before sending it is irrelevant to Podman. (This option is not
available with the remote Podman client, including Mac and Windows
(excluding WSL2) machines)

#### **\--cpp-flag**=*flags*

Set additional flags to pass to the C Preprocessor cpp(1).
Containerfiles ending with a \".in\" suffix is preprocessed via cpp(1).
This option can be used to pass additional flags to cpp.Note: You can
also set default CPPFLAGS by setting the BUILDAH_CPPFLAGS environment
variable (e.g., export BUILDAH_CPPFLAGS=\"-DDEBUG\").

#### **\--cpu-period**=*limit*

Set the CPU period for the Completely Fair Scheduler (CFS), which is a
duration in microseconds. Once the container\'s CPU quota is used up, it
will not be scheduled to run until the current period ends. Defaults to
100000 microseconds.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-quota**=*limit*

Limit the CPU Completely Fair Scheduler (CFS) quota.

Limit the container\'s CPU usage. By default, containers run with the
full CPU resource. The limit is a number in microseconds. If a number is
provided, the container is allowed to use that much CPU time until the
CPU period ends (controllable via **\--cpu-period**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-shares**, **-c**=*shares*

CPU shares (relative weight).

By default, all containers get the same proportion of CPU cycles. This
proportion can be modified by changing the container\'s CPU share
weighting relative to the combined weight of all the running containers.
Default weight is **1024**.

The proportion only applies when CPU-intensive processes are running.
When tasks in one container are idle, other containers can use the
left-over CPU time. The actual amount of CPU time varies depending on
the number of containers running on the system.

For example, consider three containers, one has a cpu-share of 1024 and
two others have a cpu-share setting of 512. When processes in all three
containers attempt to use 100% of CPU, the first container receives 50%
of the total CPU time. If a fourth container is added with a cpu-share
of 1024, the first container only gets 33% of the CPU. The remaining
containers receive 16.5%, 16.5% and 33% of the CPU.

On a multi-core system, the shares of CPU time are distributed over all
CPU cores. Even if a container is limited to less than 100% of CPU time,
it can use 100% of each individual CPU core.

For example, consider a system with more than three cores. If the
container *C0* is started with **\--cpu-shares=512** running one
process, and another container *C1* with **\--cpu-shares=1024** running
two processes, this can result in the following division of CPU shares:

  PID   container   CPU   CPU share
  ----- ----------- ----- --------------
  100   C0          0     100% of CPU0
  101   C1          1     100% of CPU1
  102   C1          2     100% of CPU2

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-cpus**=*number*

CPUs in which to allow execution. Can be specified as a comma-separated
list (e.g. **0,1**), as a range (e.g. **0-3**), or any combination
thereof (e.g. **0-3,7,11-15**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-mems**=*nodes*

Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
effective on NUMA systems.

If there are four memory nodes on the system (0-3), use
**\--cpuset-mems=0,1** then processes in the container only uses memory
from the first two memory nodes.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--creds**=*\[username\[:password\]\]*

The \[username\[:password\]\] to use to authenticate with the registry,
if required. If one or both values are not supplied, a command line
prompt appears and the value can be entered. The password is entered
without echo.

Note that the specified credentials are only used to authenticate
against target registries. They are not used for mirrors or when the
registry gets rewritten (see `containers-registries.conf(5)`); to
authenticate against those consider using a `containers-auth.json(5)`
file.

#### **\--cw**=*options*

Produce an image suitable for use as a confidential workload running in
a trusted execution environment (TEE) using krun (i.e., *crun* built
with the libkrun feature enabled and invoked as *krun*). Instead of the
conventional contents, the root filesystem of the image will contain an
encrypted disk image and configuration information for krun.

The value for *options* is a comma-separated list of key=value pairs,
supplying configuration information which is needed for producing the
additional data which will be included in the container image.

Recognized *keys* are:

*attestation_url*: The location of a key broker / attestation server. If
a value is specified, the new image\'s workload ID, along with the
passphrase used to encrypt the disk image, will be registered with the
server, and the server\'s location will be stored in the container
image. At run-time, krun is expected to contact the server to retrieve
the passphrase using the workload ID, which is also stored in the
container image. If no value is specified, a *passphrase* value *must*
be specified.

*cpus*: The number of virtual CPUs which the image expects to be run
with at run-time. If not specified, a default value will be supplied.

*firmware_library*: The location of the libkrunfw-sev shared library. If
not specified, `buildah` checks for its presence in a number of
hard-coded locations.

*memory*: The amount of memory which the image expects to be run with at
run-time, as a number of megabytes. If not specified, a default value
will be supplied.

*passphrase*: The passphrase to use to encrypt the disk image which will
be included in the container image. If no value is specified, but an
*attestation_url* value is specified, a randomly-generated passphrase
will be used. The authors recommend setting an *attestation_url* but not
a *passphrase*.

*slop*: Extra space to allocate for the disk image compared to the size
of the container image\'s contents, expressed either as a percentage
(..%) or a size value (bytes, or larger units if suffixes like KB or MB
are present), or a sum of two or more such specifications. If not
specified, `buildah` guesses that 25% more space than the contents will
be enough, but this option is provided in case its guess is wrong.

*type*: The type of trusted execution environment (TEE) which the image
should be marked for use with. Accepted values are \"SEV\" (AMD Secure
Encrypted Virtualization - Encrypted State) and \"SNP\" (AMD Secure
Encrypted Virtualization - Secure Nested Paging). If not specified,
defaults to \"SNP\".

*workload_id*: A workload identifier which will be recorded in the
container image, to be used at run-time for retrieving the passphrase
which was used to encrypt the disk image. If not specified, a
semi-random value will be derived from the base image\'s image ID.

This option is not supported on the remote client, including Mac and
Windows (excluding WSL2) machines.

#### **\--decryption-key**=*key\[:passphrase\]*

The \[key\[:passphrase\]\] to be used for decryption of images. Key can
point to keys and/or certificates. Decryption is tried with all keys. If
the key is protected by a passphrase, it is required to be passed in the
argument and omitted otherwise.

#### **\--device**=*host-device\[:container-device\]\[:permissions\]*

Add a host device to the container. Optional *permissions* parameter can
be used to specify device permissions by combining **r** for read, **w**
for write, and **m** for **mknod**(2).

Example: **\--device=/dev/sdc:/dev/xvdc:rwm**.

Note: if *host-device* is a symbolic link then it is resolved first. The
container only stores the major and minor numbers of the host device.

Podman may load kernel modules required for using the specified device.
The devices that Podman loads modules for when necessary are: /dev/fuse.

In rootless mode, the new device is bind mounted in the container from
the host rather than Podman creating it within the container space.
Because the bind mount retains its SELinux label on SELinux systems, the
container can get permission denied when accessing the mounted device.
Modify SELinux settings to allow containers to use all device labels via
the following command:

\$ sudo setsebool -P container_use_devices=true

Note: if the user only has access rights via a group, accessing the
device from inside a rootless container fails. The
**[crun(1)](https://github.com/containers/crun/tree/main/crun.html)**
runtime offers a workaround for this by adding the option
**\--annotation run.oci.keep_original_groups=1**.

#### **\--disable-compression**, **-D**

Don\'t compress filesystem layers when building the image unless it is
required by the location where the image is being written. This is the
default setting, because image layers are compressed automatically when
they are pushed to registries, and images being written to local storage
only need to be decompressed again to be stored. Compression can be
forced in all cases by specifying **\--disable-compression=false**.

#### **\--disable-content-trust**

This is a Docker-specific option to disable image verification to a
container registry and is not supported by Podman. This option is a NOOP
and provided solely for scripting compatibility.

#### **\--dns**=*ipaddr*

Set custom DNS servers.

This option can be used to override the DNS configuration passed to the
container. Typically this is necessary when the host DNS configuration
is invalid for the container (e.g., **127.0.0.1**). When this is the
case the **\--dns** flag is necessary for every run.

The special value **none** can be specified to disable creation of
*/etc/resolv.conf* in the container by Podman. The */etc/resolv.conf*
file in the image is used without changes.

This option cannot be combined with **\--network** that is set to
**none**.

Note: this option takes effect only during *RUN* instructions in the
build. It does not affect */etc/resolv.conf* in the final image.

#### **\--dns-option**=*option*

Set custom DNS options to be used during the build.

#### **\--dns-search**=*domain*

Set custom DNS search domains to be used during the build.

#### **\--env**=*env\[=value\]*

Add a value (e.g. env=*value*) to the built image. Can be used multiple
times. If neither `=` nor a *value* are specified, but *env* is set in
the current environment, the value from the current environment is added
to the image. To remove an environment variable from the built image,
use the `--unsetenv` option.

#### **\--file**, **-f**=*Containerfile*

Specifies a Containerfile which contains instructions for building the
image, either a local file or an **http** or **https** URL. If more than
one Containerfile is specified, *FROM* instructions are only be accepted
from the last specified file.

If a build context is not specified, and at least one Containerfile is a
local file, the directory in which it resides is used as the build
context.

Specifying the option `-f -` causes the Containerfile contents to be
read from stdin.

#### **\--force-rm**

Always remove intermediate containers after a build, even if the build
fails (default true).

#### **\--format**

Control the format for the built image\'s manifest and configuration
data. Recognized formats include *oci* (OCI image-spec v1.0, the
default) and *docker* (version 2, using schema format 2 for the
manifest).

Note: You can also override the default format by setting the
BUILDAH_FORMAT environment variable. `export BUILDAH_FORMAT=docker`

#### **\--from**

Overrides the first `FROM` instruction within the Containerfile. If
there are multiple FROM instructions in a Containerfile, only the first
is changed.

With the remote podman client, not all container transports work as
expected. For example, oci-archive:/x.tar references /x.tar on the
remote machine instead of on the client. When using podman remote
clients it is best to restrict use to *containers-storage*, and
*docker:// transports*.

#### **\--group-add**=*group* \| *keep-groups*

Assign additional groups to the primary user running within the
container process.

-   `keep-groups` is a special flag that tells Podman to keep the
    supplementary group access.

Allows container to use the user\'s supplementary group access. If file
systems or devices are only accessible by the rootless user\'s group,
this flag tells the OCI runtime to pass the group access into the
container. Currently only available with the `crun` OCI runtime. Note:
`keep-groups` is exclusive, other groups cannot be specified with this
flag. (Not available for remote commands, including Mac and Windows
(excluding WSL2) machines)

#### **\--help**, **-h**

Print usage statement

#### **\--hooks-dir**=*path*

Each \*.json file in the path configures a hook for buildah build
containers. For more details on the syntax of the JSON files and the
semantics of hook injection. Buildah currently support both the 1.0.0
and 0.1.0 hook schemas, although the 0.1.0 schema is deprecated.

This option may be set multiple times; paths from later options have
higher precedence.

For the annotation conditions, buildah uses any annotations set in the
generated OCI configuration.

For the bind-mount conditions, only mounts explicitly requested by the
caller via \--volume are considered. Bind mounts that buildah inserts by
default (e.g. /dev/shm) are not considered.

If \--hooks-dir is unset for root callers, Buildah currently defaults to
/usr/share/containers/oci/hooks.d and /etc/containers/oci/hooks.d in
order of increasing precedence. Using these defaults is deprecated.
Migrate to explicitly setting \--hooks-dir.

#### **\--http-proxy**

By default proxy environment variables are passed into the container if
set for the Podman process. This can be disabled by setting the value to
**false**. The environment variables passed in include **http_proxy**,
**https_proxy**, **ftp_proxy**, **no_proxy**, and also the upper case
versions of those. This option is only needed when the host system must
use a proxy but the container does not use any proxy. Proxy environment
variables specified for the container in any other way overrides the
values that have been passed through from the host. (Other ways to
specify the proxy for the container include passing the values with the
**\--env** flag, or hard coding the proxy environment at container build
time.) When used with the remote client it uses the proxy environment
variables that are set on the server process.

Defaults to **true**.

#### **\--identity-label**

Adds default identity label `io.buildah.version` if set. (default true).

#### **\--ignorefile**

Path to an alternative .containerignore file.

#### **\--iidfile**=*ImageIDfile*

Write the built image\'s ID to the file. When `--platform` is specified
more than once, attempting to use this option triggers an error.

#### **\--ipc**=*how*

Sets the configuration for IPC namespaces when handling `RUN`
instructions. The configured value can be \"\" (the empty string) or
\"container\" to indicate that a new IPC namespace is created, or it can
be \"host\" to indicate that the IPC namespace in which `podman` itself
is being run is reused, or it can be the path to an IPC namespace which
is already in use by another process.

#### **\--isolation**=*type*

Controls what type of isolation is used for running processes as part of
`RUN` instructions. Recognized types include *oci* (OCI-compatible
runtime, the default), *rootless* (OCI-compatible runtime invoked using
a modified configuration and its \--rootless option enabled, with
*\--no-new-keyring \--no-pivot* added to its *create* invocation, with
network and UTS namespaces disabled, and IPC, PID, and user namespaces
enabled; the default for unprivileged users), and *chroot* (an internal
wrapper that leans more toward chroot(1) than container technology).

Note: You can also override the default isolation type by setting the
BUILDAH_ISOLATION environment variable. `export BUILDAH_ISOLATION=oci`

#### **\--jobs**=*number*

Run up to N concurrent stages in parallel. If the number of jobs is
greater than 1, stdin is read from /dev/null. If 0 is specified, then
there is no limit in the number of jobs that run in parallel.

#### **\--label**=*label*

Add an image *label* (e.g. label=*value*) to the image metadata. Can be
used multiple times.

Users can set a special LABEL
**io.containers.capabilities=CAP1,CAP2,CAP3** in a Containerfile that
specifies the list of Linux capabilities required for the container to
run properly. This label specified in a container image tells Podman to
run the container with just these capabilities. Podman launches the
container with just the specified capabilities, as long as this list of
capabilities is a subset of the default list.

If the specified capabilities are not in the default set, Podman prints
an error message and runs the container with the default capabilities.

#### **\--layer-label**=*label\[=value\]*

Add an intermediate image *label* (e.g. label=*value*) to the
intermediate image metadata. It can be used multiple times.

If *label* is named, but neither `=` nor a `value` is provided, then the
*label* is set to an empty value.

#### **\--layers**

Cache intermediate images during the build process (Default is `true`).

Note: You can also override the default value of layers by setting the
BUILDAH_LAYERS environment variable. `export BUILDAH_LAYERS=true`

#### **\--logfile**=*filename*

Log output which is sent to standard output and standard error to the
specified file instead of to standard output and standard error. This
option is not supported on the remote client, including Mac and Windows
(excluding WSL2) machines.

#### **\--logsplit**=*bool-value*

If `--logfile` and `--platform` are specified, the `--logsplit` option
allows end-users to split the log file for each platform into different
files in the following format:
`${logfile}_${platform-os}_${platform-arch}`. This option is not
supported on the remote client, including Mac and Windows (excluding
WSL2) machines.

#### **\--manifest**=*manifest*

Name of the manifest list to which the image is added. Creates the
manifest list if it does not exist. This option is useful for building
multi architecture images.

#### **\--memory**, **-m**=*number\[unit\]*

Memory limit. A *unit* can be **b** (bytes), **k** (kibibytes), **m**
(mebibytes), or **g** (gibibytes).

Allows the memory available to a container to be constrained. If the
host supports swap memory, then the **-m** memory setting can be larger
than physical RAM. If a limit of 0 is specified (not using **-m**), the
container\'s memory is not limited. The actual limit may be rounded up
to a multiple of the operating system\'s page size (the value is very
large, that\'s millions of trillions).

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swap**=*number\[unit\]*

A limit value equal to memory plus swap. A *unit* can be **b** (bytes),
**k** (kibibytes), **m** (mebibytes), or **g** (gibibytes).

Must be used with the **-m** (**\--memory**) flag. The argument value
must be larger than that of **-m** (**\--memory**) By default, it is set
to double the value of **\--memory**.

Set *number* to **-1** to enable unlimited swap.

This option is not supported on cgroups V1 rootless systems.

#### **\--network**=*mode*, **\--net**

Sets the configuration for network namespaces when handling `RUN`
instructions.

Valid *mode* values are:

-   **none**: no networking.
-   **host**: use the Podman host network stack. Note: the host mode
    gives the container full access to local system services such as
    D-bus and is therefore considered insecure.
-   **ns:**\_path\_: path to a network namespace to join.
-   **private**: create a new namespace for the container (default)
-   **\<network name\|ID\>**: Join the network with the given name or
    ID, e.g. use `--network mynet` to join the network with the name
    mynet. Only supported for rootful users.
-   **slirp4netns\[:OPTIONS,\...\]**: use **slirp4netns**(1) to create a
    user network stack. It is possible to specify these additional
    options, they can also be set with `network_cmd_options` in
    containers.conf:
    -   **allow_host_loopback=true\|false**: Allow slirp4netns to reach
        the host loopback IP (default is 10.0.2.2 or the second IP from
        slirp4netns cidr subnet when changed, see the cidr option
        below). The default is false.
    -   **mtu=MTU**: Specify the MTU to use for this network. (Default
        is `65520`).
    -   **cidr=CIDR**: Specify ip range to use for this network.
        (Default is `10.0.2.0/24`).
    -   **enable_ipv6=true\|false**: Enable IPv6. Default is true.
        (Required for `outbound_addr6`).
    -   **outbound_addr=INTERFACE**: Specify the outbound interface
        slirp binds to (ipv4 traffic only).
    -   **outbound_addr=IPv4**: Specify the outbound ipv4 address slirp
        binds to.
    -   **outbound_addr6=INTERFACE**: Specify the outbound interface
        slirp binds to (ipv6 traffic only).
    -   **outbound_addr6=IPv6**: Specify the outbound ipv6 address slirp
        binds to.
-   **pasta\[:OPTIONS,\...\]**: use **pasta**(1) to create a user-mode
    networking stack.\
    This is the default for rootless containers and only supported in
    rootless mode.\
    By default, IPv4 and IPv6 addresses and routes, as well as the pod
    interface name, are copied from the host. If port forwarding isn\'t
    configured, ports are forwarded dynamically as services are bound on
    either side (init namespace or container namespace). Port forwarding
    preserves the original source IP address. Options described in
    pasta(1) can be specified as comma-separated arguments.\
    In terms of pasta(1) options, **\--config-net** is given by default,
    in order to configure networking when the container is started, and
    **\--no-map-gw** is also assumed by default, to avoid direct access
    from container to host using the gateway address. The latter can be
    overridden by passing **\--map-gw** in the pasta-specific options
    (despite not being an actual pasta(1) option).\
    Also, **-t none** and **-u none** are passed to disable automatic
    port forwarding based on bound ports. Similarly, **-T none** and
    **-U none** are given to disable the same functionality from
    container to host.\
    Some examples:
    -   **pasta:\--map-gw**: Allow the container to directly reach the
        host using the gateway address.
    -   **pasta:\--mtu,1500**: Specify a 1500 bytes MTU for the *tap*
        interface in the container.
    -   **pasta:\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,-m,1500,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options: disable IPv6,
        assign `10.0.2.0/24` to the `tap0` interface in the container,
        with gateway `10.0.2.3`, enable DNS forwarder reachable at
        `10.0.2.3`, set MTU to 1500 bytes, disable NDP, DHCPv6 and DHCP
        support.
    -   **pasta:-I,tap0,\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options with Podman
        overrides: same as above, but leave the MTU to 65520 bytes
    -   **pasta:-t,auto,-u,auto,-T,auto,-U,auto**: enable automatic port
        forwarding based on observed bound ports from both host and
        container sides
    -   **pasta:-T,5201**: enable forwarding of TCP port 5201 from
        container to host, using the loopback interface instead of the
        tap interface for improved performance

#### **\--no-cache**

Do not use existing cached images for the container build. Build from
the start with a new set of cached layers.

#### **\--no-hostname**

Do not create the */etc/hostname* file in the container for RUN
instructions.

By default, Buildah manages the */etc/hostname* file, adding the
container\'s own hostname. When the **\--no-hostname** option is set,
the image\'s */etc/hostname* will be preserved unmodified if it exists.

#### **\--no-hosts**

Do not modify the `/etc/hosts` file in the container.

Podman assumes control over the container\'s `/etc/hosts` file by
default and adds entries for the container\'s name (see **\--name**
option) and hostname (see **\--hostname** option), the internal
`host.containers.internal` and `host.docker.internal` hosts, as well as
any hostname added using the **\--add-host** option. Refer to the
**\--add-host** option for details. Passing **\--no-hosts** disables
this, so that the image\'s `/etc/hosts` file is kept unmodified. The
same can be achieved globally by setting *no_hosts=true* in
`containers.conf`.

This option conflicts with **\--add-host**.

#### **\--omit-history**

Omit build history information in the built image. (default false).

This option is useful for the cases where end users explicitly want to
set `--omit-history` to omit the optional `History` from built images or
when working with images built using build tools that do not include
`History` information in their images.

#### **\--os**=*string*

Set the OS of the image to be built, and that of the base image to be
pulled, if the build uses one, instead of using the current operating
system of the build host. Unless overridden, subsequent lookups of the
same image in the local storage matches this OS, regardless of the host.

#### **\--os-feature**=*feature*

Set the name of a required operating system *feature* for the image
which is built. By default, if the image is not based on *scratch*, the
base image\'s required OS feature list is kept, if the base image
specified any. This option is typically only meaningful when the
image\'s OS is Windows.

If *feature* has a trailing `-`, then the *feature* is removed from the
set of required features which is listed in the image.

#### **\--os-version**=*version*

Set the exact required operating system *version* for the image which is
built. By default, if the image is not based on *scratch*, the base
image\'s required OS version is kept, if the base image specified one.
This option is typically only meaningful when the image\'s OS is
Windows, and is typically set in Windows base images, so using this
option is usually unnecessary.

#### **\--output**, **-o**=*output-opts*

Output destination (format: type=local,dest=path)

The \--output (or -o) option extends the default behavior of building a
container image by allowing users to export the contents of the image as
files on the local filesystem, which can be useful for generating local
binaries, code generation, etc. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

The value for \--output is a comma-separated sequence of key=value
pairs, defining the output type and options.

Supported *keys* are: - **dest**: Destination path for exported output.
Valid value is absolute or relative path, `-` means the standard
output. - **type**: Defines the type of output to be used. Valid values
is documented below.

Valid *type* values are: - **local**: write the resulting build files to
a directory on the client-side. - **tar**: write the resulting files as
a single tarball (.tar).

If no type is specified, the value defaults to **local**. Alternatively,
instead of a comma-separated sequence, the value of **\--output** can be
just a destination (in the **dest** format) (e.g. `--output some-path`,
`--output -`) where `--output some-path` is treated as if **type=local**
and `--output -` is treated as if **type=tar**.

#### **\--pid**=*pid*

Sets the configuration for PID namespaces when handling `RUN`
instructions. The configured value can be \"\" (the empty string) or
\"container\" to indicate that a new PID namespace is created, or it can
be \"host\" to indicate that the PID namespace in which `podman` itself
is being run is reused, or it can be the path to a PID namespace which
is already in use by another process.

#### **\--platform**=*os/arch\[/variant\]\[,\...\]*

Set the *os/arch* of the built image (and its base image, when using
one) to the provided value instead of using the current operating system
and architecture of the host (for example `linux/arm`). Unless
overridden, subsequent lookups of the same image in the local storage
matches this platform, regardless of the host.

If `--platform` is set, then the values of the `--arch`, `--os`, and
`--variant` options is overridden.

The `--platform` option can be specified more than once, or given a
comma-separated list of values as its argument. When more than one
platform is specified, the `--manifest` option is used instead of the
`--tag` option.

Os/arch pairs are those used by the Go Programming Language. In several
cases the *arch* value for a platform differs from one produced by other
tools such as the `arch` command. Valid OS and architecture name
combinations are listed as values for \$GOOS and \$GOARCH at
https://golang.org/doc/install/source#environment, and can also be found
by running `go tool dist list`.

While `podman build` is happy to use base images and build images for
any platform that exists, `RUN` instructions are unable to succeed
without the help of emulation provided by packages like
`qemu-user-static`.

#### **\--pull**=*policy*

Pull image policy. The default is **missing**.

-   **always**: Always pull the image and throw an error if the pull
    fails.
-   **missing**: Only pull the image when it does not exist in the local
    containers storage. Throw an error if no image is found and the pull
    fails.
-   **never**: Never pull the image but use the one from the local
    containers storage. Throw an error when no image is found.
-   **newer**: Pull if the image on the registry is newer than the one
    in the local containers storage. An image is considered to be newer
    when the digests are different. Comparing the time stamps is prone
    to errors. Pull errors are suppressed if a local image was found.

#### **\--quiet**, **-q**

Suppress output messages which indicate which instruction is being
processed, and of progress when pulling images from a registry, and when
writing the output image.

#### **\--retry**=*attempts*

Number of times to retry pulling or pushing images between the registry
and local storage in case of failure. Default is **3**.

#### **\--retry-delay**=*duration*

Duration of delay between retry attempts when pulling or pushing images
between the registry and local storage in case of failure. The default
is to start at two seconds and then exponentially back off. The delay is
used when this value is set, and no exponential back off occurs.

#### **\--rm**

Remove intermediate containers after a successful build (default true).

#### **\--runtime**=*path*

The *path* to an alternate OCI-compatible runtime, which is used to run
commands specified by the **RUN** instruction.

Note: You can also override the default runtime by setting the
BUILDAH_RUNTIME environment variable.
`export BUILDAH_RUNTIME=/usr/local/bin/runc`

#### **\--runtime-flag**=*flag*

Adds global flags for the container runtime. To list the supported
flags, please consult the manpages of the selected container runtime.

Note: Do not pass the leading \-- to the flag. To pass the runc flag
\--log-format json to buildah build, the option given is \--runtime-flag
log-format=json.

#### **\--sbom**=*preset*

Generate SBOMs (Software Bills Of Materials) for the output image by
scanning the working container and build contexts using the named
combination of scanner image, scanner commands, and merge strategy. Must
be specified with one or more of **\--sbom-image-output**,
**\--sbom-image-purl-output**, **\--sbom-output**, and
**\--sbom-purl-output**. Recognized presets, and the set of options
which they equate to:

-   \"syft\", \"syft-cyclonedx\":
    \--sbom-scanner-image=ghcr.io/anchore/syft
    \--sbom-scanner-command=\"/syft scan -q dir:{ROOTFS} \--output
    cyclonedx-json={OUTPUT}\" \--sbom-scanner-command=\"/syft scan -q
    dir:{CONTEXT} \--output cyclonedx-json={OUTPUT}\"
    \--sbom-merge-strategy=merge-cyclonedx-by-component-name-and-version
-   \"syft-spdx\": \--sbom-scanner-image=ghcr.io/anchore/syft
    \--sbom-scanner-command=\"/syft scan -q dir:{ROOTFS} \--output
    spdx-json={OUTPUT}\" \--sbom-scanner-command=\"/syft scan -q
    dir:{CONTEXT} \--output spdx-json={OUTPUT}\"
    \--sbom-merge-strategy=merge-spdx-by-package-name-and-versioninfo
-   \"trivy\", \"trivy-cyclonedx\":
    \--sbom-scanner-image=ghcr.io/aquasecurity/trivy
    \--sbom-scanner-command=\"trivy filesystem -q {ROOTFS} \--format
    cyclonedx \--output {OUTPUT}\" \--sbom-scanner-command=\"trivy
    filesystem -q {CONTEXT} \--format cyclonedx \--output {OUTPUT}\"
    \--sbom-merge-strategy=merge-cyclonedx-by-component-name-and-version
-   \"trivy-spdx\": \--sbom-scanner-image=ghcr.io/aquasecurity/trivy
    \--sbom-scanner-command=\"trivy filesystem -q {ROOTFS} \--format
    spdx-json \--output {OUTPUT}\" \--sbom-scanner-command=\"trivy
    filesystem -q {CONTEXT} \--format spdx-json \--output {OUTPUT}\"
    \--sbom-merge-strategy=merge-spdx-by-package-name-and-versioninfo

#### **\--sbom-image-output**=*path*

When generating SBOMs, store the generated SBOM in the specified path in
the output image. There is no default.

#### **\--sbom-image-purl-output**=*path*

When generating SBOMs, scan them for PURL ([package
URL](https://github.com/package-url/purl-spec/blob/master/PURL-SPECIFICATION.rst))
information, and save a list of found PURLs to the specified path in the
output image. There is no default.

#### **\--sbom-merge-strategy**=*method*

If more than one **\--sbom-scanner-command** value is being used, use
the specified method to merge the output from later commands with output
from earlier commands. Recognized values include:

-   cat Concatenate the files.
-   merge-cyclonedx-by-component-name-and-version Merge the
    \"component\" fields of JSON documents, ignoring values from
    documents when the combination of their \"name\" and \"version\"
    values is already present. Documents are processed in the order in
    which they are generated, which is the order in which the commands
    that generate them were specified.
-   merge-spdx-by-package-name-and-versioninfo Merge the \"package\"
    fields of JSON documents, ignoring values from documents when the
    combination of their \"name\" and \"versionInfo\" values is already
    present. Documents are processed in the order in which they are
    generated, which is the order in which the commands that generate
    them were specified.

#### **\--sbom-output**=*file*

When generating SBOMs, store the generated SBOM in the named file on the
local filesystem. There is no default.

#### **\--sbom-purl-output**=*file*

When generating SBOMs, scan them for PURL ([package
URL](https://github.com/package-url/purl-spec/blob/master/PURL-SPECIFICATION.rst))
information, and save a list of found PURLs to the named file in the
local filesystem. There is no default.

#### **\--sbom-scanner-command**=*image*

Generate SBOMs by running the specified command from the scanner image.
If multiple commands are specified, they are run in the order in which
they are specified. These text substitutions are performed: - {ROOTFS}
The root of the built image\'s filesystem, bind mounted. - {CONTEXT} The
build context and additional build contexts, bind mounted. - {OUTPUT}
The name of a temporary output file, to be read and merged with others
or copied elsewhere.

#### **\--sbom-scanner-image**=*image*

Generate SBOMs using the specified scanner image.

#### **\--secret**=**id=id,src=path**

Pass secret information used in the Containerfile for building images in
a safe way that are not stored in the final image, or be seen in other
stages. The secret is mounted in the container at the default location
of `/run/secrets/id`.

To later use the secret, use the \--mount option in a `RUN` instruction
within a `Containerfile`:

`RUN --mount=type=secret,id=mysecret cat /run/secrets/mysecret`

#### **\--security-opt**=*option*

Security Options

-   `apparmor=unconfined` : Turn off apparmor confinement for the
    container

-   `apparmor=alternate-profile` : Set the apparmor confinement profile
    for the container

-   `label=user:USER` : Set the label user for the container processes

-   `label=role:ROLE` : Set the label role for the container processes

-   `label=type:TYPE` : Set the label process type for the container
    processes

-   `label=level:LEVEL` : Set the label level for the container
    processes

-   `label=filetype:TYPE` : Set the label file type for the container
    files

-   `label=disable` : Turn off label separation for the container

-   `no-new-privileges` : Not supported

-   `seccomp=unconfined` : Turn off seccomp confinement for the
    container

-   `seccomp=profile.json` : JSON file to be used as the seccomp filter
    for the container.

#### **\--shm-size**=*number\[unit\]*

Size of */dev/shm*. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes). If the unit is omitted, the
system uses bytes. If the size is omitted, the default is **64m**. When
*size* is **0**, there is no limit on the amount of memory used for IPC
by the container. This option conflicts with **\--ipc=host**.

#### **\--sign-by**=*fingerprint*

Sign the image using a GPG key with the specified FINGERPRINT. (This
option is not available with the remote Podman client, including Mac and
Windows (excluding WSL2) machines,)

#### **\--skip-unused-stages**

Skip stages in multi-stage builds which don\'t affect the target stage.
(Default: **true**).

#### **\--squash**

Squash all of the image\'s new layers into a single new layer; any
preexisting layers are not squashed.

#### **\--squash-all**

Squash all of the new image\'s layers (including those inherited from a
base image) into a single new layer.

#### **\--ssh**=*default* \| *id\[=socket\>*

SSH agent socket or keys to expose to the build. The socket path can be
left empty to use the value of `default=$SSH_AUTH_SOCK`

To later use the ssh agent, use the \--mount option in a `RUN`
instruction within a `Containerfile`:

`RUN --mount=type=ssh,id=id mycmd`

#### **\--stdin**

Pass stdin into the RUN containers. Sometime commands being RUN within a
Containerfile want to request information from the user. For example apt
asking for a confirmation for install. Use \--stdin to be able to
interact from the terminal during the build.

#### **\--tag**, **-t**=*imageName*

Specifies the name which is assigned to the resulting image if the build
process completes successfully. If *imageName* does not include a
registry name, the registry name *localhost* is prepended to the image
name.

#### **\--target**=*stageName*

Set the target build stage to build. When building a Containerfile with
multiple build stages, \--target can be used to specify an intermediate
build stage by name as the final stage for the resulting image. Commands
after the target stage is skipped.

#### **\--timestamp**=*seconds*

Set the create timestamp to seconds since epoch to allow for
deterministic builds (defaults to current time). By default, the created
timestamp is changed and written into the image manifest with every
commit, causing the image\'s sha256 hash to be different even if the
sources are exactly the same otherwise. When \--timestamp is set, the
created timestamp is always set to the time specified and therefore not
changed, allowing the image\'s sha256 hash to remain the same. All files
committed to the layers of the image is created with the timestamp.

If the only instruction in a Containerfile is `FROM`, this flag has no
effect.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

#### **\--ulimit**=*type=soft-limit\[:hard-limit\]*

Specifies resource limits to apply to processes launched when processing
`RUN` instructions. This option can be specified multiple times.
Recognized resource types include: \"core\": maximum core dump size
(ulimit -c) \"cpu\": maximum CPU time (ulimit -t) \"data\": maximum size
of a process\'s data segment (ulimit -d) \"fsize\": maximum size of new
files (ulimit -f) \"locks\": maximum number of file locks (ulimit -x)
\"memlock\": maximum amount of locked memory (ulimit -l) \"msgqueue\":
maximum amount of data in message queues (ulimit -q) \"nice\": niceness
adjustment (nice -n, ulimit -e) \"nofile\": maximum number of open files
(ulimit -n) \"nproc\": maximum number of processes (ulimit -u) \"rss\":
maximum size of a process\'s (ulimit -m) \"rtprio\": maximum real-time
scheduling priority (ulimit -r) \"rttime\": maximum amount of real-time
execution between blocking syscalls \"sigpending\": maximum number of
pending signals (ulimit -i) \"stack\": maximum stack size (ulimit -s)

#### **\--unsetenv**=*env*

Unset environment variables from the final image.

#### **\--unsetlabel**=*label*

Unset the image label, causing the label not to be inherited from the
base image.

#### **\--userns**=*how*

Sets the configuration for user namespaces when handling `RUN`
instructions. The configured value can be \"\" (the empty string) or
\"container\" to indicate that a new user namespace is created, it can
be \"host\" to indicate that the user namespace in which `podman` itself
is being run is reused, or it can be the path to a user namespace which
is already in use by another process.

#### **\--userns-gid-map**=*mapping*

Directly specifies a GID mapping to be used to set ownership, at the
filesystem level, on the working container\'s contents. Commands run
when handling `RUN` instructions defaults to being run in their own user
namespaces, configured using the UID and GID maps.

Entries in this map take the form of one or more triples of a starting
in-container GID, a corresponding starting host-level GID, and the
number of consecutive IDs which the map entry represents.

This option overrides the *remap-gids* setting in the *options* section
of /etc/containers/storage.conf.

If this option is not specified, but a global \--userns-gid-map setting
is supplied, settings from the global option is used.

If none of \--userns-uid-map-user, \--userns-gid-map-group, or
\--userns-gid-map are specified, but \--userns-uid-map is specified, the
GID map is set to use the same numeric values as the UID map.

#### **\--userns-gid-map-group**=*group*

Specifies that a GID mapping to be used to set ownership, at the
filesystem level, on the working container\'s contents, can be found in
entries in the `/etc/subgid` file which correspond to the specified
group. Commands run when handling `RUN` instructions defaults to being
run in their own user namespaces, configured using the UID and GID maps.
If \--userns-uid-map-user is specified, but \--userns-gid-map-group is
not specified, `podman` assumes that the specified user name is also a
suitable group name to use as the default setting for this option.

**NOTE:** When this option is specified by a rootless user, the
specified mappings are relative to the rootless user namespace in the
container, rather than being relative to the host as it is when run
rootful.

#### **\--userns-uid-map**=*mapping*

Directly specifies a UID mapping to be used to set ownership, at the
filesystem level, on the working container\'s contents. Commands run
when handling `RUN` instructions default to being run in their own user
namespaces, configured using the UID and GID maps.

Entries in this map take the form of one or more triples of a starting
in-container UID, a corresponding starting host-level UID, and the
number of consecutive IDs which the map entry represents.

This option overrides the *remap-uids* setting in the *options* section
of /etc/containers/storage.conf.

If this option is not specified, but a global \--userns-uid-map setting
is supplied, settings from the global option is used.

If none of \--userns-uid-map-user, \--userns-gid-map-group, or
\--userns-uid-map are specified, but \--userns-gid-map is specified, the
UID map is set to use the same numeric values as the GID map.

#### **\--userns-uid-map-user**=*user*

Specifies that a UID mapping to be used to set ownership, at the
filesystem level, on the working container\'s contents, can be found in
entries in the `/etc/subuid` file which correspond to the specified
user. Commands run when handling `RUN` instructions defaults to being
run in their own user namespaces, configured using the UID and GID maps.
If \--userns-gid-map-group is specified, but \--userns-uid-map-user is
not specified, `podman` assumes that the specified group name is also a
suitable user name to use as the default setting for this option.

**NOTE:** When this option is specified by a rootless user, the
specified mappings are relative to the rootless user namespace in the
container, rather than being relative to the host as it is when run
rootful.

#### **\--uts**=*how*

Sets the configuration for UTS namespaces when handling `RUN`
instructions. The configured value can be \"\" (the empty string) or
\"container\" to indicate that a new UTS namespace to be created, or it
can be \"host\" to indicate that the UTS namespace in which `podman`
itself is being run is reused, or it can be the path to a UTS namespace
which is already in use by another process.

#### **\--variant**=*variant*

Set the architecture variant of the image to be built, and that of the
base image to be pulled, if the build uses one, to the provided value
instead of using the architecture variant of the build host.

#### **\--volume**, **-v**=*\[HOST-DIR:CONTAINER-DIR\[:OPTIONS\]\]*

Mount a host directory into containers when executing RUN instructions
during the build.

The `OPTIONS` are a comma-separated list and can be one or more of:

-   \[rw\|ro\]
-   \[z\|Z\|O\]
-   \[U\]
-   \[`[r]shared`\|`[r]slave`\|`[r]private`\]^[\[1\]](#Footnote1)^

The `CONTAINER-DIR` must be an absolute path such as `/src/docs`. The
`HOST-DIR` must be an absolute path as well. Podman bind-mounts the
`HOST-DIR` to the specified path when processing RUN instructions.

You can specify multiple **-v** options to mount one or more mounts.

You can add the `:ro` or `:rw` suffix to a volume to mount it read-only
or read-write mode, respectively. By default, the volumes are mounted
read-write. See examples.

`Chowning Volume Mounts`

By default, Podman does not change the owner and group of source volume
directories mounted. When running using user namespaces, the UID and GID
inside the namespace may correspond to another UID and GID on the host.

The `:U` suffix tells Podman to use the correct host UID and GID based
on the UID and GID within the namespace, to change recursively the owner
and group of the source volume.

**Warning** use with caution since this modifies the host filesystem.

`Labeling Volume Mounts`

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a container. Without a label, the security
system might prevent the processes running inside the container from
using the content. By default, Podman does not change the labels set by
the OS.

To change a label in the container context, add one of these two
suffixes `:z` or `:Z` to the volume mount. These suffixes tell Podman to
relabel file objects on the shared volumes. The `z` option tells Podman
that two containers share the volume content. As a result, Podman labels
the content with a shared content label. Shared volume labels allow all
containers to read/write content. The `Z` option tells Podman to label
the content with a private unshared label. Only the current container
can use a private volume.

Note: Do not relabel system files and directories. Relabeling system
content might cause other confined services on the host machine to fail.
For these types of containers, disabling SELinux separation is
recommended. The option `--security-opt label=disable` disables SELinux
separation for the container. For example, if a user wanted to volume
mount their entire home directory into the build containers, they need
to disable SELinux separation.

    $ podman build --security-opt label=disable -v $HOME:/home/user .

`Overlay Volume Mounts`

The `:O` flag tells Podman to mount the directory from the host as a
temporary storage using the Overlay file system. The `RUN` command
containers are allowed to modify contents within the mountpoint and are
stored in the container storage in a separate directory. In Overlay FS
terms the source directory is the lower, and the container storage
directory is the upper. Modifications to the mount point are destroyed
when the `RUN` command finishes executing, similar to a tmpfs mount
point.

Any subsequent execution of `RUN` commands sees the original source
directory content, any changes from previous RUN commands no longer
exists.

One use case of the `overlay` mount is sharing the package cache from
the host into the container to allow speeding up builds.

Note:

-   Overlay mounts are not currently supported in rootless mode.
-   The `O` flag is not allowed to be specified with the `Z` or `z`
    flags. Content mounted into the container is labeled with the
    private label. On SELinux systems, labels in the source directory
    needs to be readable by the container label. If not, SELinux
    container separation must be disabled for the container to work.
-   Modification of the directory volume mounted into the container with
    an overlay mount can cause unexpected failures. Do not modify the
    directory until the container finishes running.

By default bind mounted volumes are `private`. That means any mounts
done inside containers are not be visible on the host and vice versa.
This behavior can be changed by specifying a volume mount propagation
property.

When the mount propagation policy is set to `shared`, any mounts
completed inside the container on that volume is visible to both the
host and container. When the mount propagation policy is set to `slave`,
one way mount propagation is enabled and any mounts completed on the
host for that volume is visible only inside of the container. To control
the mount propagation property of volume use the `:[r]shared`,
`:[r]slave` or `:[r]private` propagation flag. For mount propagation to
work on the source mount point (mount point where source dir is mounted
on) has to have the right propagation properties. For shared volumes,
the source mount point has to be shared. And for slave volumes, the
source mount has to be either shared or slave. ^[\[1\]](#Footnote1)^

Use `df <source-dir>` to determine the source mount and then use
`findmnt -o TARGET,PROPAGATION <source-mount-dir>` to determine
propagation properties of source mount, if `findmnt` utility is not
available, the source mount point can be determined by looking at the
mount entry in `/proc/self/mountinfo`. Look at `optional fields` and see
if any propagation properties are specified. `shared:X` means the mount
is `shared`, `master:X` means the mount is `slave` and if nothing is
there that means the mount is `private`. ^[\[1\]](#Footnote1)^

To change propagation properties of a mount point use the `mount`
command. For example, to bind mount the source directory `/foo` do
`mount --bind /foo /foo` and `mount --make-private --make-shared /foo`.
This converts /foo into a `shared` mount point. The propagation
properties of the source mount can be changed directly. For instance if
`/` is the source mount for `/foo`, then use `mount --make-shared /` to
convert `/` into a `shared` mount.

##  EXAMPLES

### Build an image using local Containerfiles

Build image using Containerfile with content from current directory:

    $ podman build .

Build image using specified Containerfile with content from current
directory:

    $ podman build -f Containerfile.simple .

Build image using Containerfile from stdin with content from current
directory:

    $ cat $HOME/Containerfile | podman build -f - .

Build image using multiple Containerfiles with content from current
directory:

    $ podman build -f Containerfile.simple -f Containerfile.notsosimple .

Build image with specified Containerfile with content from \$HOME
directory. Note `cpp` is applied to Containerfile.in before processing
as Containerfile:

    $ podman build -f Containerfile.in $HOME

Build image with the specified tag with Containerfile and content from
current directory:

    $ podman build -t imageName .

Build image ignoring registry verification for any images pulled via the
Containerfile:

    $ podman build --tls-verify=false -t imageName .

Build image with the specified logging format:

    $ podman build --runtime-flag log-format=json .

Build image using debug mode for logging:

    $ podman build --runtime-flag debug .

Build image using specified registry attributes when pulling images from
the selected Containerfile:

    $ podman build --authfile /tmp/auths/myauths.json --cert-dir $HOME/auth --tls-verify=true --creds=username:password -t imageName -f Containerfile.simple .

Build image using specified resource controls when running containers
during the build:

    $ podman build --memory 40m --cpu-period 10000 --cpu-quota 50000 --ulimit nofile=1024:1028 -t imageName .

Build image using specified SELinux labels and cgroup config running
containers during the build:

    $ podman build --security-opt label=level:s0:c100,c200 --cgroup-parent /path/to/cgroup/parent -t imageName .

Build image with read-only and SELinux relabeled volume mounted from the
host into running containers during the build:

    $ podman build --volume /home/test:/myvol:ro,Z -t imageName .

Build image with overlay volume mounted from the host into running
containers during the build:

    $ podman build -v /var/lib/yum:/var/lib/yum:O -t imageName .

Build image using layers and then removing intermediate containers even
if the build fails.

    $ podman build --layers --force-rm -t imageName .

Build image ignoring cache and not removing intermediate containers even
if the build succeeds:

    $ podman build --no-cache --rm=false -t imageName .

Build image using the specified network when running containers during
the build:

    $ podman build --network mynet .

### Building a multi-architecture image using the \--manifest option (requires emulation software)

Build image using the specified architectures and link to a single
manifest on successful completion:

    $ podman build --arch arm --manifest myimage /tmp/mysrc
    $ podman build --arch amd64 --manifest myimage /tmp/mysrc
    $ podman build --arch s390x --manifest myimage /tmp/mysrc

Similarly build using a single command

    $ podman build --platform linux/s390x,linux/ppc64le,linux/amd64 --manifest myimage /tmp/mysrc

Build image using multiple specified architectures and link to single
manifest on successful completion:

    $ podman build --platform linux/arm64 --platform linux/amd64 --manifest myimage /tmp/mysrc

### Building an image using a URL, Git repo, or archive

The build context directory can be specified as a URL to a
Containerfile, a Git repository, or URL to an archive. If the URL is a
Containerfile, it is downloaded to a temporary location and used as the
context. When a Git repository is set as the URL, the repository is
cloned locally to a temporary location and then used as the context.
Lastly, if the URL is an archive, it is downloaded to a temporary
location and extracted before being used as the context.

#### Building an image using a URL to a Containerfile

Build image from Containerfile downloaded into temporary location used
as the build context:

    $ podman build https://10.10.10.1/podman/Containerfile

#### Building an image using a Git repository

Podman clones the specified GitHub repository to a temporary location
and uses it as the context. The Containerfile at the root of the
repository is used and it only works if the GitHub repository is a
dedicated repository.

Build image from specified git repository downloaded into temporary
location used as the build context:

    $ podman build -t hello  https://github.com/containers/PodmanHello.git
    $ podman run hello

Note: GitHub does not support using `git://` for performing `clone`
operation due to recent changes in their security guidance
(https://github.blog/2021-09-01-improving-git-protocol-security-github/).
Use an `https://` URL if the source repository is hosted on GitHub.

#### Building an image using a URL to an archive

Podman fetches the archive file, decompresses it, and uses its contents
as the build context. The Containerfile at the root of the archive and
the rest of the archive are used as the context of the build. Passing
the `-f PATH/Containerfile` option as well tells the system to look for
that file inside the contents of the archive.

    $ podman build -f dev/Containerfile https://10.10.10.1/podman/context.tar.gz

Note: supported compression formats are \'xz\', \'bzip2\', \'gzip\' and
\'identity\' (no compression).

##  Files

### .containerignore/.dockerignore

If the file *.containerignore* or *.dockerignore* exists in the context
directory, `podman build` reads its contents. Use the `--ignorefile`
option to override the .containerignore path location. Podman uses the
content to exclude files and directories from the context directory,
when executing COPY and ADD directives in the Containerfile/Dockerfile

The .containerignore and .dockerignore files use the same syntax; if
both are in the context directory, podman build only uses
.containerignore.

Users can specify a series of Unix shell globs in a .containerignore
file to identify files/directories to exclude.

Podman supports a special wildcard string `**` which matches any number
of directories (including zero). For example, \*\*/\*.go excludes all
files that end with .go that are found in all directories.

Example .containerignore file:

    # exclude this content for image
    */*.c
    **/output*
    src

`*/*.c` Excludes files and directories whose names ends with .c in any
top level subdirectory. For example, the source file include/rootless.c.

`**/output*` Excludes files and directories starting with `output` from
any directory.

`src` Excludes files named src and the directory src as well as any
content in it.

Lines starting with ! (exclamation mark) can be used to make exceptions
to exclusions. The following is an example .containerignore file that
uses this mechanism:

    *.doc
    !Help.doc

Exclude all doc files except Help.doc from the image.

This functionality is compatible with the handling of .containerignore
files described here:

https://github.com/containers/common/blob/main/docs/containerignore.5.md

**registries.conf** (`/etc/containers/registries.conf`)

registries.conf is the configuration file which specifies which
container registries is consulted when completing image names which do
not include a registry or domain portion.

##  Troubleshooting

### lastlog sparse file

Using a useradd command within a Containerfile with a large UID/GID
creates a large sparse file `/var/log/lastlog`. This can cause the build
to hang forever. Go language does not support sparse files correctly,
which can lead to some huge files being created in the container image.

When using the `useradd` command within the build script, pass the
`--no-log-init or -l` option to the `useradd` command. This option tells
useradd to stop creating the lastlog file.

##  SEE ALSO

**[podman(1)](podman.html)**,
**[buildah(1)](https://github.com/containers/buildah/blob/main/docs/buildah.html)**,
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**,
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**,
**[crun(1)](https://github.com/containers/crun/blob/main/crun.html)**,
**[runc(8)](https://github.com/opencontainers/runc/blob/main/man/runc.8.md)**,
**[useradd(8)](https://www.unix.com/man-page/redhat/8/useradd)**,
**[podman-ps(1)](podman-ps.html)**, **[podman-rm(1)](podman-rm.html)**,
**[Containerfile(5)](https://github.com/containers/common/blob/main/docs/Containerfile.5.md)**,
**[containerignore(5)](https://github.com/containers/common/blob/main/docs/containerignore.5.md)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

See
[podman-rootless(7)](https://github.com/containers/podman/blob/main/rootless.md)
for rootless issues.

##  HISTORY

Aug 2020, Additional options and .containerignore added by Dan Walsh
`<dwalsh@redhat.com>`

May 2018, Minor revisions added by Joe Doss `<joe@solidadmin.com>`

December 2017, Originally compiled by Tom Sweeney
`<tsweeney@redhat.com>`

##  FOOTNOTES

[1]{#Footnote1}: The Podman project is committed to inclusivity, a core
value of open source. The `master` and `slave` mount propagation
terminology used here is problematic and divisive, and needs to be
changed. However, these terms are currently used within the Linux kernel
and must be used as-is at this time. When the kernel maintainers rectify
this usage, Podman will follow suit immediately.


---

<a id='podman-commit'></a>

## podman-commit - Create new image based on the changed container

##  NAME

podman-commit - Create new image based on the changed container

##  SYNOPSIS

**podman commit** \[*options*\] *container* \[*image*\]

**podman container commit** \[*options*\] *container* \[*image*\]

##  DESCRIPTION

**podman commit** creates an image based on a changed *container*. The
author of the image can be set using the **\--author** OPTION. Various
image instructions can be configured with the **\--change** OPTION and a
commit message can be set using the **\--message** OPTION. The
*container* and its processes aren\'t paused while the image is
committed. If this is not desired, the **\--pause** OPTION can be set to
*true*. When the commit is complete, Podman prints out the ID of the new
image.

If `image` does not begin with a registry name component, `localhost` is
added to the name. If `image` is not provided, the values for the
`REPOSITORY` and `TAG` values of the created image is set to `<none>`.

##  OPTIONS

#### **\--author**, **-a**=*author*

Set the author for the committed image.

#### **\--change**, **-c**=*instruction*

Apply the following possible instructions to the created image:

-   *CMD*
-   *ENTRYPOINT*
-   *ENV*
-   *EXPOSE*
-   *LABEL*
-   *ONBUILD*
-   *STOPSIGNAL*
-   *USER*
-   *VOLUME*
-   *WORKDIR*

Can be set multiple times.

#### **\--config**=*ConfigBlobFile*

Merge the container configuration from the specified file into the
configuration for the image as it is being committed. The file contents
should be a JSON-encoded version of a Schema2Config structure, which is
defined at
https://github.com/containers/image/blob/v5.29.0/manifest/docker_schema2.go#L67.

#### **\--format**, **-f**=**oci** \| *docker*

Set the format of the image manifest and metadata. The currently
supported formats are **oci** and *docker*.\
The default is **oci**.

#### **\--iidfile**=*ImageIDfile*

Write the image ID to the file.

#### **\--include-volumes**

Include in the committed image any volumes added to the container by the
**\--volume** or **\--mount** OPTIONS to the **[podman
create](podman-create.html)** and **[podman run](podman-run.html)**
commands.\
The default is **false**.

#### **\--message**, **-m**=*message*

Set commit message for committed image.\
*IMPORTANT: The message field is not supported in `oci` format.*

#### **\--pause**, **-p**

Pause the container when creating an image.\
The default is **false**.

#### **\--quiet**, **-q**

Suppresses output.\
The default is **false**.

#### **\--squash**, **-s**

Squash newly built layers into a single new layer.\
The default is **false**.

##  EXAMPLES

Create image from container with entrypoint and label:

    $ podman commit --change CMD=/bin/bash --change ENTRYPOINT=/bin/sh --change "LABEL blue=image" reverent_golick image-committed
    Getting image source signatures
    Copying blob sha256:b41deda5a2feb1f03a5c1bb38c598cbc12c9ccd675f438edc6acd815f7585b86
     25.80 MB / 25.80 MB [======================================================] 0s
    Copying config sha256:c16a6d30f3782288ec4e7521c754acc29d37155629cb39149756f486dae2d4cd
     448 B / 448 B [============================================================] 0s
    Writing manifest to image destination
    Storing signatures
    e3ce4d93051ceea088d1c242624d659be32cf1667ef62f1d16d6b60193e2c7a8

Create image from container with commit message:

    $ podman commit -q --message "committing container to image"
    reverent_golick image-committed
    e3ce4d93051ceea088d1c242624d659be32cf1667ef62f1d16d6b60193e2c7a8

Create image from container with author:

    $ podman commit -q --author "firstName lastName" reverent_golick image-committed
    e3ce4d93051ceea088d1c242624d659be32cf1667ef62f1d16d6b60193e2c7a8

Pause running container while creating image:

    $ podman commit -q --pause=true containerID image-committed
    e3ce4d93051ceea088d1c242624d659be32cf1667ef62f1d16d6b60193e2c7a8

Create image from container with default image tag:

    $ podman commit containerID
    e3ce4d93051ceea088d1c242624d659be32cf1667ef62f1d16d6b60193e2c7a8

Create image from container with default required capabilities:

    $ podman commit -q --change LABEL=io.containers.capabilities=setuid,setgid epic_nobel privimage
    400d31a3f36dca751435e80a0e16da4859beb51ff84670ce6bdc5edb30b94066

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-run(1)](podman-run.html)**,
**[podman-create(1)](podman-create.html)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

##  HISTORY

December 2017, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-compose'></a>

## podman-compose - Run Compose workloads via an external compose
provider

##  NAME

podman-compose - Run Compose workloads via an external compose provider

##  SYNOPSIS

**podman compose** \[*options*\] \[*command* \[*arg* \...\]\]

##  DESCRIPTION

**podman compose** is a thin wrapper around an external compose provider
such as docker-compose or podman-compose. This means that
`podman compose` is executing another tool that implements the compose
functionality but sets up the environment in a way to let the compose
provider communicate transparently with the local Podman socket. The
specified options as well the command and argument are passed directly
to the compose provider.

The default compose providers are `docker-compose` and `podman-compose`.
If installed, `docker-compose` takes precedence since it is the original
implementation of the Compose specification and is widely used on the
supported platforms (i.e., Linux, Mac OS, Windows).

If you want to change the default behavior or have a custom installation
path for your provider of choice, please change the `compose_provider`
field in `containers.conf(5)`. You may also set the
`PODMAN_COMPOSE_PROVIDER` environment variable.

By default, `podman compose` will emit a warning saying that it executes
an external command. This warning can be disabled by setting
`compose_warning_logs` to false in `containers.conf(5)` or setting the
`PODMAN_COMPOSE_WARNING_LOGS` environment variable to false. See the man
page for `containers.conf(5)` for more information.

##  OPTIONS

To see supported options of the installed compose provider, please run
`podman compose --help`.

##  SEE ALSO

**[podman(1)](podman.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**


---

<a id='podman-container'></a>

## podman-container - Manage containers

##  NAME

podman-container - Manage containers

##  SYNOPSIS

**podman container** *subcommand*

##  DESCRIPTION

The container command allows management of containers

##  COMMANDS

  -------------------------------------------------------------------------------------------------------------------------
  Command      Man Page                                                             Description
  ------------ -------------------------------------------------------------------- ---------------------------------------
  attach       [podman-attach(1)](podman-attach.html)                               Attach to a running container.

  checkpoint   [podman-container-checkpoint(1)](podman-container-checkpoint.html)   Checkpoint one or more running
                                                                                    containers.

  cleanup      [podman-container-cleanup(1)](podman-container-cleanup.html)         Clean up the container\'s network and
                                                                                    mountpoints.

  clone        [podman-container-clone(1)](podman-container-clone.html)             Create a copy of an existing container.

  commit       [podman-commit(1)](podman-commit.html)                               Create new image based on the changed
                                                                                    container.

  cp           [podman-cp(1)](podman-cp.html)                                       Copy files/folders between a container
                                                                                    and the local filesystem.

  create       [podman-create(1)](podman-create.html)                               Create a new container.

  diff         [podman-container-diff(1)](podman-container-diff.html)               Inspect changes on a container\'s
                                                                                    filesystem

  exec         [podman-exec(1)](podman-exec.html)                                   Execute a command in a running
                                                                                    container.

  exists       [podman-container-exists(1)](podman-container-exists.html)           Check if a container exists in local
                                                                                    storage

  export       [podman-export(1)](podman-export.html)                               Export a container\'s filesystem
                                                                                    contents as a tar archive.

  init         [podman-init(1)](podman-init.html)                                   Initialize a container

  inspect      [podman-container-inspect(1)](podman-container-inspect.html)         Display a container\'s configuration.

  kill         [podman-kill(1)](podman-kill.html)                                   Kill the main process in one or more
                                                                                    containers.

  list         [podman-ps(1)](podman-ps.html)                                       List the containers on the
                                                                                    system.(alias ls)

  logs         [podman-logs(1)](podman-logs.html)                                   Display the logs of a container.

  mount        [podman-mount(1)](podman-mount.html)                                 Mount a working container\'s root
                                                                                    filesystem.

  pause        [podman-pause(1)](podman-pause.html)                                 Pause one or more containers.

  port         [podman-port(1)](podman-port.html)                                   List port mappings for the container.

  prune        [podman-container-prune(1)](podman-container-prune.html)             Remove all stopped containers from
                                                                                    local storage.

  ps           [podman-ps(1)](podman-ps.html)                                       Print out information about containers.

  rename       [podman-rename(1)](podman-rename.html)                               Rename an existing container.

  restart      [podman-restart(1)](podman-restart.html)                             Restart one or more containers.

  restore      [podman-container-restore(1)](podman-container-restore.html)         Restore one or more containers from a
                                                                                    checkpoint.

  rm           [podman-rm(1)](podman-rm.html)                                       Remove one or more containers.

  run          [podman-run(1)](podman-run.html)                                     Run a command in a container.

  runlabel     [podman-container-runlabel(1)](podman-container-runlabel.html)       Execute a command as described by a
                                                                                    container-image label.

  start        [podman-start(1)](podman-start.html)                                 Start one or more containers.

  stats        [podman-stats(1)](podman-stats.html)                                 Display a live stream of one or more
                                                                                    container\'s resource usage statistics.

  stop         [podman-stop(1)](podman-stop.html)                                   Stop one or more running containers.

  top          [podman-top(1)](podman-top.html)                                     Display the running processes of a
                                                                                    container.

  unmount      [podman-unmount(1)](podman-unmount.html)                             Unmount a working container\'s root
                                                                                    filesystem.(Alias unmount)

  unpause      [podman-unpause(1)](podman-unpause.html)                             Unpause one or more containers.

  update       [podman-update(1)](podman-update.html)                               Update the cgroup configuration of a
                                                                                    given container.

  wait         [podman-wait(1)](podman-wait.html)                                   Wait on one or more containers to stop
                                                                                    and print their exit codes.
  -------------------------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-exec(1)](podman-exec.html)**,
**[podman-run(1)](podman-run.html)**


---

<a id='podman-cp'></a>

## podman-cp - Copy files/folders between a container and the local
filesystem

##  NAME

podman-cp - Copy files/folders between a container and the local
filesystem

##  SYNOPSIS

**podman cp** \[*options*\] \[*container*:\]*src_path*
\[*container*:\]*dest_path*

**podman container cp** \[*options*\] \[*container*:\]*src_path*
\[*container*:\]*dest_path*

##  DESCRIPTION

**podman cp** allows copying the contents of **src_path** to the
**dest_path**. Files can be copied from a container to the local machine
and vice versa or between two containers. If `-` is specified for either
the `SRC_PATH` or `DEST_PATH`, one can also stream a tar archive from
`STDIN` or to `STDOUT`.

The containers can be either running or stopped and the *src_path* or
*dest_path* can be a file or directory.

*IMPORTANT: The **podman cp** command assumes container paths are
relative to the container\'s root directory (`/`), which means supplying
the initial forward slash is optional and therefore sees
`compassionate_darwin:/tmp/foo/myfile.txt` and
`compassionate_darwin:tmp/foo/myfile.txt` as identical.*

Local machine paths can be an absolute or relative value. The command
interprets a local machine\'s relative paths as relative to the current
working directory where **podman cp** is run.

Assuming a path separator of `/`, a first argument of **src_path** and
second argument of **dest_path**, the behavior is as follows:

**src_path** specifies a file: - **dest_path** does not exist - the file
is saved to a file created at **dest_path** (note that parent directory
must exist). - **dest_path** exists and is a file - the destination is
overwritten with the source file\'s contents. - **dest_path** exists and
is a directory - the file is copied into this directory using the base
name from **src_path**.

**src_path** specifies a directory: - **dest_path** does not exist -
**dest_path** is created as a directory and the contents of the source
directory are copied into this directory. - **dest_path** exists and is
a file - Error condition: cannot copy a directory to a file. -
**dest_path** exists and is a directory - **src_path** ends with `/` -
the source directory is copied into this directory. - **src_path** ends
with `/.` (i.e., slash followed by dot) - the content of the source
directory is copied into this directory.

The command requires **src_path** and **dest_path** to exist according
to the above rules.

If **src_path** is local and is a symbolic link, the symbolic target, is
copied by default.

A *colon* ( : ) is used as a delimiter between a container and its path,
it can also be used when specifying paths to a **src_path** or
**dest_path** on a local machine, for example, `file:name.txt`.

*IMPORTANT: while using a* colon\* ( : ) in a local machine path, one
must be explicit with a relative or absolute path, for example:
`/path/to/file:name.txt` or `./file:name.txt`\*

Using `-` as the **src_path** streams the contents of `STDIN` as a tar
archive. The command extracts the content of the tar to the `DEST_PATH`
in the container. In this case, **dest_path** must specify a directory.
Using `-` as the **dest_path** streams the contents of the resource (can
be a directory) as a tar archive to `STDOUT`.

Note that `podman cp` ignores permission errors when copying from a
running rootless container. The TTY devices inside a rootless container
are owned by the host\'s root user and hence cannot be read inside the
container\'s user namespace.

Further note that `podman cp` does not support globbing (e.g.,
`cp dir/*.txt`). To copy multiple files from the host to the container
use xargs(1) or find(1) (or similar tools for chaining commands) in
conjunction with `podman cp`. To copy multiple files from the container
to the host, use `podman mount CONTAINER` and operate on the returned
mount point instead (see ALTERNATIVES below).

##  OPTIONS

#### **\--archive**, **-a**

Archive mode (copy all UID/GID information). When set to true, files
copied to a container have changed ownership to the primary UID/GID of
the container. When set to false, maintain UID/GID from archive sources
instead of changing them to the primary UID/GID of the destination
container. The default is **true**.

#### **\--overwrite**

Allow directories to be overwritten with non-directories and vice versa.
By default, `podman cp` errors out when attempting to overwrite, for
instance, a regular file with a directory.

##  ALTERNATIVES

Podman has much stronger capabilities than just `podman cp` to achieve
copying files between the host and containers.

Using standard **[podman-mount(1)](podman-mount.html)** and
**[podman-unmount(1)](podman-unmount.html)** takes advantage of the
entire linux tool chain, rather than just cp.

copying contents out of a container or into a container, can be achieved
with a few simple commands. For example:

To copy the `/etc/foobar` directory out of a container and onto `/tmp`
on the host, the following commands can be executed:

    mnt=$(podman mount CONTAINERID)
    cp -R ${mnt}/etc/foobar /tmp
    podman umount CONTAINERID

To untar a tar ball into a container, following commands can be
executed:

    mnt=$(podman mount CONTAINERID)
    tar xf content.tgz -C ${mnt}
    podman umount CONTAINERID

To install a package into a container that does not have dnf installed,
following commands can be executed:

    mnt=$(podman mount CONTAINERID)
    dnf install --installroot=${mnt} httpd
    chroot ${mnt} rm -rf /var/log/dnf /var/cache/dnf
    podman umount CONTAINERID

By using `podman mount` and `podman unmount`, one can use all of the
standard linux tools for moving files into and out of containers, not
just the cp command.

##  EXAMPLES

Copy a file from the host to a container:

    podman cp /myapp/app.conf containerID:/myapp/app.conf

Copy a file from a container to a directory on another container:

    podman cp containerID1:/myfile.txt containerID2:/tmp

Copy a directory on a container to a directory on the host:

    podman cp containerID:/myapp/ /myapp/

Copy the contents of a directory on a container to a directory on the
host:

    podman cp containerID:/home/myuser/. /home/myuser/

Copy a directory on a container into a directory on another:

    podman cp containerA:/myapp containerB:/newapp

Stream a tar archive from `STDIN` to a container:

    podman cp - containerID:/myfiles.tar.gz < myfiles.tar.gz

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-mount(1)](podman-mount.html)**,
**[podman-unmount(1)](podman-unmount.html)**


---

<a id='podman-create'></a>

## podman-create - Create a new container

##  NAME

podman-create - Create a new container

##  SYNOPSIS

**podman create** \[*options*\] *image* \[*command* \[*arg* \...\]\]

**podman container create** \[*options*\] *image* \[*command* \[*arg*
\...\]\]

##  DESCRIPTION

Creates a writable container layer over the specified image and prepares
it for running the specified command. The container ID is then printed
to STDOUT. This is similar to **podman run -d** except the container is
never started. Use the **podman start** *container* command to start the
container at any point.

The initial status of the container created with **podman create** is
\'created\'.

Default settings for flags are defined in `containers.conf`. Most
settings for remote connections use the server\'s containers.conf,
except when documented in man pages.

##  IMAGE

The image is specified using transport:path format. If no transport is
specified, the `docker` (container registry) transport is used by
default. For remote Podman, including Mac and Windows (excluding WSL2)
machines, `docker` is the only allowed transport.

**dir:**\_path\_ An existing local directory *path* storing the
manifest, layer tarballs and signatures as individual files. This is a
non-standardized format, primarily useful for debugging or noninvasive
container inspection.

    $ podman save --format docker-dir fedora -o /tmp/fedora
    $ podman create dir:/tmp/fedora echo hello

**docker://**\_docker-reference\_ (Default) An image reference stored in
a remote container image registry. Example:
\"quay.io/podman/stable:latest\". The reference can include a path to a
specific registry; if it does not, the registries listed in
registries.conf is queried to find a matching image. By default,
credentials from `podman login` (stored at
\$XDG_RUNTIME_DIR/containers/auth.json by default) is used to
authenticate; otherwise it falls back to using credentials in
\$HOME/.docker/config.json.

    $ podman create registry.fedoraproject.org/fedora:latest echo hello

**docker-archive:**\_path\_\[**:**\_docker-reference\_\] An image stored
in the `docker save` formatted file. *docker-reference* is only used
when creating such a file, and it must not contain a digest.

    $ podman save --format docker-archive fedora -o /tmp/fedora
    $ podman create docker-archive:/tmp/fedora echo hello

**docker-daemon:**\_docker-reference\_ An image in *docker-reference*
format stored in the docker daemon internal storage. The
*docker-reference* can also be an image ID (docker-daemon:algo:digest).

    $ sudo docker pull fedora
    $ sudo podman create docker-daemon:docker.io/library/fedora echo hello

**oci-archive:**\_path\_**:**\_tag\_ An image in a directory compliant
with the \"Open Container Image Layout Specification\" at the specified
*path* and specified with a *tag*.

    $ podman save --format oci-archive fedora -o /tmp/fedora
    $ podman create oci-archive:/tmp/fedora echo hello

##  OPTIONS

#### **\--add-host**=*hostname\[;hostname\[;\...\]\]*:*ip*

Add a custom host-to-IP mapping to the container\'s `/etc/hosts` file.

The option takes one or multiple semicolon-separated hostnames to be
mapped to a single IPv4 or IPv6 address, separated by a colon. It can
also be used to overwrite the IP addresses of hostnames Podman adds to
`/etc/hosts` by default (also see the **\--name** and **\--hostname**
options). This option can be specified multiple times to add additional
mappings to `/etc/hosts`. It conflicts with the **\--no-hosts** option
and conflicts with *no_hosts=true* in `containers.conf`.

Instead of an IP address, the special flag *host-gateway* can be given.
This resolves to an IP address the container can use to connect to the
host. The IP address chosen depends on your network setup, thus there\'s
no guarantee that Podman can determine the *host-gateway* address
automatically, which will then cause Podman to fail with an error
message. You can overwrite this IP address using the
*host_containers_internal_ip* option in *containers.conf*.

The *host-gateway* address is also used by Podman to automatically add
the `host.containers.internal` and `host.docker.internal` hostnames to
`/etc/hosts`. You can prevent that by either giving the **\--no-hosts**
option, or by setting *host_containers_internal_ip=\"none\"* in
*containers.conf*. If no *host-gateway* address was configured manually
and Podman fails to determine the IP address automatically, Podman will
silently skip adding these internal hostnames to `/etc/hosts`. If Podman
is running in a virtual machine using `podman machine` (this includes
Mac and Windows hosts), Podman will silently skip adding the internal
hostnames to `/etc/hosts`, unless an IP address was configured manually;
the internal hostnames are resolved by the gvproxy DNS resolver instead.

Podman will use the `/etc/hosts` file of the host as a basis by default,
i.e. any hostname present in this file will also be present in the
`/etc/hosts` file of the container. A different base file can be
configured using the *base_hosts_file* config in `containers.conf`.

#### **\--annotation**=*key=value*

Add an annotation to the container. This option can be set multiple
times.

#### **\--arch**=*ARCH*

Override the architecture, defaults to hosts, of the image to be pulled.
For example, `arm`. Unless overridden, subsequent lookups of the same
image in the local storage matches this architecture, regardless of the
host.

#### **\--attach**, **-a**=*stdin* \| *stdout* \| *stderr*

Attach to STDIN, STDOUT or STDERR.

In foreground mode (the default when **-d** is not specified), **podman
run** can start the process in the container and attach the console to
the process\'s standard input, output, and error. It can even pretend to
be a TTY (this is what most command-line executables expect) and pass
along signals. The **-a** option can be set for each of **stdin**,
**stdout**, and **stderr**.

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--blkio-weight**=*weight*

Block IO relative weight. The *weight* is a value between **10** and
**1000**.

This option is not supported on cgroups V1 rootless systems.

#### **\--blkio-weight-device**=*device:weight*

Block IO relative device weight.

#### **\--cap-add**=*capability*

Add Linux capabilities.

#### **\--cap-drop**=*capability*

Drop Linux capabilities.

#### **\--cgroup-conf**=*KEY=VALUE*

When running on cgroup v2, specify the cgroup file to write to and its
value. For example **\--cgroup-conf=memory.high=1073741824** sets the
memory.high limit to 1GB.

#### **\--cgroup-parent**=*path*

Path to cgroups under which the cgroup for the container is created. If
the path is not absolute, the path is considered to be relative to the
cgroups path of the init process. Cgroups are created if they do not
already exist.

#### **\--cgroupns**=*mode*

Set the cgroup namespace mode for the container.

-   **host**: use the host\'s cgroup namespace inside the container.
-   **container:**\_id\_: join the namespace of the specified container.
-   **private**: create a new cgroup namespace.
-   **ns:**\_path\_: join the namespace at the specified path.

If the host uses cgroups v1, the default is set to **host**. On cgroups
v2, the default is **private**.

#### **\--cgroups**=*how*

Determines whether the container creates CGroups.

Default is **enabled**.

The **enabled** option creates a new cgroup under the cgroup-parent. The
**disabled** option forces the container to not create CGroups, and thus
conflicts with CGroup options (**\--cgroupns** and
**\--cgroup-parent**). The **no-conmon** option disables a new CGroup
only for the **conmon** process. The **split** option splits the current
CGroup in two sub-cgroups: one for conmon and one for the container
payload. It is not possible to set **\--cgroup-parent** with **split**.

#### **\--chrootdirs**=*path*

Path to a directory inside the container that is treated as a `chroot`
directory. Any Podman managed file (e.g., /etc/resolv.conf, /etc/hosts,
etc/hostname) that is mounted into the root directory is mounted into
that location as well. Multiple directories are separated with a comma.

#### **\--cidfile**=*file*

Write the container ID to *file*. The file is removed along with the
container, except when used with podman \--remote run on detached
containers.

#### **\--conmon-pidfile**=*file*

Write the pid of the **conmon** process to a file. As **conmon** runs in
a separate process than Podman, this is necessary when using systemd to
restart Podman containers. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines)

#### **\--cpu-period**=*limit*

Set the CPU period for the Completely Fair Scheduler (CFS), which is a
duration in microseconds. Once the container\'s CPU quota is used up, it
will not be scheduled to run until the current period ends. Defaults to
100000 microseconds.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-quota**=*limit*

Limit the CPU Completely Fair Scheduler (CFS) quota.

Limit the container\'s CPU usage. By default, containers run with the
full CPU resource. The limit is a number in microseconds. If a number is
provided, the container is allowed to use that much CPU time until the
CPU period ends (controllable via **\--cpu-period**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-rt-period**=*microseconds*

Limit the CPU real-time period in microseconds.

Limit the container\'s Real Time CPU usage. This option tells the kernel
to restrict the container\'s Real Time CPU usage to the period
specified.

This option is only supported on cgroups V1 rootful systems.

#### **\--cpu-rt-runtime**=*microseconds*

Limit the CPU real-time runtime in microseconds.

Limit the containers Real Time CPU usage. This option tells the kernel
to limit the amount of time in a given CPU period Real Time tasks may
consume. Ex: Period of 1,000,000us and Runtime of 950,000us means that
this container can consume 95% of available CPU and leave the remaining
5% to normal priority tasks.

The sum of all runtimes across containers cannot exceed the amount
allotted to the parent cgroup.

This option is only supported on cgroups V1 rootful systems.

#### **\--cpu-shares**, **-c**=*shares*

CPU shares (relative weight).

By default, all containers get the same proportion of CPU cycles. This
proportion can be modified by changing the container\'s CPU share
weighting relative to the combined weight of all the running containers.
Default weight is **1024**.

The proportion only applies when CPU-intensive processes are running.
When tasks in one container are idle, other containers can use the
left-over CPU time. The actual amount of CPU time varies depending on
the number of containers running on the system.

For example, consider three containers, one has a cpu-share of 1024 and
two others have a cpu-share setting of 512. When processes in all three
containers attempt to use 100% of CPU, the first container receives 50%
of the total CPU time. If a fourth container is added with a cpu-share
of 1024, the first container only gets 33% of the CPU. The remaining
containers receive 16.5%, 16.5% and 33% of the CPU.

On a multi-core system, the shares of CPU time are distributed over all
CPU cores. Even if a container is limited to less than 100% of CPU time,
it can use 100% of each individual CPU core.

For example, consider a system with more than three cores. If the
container *C0* is started with **\--cpu-shares=512** running one
process, and another container *C1* with **\--cpu-shares=1024** running
two processes, this can result in the following division of CPU shares:

  PID   container   CPU   CPU share
  ----- ----------- ----- --------------
  100   C0          0     100% of CPU0
  101   C1          1     100% of CPU1
  102   C1          2     100% of CPU2

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpus**=*number*

Number of CPUs. The default is *0.0* which means no limit. This is
shorthand for **\--cpu-period** and **\--cpu-quota**, therefore the
option cannot be specified with **\--cpu-period** or **\--cpu-quota**.

On some systems, changing the CPU limits may not be allowed for non-root
users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-cpus**=*number*

CPUs in which to allow execution. Can be specified as a comma-separated
list (e.g. **0,1**), as a range (e.g. **0-3**), or any combination
thereof (e.g. **0-3,7,11-15**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-mems**=*nodes*

Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
effective on NUMA systems.

If there are four memory nodes on the system (0-3), use
**\--cpuset-mems=0,1** then processes in the container only uses memory
from the first two memory nodes.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--decryption-key**=*key\[:passphrase\]*

The \[key\[:passphrase\]\] to be used for decryption of images. Key can
point to keys and/or certificates. Decryption is tried with all keys. If
the key is protected by a passphrase, it is required to be passed in the
argument and omitted otherwise.

#### **\--device**=*host-device\[:container-device\]\[:permissions\]*

Add a host device to the container. Optional *permissions* parameter can
be used to specify device permissions by combining **r** for read, **w**
for write, and **m** for **mknod**(2).

Example: **\--device=/dev/sdc:/dev/xvdc:rwm**.

Note: if *host-device* is a symbolic link then it is resolved first. The
container only stores the major and minor numbers of the host device.

Podman may load kernel modules required for using the specified device.
The devices that Podman loads modules for when necessary are: /dev/fuse.

In rootless mode, the new device is bind mounted in the container from
the host rather than Podman creating it within the container space.
Because the bind mount retains its SELinux label on SELinux systems, the
container can get permission denied when accessing the mounted device.
Modify SELinux settings to allow containers to use all device labels via
the following command:

\$ sudo setsebool -P container_use_devices=true

Note: if the user only has access rights via a group, accessing the
device from inside a rootless container fails. Use the
`--group-add keep-groups` flag to pass the user\'s supplementary group
access into the container.

#### **\--device-cgroup-rule**=*\"type major:minor mode\"*

Add a rule to the cgroup allowed devices list. The rule is expected to
be in the format specified in the Linux kernel documentation
[admin-guide/cgroup-v1/devices](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v1/devices.html): -
*type*: `a` (all), `c` (char), or `b` (block); - *major* and *minor*:
either a number, or `*` for all; - *mode*: a composition of `r` (read),
`w` (write), and `m` (mknod(2)).

#### **\--device-read-bps**=*path:rate*

Limit read rate (in bytes per second) from a device (e.g.
**\--device-read-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-read-iops**=*path:rate*

Limit read rate (in IO operations per second) from a device (e.g.
**\--device-read-iops=/dev/sda:1000**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-bps**=*path:rate*

Limit write rate (in bytes per second) to a device (e.g.
**\--device-write-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-iops**=*path:rate*

Limit write rate (in IO operations per second) to a device (e.g.
**\--device-write-iops=/dev/sda:1000**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--disable-content-trust**

This is a Docker-specific option to disable image verification to a
container registry and is not supported by Podman. This option is a NOOP
and provided solely for scripting compatibility.

#### **\--dns**=*ipaddr*

Set custom DNS servers.

This option can be used to override the DNS configuration passed to the
container. Typically this is necessary when the host DNS configuration
is invalid for the container (e.g., **127.0.0.1**). When this is the
case the **\--dns** flag is necessary for every run.

The special value **none** can be specified to disable creation of
*/etc/resolv.conf* in the container by Podman. The */etc/resolv.conf*
file in the image is used without changes.

This option cannot be combined with **\--network** that is set to
**none** or **container:**\_id\_.

#### **\--dns-option**=*option*

Set custom DNS options. Invalid if using **\--dns-option** with
**\--network** that is set to **none** or **container:**\_id\_.

#### **\--dns-search**=*domain*

Set custom DNS search domains. Invalid if using **\--dns-search** with
**\--network** that is set to **none** or **container:**\_id\_. Use
**\--dns-search=.** to remove the search domain.

#### **\--entrypoint**=*\"command\"* \| *\'\[\"command\", \"arg1\", \...\]\'*

Override the default ENTRYPOINT from the image.

The ENTRYPOINT of an image is similar to a COMMAND because it specifies
what executable to run when the container starts, but it is (purposely)
more difficult to override. The ENTRYPOINT gives a container its default
nature or behavior. When the ENTRYPOINT is set, the container runs as if
it were that binary, complete with default options. More options can be
passed in via the COMMAND. But, if a user wants to run something else
inside the container, the **\--entrypoint** option allows a new
ENTRYPOINT to be specified.

Specify multi option commands in the form of a json string.

#### **\--env**, **-e**=*env*

Set environment variables.

This option allows arbitrary environment variables that are available
for the process to be launched inside of the container. If an
environment variable is specified without a value, Podman checks the
host environment for a value and set the variable only if it is set on
the host. As a special case, if an environment variable ending in
\_\_\*\_\_ is specified without a value, Podman searches the host
environment for variables starting with the prefix and adds those
variables to the container.

See [**Environment**](#environment) note below for precedence and
examples.

#### **\--env-file**=*file*

Read in a line-delimited file of environment variables.

See [**Environment**](#environment) note below for precedence and
examples.

#### **\--env-host**

Use host environment inside of the container. See **Environment** note
below for precedence. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines)

#### **\--env-merge**=*env*

Preprocess default environment variables for the containers. For example
if image contains environment variable `hello=world` user can preprocess
it using `--env-merge hello=${hello}-some` so new value is
`hello=world-some`.

Please note that if the environment variable `hello` is not present in
the image, then it\'ll be replaced by an empty string and so using
`--env-merge hello=${hello}-some` would result in the new value of
`hello=-some`, notice the leading `-` delimiter.

#### **\--expose**=*port\[/protocol\]*

Expose a port or a range of ports (e.g. **\--expose=3300-3310**). The
protocol can be `tcp`, `udp` or `sctp` and if not given `tcp` is
assumed. This option matches the EXPOSE instruction for image builds and
has no effect on the actual networking rules unless
**-P/\--publish-all** is used to forward to all exposed ports from
random host ports. To forward specific ports from the host into the
container use the **-p/\--publish** option instead.

#### **\--gidmap**=*\[flags\]container_uid:from_uid\[:amount\]*

Run the container in a new user namespace using the supplied GID
mapping. This option conflicts with the **\--userns** and
**\--subgidname** options. This option provides a way to map host GIDs
to container GIDs in the same way as **\--uidmap** maps host UIDs to
container UIDs. For details see **\--uidmap**.

Note: the **\--gidmap** option cannot be called in conjunction with the
**\--pod** option as a gidmap cannot be set on the container level when
in a pod.

#### **\--gpus**=*ENTRY*

GPU devices to add to the container (\'all\' to pass all GPUs) Currently
only Nvidia devices are supported.

#### **\--group-add**=*group* \| *keep-groups*

Assign additional groups to the primary user running within the
container process.

-   `keep-groups` is a special flag that tells Podman to keep the
    supplementary group access.

Allows container to use the user\'s supplementary group access. If file
systems or devices are only accessible by the rootless user\'s group,
this flag tells the OCI runtime to pass the group access into the
container. Currently only available with the `crun` OCI runtime. Note:
`keep-groups` is exclusive, other groups cannot be specified with this
flag. (Not available for remote commands, including Mac and Windows
(excluding WSL2) machines)

#### **\--group-entry**=*ENTRY*

Customize the entry that is written to the `/etc/group` file within the
container when `--user` is used.

The variables \$GROUPNAME, \$GID, and \$USERLIST are automatically
replaced with their value at runtime if present.

#### **\--health-cmd**=*\"command\"* \| *\'\[\"command\", \"arg1\", \...\]\'*

Set or alter a healthcheck command for a container. The command is a
command to be executed inside the container that determines the
container health. The command is required for other healthcheck options
to be applied. A value of **none** disables existing healthchecks.

Multiple options can be passed in the form of a JSON array; otherwise,
the command is interpreted as an argument to **/bin/sh -c**.

#### **\--health-interval**=*interval*

Set an interval for the healthchecks. An *interval* of **disable**
results in no automatic timer setup. The default is **30s**.

#### **\--health-log-destination**=*directory_path*

Set the destination of the HealthCheck log. Directory path, local or
events_logger (local use container state file) (Default: local)

-   `local`: (default) HealthCheck logs are stored in overlay
    containers. (For example: `$runroot/healthcheck.log`)
-   `directory`: creates a log file named
    `<container-ID>-healthcheck.log` with HealthCheck logs in the
    specified directory.
-   `events_logger`: The log will be written with logging mechanism set
    by events_logger. It also saves the log to a default directory, for
    performance on a system with a large number of logs.

#### **\--health-max-log-count**=*number of stored logs*

Set maximum number of attempts in the HealthCheck log file. (\'0\' value
means an infinite number of attempts in the log file) (Default: 5
attempts)

#### **\--health-max-log-size**=*size of stored logs*

Set maximum length in characters of stored HealthCheck log. (\"0\" value
means an infinite log length) (Default: 500 characters)

#### **\--health-on-failure**=*action*

Action to take once the container transitions to an unhealthy state. The
default is **none**.

-   **none**: Take no action.
-   **kill**: Kill the container.
-   **restart**: Restart the container. Do not combine the `restart`
    action with the `--restart` flag. When running inside of a systemd
    unit, consider using the `kill` or `stop` action instead to make use
    of systemd\'s restart policy.
-   **stop**: Stop the container.

#### **\--health-retries**=*retries*

The number of retries allowed before a healthcheck is considered to be
unhealthy. The default value is **3**.

#### **\--health-start-period**=*period*

The initialization time needed for a container to bootstrap. The value
can be expressed in time format like **2m3s**. The default value is
**0s**.

Note: The health check command is executed as soon as a container is
started, if the health check is successful the container\'s health state
will be updated to `healthy`. However, if the health check fails, the
health state will stay as `starting` until either the health check is
successful or until the `--health-start-period` time is over. If the
health check command fails after the `--health-start-period` time is
over, the health state will be updated to `unhealthy`. The health check
command is executed periodically based on the value of
`--health-interval`.

#### **\--health-startup-cmd**=*\"command\"* \| *\'\[\"command\", \"arg1\", \...\]\'*

Set a startup healthcheck command for a container. This command is
executed inside the container and is used to gate the regular
healthcheck. When the startup command succeeds, the regular healthcheck
begins and the startup healthcheck ceases. Optionally, if the command
fails for a set number of attempts, the container is restarted. A
startup healthcheck can be used to ensure that containers with an
extended startup period are not marked as unhealthy until they are fully
started. Startup healthchecks can only be used when a regular
healthcheck (from the container\'s image or the **\--health-cmd**
option) is also set.

#### **\--health-startup-interval**=*interval*

Set an interval for the startup healthcheck. An *interval* of
**disable** results in no automatic timer setup. The default is **30s**.

#### **\--health-startup-retries**=*retries*

The number of attempts allowed before the startup healthcheck restarts
the container. If set to **0**, the container is never restarted. The
default is **0**.

#### **\--health-startup-success**=*retries*

The number of successful runs required before the startup healthcheck
succeeds and the regular healthcheck begins. A value of **0** means that
any success begins the regular healthcheck. The default is **0**.

#### **\--health-startup-timeout**=*timeout*

The maximum time a startup healthcheck command has to complete before it
is marked as failed. The value can be expressed in a time format like
**2m3s**. The default value is **30s**.

#### **\--health-timeout**=*timeout*

The maximum time allowed to complete the healthcheck before an interval
is considered failed. Like start-period, the value can be expressed in a
time format such as **1m22s**. The default value is **30s**.

#### **\--help**

Print usage statement

#### **\--hostname**, **-h**=*name*

Set the container\'s hostname inside the container.

This option can only be used with a private UTS namespace
`--uts=private` (default). If `--pod` is given and the pod shares the
same UTS namespace (default), the pod\'s hostname is used. The given
hostname is also added to the `/etc/hosts` file using the container\'s
primary IP address (also see the **\--add-host** option).

#### **\--hostuser**=*name*

Add a user account to /etc/passwd from the host to the container. The
Username or UID must exist on the host system.

#### **\--http-proxy**

By default proxy environment variables are passed into the container if
set for the Podman process. This can be disabled by setting the value to
**false**. The environment variables passed in include **http_proxy**,
**https_proxy**, **ftp_proxy**, **no_proxy**, and also the upper case
versions of those. This option is only needed when the host system must
use a proxy but the container does not use any proxy. Proxy environment
variables specified for the container in any other way overrides the
values that have been passed through from the host. (Other ways to
specify the proxy for the container include passing the values with the
**\--env** flag, or hard coding the proxy environment at container build
time.) When used with the remote client it uses the proxy environment
variables that are set on the server process.

Defaults to **true**.

#### **\--image-volume**=**bind** \| *tmpfs* \| *ignore*

Tells Podman how to handle the builtin image volumes. Default is
**bind**.

-   **bind**: An anonymous named volume is created and mounted into the
    container.
-   **tmpfs**: The volume is mounted onto the container as a tmpfs,
    which allows the users to create content that disappears when the
    container is stopped.
-   **ignore**: All volumes are just ignored and no action is taken.

#### **\--init**

Run an init inside the container that forwards signals and reaps
processes. The container-init binary is mounted at `/run/podman-init`.
Mounting over `/run` breaks container execution.

#### **\--init-ctr**=*type*

(Pods only). When using pods, create an init style container, which is
run after the infra container is started but before regular pod
containers are started. Init containers are useful for running setup
operations for the pod\'s applications.

Valid values for `init-ctr` type are *always* or *once*. The *always*
value means the container runs with each and every `pod start`, whereas
the *once* value means the container only runs once when the pod is
started and then the container is removed.

Init containers are only run on pod `start`. Restarting a pod does not
execute any init containers. Furthermore, init containers can only be
created in a pod when that pod is not running.

#### **\--init-path**=*path*

Path to the container-init binary.

#### **\--interactive**, **-i**

When set to **true**, make stdin available to the contained process. If
**false**, the stdin of the contained process is empty and immediately
closed.

If attached, stdin is piped to the contained process. If detached,
reading stdin will block until later attached.

**Caveat:** Podman will consume input from stdin as soon as it becomes
available, even if the contained process doesn\'t request it.

#### **\--ip**=*ipv4*

Specify a static IPv4 address for the container, for example
**10.88.64.128**. This option can only be used if the container is
joined to only a single network - i.e., **\--network=network-name** is
used at most once - and if the container is not joining another
container\'s network namespace via **\--network=container:*id***. The
address must be within the network\'s IP address pool (default
**10.88.0.0/16**).

To specify multiple static IP addresses per container, set multiple
networks using the **\--network** option with a static IP address
specified for each using the `ip` mode for that option.

#### **\--ip6**=*ipv6*

Specify a static IPv6 address for the container, for example
**fd46:db93:aa76:ac37::10**. This option can only be used if the
container is joined to only a single network - i.e.,
**\--network=network-name** is used at most once - and if the container
is not joining another container\'s network namespace via
**\--network=container:*id***. The address must be within the network\'s
IPv6 address pool.

To specify multiple static IPv6 addresses per container, set multiple
networks using the **\--network** option with a static IPv6 address
specified for each using the `ip6` mode for that option.

#### **\--ipc**=*ipc*

Set the IPC namespace mode for a container. The default is to create a
private IPC namespace.

-   \"\": Use Podman\'s default, defined in containers.conf.
-   **container:**\_id\_: reuses another container\'s shared memory,
    semaphores, and message queues
-   **host**: use the host\'s shared memory, semaphores, and message
    queues inside the container. Note: the host mode gives the container
    full access to local shared memory and is therefore considered
    insecure.
-   **none**: private IPC namespace, with /dev/shm not mounted.
-   **ns:**\_path\_: path to an IPC namespace to join.
-   **private**: private IPC namespace.
-   **shareable**: private IPC namespace with a possibility to share it
    with other containers.

#### **\--label**, **-l**=*key=value*

Add metadata to a container.

#### **\--label-file**=*file*

Read in a line-delimited file of labels.

#### **\--link-local-ip**=*ip*

Not implemented.

#### **\--log-driver**=*driver*

Logging driver for the container. Currently available options are
**k8s-file**, **journald**, **none**, **passthrough** and
**passthrough-tty**, with **json-file** aliased to **k8s-file** for
scripting compatibility. (Default **journald**).

The podman info command below displays the default log-driver for the
system.

    $ podman info --format '{{ .Host.LogDriver }}'
    journald

The **passthrough** driver passes down the standard streams (stdin,
stdout, stderr) to the container. It is not allowed with the remote
Podman client, including Mac and Windows (excluding WSL2) machines, and
on a tty, since it is vulnerable to attacks via TIOCSTI.

The **passthrough-tty** driver is the same as **passthrough** except
that it also allows it to be used on a TTY if the user really wants it.

#### **\--log-opt**=*name=value*

Logging driver specific options.

Set custom logging configuration. The following *name*s are supported:

**path**: specify a path to the log file (e.g. **\--log-opt
path=/var/log/container/mycontainer.json**);

**max-size**: specify a max size of the log file (e.g. **\--log-opt
max-size=10mb**);

**tag**: specify a custom log tag for the container (e.g. **\--log-opt
tag=\"{{.ImageName}}\"**. It supports the same keys as **podman inspect
\--format**. This option is currently supported only by the **journald**
log driver.

#### **\--mac-address**=*address*

Container network interface MAC address (e.g. 92:d0:c6:0a:29:33) This
option can only be used if the container is joined to only a single
network - i.e., **\--network=*network-name*** is used at most once - and
if the container is not joining another container\'s network namespace
via **\--network=container:*id***.

Remember that the MAC address in an Ethernet network must be unique. The
IPv6 link-local address is based on the device\'s MAC address according
to RFC4862.

To specify multiple static MAC addresses per container, set multiple
networks using the **\--network** option with a static MAC address
specified for each using the `mac` mode for that option.

#### **\--memory**, **-m**=*number\[unit\]*

Memory limit. A *unit* can be **b** (bytes), **k** (kibibytes), **m**
(mebibytes), or **g** (gibibytes).

Allows the memory available to a container to be constrained. If the
host supports swap memory, then the **-m** memory setting can be larger
than physical RAM. If a limit of 0 is specified (not using **-m**), the
container\'s memory is not limited. The actual limit may be rounded up
to a multiple of the operating system\'s page size (the value is very
large, that\'s millions of trillions).

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-reservation**=*number\[unit\]*

Memory soft limit. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes).

After setting memory reservation, when the system detects memory
contention or low memory, containers are forced to restrict their
consumption to their reservation. So always set the value below
**\--memory**, otherwise the hard limit takes precedence. By default,
memory reservation is the same as memory limit.

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swap**=*number\[unit\]*

A limit value equal to memory plus swap. A *unit* can be **b** (bytes),
**k** (kibibytes), **m** (mebibytes), or **g** (gibibytes).

Must be used with the **-m** (**\--memory**) flag. The argument value
must be larger than that of **-m** (**\--memory**) By default, it is set
to double the value of **\--memory**.

Set *number* to **-1** to enable unlimited swap.

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swappiness**=*number*

Tune a container\'s memory swappiness behavior. Accepts an integer
between *0* and *100*.

This flag is only supported on cgroups V1 rootful systems.

#### **\--mount**=*type=TYPE,TYPE-SPECIFIC-OPTION\[,\...\]*

Attach a filesystem mount to the container

Current supported mount TYPEs are **bind**, **devpts**, **glob**,
**image**, **ramfs**, **tmpfs** and **volume**.

Options common to all mount types:

-   *src*, *source*: mount source spec for **bind**, **glob**, and
    **volume**. Mandatory for **bind** and **glob**.

-   *dst*, *destination*, *target*: mount destination spec.

When source globs are specified without the destination directory, the
files and directories are mounted with their complete path within the
container. When the destination is specified, the files and directories
matching the glob on the base file name on the destination directory are
mounted. The option `type=glob,src=/foo*,destination=/tmp/bar` tells
container engines to mount host files matching /foo\* to the /tmp/bar/
directory in the container.

Options specific to type=**volume**:

-   *ro*, *readonly*: *true* or *false* (default if unspecified:
    *false*).

-   *U*, *chown*: *true* or *false* (default if unspecified: *false*).
    Recursively change the owner and group of the source volume based on
    the UID and GID of the container.

-   *idmap*: If specified, create an idmapped mount to the target user
    namespace in the container. The idmap option supports a custom
    mapping that can be different than the user namespace used by the
    container. The mapping can be specified after the idmap option like:
    `idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the
    first value is the start of the backing file system IDs that are
    mapped to the second value on the host. The length of this mapping
    is given in the third value. Multiple ranges are separated with #.
    If the specified mapping is prepended with a \'@\' then the mapping
    is considered relative to the container user namespace. The host ID
    for the mapping is changed to account for the relative position of
    the container user in the container user namespace.

Options specific to type=**image**:

-   *rw*, *readwrite*: *true* or *false* (default if unspecified:
    *false*).

-   *subpath*: Mount only a specific path within the image, instead of
    the whole image.

Options specific to **bind** and **glob**:

-   *ro*, *readonly*: *true* or *false* (default if unspecified:
    *false*).

-   *bind-propagation*: *shared*, *slave*, *private*, *unbindable*,
    *rshared*, *rslave*, *runbindable*, or **rprivate**
    (default).^[\[1\]](#Footnote1)^ See also mount(2).

-   *bind-nonrecursive*: do not set up a recursive bind mount. By
    default it is recursive.

-   *relabel*: *shared*, *private*.

-   *idmap*: *true* or *false* (default if unspecified: *false*). If
    true, create an idmapped mount to the target user namespace in the
    container.

-   *U*, *chown*: *true* or *false* (default if unspecified: *false*).
    Recursively change the owner and group of the source volume based on
    the UID and GID of the container.

-   *no-dereference*: do not dereference symlinks but copy the link
    source into the mount destination.

Options specific to type=**tmpfs** and **ramfs**:

-   *ro*, *readonly*: *true* or *false* (default if unspecified:
    *false*).

-   *tmpfs-size*: Size of the tmpfs/ramfs mount, in bytes. Unlimited by
    default in Linux.

-   *tmpfs-mode*: Octal file mode of the tmpfs/ramfs (e.g. 700 or
    0700.).

-   *tmpcopyup*: Enable copyup from the image directory at the same
    location to the tmpfs/ramfs. Used by default.

-   *notmpcopyup*: Disable copying files from the image to the
    tmpfs/ramfs.

-   *U*, *chown*: *true* or *false* (default if unspecified: *false*).
    Recursively change the owner and group of the source volume based on
    the UID and GID of the container.

Options specific to type=**devpts**:

-   *uid*: numeric UID of the file owner (default: 0).

-   *gid*: numeric GID of the file owner (default: 0).

-   *mode*: octal permission mask for the file (default: 600).

-   *max*: maximum number of PTYs (default: 1048576).

Examples:

-   `type=bind,source=/path/on/host,destination=/path/in/container`

-   `type=bind,src=/path/on/host,dst=/path/in/container,relabel=shared`

-   `type=bind,src=/path/on/host,dst=/path/in/container,relabel=shared,U=true`

-   `type=devpts,destination=/dev/pts`

-   `type=glob,src=/usr/lib/libfoo*,destination=/usr/lib,ro=true`

-   `type=image,source=fedora,destination=/fedora-image,rw=true`

-   `type=ramfs,tmpfs-size=512M,destination=/path/in/container`

-   `type=tmpfs,tmpfs-size=512M,destination=/path/in/container`

-   `type=tmpfs,destination=/path/in/container,noswap`

-   `type=volume,source=vol1,destination=/path/in/container,ro=true`

#### **\--name**=*name*

Assign a name to the container.

The operator can identify a container in three ways:

-   UUID long identifier
    ("f78375b1c487e03c9438c729345e54db9d20cfa2ac1fc3494b6eb60872e74778");
-   UUID short identifier ("f78375b1c487");
-   Name ("jonah").

Podman generates a UUID for each container, and if no name is assigned
to the container using **\--name**, Podman generates a random string
name. The name can be useful as a more human-friendly way to identify
containers. This works for both background and foreground containers.
The container\'s name is also added to the `/etc/hosts` file using the
container\'s primary IP address (also see the **\--add-host** option).

#### **\--network**=*mode*, **\--net**

Set the network mode for the container.

Valid *mode* values are:

-   **bridge\[:OPTIONS,\...\]**: Create a network stack on the default
    bridge. This is the default for rootful containers. It is possible
    to specify these additional options:

    -   **alias=**\_name\_: Add network-scoped alias for the container.
    -   **ip=**\_IPv4\_: Specify a static IPv4 address for this
        container.
    -   **ip6=**\_IPv6\_: Specify a static IPv6 address for this
        container.
    -   **mac=**\_MAC\_: Specify a static MAC address for this
        container.
    -   **interface_name=**\_name\_: Specify a name for the created
        network interface inside the container.

    For example, to set a static ipv4 address and a static mac address,
    use `--network bridge:ip=10.88.0.10,mac=44:33:22:11:00:99`.

-   *\<network name or ID\>***\[:OPTIONS,\...\]**: Connect to a
    user-defined network; this is the network name or ID from a network
    created by **[podman network create](podman-network-create.html)**.
    It is possible to specify the same options described under the
    bridge mode above. Use the **\--network** option multiple times to
    specify additional networks.\
    For backwards compatibility it is also possible to specify
    comma-separated networks on the first **\--network** argument,
    however this prevents you from using the options described under the
    bridge section above.

-   **none**: Create a network namespace for the container but do not
    configure network interfaces for it, thus the container has no
    network connectivity.

-   **container:**\_id\_: Reuse another container\'s network stack.

-   **host**: Do not create a network namespace, the container uses the
    host\'s network. Note: The host mode gives the container full access
    to local system services such as D-bus and is therefore considered
    insecure.

-   **ns:**\_path\_: Path to a network namespace to join.

-   **private**: Create a new namespace for the container. This uses the
    **bridge** mode for rootful containers and **slirp4netns** for
    rootless ones.

-   **slirp4netns\[:OPTIONS,\...\]**: use **slirp4netns**(1) to create a
    user network stack. It is possible to specify these additional
    options, they can also be set with `network_cmd_options` in
    containers.conf:

    -   **allow_host_loopback=true\|false**: Allow slirp4netns to reach
        the host loopback IP (default is 10.0.2.2 or the second IP from
        slirp4netns cidr subnet when changed, see the cidr option
        below). The default is false.
    -   **mtu=**\_MTU\_: Specify the MTU to use for this network.
        (Default is `65520`).
    -   **cidr=**\_CIDR\_: Specify ip range to use for this network.
        (Default is `10.0.2.0/24`).
    -   **enable_ipv6=true\|false**: Enable IPv6. Default is true.
        (Required for `outbound_addr6`).
    -   **outbound_addr=**\_INTERFACE\_: Specify the outbound interface
        slirp binds to (ipv4 traffic only).
    -   **outbound_addr=**\_IPv4\_: Specify the outbound ipv4 address
        slirp binds to.
    -   **outbound_addr6=**\_INTERFACE\_: Specify the outbound interface
        slirp binds to (ipv6 traffic only).
    -   **outbound_addr6=**\_IPv6\_: Specify the outbound ipv6 address
        slirp binds to.
    -   **port_handler=rootlesskit**: Use rootlesskit for port
        forwarding. Default.\
        Note: Rootlesskit changes the source IP address of incoming
        packets to an IP address in the container network namespace,
        usually `10.0.2.100`. If the application requires the real
        source IP address, e.g. web server logs, use the slirp4netns
        port handler. The rootlesskit port handler is also used for
        rootless containers when connected to user-defined networks.
    -   **port_handler=slirp4netns**: Use the slirp4netns port
        forwarding, it is slower than rootlesskit but preserves the
        correct source IP address. This port handler cannot be used for
        user-defined networks.

-   **pasta\[:OPTIONS,\...\]**: use **pasta**(1) to create a user-mode
    networking stack.\
    This is the default for rootless containers and only supported in
    rootless mode.\
    By default, IPv4 and IPv6 addresses and routes, as well as the pod
    interface name, are copied from the host. If port forwarding isn\'t
    configured, ports are forwarded dynamically as services are bound on
    either side (init namespace or container namespace). Port forwarding
    preserves the original source IP address. Options described in
    pasta(1) can be specified as comma-separated arguments.\
    In terms of pasta(1) options, **\--config-net** is given by default,
    in order to configure networking when the container is started, and
    **\--no-map-gw** is also assumed by default, to avoid direct access
    from container to host using the gateway address. The latter can be
    overridden by passing **\--map-gw** in the pasta-specific options
    (despite not being an actual pasta(1) option).\
    Also, **-t none** and **-u none** are passed if, respectively, no
    TCP or UDP port forwarding from host to container is configured, to
    disable automatic port forwarding based on bound ports. Similarly,
    **-T none** and **-U none** are given to disable the same
    functionality from container to host.\
    Some examples:

    -   **pasta:\--map-gw**: Allow the container to directly reach the
        host using the gateway address.
    -   **pasta:\--mtu,1500**: Specify a 1500 bytes MTU for the *tap*
        interface in the container.
    -   **pasta:\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,-m,1500,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options: disable IPv6,
        assign `10.0.2.0/24` to the `tap0` interface in the container,
        with gateway `10.0.2.3`, enable DNS forwarder reachable at
        `10.0.2.3`, set MTU to 1500 bytes, disable NDP, DHCPv6 and DHCP
        support.
    -   **pasta:-I,tap0,\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options with Podman
        overrides: same as above, but leave the MTU to 65520 bytes
    -   **pasta:-t,auto,-u,auto,-T,auto,-U,auto**: enable automatic port
        forwarding based on observed bound ports from both host and
        container sides
    -   **pasta:-T,5201**: enable forwarding of TCP port 5201 from
        container to host, using the loopback interface instead of the
        tap interface for improved performance

Invalid if using **\--dns**, **\--dns-option**, or **\--dns-search**
with **\--network** set to **none** or **container:**\_id\_.

If used together with **\--pod**, the container does not join the pod\'s
network namespace.

#### **\--network-alias**=*alias*

Add a network-scoped alias for the container, setting the alias for all
networks that the container joins. To set a name only for a specific
network, use the alias option as described under the **\--network**
option. If the network has DNS enabled
(`podman network inspect -f {{.DNSEnabled}} <name>`), these aliases can
be used for name resolution on the given network. This option can be
specified multiple times. NOTE: When using CNI a container only has
access to aliases on the first network that it joins. This limitation
does not exist with netavark/aardvark-dns.

#### **\--no-healthcheck**

Disable any defined healthchecks for container.

#### **\--no-hosts**

Do not modify the `/etc/hosts` file in the container.

Podman assumes control over the container\'s `/etc/hosts` file by
default and adds entries for the container\'s name (see **\--name**
option) and hostname (see **\--hostname** option), the internal
`host.containers.internal` and `host.docker.internal` hosts, as well as
any hostname added using the **\--add-host** option. Refer to the
**\--add-host** option for details. Passing **\--no-hosts** disables
this, so that the image\'s `/etc/hosts` file is kept unmodified. The
same can be achieved globally by setting *no_hosts=true* in
`containers.conf`.

This option conflicts with **\--add-host**.

#### **\--oom-kill-disable**

Whether to disable OOM Killer for the container or not.

This flag is not supported on cgroups V2 systems.

#### **\--oom-score-adj**=*num*

Tune the host\'s OOM preferences for containers (accepts values from
**-1000** to **1000**).

When running in rootless mode, the specified value can\'t be lower than
the oom_score_adj for the current process. In this case, the
oom-score-adj is clamped to the current process value.

#### **\--os**=*OS*

Override the OS, defaults to hosts, of the image to be pulled. For
example, `windows`. Unless overridden, subsequent lookups of the same
image in the local storage matches this OS, regardless of the host.

#### **\--passwd-entry**=*ENTRY*

Customize the entry that is written to the `/etc/passwd` file within the
container when `--passwd` is used.

The variables \$USERNAME, \$UID, \$GID, \$NAME, \$HOME are automatically
replaced with their value at runtime.

#### **\--personality**=*persona*

Personality sets the execution domain via Linux personality(2).

#### **\--pid**=*mode*

Set the PID namespace mode for the container. The default is to create a
private PID namespace for the container.

-   **container:**\_id\_: join another container\'s PID namespace;
-   **host**: use the host\'s PID namespace for the container. Note the
    host mode gives the container full access to local PID and is
    therefore considered insecure;
-   **ns:**\_path\_: join the specified PID namespace;
-   **private**: create a new namespace for the container (default).

#### **\--pidfile**=*path*

When the pidfile location is specified, the container process\' PID is
written to the pidfile. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines) If
the pidfile option is not specified, the container process\' PID is
written to
/run/containers/storage/[*storage* − *driver* − *containers*/]{.math
.inline}CID/userdata/pidfile.

After the container is started, the location for the pidfile can be
discovered with the following `podman inspect` command:

    $ podman inspect --format '{{ .PidFile }}' $CID
    /run/containers/storage/${storage-driver}-containers/$CID/userdata/pidfile

#### **\--pids-limit**=*limit*

Tune the container\'s pids limit. Set to **-1** to have unlimited pids
for the container. The default is **2048** on systems that support
\"pids\" cgroup controller.

#### **\--platform**=*OS/ARCH*

Specify the platform for selecting the image. (Conflicts with \--arch
and \--os) The `--platform` option can be used to override the current
architecture and operating system. Unless overridden, subsequent lookups
of the same image in the local storage matches this platform, regardless
of the host.

#### **\--pod**=*name*

Run container in an existing pod. Podman makes the pod automatically if
the pod name is prefixed with **new:**. To make a pod with more granular
options, use the **podman pod create** command before creating a
container. When a container is run with a pod with an infra-container,
the infra-container is started first.

#### **\--pod-id-file**=*file*

Run container in an existing pod and read the pod\'s ID from the
specified *file*. When a container is run within a pod which has an
infra-container, the infra-container starts first.

#### **\--privileged**

Give extended privileges to this container. The default is **false**.

By default, Podman containers are unprivileged (**=false**) and cannot,
for example, modify parts of the operating system. This is because by
default a container is only allowed limited access to devices. A
\"privileged\" container is given the same access to devices as the user
launching the container, with the exception of virtual consoles
(*/dev/tty*) when running in systemd mode (**\--systemd=always**).

A privileged container turns off the security features that isolate the
container from the host. Dropped Capabilities, limited devices,
read-only mount points, Apparmor/SELinux separation, and Seccomp filters
are all disabled. Due to the disabled security features, the privileged
field should almost never be set as containers can easily break out of
confinement.

Containers running in a user namespace (e.g., rootless containers)
cannot have more privileges than the user that launched them.

#### **\--publish**, **-p**=*\[\[ip:\]\[hostPort\]:\]containerPort\[/protocol\]*

Publish a container\'s port, or range of ports, to the host.

Both *hostPort* and *containerPort* can be specified as a range of
ports. When specifying ranges for both, the number of container ports in
the range must match the number of host ports in the range.

If host IP is set to 0.0.0.0 or not set at all, the port is bound on all
IPs on the host.

By default, Podman publishes TCP ports. To publish a UDP port instead,
give `udp` as protocol. To publish both TCP and UDP ports, set
`--publish` twice, with `tcp`, and `udp` as protocols respectively.
Rootful containers can also publish ports using the `sctp` protocol.

Host port does not have to be specified (e.g.
`podman run -p 127.0.0.1::80`). If it is not, the container port is
randomly assigned a port on the host.

Use **podman port** to see the actual mapping:
`podman port $CONTAINER $CONTAINERPORT`.

Note that the network drivers `macvlan` and `ipvlan` do not support port
forwarding, it will have no effect on these networks.

**Note:** If a container runs within a pod, it is not necessary to
publish the port for the containers in the pod. The port must only be
published by the pod itself. Pod network stacks act like the network
stack on the host - when there are a variety of containers in the pod,
and programs in the container, all sharing a single interface and IP
address, and associated ports. If one container binds to a port, no
other container can use that port within the pod while it is in use.
Containers in the pod can also communicate over localhost by having one
container bind to localhost in the pod, and another connect to that
port.

#### **\--publish-all**, **-P**

Publish all exposed ports to random ports on the host interfaces. The
default is **false**.

When set to **true**, publish all exposed ports to the host interfaces.
If the operator uses **-P** (or **-p**) then Podman makes the exposed
port accessible on the host and the ports are available to any client
that can reach the host.

When using this option, Podman binds any exposed port to a random port
on the host within an ephemeral port range defined by
*/proc/sys/net/ipv4/ip_local_port_range*. To find the mapping between
the host ports and the exposed ports, use **podman port**.

#### **\--pull**=*policy*

Pull image policy. The default is **missing**.

-   **always**: Always pull the image and throw an error if the pull
    fails.
-   **missing**: Pull the image only when the image is not in the local
    containers storage. Throw an error if no image is found and the pull
    fails.
-   **never**: Never pull the image but use the one from the local
    containers storage. Throw an error if no image is found.
-   **newer**: Pull if the image on the registry is newer than the one
    in the local containers storage. An image is considered to be newer
    when the digests are different. Comparing the time stamps is prone
    to errors. Pull errors are suppressed if a local image was found.

#### **\--quiet**, **-q**

Suppress output information when pulling images

#### **\--rdt-class**=*intel-rdt-class-of-service*

Rdt-class sets the class of service (CLOS or COS) for the container to
run in. Based on the Cache Allocation Technology (CAT) feature that is
part of Intel\'s Resource Director Technology (RDT) feature set, all
container processes will run within the pre-configured COS, representing
a part of the cache. The COS has to be created and configured using a
pseudo file system (usually mounted at `/sys/fs/resctrl`) that the
resctrl kernel driver provides. Assigning the container to a COS
requires root privileges and thus doesn\'t work in a rootless
environment. Currently, the feature is only supported using `runc` as a
runtime. See <https://docs.kernel.org/arch/x86/resctrl.html> for more
details on creating a COS before a container can be assigned to it.

#### **\--read-only**

Mount the container\'s root filesystem as read-only.

By default, container root filesystems are writable, allowing processes
to write files anywhere. By specifying the **\--read-only** flag, the
containers root filesystem are mounted read-only prohibiting any writes.

#### **\--read-only-tmpfs**

When running \--read-only containers, mount a read-write tmpfs on
*/dev*, */dev/shm*, */run*, */tmp*, and */var/tmp*. The default is
**true**.

  ------------------------------------------------------------------------------
  \--read-only   \--read-only-tmpfs   /     /run, /tmp, /var/tmp
  -------------- -------------------- ----- ------------------------------------
  true           true                 r/o   r/w

  true           false                r/o   r/o

  false          false                r/w   r/w

  false          true                 r/w   r/w
  ------------------------------------------------------------------------------

When **\--read-only=true** and **\--read-only-tmpfs=true** additional
tmpfs are mounted on the /tmp, /run, and /var/tmp directories.

When **\--read-only=true** and **\--read-only-tmpfs=false** /dev and
/dev/shm are marked Read/Only and no tmpfs are mounted on /tmp, /run and
/var/tmp. The directories are exposed from the underlying image, meaning
they are read-only by default. This makes the container totally
read-only. No writable directories exist within the container. In this
mode writable directories need to be added via external volumes or
mounts.

By default, when **\--read-only=false**, the /dev and /dev/shm are
read/write, and the /tmp, /run, and /var/tmp are read/write directories
from the container image.

#### **\--replace**

If another container with the same name already exists, replace and
remove it. The default is **false**.

#### **\--requires**=*container*

Specify one or more requirements. A requirement is a dependency
container that is started before this container. Containers can be
specified by name or ID, with multiple containers being separated by
commas.

#### **\--restart**=*policy*

Restart policy to follow when containers exit. Restart policy does not
take effect if a container is stopped via the **podman kill** or
**podman stop** commands.

Valid *policy* values are:

-   `no` : Do not restart containers on exit
-   `never` : Synonym for **no**; do not restart containers on exit
-   `on-failure[:max_retries]` : Restart containers when they exit with
    a non-zero exit code, retrying indefinitely or until the optional
    *max_retries* count is hit
-   `always` : Restart containers when they exit, regardless of status,
    retrying indefinitely
-   `unless-stopped` : Identical to **always**

Podman provides a systemd unit file, podman-restart.service, which
restarts containers after a system reboot.

When running containers in systemd services, use the restart
functionality provided by systemd. In other words, do not use this
option in a container unit, instead set the `Restart=` systemd directive
in the `[Service]` section. See **podman-systemd.unit**(5) and
**systemd.service**(5).

#### **\--retry**=*attempts*

Number of times to retry pulling or pushing images between the registry
and local storage in case of failure. Default is **3**.

#### **\--retry-delay**=*duration*

Duration of delay between retry attempts when pulling or pushing images
between the registry and local storage in case of failure. The default
is to start at two seconds and then exponentially back off. The delay is
used when this value is set, and no exponential back off occurs.

#### **\--rm**

Automatically remove the container and any anonymous unnamed volume
associated with the container when it exits. The default is **false**.

#### **\--rootfs**

If specified, the first argument refers to an exploded container on the
file system.

This is useful to run a container without requiring any image
management, the rootfs of the container is assumed to be managed
externally.

`Overlay Rootfs Mounts`

The `:O` flag tells Podman to mount the directory from the rootfs path
as storage using the `overlay file system`. The container processes can
modify content within the mount point which is stored in the container
storage in a separate directory. In overlay terms, the source directory
is the lower, and the container storage directory is the upper.
Modifications to the mount point are destroyed when the container
finishes executing, similar to a tmpfs mount point being unmounted.

Note: On **SELinux** systems, the rootfs needs the correct label, which
is by default **unconfined_u:object_r:container_file_t:s0**.

`idmap`

If `idmap` is specified, create an idmapped mount to the target user
namespace in the container. The idmap option supports a custom mapping
that can be different than the user namespace used by the container. The
mapping can be specified after the idmap option like:
`idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the first
value is the start of the backing file system IDs that are mapped to the
second value on the host. The length of this mapping is given in the
third value. Multiple ranges are separated with #.

#### **\--sdnotify**=**container** \| *conmon* \| *healthy* \| *ignore*

Determines how to use the NOTIFY_SOCKET, as passed with systemd and
Type=notify.

Default is **container**, which means allow the OCI runtime to proxy the
socket into the container to receive ready notification. Podman sets the
MAINPID to conmon\'s pid. The **conmon** option sets MAINPID to
conmon\'s pid, and sends READY when the container has started. The
socket is never passed to the runtime or the container. The **healthy**
option sets MAINPID to conmon\'s pid, and sends READY when the container
has turned healthy; requires a healthcheck to be set. The socket is
never passed to the runtime or the container. The **ignore** option
removes NOTIFY_SOCKET from the environment for itself and child
processes, for the case where some other process above Podman uses
NOTIFY_SOCKET and Podman does not use it.

#### **\--seccomp-policy**=*policy*

Specify the policy to select the seccomp profile. If set to *image*,
Podman looks for a \"io.containers.seccomp.profile\" label in the
container-image config and use its value as a seccomp profile.
Otherwise, Podman follows the *default* policy by applying the default
profile unless specified otherwise via *\--security-opt seccomp* as
described below.

Note that this feature is experimental and may change in the future.

#### **\--secret**=*secret\[,opt=opt \...\]*

Give the container access to a secret. Can be specified multiple times.

A secret is a blob of sensitive data which a container needs at runtime
but is not stored in the image or in source control, such as usernames
and passwords, TLS certificates and keys, SSH keys or other important
generic strings or binary content (up to 500 kb in size).

When secrets are specified as type `mount`, the secrets are copied and
mounted into the container when a container is created. When secrets are
specified as type `env`, the secret is set as an environment variable
within the container. Secrets are written in the container at the time
of container creation, and modifying the secret using `podman secret`
commands after the container is created affects the secret inside the
container.

Secrets and its storage are managed using the `podman secret` command.

Secret Options

-   `type=mount|env` : How the secret is exposed to the container.
    `mount` mounts the secret into the container as a file. `env`
    exposes the secret as an environment variable. Defaults to `mount`.
-   `target=target` : Target of secret. For mounted secrets, this is the
    path to the secret inside the container. If a fully qualified path
    is provided, the secret is mounted at that location. Otherwise, the
    secret is mounted to `/run/secrets/target` for linux containers or
    `/var/run/secrets/target` for freebsd containers. If the target is
    not set, the secret is mounted to `/run/secrets/secretname` by
    default. For env secrets, this is the environment variable key.
    Defaults to `secretname`.
-   `uid=0` : UID of secret. Defaults to 0. Mount secret type only.
-   `gid=0` : GID of secret. Defaults to 0. Mount secret type only.
-   `mode=0` : Mode of secret. Defaults to 0444. Mount secret type only.

Examples

Mount at `/my/location/mysecret` with UID 1:

    --secret mysecret,target=/my/location/mysecret,uid=1

Mount at `/run/secrets/customtarget` with mode 0777:

    --secret mysecret,target=customtarget,mode=0777

Create a secret environment variable called `ENVSEC`:

    --secret mysecret,type=env,target=ENVSEC

#### **\--security-opt**=*option*

Security Options

-   **apparmor=unconfined** : Turn off apparmor confinement for the
    container

-   **apparmor**=*alternate-profile* : Set the apparmor confinement
    profile for the container

-   **label=user:**\_USER\_: Set the label user for the container
    processes

-   **label=role:**\_ROLE\_: Set the label role for the container
    processes

-   **label=type:**\_TYPE\_: Set the label process type for the
    container processes

-   **label=level:**\_LEVEL\_: Set the label level for the container
    processes

-   **label=filetype:**\_TYPE\_: Set the label file type for the
    container files

-   **label=disable**: Turn off label separation for the container

Note: Labeling can be disabled for all containers by setting label=false
in the **containers.conf** (`/etc/containers/containers.conf` or
`$HOME/.config/containers/containers.conf`) file.

-   **label=nested**: Allows SELinux modifications within the container.
    Containers are allowed to modify SELinux labels on files and
    processes, as long as SELinux policy allows. Without **nested**,
    containers view SELinux as disabled, even when it is enabled on the
    host. Containers are prevented from setting any labels.

-   **mask**=*/path/1:/path/2*: The paths to mask separated by a colon.
    A masked path cannot be accessed inside the container.

-   **no-new-privileges**: Disable container processes from gaining
    additional privileges.

-   **seccomp=unconfined**: Turn off seccomp confinement for the
    container.

-   **seccomp=profile.json**: JSON file to be used as a seccomp filter.
    Note that the `io.podman.annotations.seccomp` annotation is set with
    the specified value as shown in `podman inspect`.

-   **proc-opts**=*OPTIONS* : Comma-separated list of options to use for
    the /proc mount. More details for the possible mount options are
    specified in the **proc(5)** man page.

-   **unmask**=*ALL* or */path/1:/path/2*, or shell expanded paths
    (/proc/\*): Paths to unmask separated by a colon. If set to **ALL**,
    it unmasks all the paths that are masked or made read-only by
    default. The default masked paths are **/proc/acpi, /proc/kcore,
    /proc/keys, /proc/latency_stats, /proc/sched_debug, /proc/scsi,
    /proc/timer_list, /proc/timer_stats, /sys/firmware, and
    /sys/fs/selinux**, **/sys/devices/virtual/powercap**. The default
    paths that are read-only are **/proc/asound**, **/proc/bus**,
    **/proc/fs**, **/proc/irq**, **/proc/sys**, **/proc/sysrq-trigger**,
    **/sys/fs/cgroup**.

Note: Labeling can be disabled for all containers by setting
**label=false** in the **containers.conf**(5) file.

#### **\--shm-size**=*number\[unit\]*

Size of */dev/shm*. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes). If the unit is omitted, the
system uses bytes. If the size is omitted, the default is **64m**. When
*size* is **0**, there is no limit on the amount of memory used for IPC
by the container. This option conflicts with **\--ipc=host**.

#### **\--shm-size-systemd**=*number\[unit\]*

Size of systemd-specific tmpfs mounts such as /run, /run/lock,
/var/log/journal and /tmp. A *unit* can be **b** (bytes), **k**
(kibibytes), **m** (mebibytes), or **g** (gibibytes). If the unit is
omitted, the system uses bytes. If the size is omitted, the default is
**64m**. When *size* is **0**, the usage is limited to 50% of the
host\'s available memory.

#### **\--stop-signal**=*signal*

Signal to stop a container. Default is **SIGTERM**.

#### **\--stop-timeout**=*seconds*

Timeout to stop a container. Default is **10**. Remote connections use
local containers.conf for defaults.

#### **\--subgidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subgid* file. If running rootless, the user needs to have the
right to use the mapping. See **subgid**(5). This flag conflicts with
**\--userns** and **\--gidmap**.

#### **\--subuidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subuid* file. If running rootless, the user needs to have the
right to use the mapping. See **subuid**(5). This flag conflicts with
**\--userns** and **\--uidmap**.

#### **\--sysctl**=*name=value*

Configure namespaced kernel parameters at runtime.

For the IPC namespace, the following sysctls are allowed:

-   kernel.msgmax
-   kernel.msgmnb
-   kernel.msgmni
-   kernel.sem
-   kernel.shmall
-   kernel.shmmax
-   kernel.shmmni
-   kernel.shm_rmid_forced
-   Sysctls beginning with fs.mqueue.\*

Note: if using the **\--ipc=host** option, the above sysctls are not
allowed.

For the network namespace, only sysctls beginning with net.\* are
allowed.

Note: if using the **\--network=host** option, the above sysctls are not
allowed.

#### **\--systemd**=*true* \| *false* \| *always*

Run container in systemd mode. The default is **true**.

-   **true** enables systemd mode only when the command executed inside
    the container is *systemd*, */usr/sbin/init*, */sbin/init* or
    */usr/local/sbin/init*.

-   **false** disables systemd mode.

-   **always** enforces the systemd mode to be enabled.

Running the container in systemd mode causes the following changes:

-   Podman mounts tmpfs file systems on the following directories
    -   */run*
    -   */run/lock*
    -   */tmp*
    -   */sys/fs/cgroup/systemd* (on a cgroup v1 system)
    -   */var/lib/journal*
-   Podman sets the default stop signal to **SIGRTMIN+3**.
-   Podman sets **container_uuid** environment variable in the container
    to the first 32 characters of the container ID.
-   Podman does not mount virtual consoles (*/dev/tty*) when running
    with **\--privileged**.
-   On cgroup v2, */sys/fs/cgroup* is mounted writeable.

This allows systemd to run in a confined container without any
modifications.

Note that on **SELinux** systems, systemd attempts to write to the
cgroup file system. Containers writing to the cgroup file system are
denied by default. The **container_manage_cgroup** boolean must be
enabled for this to be allowed on an SELinux separated system.

    setsebool -P container_manage_cgroup true

#### **\--timeout**=*seconds*

Maximum time a container is allowed to run before conmon sends it the
kill signal. By default containers run until they exit or are stopped by
`podman stop`.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

#### **\--tmpfs**=*fs*

Create a tmpfs mount.

Mount a temporary filesystem (**tmpfs**) mount into a container, for
example:

    $ podman create -d --tmpfs /tmp:rw,size=787448k,mode=1777 my_image

This command mounts a **tmpfs** at */tmp* within the container. The
supported mount options are the same as the Linux default mount flags.
If no options are specified, the system uses the following options:
**rw,noexec,nosuid,nodev**.

#### **\--tty**, **-t**

Allocate a pseudo-TTY. The default is **false**.

When set to **true**, Podman allocates a pseudo-tty and attach to the
standard input of the container. This can be used, for example, to run a
throwaway interactive shell.

**NOTE**: The \--tty flag prevents redirection of standard output. It
combines STDOUT and STDERR, it can insert control characters, and it can
hang pipes. This option is only used when run interactively in a
terminal. When feeding input to Podman, use -i only, not -it.

#### **\--tz**=*timezone*

Set timezone in container. This flag takes area-based timezones, GMT
time, as well as `local`, which sets the timezone in the container to
match the host machine. See `/usr/share/zoneinfo/` for valid timezones.
Remote connections use local containers.conf for defaults

#### **\--uidmap**=*\[flags\]container_uid:from_uid\[:amount\]*

Run the container in a new user namespace using the supplied UID
mapping. This option conflicts with the **\--userns** and
**\--subuidname** options. This option provides a way to map host UIDs
to container UIDs. It can be passed several times to map different
ranges.

The possible values of the optional *flags* are discussed further down
on this page. The *amount* value is optional and assumed to be **1** if
not given.

The *from_uid* value is based upon the user running the command, either
rootful or rootless users.

-   rootful user: \[*flags*\]*container_uid*:*host_uid*\[:*amount*\]

-   rootless user:
    \[*flags*\]*container_uid*:*intermediate_uid*\[:*amount*\]

    `Rootful mappings`

When **podman create** is called by a privileged user, the option
**\--uidmap** works as a direct mapping between host UIDs and container
UIDs.

host UID -\> container UID

The *amount* specifies the number of consecutive UIDs that is mapped. If
for example *amount* is **4** the mapping looks like:

  host UID         container UID
  ---------------- ---------------------
  *from_uid*       *container_uid*
  *from_uid* + 1   *container_uid* + 1
  *from_uid* + 2   *container_uid* + 2
  *from_uid* + 3   *container_uid* + 3

`Rootless mappings`

When **podman create** is called by an unprivileged user (i.e. running
rootless), the value *from_uid* is interpreted as an \"intermediate
UID\". In the rootless case, host UIDs are not mapped directly to
container UIDs. Instead the mapping happens over two mapping steps:

host UID -\> intermediate UID -\> container UID

The **\--uidmap** option only influences the second mapping step.

The first mapping step is derived by Podman from the contents of the
file */etc/subuid* and the UID of the user calling Podman.

First mapping step:

  host UID              intermediate UID
  --------------------- ------------------
  UID for Podman user   0
  1st subordinate UID   1
  2nd subordinate UID   2
  3rd subordinate UID   3
  nth subordinate UID   n

To be able to use intermediate UIDs greater than zero, the user needs to
have subordinate UIDs configured in */etc/subuid*. See **subuid**(5).

The second mapping step is configured with **\--uidmap**.

If for example *amount* is **5** the second mapping step looks like:

  intermediate UID   container UID
  ------------------ ---------------------
  *from_uid*         *container_uid*
  *from_uid* + 1     *container_uid* + 1
  *from_uid* + 2     *container_uid* + 2
  *from_uid* + 3     *container_uid* + 3
  *from_uid* + 4     *container_uid* + 4

When running as rootless, Podman uses all the ranges configured in the
*/etc/subuid* file.

The current user ID is mapped to UID=0 in the rootless user namespace.
Every additional range is added sequentially afterward:

  ----------------------------------------------------------------------------------------------------------
  host         rootless user namespace                        length
  ------------ ---------------------------------------------- ----------------------------------------------
  \$UID        0                                              1

  1            \$FIRST_RANGE_ID                               [*FIRST*~*R*~*ANGE*~*L*~*ENGTH*\|\|1+]{.math
                                                              .inline}FIRST_RANGE_LENGTH
  ----------------------------------------------------------------------------------------------------------

`Referencing a host ID from the parent namespace`

As a rootless user, the given host ID in **\--uidmap** or **\--gidmap**
is mapped from the *intermediate namespace* generated by Podman.
Sometimes it is desirable to refer directly at the *host namespace*. It
is possible to manually do so, by running
`podman unshare cat /proc/self/gid_map`, finding the desired host id at
the second column of the output, and getting the corresponding
intermediate id from the first column.

Podman can perform all that by preceding the host id in the mapping with
the `@` symbol. For instance, by specifying `--gidmap 100000:@2000:1`,
podman will look up the intermediate id corresponding to host id `2000`
and it will map the found intermediate id to the container id `100000`.
The given host id must have been subordinated (otherwise it would not be
mapped into the intermediate space in the first place).

If the length is greater than one, for instance with
`--gidmap 100000:@2000:2`, Podman will map host ids `2000` and `2001` to
`100000` and `100001`, respectively, regardless of how the intermediate
mapping is defined.

`Extending previous mappings`

Some mapping modifications may be cumbersome. For instance, a user
starts with a mapping such as `--gidmap="0:0:65000"`, that needs to be
changed such as the parent id `1000` is mapped to container id `100000`
instead, leaving container id `1` unassigned. The corresponding
`--gidmap` becomes
`--gidmap="0:0:1" --gidmap="2:2:65534" --gidmap="100000:1:1"`.

This notation can be simplified using the `+` flag, that takes care of
breaking previous mappings removing any conflicting assignment with the
given mapping. The flag is given before the container id as follows:
`--gidmap="0:0:65000" --gidmap="+100000:1:1"`

  Flag   Example         Description
  ------ --------------- -----------------------------
  `+`    `+100000:1:1`   Extend the previous mapping

This notation leads to gaps in the assignment, so it may be convenient
to fill those gaps afterwards:
`--gidmap="0:0:65000" --gidmap="+100000:1:1" --gidmap="1:65001:1"`

One specific use case for this flag is in the context of rootless users.
A rootless user may specify mappings with the `+` flag as in
`--gidmap="+100000:1:1"`. Podman will then \"fill the gaps\" starting
from zero with all the remaining intermediate ids. This is convenient
when a user wants to map a specific intermediate id to a container id,
leaving the rest of subordinate ids to be mapped by Podman at will.

`Passing only one of --uidmap or --gidmap`

Usually, subordinated user and group ids are assigned simultaneously,
and for any user the subordinated user ids match the subordinated group
ids. For convenience, if only one of **\--uidmap** or **\--gidmap** is
given, podman assumes the mapping refers to both UIDs and GIDs and
applies the given mapping to both. If only one value of the two needs to
be changed, the mappings should include the `u` or the `g` flags to
specify that they only apply to UIDs or GIDs and should not be copied
over.

  flag   Example           Description
  ------ ----------------- ----------------------------------
  `u`    `u20000:2000:1`   The mapping only applies to UIDs
  `g`    `g10000:1000:1`   The mapping only applies to GIDs

For instance given the command

    podman create --gidmap "0:0:1000" --gidmap "g2000:2000:1"

Since no **\--uidmap** is given, the **\--gidmap** is copied to
**\--uidmap**, giving a command equivalent to

    podman create --gidmap "0:0:1000" --gidmap "2000:2000:1" --uidmap "0:0:1000"

The `--gidmap "g2000:2000:1"` used the `g` flag and therefore it was not
copied to **\--uidmap**.

`Rootless mapping of additional host GIDs`

A rootless user may desire to map a specific host group that has already
been subordinated within */etc/subgid* without specifying the rest of
the mapping.

This can be done with \*\*\--gidmap
\"+g*container_gid*:[@\*host_gid]{.citation cites="*host_gid"}\*\"\*\*

Where:

-   The host GID is given through the `@` symbol
-   The mapping of this GID is not copied over to **\--usermap** thanks
    to the `g` flag.
-   The rest of the container IDs will be mapped starting from 0 to n,
    with all the remaining subordinated GIDs, thanks to the `+` flag.

For instance, if a user belongs to the group `2000` and that group is
subordinated to that user (with
`usermod --add-subgids 2000-2000 $USER`), the user can map the group
into the container with: **\--gidmap=+g100000:[\@2000]{.citation
cites="2000"}**.

If this mapping is combined with the option,
**\--group-add=keep-groups**, the process in the container will belong
to group `100000`, and files belonging to group `2000` in the host will
appear as being owned by group `100000` inside the container.

    podman run --group-add=keep-groups --gidmap="+g100000:@2000" ...

`No subordinate UIDs`

Even if a user does not have any subordinate UIDs in */etc/subuid*,
**\--uidmap** can be used to map the normal UID of the user to a
container UID by running
`podman create --uidmap $container_uid:0:1 --user $container_uid ...`.

`Pods`

The **\--uidmap** option cannot be called in conjunction with the
**\--pod** option as a uidmap cannot be set on the container level when
in a pod.

#### **\--ulimit**=*option*

Ulimit options. Sets the ulimits values inside of the container.

\--ulimit with a soft and hard limit in the format =\[:\]. For example:

\$ podman run \--ulimit nofile=1024:1024 \--rm ubi9 ulimit -n 1024

Set -1 for the soft or hard limit to set the limit to the maximum limit
of the current process. In rootful mode this is often unlimited.

If nofile and nproc are unset, a default value of 1048576 will be used,
unless overridden in containers.conf(5). However, if the default value
exceeds the hard limit for the current rootless user, the current hard
limit will be applied instead.

Use **host** to copy the current configuration from the host.

Don\'t use nproc with the ulimit flag as Linux uses nproc to set the
maximum number of processes available to a user, not to a container.

Use the \--pids-limit option to modify the cgroup control to limit the
number of processes within a container.

#### **\--umask**=*umask*

Set the umask inside the container. Defaults to `0022`. Remote
connections use local containers.conf for defaults

#### **\--unsetenv**=*env*

Unset default environment variables for the container. Default
environment variables include variables provided natively by Podman,
environment variables configured by the image, and environment variables
from containers.conf.

#### **\--unsetenv-all**

Unset all default environment variables for the container. Default
environment variables include variables provided natively by Podman,
environment variables configured by the image, and environment variables
from containers.conf.

#### **\--user**, **-u**=*user\[:group\]*

Sets the username or UID used and, optionally, the groupname or GID for
the specified command. Both *user* and *group* may be symbolic or
numeric.

Without this argument, the command runs as the user specified in the
container image. Unless overridden by a `USER` command in the
Containerfile or by a value passed to this option, this user generally
defaults to root.

When a user namespace is not in use, the UID and GID used within the
container and on the host match. When user namespaces are in use,
however, the UID and GID in the container may correspond to another UID
and GID on the host. In rootless containers, for example, a user
namespace is always used, and root in the container by default
corresponds to the UID and GID of the user invoking Podman.

#### **\--userns**=*mode*

Set the user namespace mode for the container.

If `--userns` is not set, the default value is determined as follows. -
If `--pod` is set, `--userns` is ignored and the user namespace of the
pod is used. - If the environment variable **PODMAN_USERNS** is set its
value is used. - If `userns` is specified in `containers.conf` this
value is used. - Otherwise, `--userns=host` is assumed.

`--userns=""` (i.e., an empty string) is an alias for `--userns=host`.

This option is incompatible with **\--gidmap**, **\--uidmap**,
**\--subuidname** and **\--subgidname**.

Rootless user \--userns=Key mappings:

  ----------------------------------------------------------------------
  Key                           Host User     Container User
  ----------------------------- ------------- --------------------------
  auto                          \$UID         nil (Host User UID is not
                                              mapped into container.)

  host                          \$UID         0 (Default User account
                                              mapped to root user in
                                              container.)

  keep-id                       \$UID         \$UID (Map user account to
                                              same UID within
                                              container.)

  keep-id:uid=200,gid=210       \$UID         200:210 (Map user account
                                              to specified UID, GID
                                              value within container.)

  nomap                         \$UID         nil (Host User UID is not
                                              mapped into container.)
  ----------------------------------------------------------------------

Valid *mode* values are:

**auto**\[:*OPTIONS,\...*\]: automatically create a unique user
namespace.

-   `rootful mode`: The `--userns=auto` flag requires that the user name
    **containers** be specified in the /etc/subuid and /etc/subgid
    files, with an unused range of subordinate user IDs that Podman
    containers are allowed to allocate.

         Example: `containers:2147483647:2147483648`.

-   `rootless mode`: The users range from the /etc/subuid and
    /etc/subgid files will be used. Note running a single container
    without using \--userns=auto will use the entire range of UIDs and
    not allow further subdividing. See subuid(5).

Podman allocates unique ranges of UIDs and GIDs from the `containers`
subordinate user IDs. The size of the ranges is based on the number of
UIDs required in the image. The number of UIDs and GIDs can be
overridden with the `size` option.

The option `--userns=keep-id` uses all the subuids and subgids of the
user. The option `--userns=nomap` uses all the subuids and subgids of
the user except the user\'s own ID. Using `--userns=auto` when starting
new containers does not work as long as any containers exist that were
started with `--userns=keep-id` or `--userns=nomap`.

Valid `auto` options:

-   *gidmapping*=*CONTAINER_GID:HOST_GID:SIZE*: to force a GID mapping
    to be present in the user namespace.
-   *size*=*SIZE*: to specify an explicit size for the automatic user
    namespace. e.g. `--userns=auto:size=8192`. If `size` is not
    specified, `auto` estimates a size for the user namespace.
-   *uidmapping*=*CONTAINER_UID:HOST_UID:SIZE*: to force a UID mapping
    to be present in the user namespace.

The host UID and GID in *gidmapping* and *uidmapping* can optionally be
prefixed with the `@` symbol. In this case, podman will look up the
intermediate ID corresponding to host ID and it will map the found
intermediate ID to the container id. For details see **\--uidmap**.

**container:**\_id\_: join the user namespace of the specified
container.

**host** or **\"\"** (empty string): run in the user namespace of the
caller. The processes running in the container have the same privileges
on the host as any other process launched by the calling user.

**keep-id**: creates a user namespace where the current user\'s UID:GID
are mapped to the same values in the container. For containers created
by root, the current mapping is created into a new user namespace.

Valid `keep-id` options:

-   *uid*=UID: override the UID inside the container that is used to map
    the current user to.
-   *gid*=GID: override the GID inside the container that is used to map
    the current user to.

**nomap**: creates a user namespace where the current rootless user\'s
UID:GID are not mapped into the container. This option is not allowed
for containers created by the root user.

**ns:**\_namespace\_: run the container in the given existing user
namespace.

#### **\--uts**=*mode*

Set the UTS namespace mode for the container. The following values are
supported:

-   **host**: use the host\'s UTS namespace inside the container.
-   **private**: create a new namespace for the container (default).
-   **ns:\[path\]**: run the container in the given existing UTS
    namespace.
-   **container:\[container\]**: join the UTS namespace of the specified
    container.

#### **\--variant**=*VARIANT*

Use *VARIANT* instead of the default architecture variant of the
container image. Some images can use multiple variants of the arm
architectures, such as arm/v5 and arm/v7.

#### **\--volume**, **-v**=*\[\[SOURCE-VOLUME\|HOST-DIR:\]CONTAINER-DIR\[:OPTIONS\]\]*

Create a bind mount. If `-v /HOST-DIR:/CONTAINER-DIR` is specified,
Podman bind mounts `/HOST-DIR` from the host into `/CONTAINER-DIR` in
the Podman container. Similarly, `-v SOURCE-VOLUME:/CONTAINER-DIR`
mounts the named volume from the host into the container. If no such
named volume exists, Podman creates one. If no source is given, the
volume is created as an anonymously named volume with a randomly
generated name, and is removed when the container is removed via the
`--rm` flag or the `podman rm --volumes` command.

(Note when using the remote client, including Mac and Windows (excluding
WSL2) machines, the volumes are mounted from the remote server, not
necessarily the client machine.)

The *OPTIONS* is a comma-separated list and can be one or more of:

-   **rw**\|**ro**
-   **z**\|**Z**
-   \[**O**\]
-   \[**U**\]
-   \[**no**\]**copy**
-   \[**no**\]**dev**
-   \[**no**\]**exec**
-   \[**no**\]**suid**
-   \[**r**\]**bind**
-   \[**r**\]**shared**\|\[**r**\]**slave**\|\[**r**\]**private**\[**r**\]**unbindable**
    ^[\[1\]](#Footnote1)^
-   **idmap**\[=**options**\]

The `CONTAINER-DIR` must be an absolute path such as `/src/docs`. The
volume is mounted into the container at this directory.

If a volume source is specified, it must be a path on the host or the
name of a named volume. Host paths are allowed to be absolute or
relative; relative paths are resolved relative to the directory Podman
is run in. If the source does not exist, Podman returns an error. Users
must pre-create the source files or directories.

Any source that does not begin with a `.` or `/` is treated as the name
of a named volume. If a volume with that name does not exist, it is
created. Volumes created with names are not anonymous, and they are not
removed by the `--rm` option and the `podman rm --volumes` command.

Specify multiple **-v** options to mount one or more volumes into a
container.

`Write Protected Volume Mounts`

Add **:ro** or **:rw** option to mount a volume in read-only or
read-write mode, respectively. By default, the volumes are mounted
read-write. See examples.

`Chowning Volume Mounts`

By default, Podman does not change the owner and group of source volume
directories mounted into containers. If a container is created in a new
user namespace, the UID and GID in the container may correspond to
another UID and GID on the host.

The `:U` suffix tells Podman to use the correct host UID and GID based
on the UID and GID within the container, to change recursively the owner
and group of the source volume. Chowning walks the file system under the
volume and changes the UID/GID on each file. If the volume has thousands
of inodes, this process takes a long time, delaying the start of the
container.

**Warning** use with caution since this modifies the host filesystem.

`Labeling Volume Mounts`

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a container. Without a label, the security
system might prevent the processes running inside the container from
using the content. By default, Podman does not change the labels set by
the OS.

To change a label in the container context, add either of two suffixes
**:z** or **:Z** to the volume mount. These suffixes tell Podman to
relabel file objects on the shared volumes. The **z** option tells
Podman that two or more containers share the volume content. As a
result, Podman labels the content with a shared content label. Shared
volume labels allow all containers to read/write content. The **Z**
option tells Podman to label the content with a private unshared label
Only the current container can use a private volume. Note: all
containers within a `pod` share the same SELinux label. This means all
containers within said pod can read/write volumes shared into the
container created with the `:Z` on any of one the containers. Relabeling
walks the file system under the volume and changes the label on each
file, if the volume has thousands of inodes, this process takes a long
time, delaying the start of the container. If the volume was previously
relabeled with the `z` option, Podman is optimized to not relabel a
second time. If files are moved into the volume, then the labels can be
manually change with the `chcon -Rt container_file_t PATH` command.

Note: Do not relabel system files and directories. Relabeling system
content might cause other confined services on the machine to fail. For
these types of containers we recommend disabling SELinux separation. The
option **\--security-opt label=disable** disables SELinux separation for
the container. For example if a user wanted to volume mount their entire
home directory into a container, they need to disable SELinux
separation.

    $ podman create --security-opt label=disable -v $HOME:/home/user fedora touch /home/user/file

`Overlay Volume Mounts`

The `:O` flag tells Podman to mount the directory from the host as a
temporary storage using the `overlay file system`. The container
processes can modify content within the mountpoint which is stored in
the container storage in a separate directory. In overlay terms, the
source directory is the lower, and the container storage directory is
the upper. Modifications to the mount point are destroyed when the
container finishes executing, similar to a tmpfs mount point being
unmounted.

For advanced users, the **overlay** option also supports custom
non-volatile **upperdir** and **workdir** for the overlay mount. Custom
**upperdir** and **workdir** can be fully managed by the users
themselves, and Podman does not remove it on lifecycle completion.
Example **:O,upperdir=/some/upper,workdir=/some/work**

Subsequent executions of the container sees the original source
directory content, any changes from previous container executions no
longer exist.

One use case of the overlay mount is sharing the package cache from the
host into the container to allow speeding up builds.

Note: The `O` flag conflicts with other options listed above.

Content mounted into the container is labeled with the private label. On
SELinux systems, labels in the source directory must be readable by the
container label. Usually containers can read/execute `container_share_t`
and can read/write `container_file_t`. If unable to change the labels on
a source volume, SELinux container separation must be disabled for the
container to work.

Do not modify the source directory mounted into the container with an
overlay mount, it can cause unexpected failures. Only modify the
directory after the container finishes running.

`Mounts propagation`

By default, bind-mounted volumes are `private`. That means any mounts
done inside the container are not visible on the host and vice versa.
One can change this behavior by specifying a volume mount propagation
property. When a volume is `shared`, mounts done under that volume
inside the container are visible on host and vice versa. Making a volume
**slave**^[\[1\]](#Footnote1)^ enables only one-way mount propagation:
mounts done on the host under that volume are visible inside the
container but not the other way around.

To control mount propagation property of a volume one can use the
\[**r**\]**shared**, \[**r**\]**slave**, \[**r**\]**private** or the
\[**r**\]**unbindable** propagation flag. Propagation property can be
specified only for bind mounted volumes and not for internal volumes or
named volumes. For mount propagation to work the source mount point (the
mount point where source dir is mounted on) has to have the right
propagation properties. For shared volumes, the source mount point has
to be shared. And for slave volumes, the source mount point has to be
either shared or slave. ^[\[1\]](#Footnote1)^

To recursively mount a volume and all of its submounts into a container,
use the **rbind** option. By default the bind option is used, and
submounts of the source directory is not mounted into the container.

Mounting the volume with a **copy** option tells podman to copy content
from the underlying destination directory onto newly created internal
volumes. The **copy** only happens on the initial creation of the
volume. Content is not copied up when the volume is subsequently used on
different containers. The **copy** option is ignored on bind mounts and
has no effect.

Mounting volumes with the **nosuid** options means that SUID executables
on the volume can not be used by applications to change their privilege.
By default volumes are mounted with **nosuid**.

Mounting the volume with the **noexec** option means that no executables
on the volume can be executed within the container.

Mounting the volume with the **nodev** option means that no devices on
the volume can be used by processes within the container. By default
volumes are mounted with **nodev**.

If the *HOST-DIR* is a mount point, then **dev**, **suid**, and **exec**
options are ignored by the kernel.

Use **df HOST-DIR** to figure out the source mount, then use **findmnt
-o TARGET,PROPAGATION *source-mount-dir*** to figure out propagation
properties of source mount. If **findmnt**(1) utility is not available,
then one can look at the mount entry for the source mount point in
*/proc/self/mountinfo*. Look at the \"optional fields\" and see if any
propagation properties are specified. In there, **shared:N** means the
mount is shared, **master:N** means mount is slave, and if nothing is
there, the mount is private. ^[\[1\]](#Footnote1)^

To change propagation properties of a mount point, use **mount**(8)
command. For example, if one wants to bind mount source directory
*/foo*, one can do **mount \--bind /foo /foo** and **mount
\--make-private \--make-shared /foo**. This converts /foo into a shared
mount point. Alternatively, one can directly change propagation
properties of source mount. Say */* is source mount for */foo*, then use
**mount \--make-shared /** to convert */* into a shared mount.

Note: if the user only has access rights via a group, accessing the
volume from inside a rootless container fails.

`Idmapped mount`

If `idmap` is specified, create an idmapped mount to the target user
namespace in the container. The idmap option supports a custom mapping
that can be different than the user namespace used by the container. The
mapping can be specified after the idmap option like:
`idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the first
value is the start of the backing file system IDs that are mapped to the
second value on the host. The length of this mapping is given in the
third value. Multiple ranges are separated with #.

Use the **\--group-add keep-groups** option to pass the user\'s
supplementary group access into the container.

#### **\--volumes-from**=*CONTAINER\[:OPTIONS\]*

Mount volumes from the specified container(s). Used to share volumes
between containers. The *options* is a comma-separated list with the
following available elements:

-   **rw**\|**ro**
-   **z**

Mounts already mounted volumes from a source container onto another
container. *CONTAINER* may be a name or ID. To share a volume, use the
\--volumes-from option when running the target container. Volumes can be
shared even if the source container is not running.

By default, Podman mounts the volumes in the same mode (read-write or
read-only) as it is mounted in the source container. This can be changed
by adding a `ro` or `rw` *option*.

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a container. Without a label, the security
system might prevent the processes running inside the container from
using the content. By default, Podman does not change the labels set by
the OS.

To change a label in the container context, add `z` to the volume mount.
This suffix tells Podman to relabel file objects on the shared volumes.
The `z` option tells Podman that two entities share the volume content.
As a result, Podman labels the content with a shared content label.
Shared volume labels allow all containers to read/write content.

If the location of the volume from the source container overlaps with
data residing on a target container, then the volume hides that data on
the target.

#### **\--workdir**, **-w**=*dir*

Working directory inside the container.

The default working directory for running binaries within a container is
the root directory (**/**). The image developer can set a different
default with the WORKDIR instruction. The operator can override the
working directory by using the **-w** option.

##  EXAMPLES

Create a container using a local image:

    $ podman create alpine ls

Create a container using a local image and annotate it:

    $ podman create --annotation HELLO=WORLD alpine ls

Create a container using a local image, allocating a pseudo-TTY, keeping
stdin open and name it myctr:

      podman create -t -i --name myctr alpine ls

Running a container in a new user namespace requires a mapping of the
UIDs and GIDs from the host:

    $ podman create --uidmap 0:30000:7000 --gidmap 0:30000:7000 fedora echo hello

Setting automatic user-namespace separated containers:

    # podman create --userns=auto:size=65536 ubi8-init

Configure the timezone in a container:

    $ podman create --tz=local alpine date
    $ podman create --tz=Asia/Shanghai alpine date
    $ podman create --tz=US/Eastern alpine date

Ensure the first container (container1) is running before the second
container (container2) is started:

    $ podman create --name container1 -t -i fedora bash
    $ podman create --name container2 --requires container1 -t -i fedora bash
    $ podman start --attach container2

Create a container which requires multiple containers:

    $ podman create --name container1 -t -i fedora bash
    $ podman create --name container2 -t -i fedora bash
    $ podman create --name container3 --requires container1,container2 -t -i fedora bash
    $ podman start --attach container3

Expose shared libraries inside of container as read-only using a glob:

    $ podman create --mount type=glob,src=/usr/lib64/libnvidia\*,ro -i -t fedora /bin/bash

Create a container allowing supplemental groups to have access to the
volume:

    $ podman create -v /var/lib/design:/var/lib/design --group-add keep-groups ubi8

Configure execution domain for containers using the personality option:

    $ podman create --name container1 --personality=LINUX32 fedora bash

Create a container with external rootfs mounted as an overlay:

    $ podman create --name container1 --rootfs /path/to/rootfs:O bash

Create a container connected to two networks (called net1 and net2) with
a static ip:

    $ podman create --network net1:ip=10.89.1.5 --network net2:ip=10.89.10.10 alpine ip addr

### Rootless Containers

Podman runs as a non-root user on most systems. This feature requires
that a new enough version of shadow-utils be installed. The shadow-utils
package must include the newuidmap and newgidmap executables.

In order for users to run rootless, there must be an entry for their
username in /etc/subuid and /etc/subgid which lists the UIDs for their
user namespace.

Rootless Podman works better if the fuse-overlayfs and slirp4netns
packages are installed. The fuse-overlayfs package provides a userspace
overlay storage driver, otherwise users need to use the vfs storage
driver, which can be disk space expensive and less performant than other
drivers.

To enable VPN on the container, slirp4netns or pasta needs to be
specified; without either, containers need to be run with the
\--network=host flag.

##  ENVIRONMENT

Environment variables within containers can be set using multiple
different options: This section describes the precedence.

Precedence order (later entries override earlier entries):

-   **\--env-host** : Host environment of the process executing Podman
    is added.
-   **\--http-proxy**: By default, several environment variables are
    passed in from the host, such as **http_proxy** and **no_proxy**.
    See **\--http-proxy** for details.
-   Container image : Any environment variables specified in the
    container image.
-   **\--env-file** : Any environment variables specified via env-files.
    If multiple files specified, then they override each other in order
    of entry.
-   **\--env** : Any environment variables specified overrides previous
    settings.

Create containers and set the environment ending with a *****. The
trailing***** glob functionality is only active when no value is
specified:

    $ export ENV1=a
    $ podman create --name ctr1 --env 'ENV*' alpine env
    $ podman start --attach ctr1 | grep ENV
    ENV1=a
    $ podman create --name ctr2 --env 'ENV*=b' alpine env
    $ podman start --attach ctr2 | grep ENV
    ENV*=b

##  CONMON

When Podman starts a container it actually executes the conmon program,
which then executes the OCI Runtime. Conmon is the container monitor. It
is a small program whose job is to watch the primary process of the
container, and if the container dies, save the exit code. It also holds
open the tty of the container, so that it can be attached to later. This
is what allows Podman to run in detached mode (backgrounded), so Podman
can exit but conmon continues to run. Each container has their own
instance of conmon. Conmon waits for the container to exit, gathers and
saves the exit code, and then launches a Podman process to complete the
container cleanup, by shutting down the network and storage. For more
information about conmon, see the conmon(8) man page.

##  FILES

**/etc/subuid** **/etc/subgid**

NOTE: Use the environment variable `TMPDIR` to change the temporary
storage location of downloaded container images. Podman defaults to use
`/var/tmp`.

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-save(1)](podman-save.html)**,
**[podman-ps(1)](podman-ps.html)**,
**[podman-attach(1)](podman-attach.html)**,
**[podman-pod-create(1)](podman-pod-create.html)**,
**[podman-port(1)](podman-port.html)**,
**[podman-start(1)](podman-start.html)**,
**[podman-kill(1)](podman-kill.html)**,
**[podman-stop(1)](podman-stop.html)**,
**[podman-generate-systemd(1)](podman-generate-systemd.html)**,
**[podman-rm(1)](podman-rm.html)**,
**[subgid(5)](https://www.unix.com/man-page/linux/5/subgid)**,
**[subuid(5)](https://www.unix.com/man-page/linux/5/subuid)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**,
**[systemd.unit(5)](https://www.freedesktop.org/software/systemd/man/systemd.unit.html)**,
**[setsebool(8)](https://man7.org/linux/man-pages/man8/setsebool.8.html)**,
**[slirp4netns(1)](https://github.com/rootless-containers/slirp4netns/blob/master/slirp4netns.html)**,
**[pasta(1)](https://passt.top/builds/latest/web/passt.1.html)**,
**[fuse-overlayfs(1)](https://github.com/containers/fuse-overlayfs/blob/main/fuse-overlayfs.html)**,
**proc(5)**,
**[conmon(8)](https://github.com/containers/conmon/blob/main/docs/conmon.8.md)**,
**personality(2)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

See
[podman-rootless(7)](https://github.com/containers/podman/blob/main/rootless.md)
for rootless issues.

##  HISTORY

October 2017, converted from Docker documentation to Podman by Dan Walsh
for Podman `<dwalsh@redhat.com>`

November 2014, updated by Sven Dowideit `<SvenDowideit@home.org.au>`

September 2014, updated by Sven Dowideit `<SvenDowideit@home.org.au>`

August 2014, updated by Sven Dowideit `<SvenDowideit@home.org.au>`

##  FOOTNOTES

[1]{#Footnote1}: The Podman project is committed to inclusivity, a core
value of open source. The `master` and `slave` mount propagation
terminology used here is problematic and divisive, and needs to be
changed. However, these terms are currently used within the Linux kernel
and must be used as-is at this time. When the kernel maintainers rectify
this usage, Podman will follow suit immediately.


---

<a id='podman-diff'></a>

## podman-diff - Inspect changes on a container or image's
filesystem

##  NAME

podman-diff - Inspect changes on a container or image\'s filesystem

##  SYNOPSIS

**podman diff** \[*options*\] *container\|image* \[*container\|image*\]

##  DESCRIPTION

Displays changes on a container or image\'s filesystem. The container or
image is compared to its parent layer or the second argument when given.

The output is prefixed with the following symbols:

  Symbol   Description
  -------- ----------------------------------
  A        A file or directory was added.
  D        A file or directory was deleted.
  C        A file or directory was changed.

##  OPTIONS

#### **\--format**

Alter the output into a different format. The only valid format for
**podman diff** is `json`.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

##  EXAMPLE

Show container-modified files versus the container\'s image:

    $ podman diff container1
    A /myscript.sh

Show container-modified files versus the container\'s image in JSON
format:

    $ podman diff --format json myimage
    {
      "changed": [
        "/usr",
        "/usr/local",
        "/usr/local/bin"
      ],
      "added": [
        "/usr/local/bin/docker-entrypoint.sh"
      ]
    }

Show the difference between the specified container and the image:

    $ podman diff container1 image1
    A /test

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-container-diff(1)](podman-container-diff.html)**,
**[podman-image-diff(1)](podman-image-diff.html)**

##  HISTORY

August 2017, Originally compiled by Ryan Cole <rycole@redhat.com>


---

<a id='podman-events'></a>

## podman-events - Monitor Podman events

##  NAME

podman-events - Monitor Podman events

##  SYNOPSIS

**podman events** \[*options*\]

**podman system events** \[*options*\]

##  DESCRIPTION

Monitor and print events that occur in Podman. Each event includes a
timestamp, a type, a status, name (if applicable), and image (if
applicable). The default logging mechanism is *journald*. This can be
changed in containers.conf by changing the `events_logger` value to
`file`. Only `file` and `journald` are accepted. A `none` logger is also
available, but this logging mechanism completely disables events;
nothing is reported by `podman events`.

By default, streaming mode is used, printing new events as they occur.
Previous events can be listed via `--since` and `--until`.

The *container* event type reports the follow statuses: \* attach \*
checkpoint \* cleanup \* commit \* connect \* create \* died \*
disconnect \* exec \* exec_died \* exited \* export \* import \* init \*
kill \* mount \* pause \* prune \* remove \* rename \* restart \*
restore \* start \* stop \* sync \* unmount \* unpause \* update

The *pod* event type reports the follow statuses: \* create \* kill \*
pause \* remove \* start \* stop \* unpause

The *image* event type reports the following statuses: \*
loadFromArchive, \* mount \* pull \* pull-error \* push \* remove \*
save \* tag \* unmount \* untag

The *system* type reports the following statuses: \* refresh \* renumber

The *volume* type reports the following statuses: \* create \* prune \*
remove

#### Verbose Create Events

Setting `events_container_create_inspect_data=true` in
containers.conf(5) instructs Podman to create more verbose
container-create events which include a JSON payload with detailed
information about the containers. The JSON payload is identical to the
one of podman-container-inspect(1). The associated field in journald is
named `PODMAN_CONTAINER_INSPECT_DATA`.

##  OPTIONS

#### **\--filter**, **-f**=*filter*

Filter events that are displayed. They must be in the format of
\"filter=value\". The following filters are supported:

  **Filter**   **Description**
  ------------ ----------------------------------------
  container    \[Name or ID\] Container\'s name or ID
  event        event_status (described above)
  image        \[Name or ID\] Image name or ID
  label        \[key=value\] label
  pod          \[Name or ID\] Pod name or ID
  volume       \[Name or ID\] Volume name or ID
  type         Event_type (described above)

In the case where an ID is used, the ID may be in its full or shortened
form. The \"die\" event is mapped to \"died\" for Docker compatibility.

#### **\--format**

Format the output to JSON Lines or using the given Go template.

  -----------------------------------------------------------------------------
  **Placeholder**         **Description**
  ----------------------- -----------------------------------------------------
  .Attributes \...        created_at, \_by, labels, and more (map\[\])

  .ContainerExitCode      Exit code (int)

  .ContainerInspectData   Payload of the container\'s inspect

  .Error                  Error message in case the event status is an error
                          (e.g. pull-error)

  .HealthStatus           Health Status (string)

  .ID                     Container ID (full 64-bit SHA)

  .Image                  Name of image being run (string)

  .Name                   Container name (string)

  .Network                Name of network being used (string)

  .PodID                  ID of pod associated with container, if any

  .Status                 Event status (e.g., create, start, died, \...)

  .Time                   Event timestamp (string)

  .TimeNano               Event timestamp with nanosecond precision (int64)

  .Type                   Event type (e.g., image, container, pod, \...)
  -----------------------------------------------------------------------------

#### **\--help**

Print usage statement.

#### **\--no-trunc**

Do not truncate the output (default *true*).

#### **\--since**=*timestamp*

Show all events created since the given timestamp

#### **\--stream**

Stream events and do not exit after reading the last known event
(default *true*).

#### **\--until**=*timestamp*

Show all events created until the given timestamp

The *since* and *until* values can be RFC3339Nano time stamps or a Go
duration string such as 10m, 5h. If no *since* or *until* values are
provided, only new events are shown.

##  JOURNALD IDENTIFIERS

The journald events-backend of Podman uses the following journald
identifiers. You can use the identifiers to filter Podman events
directly with `journalctl`.

  -----------------------------------------------------------------------------
  **Identifier**                  **Description**
  ------------------------------- ---------------------------------------------
  SYSLOG_IDENTIFIER               Always set to \"podman\"

  PODMAN_EVENT                    The event status as described above

  PODMAN_TYPE                     The event type as described above

  PODMAN_TIME                     The time stamp when the event was written

  PODMAN_NAME                     Name of the event object (e.g., container,
                                  image)

  PODMAN_ID                       ID of the event object (e.g., container,
                                  image)

  PODMAN_EXIT_CODE                Exit code of the container

  PODMAN_POD_ID                   Pod ID of the container

  PODMAN_LABELS                   Labels of the container

  PODMAN_HEALTH_STATUS            Health status of the container

  PODMAN_CONTAINER_INSPECT_DATA   The JSON payload of `podman-inspect` as
                                  described above

  PODMAN_NETWORK_NAME             The name of the network
  -----------------------------------------------------------------------------

##  EXAMPLES

Show Podman events:

    $ podman events
    2019-03-02 10:33:42.312377447 -0600 CST container create 34503c192940 (image=docker.io/library/alpine:latest, name=friendly_allen)
    2019-03-02 10:33:46.958768077 -0600 CST container init 34503c192940 (image=docker.io/library/alpine:latest, name=friendly_allen)
    2019-03-02 10:33:46.973661968 -0600 CST container start 34503c192940 (image=docker.io/library/alpine:latest, name=friendly_allen)
    2019-03-02 10:33:50.833761479 -0600 CST container stop 34503c192940 (image=docker.io/library/alpine:latest, name=friendly_allen)
    2019-03-02 10:33:51.047104966 -0600 CST container cleanup 34503c192940 (image=docker.io/library/alpine:latest, name=friendly_allen)

Show only Podman container create events:

    $ podman events -f event=create
    2019-03-02 10:36:01.375685062 -0600 CST container create 20dc581f6fbf (image=docker.io/library/alpine:latest, name=sharp_morse)
    2019-03-02 10:36:08.561188337 -0600 CST container create 58e7e002344c (image=registry.k8s.io/pause:3.1, name=3e701f270d54-infra)
    2019-03-02 10:36:13.146899437 -0600 CST volume create cad6dc50e087 (image=, name=cad6dc50e0879568e7d656bd004bd343d6035e7fc4024e1711506fe2fd459e6f)
    2019-03-02 10:36:29.978806894 -0600 CST container create d81e30f1310f (image=docker.io/library/busybox:latest, name=musing_newton)

Show only Podman pod create events:

    $ podman events --filter event=create --filter type=pod
    2019-03-02 10:44:29.601746633 -0600 CST pod create 1df5ebca7b44 (image=, name=confident_hawking)
    2019-03-02 10:44:42.374637304 -0600 CST pod create ca731231718e (image=, name=webapp)
    2019-03-02 10:44:47.486759133 -0600 CST pod create 71e807fc3a8e (image=, name=reverent_swanson)

Show only Podman events created in the last five minutes:

    $ sudo podman events --since 5m
    2019-03-02 10:44:29.598835409 -0600 CST container create b629d10d3831 (image=registry.k8s.io/pause:3.1, name=1df5ebca7b44-infra)
    2019-03-02 10:44:29.601746633 -0600 CST pod create 1df5ebca7b44 (image=, name=confident_hawking)
    2019-03-02 10:44:42.371100253 -0600 CST container create 170a0f457d00 (image=registry.k8s.io/pause:3.1, name=ca731231718e-infra)
    2019-03-02 10:44:42.374637304 -0600 CST pod create ca731231718e (image=, name=webapp)

Show Podman events in JSON Lines format:

    $ podman events --format json
    {"ID":"683b0909d556a9c02fa8cd2b61c3531a965db42158627622d1a67b391964d519","Image":"localhost/myshdemo:latest","Name":"agitated_diffie","Status":"cleanup","Time":"2019-04-27T22:47:00.849932843-04:00","Type":"container"}
    {"ID":"a0f8ab051bfd43f9c5141a8a2502139707e4b38d98ac0872e57c5315381e88ad","Image":"docker.io/library/alpine:latest","Name":"friendly_tereshkova","Status":"unmount","Time":"2019-04-28T13:43:38.063017276-04:00","Type":"container"}

##  SEE ALSO

**[podman(1)](podman.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**

##  HISTORY

March 2019, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-exec'></a>

## podman-exec - Execute a command in a running container

##  NAME

podman-exec - Execute a command in a running container

##  SYNOPSIS

**podman exec** \[*options*\] *container* *command* \[*arg* \...\]

**podman container exec** \[*options*\] *container* *command* \[*arg*
\...\]

##  DESCRIPTION

**podman exec** executes a command in a running container.

##  OPTIONS

#### **\--detach**, **-d**

Start the exec session, but do not attach to it. The command runs in the
background, and the exec session is automatically removed when it
completes. The **podman exec** command prints the ID of the exec session
and exits immediately after it starts.

#### **\--detach-keys**=*sequence*

Specify the key sequence for detaching a container. Format is a single
character `[a-Z]` or one or more `ctrl-<value>` characters where
`<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`. Specifying \"\"
disables this feature. The default is *ctrl-p,ctrl-q*.

This option can also be set in **containers.conf**(5) file.

#### **\--env**, **-e**=*env*

Set environment variables.

This option allows arbitrary environment variables that are available
for the process to be launched inside of the container. If an
environment variable is specified without a value, Podman checks the
host environment for a value and set the variable only if it is set on
the host. As a special case, if an environment variable ending in
\_\_\*\_\_ is specified without a value, Podman searches the host
environment for variables starting with the prefix and adds those
variables to the container.

#### **\--env-file**=*file*

Read in a line-delimited file of environment variables.

#### **\--interactive**, **-i**

When set to **true**, make stdin available to the contained process. If
**false**, the stdin of the contained process is empty and immediately
closed.

If attached, stdin is piped to the contained process. If detached,
reading stdin will block until later attached.

**Caveat:** Podman will consume input from stdin as soon as it becomes
available, even if the contained process doesn\'t request it.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--preserve-fd**=*FD1\[,FD2,\...\]*

Pass down to the process the additional file descriptors specified in
the comma separated list. It can be specified multiple times. This
option is only supported with the crun OCI runtime. It might be a
security risk to use this option with other OCI runtimes.

(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--preserve-fds**=*N*

Pass down to the process N additional file descriptors (in addition to
0, 1, 2). The total FDs are 3+N. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--privileged**

Give extended privileges to this container. The default is **false**.

By default, Podman containers are unprivileged (**=false**) and cannot,
for example, modify parts of the operating system. This is because by
default a container is only allowed limited access to devices. A
\"privileged\" container is given the same access to devices as the user
launching the container, with the exception of virtual consoles
(*/dev/tty*) when running in systemd mode (**\--systemd=always**).

A privileged container turns off the security features that isolate the
container from the host. Dropped Capabilities, limited devices,
read-only mount points, Apparmor/SELinux separation, and Seccomp filters
are all disabled. Due to the disabled security features, the privileged
field should almost never be set as containers can easily break out of
confinement.

Containers running in a user namespace (e.g., rootless containers)
cannot have more privileges than the user that launched them.

#### **\--tty**, **-t**

Allocate a pseudo-TTY. The default is **false**.

When set to **true**, Podman allocates a pseudo-tty and attach to the
standard input of the container. This can be used, for example, to run a
throwaway interactive shell.

**NOTE**: The \--tty flag prevents redirection of standard output. It
combines STDOUT and STDERR, it can insert control characters, and it can
hang pipes. This option is only used when run interactively in a
terminal. When feeding input to Podman, use -i only, not -it.

#### **\--user**, **-u**=*user\[:group\]*

Sets the username or UID used and, optionally, the groupname or GID for
the specified command. Both *user* and *group* may be symbolic or
numeric.

Without this argument, the command runs as the user specified in the
container image. Unless overridden by a `USER` command in the
Containerfile or by a value passed to this option, this user generally
defaults to root.

When a user namespace is not in use, the UID and GID used within the
container and on the host match. When user namespaces are in use,
however, the UID and GID in the container may correspond to another UID
and GID on the host. In rootless containers, for example, a user
namespace is always used, and root in the container by default
corresponds to the UID and GID of the user invoking Podman.

#### **\--workdir**, **-w**=*dir*

Working directory inside the container.

The default working directory for running binaries within a container is
the root directory (**/**). The image developer can set a different
default with the WORKDIR instruction. The operator can override the
working directory by using the **-w** option.

##  Exit Status

The exit code from `podman exec` gives information about why the command
within the container failed to run or why it exited. When `podman exec`
exits with a non-zero code, the exit codes follow the `chroot` standard,
see below:

**125** The error is with Podman itself

    $ podman exec --foo ctrID /bin/sh; echo $?
    Error: unknown flag: --foo
    125

**126** The *contained command* cannot be invoked

    $ podman exec ctrID /etc; echo $?
    Error: container_linux.go:346: starting container process caused "exec: \"/etc\": permission denied": OCI runtime error
    126

**127** The *contained command* cannot be found

    $ podman exec ctrID foo; echo $?
    Error: container_linux.go:346: starting container process caused "exec: \"foo\": executable file not found in $PATH": OCI runtime error
    127

**Exit code** The *contained command* exit code

    $ podman exec ctrID /bin/sh -c 'exit 3'; echo $?
    3

##  EXAMPLES

Execute command in selected container with a stdin and a tty allocated:

    $ podman exec -it ctrID ls

Execute command with the overridden working directory in selected
container with a stdin and a tty allocated:

    $ podman exec -it -w /tmp myCtr pwd

Execute command as the specified user in selected container:

    $ podman exec --user root ctrID ls

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-run(1)](podman-run.html)**

##  HISTORY

December 2017, Originally compiled by Brent Baude<bbaude@redhat.com>


---

<a id='podman-export'></a>

## podman-export - Export a container's filesystem contents as a tar
archive

##  NAME

podman-export - Export a container\'s filesystem contents as a tar
archive

##  SYNOPSIS

**podman export** \[*options*\] *container*

**podman container export** \[*options*\] *container*

##  DESCRIPTION

**podman export** exports the filesystem of a container and saves it as
a tarball on the local machine. **podman export** writes to STDOUT by
default and can be redirected to a file using the `--output` flag. The
image of the container exported by **podman export** can be imported by
**podman import**. To export image(s) with parent layers, use **podman
save**. Note: `:` is a restricted character and cannot be part of the
file name.

**podman \[GLOBAL OPTIONS\]**

**podman export \[GLOBAL OPTIONS\]**

**podman export [OPTIONS](#options) CONTAINER**

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

#### **\--output**, **-o**

Write to a file, default is STDOUT

##  EXAMPLES

Export container into specified tar ball:

    $ podman export -o redis-container.tar 883504668ec465463bc0fe7e63d53154ac3b696ea8d7b233748918664ea90e57

Export container to stdout:

    $ podman export 883504668ec465463bc0fe7e63d53154ac3b696ea8d7b233748918664ea90e57 > redis-container.tar

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-import(1)](podman-import.html)**

##  HISTORY

August 2017, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-farm'></a>

## podman-farm - Farm out builds to machines running podman for
different architectures

##  NAME

podman-farm - Farm out builds to machines running podman for different
architectures

##  SYNOPSIS

**podman farm** *subcommand*

##  DESCRIPTION

Farm out builds to machines running Podman for different architectures.

Manage farms by creating, updating, and removing them.

Note: All farm machines must have a minimum podman version of v4.9.0.

Podman manages the farms by writing and reading the
`podman-connections.json` file located under
`$XDG_CONFIG_HOME/containers` or if the env is not set it defaults to
`$HOME/.config/containers`. Or the `PODMAN_CONNECTIONS_CONF` environment
variable can be set to a full file path which podman will use instead.
This file is managed by the podman commands and should never be edited
by users directly. To manually configure the farms use the `[farm]`
section in containers.conf.

If the ReadWrite column in the **podman farm list** output is set to
true the farm is stored in the `podman-connections.json` file otherwise
it is stored in containers.conf and can therefore not be edited with the
**podman farm remove/update** commands. It can still be used with
**podman farm build**.

##  COMMANDS

  -------------------------------------------------------------------------------------------------
  Command   Man Page                                           Description
  --------- -------------------------------------------------- ------------------------------------
  build     [podman-farm-build(1)](podman-farm-build.html)     Build images on farm nodes, then
                                                               bundle them into a manifest list

  create    [podman-farm-create(1)](podman-farm-create.html)   Create a new farm

  list      [podman-farm-list(1)](podman-farm-list.html)       List the existing farms

  remove    [podman-farm-remove(1)](podman-farm-remove.html)   Delete one or more farms

  update    [podman-farm-update(1)](podman-farm-update.html)   Update an existing farm
  -------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

July 2023, Originally compiled by Urvashi Mohnani (umohnani at redhat
dot com)


---

<a id='podman-generate'></a>

## podman-generate - Generate structured data based on containers, pods
or volumes

##  NAME

podman-generate - Generate structured data based on containers, pods or
volumes

##  SYNOPSIS

**podman generate** *subcommand*

##  DESCRIPTION

The generate command creates structured output (like YAML) based on a
container, pod or volume.

##  COMMANDS

  --------------------------------------------------------------------------------------------------------------
  Command   Man Page                                                     Description
  --------- ------------------------------------------------------------ ---------------------------------------
  kube      [podman-kube-generate(1)](podman-kube-generate.html)         Generate Kubernetes YAML based on
                                                                         containers, pods or volumes.

  spec      [podman-generate-spec(1)](podman-generate-spec.html)         Generate Specgen JSON based on
                                                                         containers or pods.

  systemd   [podman-generate-systemd(1)](podman-generate-systemd.html)   \[DEPRECATED\] Generate systemd unit
                                                                         file(s) for a container or pod.
  --------------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-container(1)](podman-container.html)**


---

<a id='podman-healthcheck'></a>

## podman-healthcheck - Manage healthchecks for containers

##  NAME

podman-healthcheck - Manage healthchecks for containers

##  SYNOPSIS

**podman healthcheck** *subcommand*

##  DESCRIPTION

podman healthcheck is a set of subcommands that manage container
healthchecks

##  SUBCOMMANDS

  --------------------------------------------------------------------------------------------------------------
  Command   Man Page                                                   Description
  --------- ---------------------------------------------------------- -----------------------------------------
  run       [podman-healthcheck-run(1)](podman-healthcheck-run.html)   Run a container healthcheck

  --------------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

Feb 2019, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-help'></a>

## podman - Simple management tool for pods, containers and images

##  NAME

podman - Simple management tool for pods, containers and images

##  SYNOPSIS

**podman** \[*options*\] *command*

##  DESCRIPTION

Podman (Pod Manager) is a fully featured container engine that is a
simple daemonless tool. Podman provides a Docker-CLI comparable command
line that eases the transition from other container engines and allows
the management of pods, containers and images. Simply put:
`alias docker=podman`. Most Podman commands can be run as a regular
user, without requiring additional privileges.

Podman uses Buildah(1) internally to create container images. Both tools
share image (not container) storage, hence each can use or manipulate
images (but not containers) created by the other.

Default settings for flags are defined in `containers.conf`. Most
settings for Remote connections use the server\'s containers.conf,
except when documented in man pages.

**podman [GLOBAL OPTIONS](#global-options)**

##  GLOBAL OPTIONS

#### **\--cgroup-manager**=*manager*

The CGroup manager to use for container cgroups. Supported values are
**cgroupfs** or **systemd**. Default is *systemd* unless overridden in
the containers.conf file.

Note: Setting this flag can cause certain commands to break when called
on containers previously created by the other CGroup manager type. Note:
CGroup manager is not supported in rootless mode when using CGroups
Version V1.

#### **\--config**

Location of config file. Mainly for docker compatibility, only the
authentication parts of the config are supported.

#### **\--conmon**

Path of the conmon binary (Default path is configured in
`containers.conf`)

#### **\--connection**, **-c**

Connection to use for remote podman, including Mac and Windows
(excluding WSL2) machines, (Default connection is configured in
`containers.conf`) Setting this option switches the **\--remote** option
to true. Remote connections use local containers.conf for default.

#### **\--events-backend**=*type*

Backend to use for storing events. Allowed values are **file**,
**journald**, and **none**. When *file* is specified, the events are
stored under `<tmpdir>/events/events.log` (see **\--tmpdir** below).

#### **\--help**, **-h**

Print usage statement

#### **\--hooks-dir**=*path*

Each `*.json` file in the path configures a hook for Podman containers.
For more details on the syntax of the JSON files and the semantics of
hook injection, see `oci-hooks(5)`. Podman and libpod currently support
both the 1.0.0 and 0.1.0 hook schemas, although the 0.1.0 schema is
deprecated.

This option may be set multiple times; paths from later options have
higher precedence (`oci-hooks(5)` discusses directory precedence).

For the annotation conditions, libpod uses any annotations set in the
generated OCI configuration.

For the bind-mount conditions, only mounts explicitly requested by the
caller via `--volume` are considered. Bind mounts that libpod inserts by
default (e.g. `/dev/shm`) are not considered.

If `--hooks-dir` is unset for root callers, Podman and libpod currently
default to `/usr/share/containers/oci/hooks.d` and
`/etc/containers/oci/hooks.d` in order of increasing precedence. Using
these defaults is deprecated. Migrate to explicitly setting
`--hooks-dir`.

Podman and libpod currently support an additional `precreate` state
which is called before the runtime\'s `create` operation. Unlike the
other stages, which receive the container state on their standard input,
`precreate` hooks receive the proposed runtime configuration on their
standard input. They may alter that configuration as they see fit, and
write the altered form to their standard output.

**WARNING**: the `precreate` hook allows powerful changes to occur, such
as adding additional mounts to the runtime configuration. That power
also makes it easy to break things. Before reporting libpod errors, try
running a container with `precreate` hooks disabled to see if the
problem is due to one of the hooks.

#### **\--identity**=*path*

Path to ssh identity file. If the identity file has been encrypted,
podman prompts the user for the passphrase. If no identity file is
provided and no user is given, podman defaults to the user running the
podman command. Podman prompts for the login password on the remote
server.

Identity value resolution precedence: - command line value - environment
variable `CONTAINER_SSHKEY`, if `CONTAINER_HOST` is found -
`containers.conf` Remote connections use local containers.conf for
default.

#### **\--imagestore**=*path*

Path of the imagestore where images are stored. By default, the storage
library stores all the images in the graphroot but if an imagestore is
provided, then the storage library will store newly pulled images in the
provided imagestore and keep using the graphroot for everything else. If
the user is using the overlay driver, then the images which were already
part of the graphroot will still be accessible.

This will override *imagestore* option in `containers-storage.conf(5)`,
refer to `containers-storage.conf(5)` for more details.

#### **\--log-level**=*level*

Log messages at and above specified level: **debug**, **info**,
**warn**, **error**, **fatal** or **panic** (default: *warn*)

#### **\--module**=*path*

Load the specified `containers.conf(5)` module. Can be an absolute or
relative path. Please refer to `containers.conf(5)` for details.

This flag is not supported on the remote client, including Mac and
Windows (excluding WSL2) machines. Further note that the flag is a
root-level flag and must be specified before any Podman sub-command.

#### **\--network-cmd-path**=*path*

Path to the `slirp4netns(1)` command binary to use for setting up a
slirp4netns network. If \"\" is used, then the binary will first be
searched using the `helper_binaries_dir` option in `containers.conf`,
and second using the `$PATH` environment variable. **Note:** This option
is deprecated and will be removed with Podman 6.0. Use the
`helper_binaries_dir` option in `containers.conf` instead.

#### **\--network-config-dir**=*directory*

Path to the directory where network configuration files are located. For
the netavark backend \"/etc/containers/networks\" is used as root and
\"[\$graphroot/networks\" as rootless. For the CNI backend the default
is \"/etc/cni/net.d\" as root and \"\$]{.math
.inline}HOME/.config/cni/net.d\" as rootless. CNI is deprecated and will
be removed in the next major Podman version 5.0 in preference of
Netavark.

#### **\--out**=*path*

Redirect the output of podman to the specified path without affecting
the container output or its logs. This parameter can be used to capture
the output from any of podman\'s commands directly into a file and
enable suppression of podman\'s output by specifying /dev/null as the
path. To explicitly disable the container logging, the **\--log-driver**
option should be used.

#### **\--remote**, **-r**

When true, access to the Podman service is remote. Defaults to false.
Settings can be modified in the containers.conf file. If the
CONTAINER_HOST environment variable is set, the **\--remote** option
defaults to true.

#### **\--root**=*value*

Storage root dir in which data, including images, is stored (default:
\"/var/lib/containers/storage\" for UID 0,
\"\$HOME/.local/share/containers/storage\" for other users). Default
root dir configured in `containers-storage.conf(5)`.

Overriding this option causes the *storage-opt* settings in
`containers-storage.conf(5)` to be ignored. The user must specify
additional options via the `--storage-opt` flag.

#### **\--runroot**=*value*

Storage state directory where all state information is stored (default:
\"/run/containers/storage\" for UID 0, \"/run/user/\$UID/run\" for other
users). Default state dir configured in `containers-storage.conf(5)`.

#### **\--runtime**=*value*

Name of the OCI runtime as specified in containers.conf or absolute path
to the OCI compatible binary used to run containers.

#### **\--runtime-flag**=*flag*

Adds global flags for the container runtime. To list the supported
flags, please consult the manpages of the selected container runtime
(`runc` is the default runtime, the manpage to consult is `runc(8)`.
When the machine is configured for cgroup V2, the default runtime is
`crun`, the manpage to consult is `crun(8)`.).

Note: Do not pass the leading `--` to the flag. To pass the runc flag
`--log-format json` to podman build, the option given can be
`--runtime-flag log-format=json`.

#### **\--ssh**=*value*

This option allows the user to change the ssh mode, meaning that rather
than using the default **golang** mode, one can instead use
**\--ssh=native** to use the installed ssh binary and config file
declared in containers.conf.

#### **\--storage-driver**=*value*

Storage driver. The default storage driver for UID 0 is configured in
`containers-storage.conf(5)` in rootless mode), and is *vfs* for
non-root users when *fuse-overlayfs* is not available. The
`STORAGE_DRIVER` environment variable overrides the default. The
\--storage-driver specified driver overrides all.

Overriding this option causes the *storage-opt* settings in
`containers-storage.conf(5)` to be ignored. The user must specify
additional options via the `--storage-opt` flag.

#### **\--storage-opt**=*value*

Specify a storage driver option. Default storage driver options are
configured in `containers-storage.conf(5)`. The `STORAGE_OPTS`
environment variable overrides the default. The \--storage-opt specified
options override all. Specify \--storage-opt=\"\" so no storage options
is used.

#### **\--syslog**

Output logging information to syslog as well as the console (default
*false*).

On remote clients, including Mac and Windows (excluding WSL2) machines,
logging is directed to the file \$HOME/.config/containers/podman.log.

#### **\--tmpdir**=*path*

Path to the tmp directory, for libpod runtime content. Defaults to
`$XDG_RUNTIME_DIR/libpod/tmp` as rootless and `/run/libpod/tmp` as
rootful.

NOTE \--tmpdir is not used for the temporary storage of downloaded
images. Use the environment variable `TMPDIR` to change the temporary
storage location of downloaded container images. Podman defaults to use
`/var/tmp`.

#### **\--transient-store**

Enables a global transient storage mode where all container metadata is
stored on non-persistent media (i.e. in the location specified by
`--runroot`). This mode allows starting containers faster, as well as
guaranteeing a fresh state on boot in case of unclean shutdowns or other
problems. However it is not compatible with a traditional model where
containers persist across reboots.

Default value for this is configured in `containers-storage.conf(5)`.

#### **\--url**=*value*

URL to access Podman service (default from `containers.conf`, rootless
`unix:///run/user/$UID/podman/podman.sock` or as root
`unix:///run/podman/podman.sock`). Setting this option switches the
**\--remote** option to true.

-   `CONTAINER_HOST` is of the format
    `<schema>://[<user[:<password>]@]<host>[:<port>][<path>]`

Details: - `schema` is one of: \* `ssh` (default): a local unix(7)
socket on the named `host` and `port`, reachable via SSH \* `tcp`: an
unencrypted, unauthenticated TCP connection to the named `host` and
`port` \* `unix`: a local unix(7) socket at the specified `path`, or the
default for the user - `user` defaults to either `root` or the current
running user (`ssh` only) - `password` has no default (`ssh` only) -
`host` must be provided and is either the IP or name of the machine
hosting the Podman service (`ssh` and `tcp`) - `port` defaults to 22
(`ssh` and `tcp`) - `path` defaults to either `/run/podman/podman.sock`,
or `/run/user/$UID/podman/podman.sock` if running rootless (`unix`), or
must be explicitly specified (`ssh`)

URL value resolution precedence: - command line value - environment
variable `CONTAINER_HOST` - `engine.service_destinations` table in
containers.conf, excluding the /usr/share/containers directory -
`unix:///run/podman/podman.sock`

Remote connections use local containers.conf for default.

Some example URL values in valid formats: -
unix:///run/podman/podman.sock -
unix:///run/user/[*UID*/*podman*/*podman*.*sock* − *ssh* : //*notroot*@*localhost* : 22/*run*/*user*/]{.math
.inline}UID/podman/podman.sock -
ssh://root@localhost:22/run/podman/podman.sock - tcp://localhost:34451 -
tcp://127.0.0.1:34451

#### **\--version**, **-v**

Print the version

#### **\--volumepath**=*value*

Volume directory where builtin volume information is stored (default:
\"/var/lib/containers/storage/volumes\" for UID 0,
\"\$HOME/.local/share/containers/storage/volumes\" for other users).
Default volume path can be overridden in `containers.conf`.

##  Environment Variables

Podman can set up environment variables from env of \[engine\] table in
containers.conf. These variables can be overridden by passing
environment variables before the `podman` commands.

#### **CONTAINERS_CONF**

Set default locations of containers.conf file

#### **CONTAINERS_REGISTRIES_CONF**

Set default location of the registries.conf file.

#### **CONTAINERS_STORAGE_CONF**

Set default location of the storage.conf file.

#### **CONTAINER_CONNECTION**

Override default `--connection` value to access Podman service.
Automatically enables the \--remote option.

#### **CONTAINER_HOST**

Set default `--url` value to access Podman service. Automatically
enables \--remote option.

#### **CONTAINER_SSHKEY**

Set default `--identity` path to ssh key file value used to access
Podman service.

#### **PODMAN_CONNECTIONS_CONF**

The path to the file where the system connections and farms created with
`podman system connection add` and `podman farm add` are stored, by
default it uses `~/.config/containers/podman-connections.json`.

#### **STORAGE_DRIVER**

Set default `--storage-driver` value.

#### **STORAGE_OPTS**

Set default `--storage-opt` value.

#### **TMPDIR**

Set the temporary storage location of downloaded container images.
Podman defaults to use `/var/tmp`.

#### **XDG_CONFIG_HOME**

In Rootless mode configuration files are read from `XDG_CONFIG_HOME`
when specified, otherwise in the home directory of the user under
`$HOME/.config/containers`.

#### **XDG_DATA_HOME**

In Rootless mode images are pulled under `XDG_DATA_HOME` when specified,
otherwise in the home directory of the user under
`$HOME/.local/share/containers/storage`.

#### **XDG_RUNTIME_DIR**

In Rootless mode temporary configuration data is stored in
`${XDG_RUNTIME_DIR}/containers`.

##  Remote Access

The Podman command can be used with remote services using the `--remote`
flag. Connections can be made using local unix domain sockets, ssh or
directly to tcp sockets. When specifying the podman \--remote flag, only
the global options `--url`, `--identity`, `--log-level`, `--connection`
are used.

Connection information can also be managed using the containers.conf
file.

##  Exit Codes

The exit code from `podman` gives information about why the container
failed to run or why it exited. When `podman` commands exit with a
non-zero code, the exit codes follow the `chroot` standard, see below:

**125** The error is with podman ***itself***

    $ podman run --foo busybox; echo $?
    Error: unknown flag: --foo
    125

**126** Executing a *container command* and the *command* cannot be
invoked

    $ podman run busybox /etc; echo $?
    Error: container_linux.go:346: starting container process caused "exec: \"/etc\": permission denied": OCI runtime error
    126

**127** Executing a *container command* and the *command* cannot be
found

    $ podman run busybox foo; echo $?
    Error: container_linux.go:346: starting container process caused "exec: \"foo\": executable file not found in $PATH": OCI runtime error
    127

**Exit code** otherwise, `podman` returns the exit code of the
*container command*

    $ podman run busybox /bin/sh -c 'exit 3'; echo $?
    3

##  COMMANDS

  ---------------------------------------------------------------------------------------------
  Command                                            Description
  -------------------------------------------------- ------------------------------------------
  [podman-attach(1)](podman-attach.html)             Attach to a running container.

  [podman-auto-update(1)](podman-auto-update.html)   Auto update containers according to their
                                                     auto-update policy

  [podman-build(1)](podman-build.html)               Build a container image using a
                                                     Containerfile.

  [podman-farm(1)](podman-farm.html)                 Farm out builds to machines running podman
                                                     for different architectures

  [podman-commit(1)](podman-commit.html)             Create new image based on the changed
                                                     container.

  [podman-completion(1)](podman-completion.html)     Generate shell completion scripts

  [podman-compose(1)](podman-compose.html)           Run Compose workloads via an external
                                                     compose provider.

  [podman-container(1)](podman-container.html)       Manage containers.

  [podman-cp(1)](podman-cp.html)                     Copy files/folders between a container and
                                                     the local filesystem.

  [podman-create(1)](podman-create.html)             Create a new container.

  [podman-diff(1)](podman-diff.html)                 Inspect changes on a container or image\'s
                                                     filesystem.

  [podman-events(1)](podman-events.html)             Monitor Podman events

  [podman-exec(1)](podman-exec.html)                 Execute a command in a running container.

  [podman-export(1)](podman-export.html)             Export a container\'s filesystem contents
                                                     as a tar archive.

  [podman-generate(1)](podman-generate.html)         Generate structured data based on
                                                     containers, pods or volumes.

  [podman-healthcheck(1)](podman-healthcheck.html)   Manage healthchecks for containers

  [podman-history(1)](podman-history.html)           Show the history of an image.

  [podman-image(1)](podman-image.html)               Manage images.

  [podman-images(1)](podman-images.html)             List images in local storage.

  [podman-import(1)](podman-import.html)             Import a tarball and save it as a
                                                     filesystem image.

  [podman-info(1)](podman-info.html)                 Display Podman related system information.

  [podman-init(1)](podman-init.html)                 Initialize one or more containers

  [podman-inspect(1)](podman-inspect.html)           Display a container, image, volume,
                                                     network, or pod\'s configuration.

  [podman-kill(1)](podman-kill.html)                 Kill the main process in one or more
                                                     containers.

  [podman-load(1)](podman-load.html)                 Load image(s) from a tar archive into
                                                     container storage.

  [podman-login(1)](podman-login.html)               Log in to a container registry.

  [podman-logout(1)](podman-logout.html)             Log out of a container registry.

  [podman-logs(1)](podman-logs.html)                 Display the logs of one or more
                                                     containers.

  [podman-machine(1)](podman-machine.html)           Manage Podman\'s virtual machine

  [podman-manifest(1)](podman-manifest.html)         Create and manipulate manifest lists and
                                                     image indexes.

  [podman-mount(1)](podman-mount.html)               Mount a working container\'s root
                                                     filesystem.

  [podman-network(1)](podman-network.html)           Manage Podman networks.

  [podman-pause(1)](podman-pause.html)               Pause one or more containers.

  [podman-kube(1)](podman-kube.html)                 Play containers, pods or volumes based on
                                                     a structured input file.

  [podman-pod(1)](podman-pod.html)                   Management tool for groups of containers,
                                                     called pods.

  [podman-port(1)](podman-port.html)                 List port mappings for a container.

  [podman-ps(1)](podman-ps.html)                     Print out information about containers.

  [podman-pull(1)](podman-pull.html)                 Pull an image from a registry.

  [podman-push(1)](podman-push.html)                 Push an image, manifest list or image
                                                     index from local storage to elsewhere.

  [podman-rename(1)](podman-rename.html)             Rename an existing container.

  [podman-restart(1)](podman-restart.html)           Restart one or more containers.

  [podman-rm(1)](podman-rm.html)                     Remove one or more containers.

  [podman-rmi(1)](podman-rmi.html)                   Remove one or more locally stored images.

  [podman-run(1)](podman-run.html)                   Run a command in a new container.

  [podman-save(1)](podman-save.html)                 Save image(s) to an archive.

  [podman-search(1)](podman-search.html)             Search a registry for an image.

  [podman-secret(1)](podman-secret.html)             Manage podman secrets.

  [podman-start(1)](podman-start.html)               Start one or more containers.

  [podman-stats(1)](podman-stats.html)               Display a live stream of one or more
                                                     container\'s resource usage statistics.

  [podman-stop(1)](podman-stop.html)                 Stop one or more running containers.

  [podman-system(1)](podman-system.html)             Manage podman.

  [podman-tag(1)](podman-tag.html)                   Add an additional name to a local image.

  [podman-top(1)](podman-top.html)                   Display the running processes of a
                                                     container.

  [podman-unmount(1)](podman-unmount.html)           Unmount a working container\'s root
                                                     filesystem.

  [podman-unpause(1)](podman-unpause.html)           Unpause one or more containers.

  [podman-unshare(1)](podman-unshare.html)           Run a command inside of a modified user
                                                     namespace.

  [podman-untag(1)](podman-untag.html)               Remove one or more names from a
                                                     locally-stored image.

  [podman-update(1)](podman-update.html)             Update the configuration of a given
                                                     container.

  [podman-version(1)](podman-version.html)           Display the Podman version information.

  [podman-volume(1)](podman-volume.html)             Simple management tool for volumes.

  [podman-wait(1)](podman-wait.html)                 Wait on one or more containers to stop and
                                                     print their exit codes.
  ---------------------------------------------------------------------------------------------

##  CONFIGURATION FILES

**containers.conf** (`/usr/share/containers/containers.conf`,
`/etc/containers/containers.conf`,
`$HOME/.config/containers/containers.conf`)

Podman has builtin defaults for command line options. These defaults can
be overridden using the containers.conf configuration files.

Distributions ship the `/usr/share/containers/containers.conf` file with
their default settings. Administrators can override fields in this file
by creating the `/etc/containers/containers.conf` file. Users can
further modify defaults by creating the
`$HOME/.config/containers/containers.conf` file. Podman merges its
builtin defaults with the specified fields from these files, if they
exist. Fields specified in the users file override the administrator\'s
file, which overrides the distribution\'s file, which override the
built-in defaults.

Podman uses builtin defaults if no containers.conf file is found.

If the **CONTAINERS_CONF** environment variable is set, then its value
is used for the containers.conf file rather than the default.

**mounts.conf** (`/usr/share/containers/mounts.conf`)

The mounts.conf file specifies volume mount directories that are
automatically mounted inside containers when executing the `podman run`
or `podman start` commands. Administrators can override the defaults
file by creating `/etc/containers/mounts.conf`.

When Podman runs in rootless mode, the file
`$HOME/.config/containers/mounts.conf` overrides the default if it
exists. For details, see containers-mounts.conf(5).

**policy.json** (`/etc/containers/policy.json`,
`$HOME/.config/containers/policy.json`)

Signature verification policy files are used to specify policy, e.g.
trusted keys, applicable when deciding whether to accept an image, or
individual signatures of that image, as valid. For details, see
containers-policy.json(5).

**registries.conf** (`/etc/containers/registries.conf`,
`$HOME/.config/containers/registries.conf`)

registries.conf is the configuration file which specifies which
container registries is consulted when completing image names which do
not include a registry or domain portion.

Non root users of Podman can create the
`$HOME/.config/containers/registries.conf` file to be used instead of
the system defaults.

If the **CONTAINERS_REGISTRIES_CONF** environment variable is set, then
its value is used for the registries.conf file rather than the default.

**storage.conf** (`/etc/containers/storage.conf`,
`$HOME/.config/containers/storage.conf`)

storage.conf is the storage configuration file for all tools using
containers/storage

The storage configuration file specifies all of the available container
storage options for tools using shared container storage.

When Podman runs in rootless mode, the file
`$HOME/.config/containers/storage.conf` is used instead of the system
defaults.

If the **CONTAINERS_STORAGE_CONF** environment variable is set, then its
value is used for the storage.conf file rather than the default.

##  Rootless mode

Podman can also be used as non-root user. When podman runs in rootless
mode, a user namespace is automatically created for the user, defined in
/etc/subuid and /etc/subgid.

Containers created by a non-root user are not visible to other users and
are not seen or managed by Podman running as root.

It is required to have multiple UIDS/GIDS set for a user. Be sure the
user is present in the files `/etc/subuid` and `/etc/subgid`.

Execute the following commands to add the ranges to the files

    $ sudo usermod --add-subuids 10000-75535 USERNAME
    $ sudo usermod --add-subgids 10000-75535 USERNAME

Or just add the content manually.

    $ echo USERNAME:10000:65536 >> /etc/subuid
    $ echo USERNAME:10000:65536 >> /etc/subgid

See the `subuid(5)` and `subgid(5)` man pages for more information.

Note: whitespace in any row of /etc/subuid or /etc/subgid, including
trailing blanks, may result in no entry failures.

Images are pulled under `XDG_DATA_HOME` when specified, otherwise in the
home directory of the user under `.local/share/containers/storage`.

Currently slirp4netns or pasta is required to be installed to create a
network device, otherwise rootless containers need to run in the network
namespace of the host.

In certain environments like HPC (High Performance Computing), users
cannot take advantage of the additional UIDs and GIDs from the
/etc/subuid and /etc/subgid systems. However, in this environment,
rootless Podman can operate with a single UID. To make this work, set
the `ignore_chown_errors` option in the `containers-storage.conf(5)`
file. This option tells Podman when pulling an image to ignore chown
errors when attempting to change a file in a container image to match
the non-root UID in the image. This means all files get saved as the
user\'s UID. Note this can cause issues when running the container.

### **NOTE:** Unsupported file systems in rootless mode

The Overlay file system (OverlayFS) is not supported with kernels prior
to 5.12.9 in rootless mode. The fuse-overlayfs package is a tool that
provides the functionality of OverlayFS in user namespace that allows
mounting file systems in rootless environments. It is recommended to
install the fuse-overlayfs package. In rootless mode, Podman
automatically uses the fuse-overlayfs program as the mount_program if
installed, as long as the \$HOME/.config/containers/storage.conf file
was not previously created. If storage.conf exists in the homedir, add
`mount_program = "/usr/bin/fuse-overlayfs"` under
`[storage.options.overlay]` to enable this feature.

The Network File System (NFS) and other distributed file systems (for
example: Lustre, Spectrum Scale, the General Parallel File System
(GPFS)) are not supported when running in rootless mode as these file
systems do not understand user namespace. However, rootless Podman can
make use of an NFS Homedir by modifying the
`$HOME/.config/containers/storage.conf` to have the `graphroot` option
point to a directory stored on local (Non NFS) storage.

##  SEE ALSO

**[containers-mounts.conf(5)](https://github.com/containers/common/blob/main/docs/containers-mounts.conf.5.md)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**,
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**,
**[containers-storage.conf(5)](https://github.com/containers/storage/blob/main/docs/containers-storage.conf.5.md)**,
**[buildah(1)](https://github.com/containers/buildah/blob/main/docs/buildah.html)**,
**oci-hooks(5)**,
**[containers-policy.json(5)](https://github.com/containers/image/blob/main/docs/containers-policy.json.5.md)**,
**[crun(1)](https://github.com/containers/crun/blob/main/crun.html)**,
**[runc(8)](https://github.com/opencontainers/runc/blob/main/man/runc.8.md)**,
**[subuid(5)](https://www.unix.com/man-page/linux/5/subuid)**,
**[subgid(5)](https://www.unix.com/man-page/linux/5/subgid)**,
**[slirp4netns(1)](https://github.com/rootless-containers/slirp4netns/blob/master/slirp4netns.html)**,
**[pasta(1)](https://passt.top/builds/latest/web/passt.1.html)**,
**[conmon(8)](https://github.com/containers/conmon/blob/main/docs/conmon.8.md)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

See
[podman-rootless(7)](https://github.com/containers/podman/blob/main/rootless.md)
for rootless issues.

##  HISTORY

Dec 2016, Originally compiled by Dan Walsh <dwalsh@redhat.com>


---

<a id='podman-history'></a>

## podman-history - Show the history of an image

##  NAME

podman-history - Show the history of an image

##  SYNOPSIS

**podman history** \[*options*\] *image*\[:*tag*\|[@\*digest]{.citation
cites="*digest"}\*\]

**podman image history** \[*options*\]
*image*\[:*tag*\|[@\*digest]{.citation cites="*digest"}\*\]

##  DESCRIPTION

**podman history** displays the history of an image by printing out
information about each layer used in the image. The information printed
out for each layer include Created (time and date), Created By, Size,
and Comment. The output can be truncated or not using the
**\--no-trunc** flag. If the **\--human** flag is set, the time of
creation and size are printed out in a human readable format. The
**\--quiet** flag displays the ID of the image only when set and the
**\--format** flag is used to print the information using the Go
template provided by the user.

##  OPTIONS

#### **\--format**=*format*

Alter the output for a format like \'json\' or a Go template.

Valid placeholders for the Go template are listed below:

  -----------------------------------------------------------------------
  **Placeholder**   **Description**
  ----------------- -----------------------------------------------------
  .Comment          Comment for the layer

  .Created          if \--human, time elapsed since creation, otherwise
                    time stamp of creation

  .CreatedAt        Time when the image layer was created

  .CreatedBy        Command used to create the layer

  .CreatedSince     Elapsed time since the image layer was created

  .ID               Image ID

  .Size             Size of layer on disk

  .Tags             Image tags
  -----------------------------------------------------------------------

#### **\--help**, **-h**

Print usage statement

#### **\--human**, **-H**

Display sizes and dates in human readable format (default *true*).

#### **\--no-trunc**

Do not truncate the output (default *false*).

#### **\--quiet**, **-q**

Print the numeric IDs only (default *false*).

##  EXAMPLES

Show the history of the specified image:

    $ podman history debian
    ID              CREATED       CREATED BY                                      SIZE       COMMENT
    b676ca55e4f2c   9 weeks ago   /bin/sh -c #(nop) CMD ["bash"]                  0 B
    <missing>       9 weeks ago   /bin/sh -c #(nop) ADD file:ebba725fb97cea4...   45.14 MB

Show the history of the specified image without truncating content and
using raw data:

    $ podman history --no-trunc=true --human=false debian
    ID              CREATED                CREATED BY                                      SIZE       COMMENT
    b676ca55e4f2c   2017-07-24T16:52:55Z   /bin/sh -c #(nop) CMD ["bash"]                  0
    <missing>       2017-07-24T16:52:54Z   /bin/sh -c #(nop) ADD file:ebba725fb97cea4...   45142935

Show the formatted history of the specified image:

    $ podman history --format "{{.ID}} {{.Created}}" debian
    b676ca55e4f2c   9 weeks ago
    <missing>       9 weeks ago

Show the history in JSON format for the specified image:

    $ podman history --format json debian
    [
        {
        "id": "b676ca55e4f2c0ce53d0636438c5372d3efeb5ae99b676fa5a5d1581bad46060",
        "created": "2017-07-24T16:52:55.195062314Z",
        "createdBy": "/bin/sh -c #(nop)  CMD [\"bash\"]",
        "size": 0,
        "comment": ""
        },
        {
        "id": "b676ca55e4f2c0ce53d0636438c5372d3efeb5ae99b676fa5a5d1581bad46060",
        "created": "2017-07-24T16:52:54.898893387Z",
        "createdBy": "/bin/sh -c #(nop) ADD file:ebba725fb97cea45d0b1b35ccc8144e766fcfc9a78530465c23b0c4674b14042 in / ",
        "size": 45142935,
        "comment": ""
        }
    ]

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

July 2017, Originally compiled by Urvashi Mohnani <umohnani@redhat.com>


---

<a id='podman-image'></a>

## podman-image - Manage images

##  NAME

podman-image - Manage images

##  SYNOPSIS

**podman image** *subcommand*

##  DESCRIPTION

The image command allows the management of images

##  COMMANDS

  -------------------------------------------------------------------------------------------------------
  Command   Man Page                                               Description
  --------- ------------------------------------------------------ --------------------------------------
  build     [podman-build(1)](podman-build.html)                   Build a container using a Dockerfile.

  diff      [podman-image-diff(1)](podman-image-diff.html)         Inspect changes on an image\'s
                                                                   filesystem.

  exists    [podman-image-exists(1)](podman-image-exists.html)     Check if an image exists in local
                                                                   storage.

  history   [podman-history(1)](podman-history.html)               Show the history of an image.

  import    [podman-import(1)](podman-import.html)                 Import a tarball and save it as a
                                                                   filesystem image.

  inspect   [podman-image-inspect(1)](podman-image-inspect.html)   Display an image\'s configuration.

  list      [podman-images(1)](podman-images.html)                 List the container images on the
                                                                   system.(alias ls)

  load      [podman-load(1)](podman-load.html)                     Load an image from the docker archive.

  mount     [podman-image-mount(1)](podman-image-mount.html)       Mount an image\'s root filesystem.

  prune     [podman-image-prune(1)](podman-image-prune.html)       Remove all unused images from the
                                                                   local store.

  pull      [podman-pull(1)](podman-pull.html)                     Pull an image from a registry.

  push      [podman-push(1)](podman-push.html)                     Push an image from local storage to
                                                                   elsewhere.

  rm        [podman-rmi(1)](podman-rmi.html)                       Remove one or more locally stored
                                                                   images.

  save      [podman-save(1)](podman-save.html)                     Save an image to docker-archive or
                                                                   oci.

  scp       [podman-image-scp(1)](podman-image-scp.html)           Securely copy an image from one host
                                                                   to another.

  search    [podman-search(1)](podman-search.html)                 Search a registry for an image.

  sign      [podman-image-sign(1)](podman-image-sign.html)         Create a signature for an image.

  tag       [podman-tag(1)](podman-tag.html)                       Add an additional name to a local
                                                                   image.

  tree      [podman-image-tree(1)](podman-image-tree.html)         Print layer hierarchy of an image in a
                                                                   tree format.

  trust     [podman-image-trust(1)](podman-image-trust.html)       Manage container registry image trust
                                                                   policy.

  unmount   [podman-image-unmount(1)](podman-image-unmount.html)   Unmount an image\'s root filesystem.

  untag     [podman-untag(1)](podman-untag.html)                   Remove one or more names from a
                                                                   locally-stored image.
  -------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**


---

<a id='podman-images'></a>

## podman-images - List images in local storage

##  NAME

podman-images - List images in local storage

##  SYNOPSIS

**podman images** \[*options*\] \[image\]

**podman image list** \[*options*\] \[image\]

**podman image ls** \[*options*\] \[image\]

##  DESCRIPTION

Displays locally stored images, their names, and their IDs.

##  OPTIONS

#### **\--all**, **-a**

Show all images (by default filter out the intermediate image layers).
The default is false.

#### **\--digests**

Show image digests

#### **\--filter**, **-f**=*filter*

Provide filter values.

The *filters* argument format is of `key=value` or `key!=value`. If
there is more than one *filter*, then pass multiple OPTIONS:
**\--filter** *foo=bar* **\--filter** *bif=baz*.

Supported filters:

  ----------------------------------------------------------------------------
      Filter     Description
  -------------- -------------------------------------------------------------
        id       Filter by image ID.

      before     Filter by images created before the given IMAGE (name or
                 tag).

    containers   Filter by images with a running container.

     dangling    Filter by dangling (unused) images.

      digest     Filter by digest.

   intermediate  Filter by images that are dangling and have no children

      label      Filter by images with (or without, in the case of
                 label!=\[\...\] is used) the specified labels.

     manifest    Filter by images that are manifest lists.

     readonly    Filter by read-only or read/write images.

    reference    Filter by image name.

   after/since   Filter by images created after the given IMAGE (name or tag).

      until      Filter by images created until the given duration or time.
  ----------------------------------------------------------------------------

The `id` *filter* accepts the image ID string.

The `before` *filter* accepts formats: `<image-name>[:<tag>]`,
`<image id>` or `<image@digest>`.

The `containers` *filter* shows images that have a running container
based on that image.

The `dangling` *filter* shows images that are taking up disk space and
serve no purpose. Dangling image is a file system layer that was used in
a previous build of an image and is no longer referenced by any image.
They are denoted with the `<none>` tag, consume disk space and serve no
active purpose.

The `digest` *filter* accepts the image digest string.

The `intermediate` *filter* shows images that are dangling and have no
children.

The `label` *filter* accepts two formats. One is the `label`=*key* or
`label`=*key*=*value*, which shows images with the specified labels. The
other format is the `label!`=*key* or `label!`=*key*=*value*, which
shows images without the specified labels.

The `manifest` *filter* shows images that are manifest lists.

The `readonly` *filter* shows, as a default, both read-only and
read/write images. Read-only images can be configured by modifying the
`additionalimagestores` in the `/etc/containers/storage.conf` file.

The `reference` *filter* accepts the pattern of an image reference
`<image-name>[:<tag>]`.

The `after` or `since` *filter* accepts formats: `<image-name>[:<tag>]`,
`<image id>` or `<image@digest>`.

The `until` *filter* accepts formats: golang duration, RFC3339 time, or
a Unix timestamp and shows all images that are created until that time.

#### **\--format**=*format*

Change the default output format. This can be of a supported type like
\'json\' or a Go template. Valid placeholders for the Go template are
listed below:

  ------------------------------------------------------------------------
  **Placeholder**   **Description**
  ----------------- ------------------------------------------------------
  .Containers       Number of containers using this image

  .Created          Elapsed time since the image was created

  .CreatedAt        Time when the image was created, YYYY-MM-DD HH:MM:SS
                    +nnnn

  .CreatedSince     Same as .Created

  .CreatedTime      Same as .CreatedAt

  .Dangling         Same as .IsDangling

  .Digest           Image digest

  .History          History of the image layer

  .ID               Image ID (truncated)

  .Id               Image ID (full SHA)

  .IsDangling       Is image dangling? (true/false)

  .IsReadOnly       Is unage read-only? (true/false)

  .Labels \...      map\[\] of labels

  .Names            Image FQIN

  .ParentId         Full SHA of parent image ID, or null (string)

  .ReadOnly         Same as .IsReadOnly

  .RepoDigests      map\[\] of zero or more repo/name@sha256:SHA strings

  .Repository       Image repository

  .RepoTags         map\[\] of zero or more FQIN strings for this image

  .SharedSize       Always seems to be 0

  .Size             Size of layer on disk (human-friendly string)

  .Tag              Image tag

  .VirtualSize      Size of layer on disk (bytes)
  ------------------------------------------------------------------------

#### **\--history**

Display the history of image names. If an image gets re-tagged or
untagged, then the image name history gets prepended (latest image
first). This is especially useful when undoing a tag operation or an
image does not contain any name because it has been untagged.

#### **\--no-trunc**

Do not truncate the output (default *false*).

#### **\--noheading**, **-n**

Omit the table headings from the listing.

#### **\--quiet**, **-q**

Lists only the image IDs.

#### **\--sort**=*sort*

Sort by *created*, *id*, *repository*, *size* or *tag* (default:
**created**) When sorting by *repository* it also sorts by the *tag* as
second criteria to provide a stable output.

##  EXAMPLE

List all non-dangling images in local storage:

    $ podman images
    REPOSITORY                         TAG         IMAGE ID      CREATED       SIZE
    quay.io/podman/stable              latest      e0b7dabc3352  22 hours ago  331 MB
    docker.io/library/alpine           latest      9c6f07244728  5 days ago    5.83 MB
    registry.fedoraproject.org/fedora  latest      2ecb6df95994  3 weeks ago   169 MB
    quay.io/libpod/testimage           20220615    f26aa69bb3f3  2 months ago  8.4 MB

List all images matching the specified name:

    $ podman images stable
    REPOSITORY             TAG         IMAGE ID      CREATED       SIZE
    quay.io/podman/stable  latest      e0b7dabc3352  22 hours ago  331 MB

List image ids of all images in containers storage:

    # podman image ls --quiet
    e3d42bcaf643
    ebb91b73692b
    4526339ae51c

List all images without showing the headers:

    # podman images --noheading
    docker.io/kubernetes/pause                   latest   e3d42bcaf643   3 years ago   251 kB
    <none>                                       <none>   ebb91b73692b   4 weeks ago   27.2 MB
    docker.io/library/ubuntu                     latest   4526339ae51c   6 weeks ago   126 MB

List all images without truncating output:

    # podman image list --no-trunc
    REPOSITORY                                   TAG      IMAGE ID                                                                  CREATED       SIZE
    docker.io/kubernetes/pause                   latest   sha256:e3d42bcaf643097dd1bb0385658ae8cbe100a80f773555c44690d22c25d16b27   3 years ago   251 kB
    <none>                                       <none>   sha256:ebb91b73692bd27890685846412ae338d13552165eacf7fcd5f139bfa9c2d6d9   4 weeks ago   27.2 MB
    docker.io/library/ubuntu                     latest   sha256:4526339ae51c3cdc97956a7a961c193c39dfc6bd9733b0d762a36c6881b5583a   6 weeks ago   126 MB

List all image content with the formatted content:

    # podman images --format "table {{.ID}} {{.Repository}} {{.Tag}}"
    IMAGE ID       REPOSITORY                                   TAG
    e3d42bcaf643   docker.io/kubernetes/pause                   latest
    ebb91b73692b   <none>                                       <none>
    4526339ae51c   docker.io/library/ubuntu                     latest

List any image that is not tagged with a name (dangling):

    # podman images --filter dangling=true
    REPOSITORY   TAG      IMAGE ID       CREATED       SIZE
    <none>       <none>   ebb91b73692b   4 weeks ago   27.2 MB

List all images in JSON format:

    # podman images --format json
    [
        {
        "id": "e3d42bcaf643097dd1bb0385658ae8cbe100a80f773555c44690d22c25d16b27",
        "names": [
            "docker.io/kubernetes/pause:latest"
        ],
        "digest": "sha256:0aecf73ff86844324847883f2e916d3f6984c5fae3c2f23e91d66f549fe7d423",
        "created": "2014-07-19T07:02:32.267701596Z",
        "size": 250665
        },
        {
        "id": "ebb91b73692bd27890685846412ae338d13552165eacf7fcd5f139bfa9c2d6d9",
        "names": [
            "\u003cnone\u003e"
        ],
        "digest": "sha256:ba7e4091d27e8114a205003ca6a768905c3395d961624a2c78873d9526461032",
        "created": "2017-10-26T03:07:22.796184288Z",
        "size": 27170520
        },
        {
        "id": "4526339ae51c3cdc97956a7a961c193c39dfc6bd9733b0d762a36c6881b5583a",
        "names": [
            "docker.io/library/ubuntu:latest"
        ],
        "digest": "sha256:193f7734ddd68e0fb24ba9af8c2b673aecb0227b026871f8e932dab45add7753",
        "created": "2017-10-10T20:59:05.10196344Z",
        "size": 126085200
        }
    ]

List all images sorted by the specified column:

    # podman images --sort repository
    REPOSITORY                                   TAG      IMAGE ID       CREATED       SIZE
    <none>                                      <none>   2460217d76fc   About a minute ago   4.41 MB
    docker.io/library/alpine                    latest   3fd9065eaf02   5 months ago         4.41 MB
    localhost/myapp                             latest   b2e0ad03474a   About a minute ago   4.41 MB
    registry.access.redhat.com/rhel7            latest   7a840db7f020   2 weeks ago          211 MB
    registry.fedoraproject.org/fedora           27       801894bc0e43   6 weeks ago          246 MB

Show the difference between listed images in use versus all images,
including dangling images:

    # podman images
    REPOSITORY                 TAG      IMAGE ID       CREATED         SIZE
    localhost/test             latest   18f0c080cd72   4 seconds ago   4.42 MB
    docker.io/library/alpine   latest   3fd9065eaf02   5 months ago    4.41 MB
    # podman images -a
    REPOSITORY                 TAG      IMAGE ID       CREATED         SIZE
    localhost/test             latest   18f0c080cd72   6 seconds ago   4.42 MB
    <none>                     <none>   270e70dc54c0   7 seconds ago   4.42 MB
    <none>                     <none>   4ed6fbe43414   8 seconds ago   4.41 MB
    <none>                     <none>   6b0df8e71508   8 seconds ago   4.41 MB
    docker.io/library/alpine   latest   3fd9065eaf02   5 months ago    4.41 MB

##  SEE ALSO

**[podman(1)](podman.html)**,
**[containers-storage.conf(5)](https://github.com/containers/storage/blob/main/docs/containers-storage.conf.5.md)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

##  HISTORY

March 2017, Originally compiled by Dan Walsh `<dwalsh@redhat.com>`


---

<a id='podman-import'></a>

## podman-import - Import a tarball and save it as a filesystem
image

##  NAME

podman-import - Import a tarball and save it as a filesystem image

##  SYNOPSIS

**podman import** \[*options*\] *path* \[*reference*\]

**podman image import** \[*options*\] *path* \[*reference*\]

##  DESCRIPTION

**podman import** imports a tarball (.tar, .tar.gz, .tgz, .bzip,
.tar.xz, .txz) and saves it as a filesystem image. Remote tarballs can
be specified using a URL. Various image instructions can be configured
with the **\--change** flag and a commit message can be set using the
**\--message** flag. **reference**, if present, is a tag to assign to
the image. **podman import** is used for importing from the archive
generated by **podman export**, that includes the container\'s
filesystem. To import the archive of image layers created by **podman
save**, use **podman load**. Note: `:` is a restricted character and
cannot be part of the file name.

##  OPTIONS

#### **\--arch**

Set architecture of the imported image.

#### **\--change**, **-c**=*instruction*

Apply the following possible instructions to the created image: **CMD**
\| **ENTRYPOINT** \| **ENV** \| **EXPOSE** \| **LABEL** \|
**STOPSIGNAL** \| **USER** \| **VOLUME** \| **WORKDIR**

Can be set multiple times

#### **\--help**, **-h**

Print usage statement

#### **\--message**, **-m**=*message*

Set commit message for imported image

#### **\--os**

Set OS of the imported image.

#### **\--quiet**, **-q**

Shows progress on the import

#### **\--variant**

Set variant of the imported image.

##  EXAMPLES

Import the selected tarball into new image, specifying the CMD,
ENTRYPOINT and LABEL:

    $ podman import --change CMD=/bin/bash --change ENTRYPOINT=/bin/sh --change LABEL=blue=image ctr.tar image-imported
    Getting image source signatures
    Copying blob sha256:b41deda5a2feb1f03a5c1bb38c598cbc12c9ccd675f438edc6acd815f7585b86
     25.80 MB / 25.80 MB [======================================================] 0s
    Copying config sha256:c16a6d30f3782288ec4e7521c754acc29d37155629cb39149756f486dae2d4cd
     448 B / 448 B [============================================================] 0s
    Writing manifest to image destination
    Storing signatures
    db65d991f3bbf7f31ed1064db9a6ced7652e3f8166c4736aa9133dadd3c7acb3

Import the selected tarball into new image, specifying the CMD,
ENTRYPOINT and LABEL:

    $ podman import --change 'ENTRYPOINT ["/bin/sh","-c","test-image"]'  --change LABEL=blue=image test-image.tar image-imported
    Getting image source signatures
    Copying blob e3b0c44298fc skipped: already exists
    Copying config 1105523502 done
    Writing manifest to image destination
    Storing signatures
    110552350206337183ceadc0bdd646dc356e06514c548b69a8917b4182414b

Import new tagged image from stdin in quiet mode:

    $ cat ctr.tar | podman -q import --message "importing the ctr.tar file" - image-imported
    db65d991f3bbf7f31ed1064db9a6ced7652e3f8166c4736aa9133dadd3c7acb3

Import an image from stdin:

    $ cat ctr.tar | podman import -
    Getting image source signatures
    Copying blob sha256:b41deda5a2feb1f03a5c1bb38c598cbc12c9ccd675f438edc6acd815f7585b86
     25.80 MB / 25.80 MB [======================================================] 0s
    Copying config sha256:d61387b4d5edf65edee5353e2340783703074ffeaaac529cde97a8357eea7645
     378 B / 378 B [============================================================] 0s
    Writing manifest to image destination
    Storing signatures
    db65d991f3bbf7f31ed1064db9a6ced7652e3f8166c4736aa9133dadd3c7acb3

Import named image from tarball via a URL:

    $ podman import http://example.com/ctr.tar url-image
    Downloading from "http://example.com/ctr.tar"
    Getting image source signatures
    Copying blob sha256:b41deda5a2feb1f03a5c1bb38c598cbc12c9ccd675f438edc6acd815f7585b86
     25.80 MB / 25.80 MB [======================================================] 0s
    Copying config sha256:5813fe8a3b18696089fd09957a12e88bda43dc1745b5240879ffffe93240d29a
     419 B / 419 B [============================================================] 0s
    Writing manifest to image destination
    Storing signatures
    db65d991f3bbf7f31ed1064db9a6ced7652e3f8166c4736aa9133dadd3c7acb3

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-export(1)](podman-export.html)**

##  HISTORY

November 2017, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-info'></a>

## podman-info - Display Podman related system information

##  NAME

podman-info - Display Podman related system information

##  SYNOPSIS

**podman info** \[*options*\]

**podman system info** \[*options*\]

##  DESCRIPTION

Displays information pertinent to the host, current storage stats,
configured container registries, and build of podman.

##  OPTIONS

#### **\--format**, **-f**=*format*

Change output format to \"json\" or a Go template.

  **Placeholder**    **Info pertaining to \...**
  ------------------ -----------------------------------------
  .Host \...         \...the host on which podman is running
  .Plugins \...      \...external plugins
  .Registries \...   \...configured registries
  .Store \...        \...the storage driver and paths
  .Version \...      \...podman version

Each of the above branch out into further subfields, more than can
reasonably be enumerated in this document.

##  EXAMPLES

Run `podman info` for a YAML formatted response:

    $ podman info
    host:
      arch: amd64
      buildahVersion: 1.23.0
      cgroupControllers: []
      cgroupManager: systemd
      cgroupVersion: v2
      conmon:
        package: conmon-2.0.29-2.fc34.x86_64
        path: /usr/bin/conmon
        version: 'conmon version 2.0.29, commit: '
     cpu_utilization:
       idle_percent: 96.84
       system_percent: 0.71
       user_percent: 2.45
      cpus: 8
      distribution:
        distribution: fedora
        variant: workstation
        version: "34"
      eventLogger: journald
      hostname: localhost.localdomain
      idMappings:
        gidmap:
        - container_id: 0
          host_id: 3267
          size: 1
        - container_id: 1
          host_id: 100000
          size: 65536
        uidmap:
        - container_id: 0
          host_id: 3267
          size: 1
        - container_id: 1
          host_id: 100000
          size: 65536
      kernel: 5.13.13-200.fc34.x86_64
      linkmode: dynamic
      logDriver: journald
      memFree: 1833385984
      memTotal: 16401895424
      networkBackend: cni
      networkBackendInfo:
        backend: cni
        dns:
          package: podman-plugins-3.4.4-1.fc34.x86_64
          path: /usr/libexec/cni/dnsname
          version: |-
            CNI dnsname plugin
            version: 1.3.1
            commit: unknown
        package: |-
          containernetworking-plugins-1.0.1-1.fc34.x86_64
          podman-plugins-3.4.4-1.fc34.x86_64
        path: /usr/libexec/cni
      ociRuntime:
        name: crun
        package: crun-1.0-1.fc34.x86_64
        path: /usr/bin/crun
        version: |-
          crun version 1.0
          commit: 139dc6971e2f1d931af520188763e984d6cdfbf8
          spec: 1.0.0
          +SYSTEMD +SELINUX +APPARMOR +CAP +SECCOMP +EBPF +CRIU +YAJL
      os: linux
      pasta:
        executable: /usr/bin/passt
        package: passt-0^20221116.gace074c-1.fc34.x86_64
        version: |
          passt 0^20221116.gace074c-1.fc34.x86_64
          Copyright Red Hat
          GNU Affero GPL version 3 or later <https://www.gnu.org/licenses/agpl-3.0.html>
          This is free software: you are free to change and redistribute it.
          There is NO WARRANTY, to the extent permitted by law.
      remoteSocket:
        path: /run/user/3267/podman/podman.sock
      security:
        apparmorEnabled: false
        capabilities: CAP_CHOWN,CAP_DAC_OVERRIDE,CAP_FOWNER,CAP_FSETID,CAP_KILL,CAP_NET_BIND_SERVICE,CAP_SETFCAP,CAP_SETGID,CAP_SETPCAP,CAP_SETUID
        rootless: true
        seccompEnabled: true
        seccompProfilePath: /usr/share/containers/seccomp.json
        selinuxEnabled: true
      serviceIsRemote: false
      slirp4netns:
        executable: /bin/slirp4netns
        package: slirp4netns-1.1.12-2.fc34.x86_64
        version: |-
          slirp4netns version 1.1.12
          commit: 7a104a101aa3278a2152351a082a6df71f57c9a3
          libslirp: 4.4.0
          SLIRP_CONFIG_VERSION_MAX: 3
          libseccomp: 2.5.0
      swapFree: 15687475200
      swapTotal: 16886259712
      uptime: 47h 15m 9.91s (Approximately 1.96 days)
    plugins:
      log:
      - k8s-file
      - none
      - journald
      network:
      - bridge
      - macvlan
      volume:
      - local
    registries:
      search:
      - registry.fedoraproject.org
      - registry.access.redhat.com
      - docker.io
      - quay.io
    store:
      configFile: /home/dwalsh/.config/containers/storage.conf
      containerStore:
        number: 9
        paused: 0
        running: 1
        stopped: 8
      graphDriverName: overlay
      graphOptions: {}
      graphRoot: /home/dwalsh/.local/share/containers/storage
      graphRootAllocated: 510389125120
      graphRootUsed: 129170714624
      graphStatus:
        Backing Filesystem: extfs
        Native Overlay Diff: "true"
        Supports d_type: "true"
        Using metacopy: "false"
      imageCopyTmpDir: /home/dwalsh/.local/share/containers/storage/tmp
      imageStore:
        number: 5
      runRoot: /run/user/3267/containers
      transientStore: false
      volumePath: /home/dwalsh/.local/share/containers/storage/volumes
    version:
      APIVersion: 4.0.0
      Built: 1631648722
      BuiltTime: Tue Sep 14 15:45:22 2021
      GitCommit: 23677f92dd83e96d2bc8f0acb611865fb8b1a56d
      GoVersion: go1.16.6
      OsArch: linux/amd64
      Version: 4.0.0

Run `podman info --format json` for a JSON formatted response:

    $ podman info --format json
    {
      "host": {
        "arch": "amd64",
        "buildahVersion": "1.23.0",
        "cgroupManager": "systemd",
        "cgroupVersion": "v2",
        "cgroupControllers": [],
        "conmon": {
          "package": "conmon-2.0.29-2.fc34.x86_64",
          "path": "/usr/bin/conmon",
          "version": "conmon version 2.0.29, commit: "
        },
        "cpus": 8,
        "distribution": {
          "distribution": "fedora",
          "version": "34"
        },
        "eventLogger": "journald",
        "hostname": "localhost.localdomain",
        "idMappings": {
          "gidmap": [
        {
          "container_id": 0,
          "host_id": 3267,
          "size": 1
        },
        {
          "container_id": 1,
          "host_id": 100000,
          "size": 65536
        }
          ],
          "uidmap": [
        {
          "container_id": 0,
          "host_id": 3267,
          "size": 1
        },
        {
          "container_id": 1,
          "host_id": 100000,
          "size": 65536
        }
          ]
        },
        "kernel": "5.13.13-200.fc34.x86_64",
        "logDriver": "journald",
        "memFree": 1785753600,
        "memTotal": 16401895424,
        "networkBackend": "cni",
        "networkBackendInfo": {
          "backend": "cni",
          "package": "containernetworking-plugins-1.0.1-1.fc34.x86_64\npodman-plugins-3.4.4-1.fc34.x86_64",
          "path": "/usr/libexec/cni",
          "dns": {
            "version": "CNI dnsname plugin\nversion: 1.3.1\ncommit: unknown",
            "package": "podman-plugins-3.4.4-1.fc34.x86_64",
            "path": "/usr/libexec/cni/dnsname"
          }
        },
        "ociRuntime": {
          "name": "crun",
          "package": "crun-1.0-1.fc34.x86_64",
          "path": "/usr/bin/crun",
          "version": "crun version 1.0\ncommit: 139dc6971e2f1d931af520188763e984d6cdfbf8\nspec: 1.0.0\n+SYSTEMD +SELINUX +APPARMOR +CAP +SECCOMP +EBPF +CRIU +YAJL"
        },
        "os": "linux",
        "remoteSocket": {
          "path": "/run/user/3267/podman/podman.sock"
        },
        "serviceIsRemote": false,
        "security": {
          "apparmorEnabled": false,
          "capabilities": "CAP_CHOWN,CAP_DAC_OVERRIDE,CAP_FOWNER,CAP_FSETID,CAP_KILL,CAP_NET_BIND_SERVICE,CAP_SETFCAP,CAP_SETGID,CAP_SETPCAP,CAP_SETUID",
          "rootless": true,
          "seccompEnabled": true,
          "seccompProfilePath": "/usr/share/containers/seccomp.json",
          "selinuxEnabled": true
        },
        "slirp4netns": {
          "executable": "/bin/slirp4netns",
          "package": "slirp4netns-1.1.12-2.fc34.x86_64",
          "version": "slirp4netns version 1.1.12\ncommit: 7a104a101aa3278a2152351a082a6df71f57c9a3\nlibslirp: 4.4.0\nSLIRP_CONFIG_VERSION_MAX: 3\nlibseccomp: 2.5.0"
        },
        "pasta": {
          "executable": "/usr/bin/passt",
          "package": "passt-0^20221116.gace074c-1.fc34.x86_64",
          "version": "passt 0^20221116.gace074c-1.fc34.x86_64\nCopyright Red Hat\nGNU Affero GPL version 3 or later \u003chttps://www.gnu.org/licenses/agpl-3.0.html\u003e\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n"
        },
        "swapFree": 15687475200,
        "swapTotal": 16886259712,
        "uptime": "47h 17m 29.75s (Approximately 1.96 days)",
        "linkmode": "dynamic"
      },
      "store": {
        "configFile": "/home/dwalsh/.config/containers/storage.conf",
        "containerStore": {
          "number": 9,
          "paused": 0,
          "running": 1,
          "stopped": 8
        },
        "graphDriverName": "overlay",
        "graphOptions": {

        },
        "graphRoot": "/home/dwalsh/.local/share/containers/storage",
        "graphStatus": {
          "Backing Filesystem": "extfs",
          "Native Overlay Diff": "true",
          "Supports d_type": "true",
          "Using metacopy": "false"
        },
        "imageCopyTmpDir": "/home/dwalsh/.local/share/containers/storage/tmp",
        "imageStore": {
          "number": 5
        },
        "runRoot": "/run/user/3267/containers",
        "volumePath": "/home/dwalsh/.local/share/containers/storage/volumes",
        "transientStore": false
      },
      "registries": {
        "search": [
      "registry.fedoraproject.org",
      "registry.access.redhat.com",
      "docker.io",
      "quay.io"
    ]
      },
      "plugins": {
        "volume": [
          "local"
        ],
        "network": [
          "bridge",
          "macvlan"
        ],
        "log": [
          "k8s-file",
          "none",
          "journald"
        ]
      },
      "version": {
        "APIVersion": "4.0.0",
        "Version": "4.0.0",
        "GoVersion": "go1.16.6",
        "GitCommit": "23677f92dd83e96d2bc8f0acb611865fb8b1a56d",
        "BuiltTime": "Tue Sep 14 15:45:22 2021",
        "Built": 1631648722,
        "OsArch": "linux/amd64"
      }
    }

#### Extracting the list of container registries with a Go template

If shell completion is enabled, type `podman info --format={{.` and then
press `[TAB]` twice.

    $ podman info --format={{.
    {{.Host.         {{.Plugins.      {{.Registries}}  {{.Store.        {{.Version.

Press `R` `[TAB]` `[ENTER]` to print the registries information.

    $ podman info -f {{.Registries}}
    map[search:[registry.fedoraproject.org registry.access.redhat.com docker.io quay.io]]
    $

The output still contains a map and an array. The map value can be
extracted with

    $ podman info -f '{{index .Registries "search"}}'
    [registry.fedoraproject.org registry.access.redhat.com docker.io quay.io]

The array can be printed as one entry per line

    $ podman info -f '{{range index .Registries "search"}}{{.}}\n{{end}}'
    registry.fedoraproject.org
    registry.access.redhat.com
    docker.io
    quay.io

#### Extracting the list of container registries from JSON with jq

The command-line JSON processor [**jq**](https://stedolan.github.io/jq/)
can be used to extract the list of container registries.

    $ podman info -f json | jq '.registries["search"]'
    [
      "registry.fedoraproject.org",
      "registry.access.redhat.com",
      "docker.io",
      "quay.io"
    ]

The array can be printed as one entry per line

    $ podman info -f json | jq -r '.registries["search"] | .[]'
    registry.fedoraproject.org
    registry.access.redhat.com
    docker.io
    quay.io

Note, the Go template struct fields start with upper case. When running
`podman info` or `podman info --format=json`, the same names start with
lower case.

##  SEE ALSO

**[podman(1)](podman.html)**,
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**,
**[containers-storage.conf(5)](https://github.com/containers/storage/blob/main/docs/containers-storage.conf.5.md)**


---

<a id='podman-init'></a>

## podman-init - Initialize one or more containers

##  NAME

podman-init - Initialize one or more containers

##  SYNOPSIS

**podman init** \[*options*\] *container* \[*container*\...\]

**podman container init** \[*options*\] *container* \[*container*\...\]

##  DESCRIPTION

Initialize one or more containers. You may use container IDs or names as
input. Initializing a container performs all tasks necessary for
starting the container (mounting filesystems, creating an OCI spec,
initializing the container network) but does not start the container. If
a container is not initialized, the `podman start` and `podman run`
commands initialize it automatically prior to starting it. This command
is intended to be used for inspecting a container\'s filesystem or OCI
spec prior to starting it. This can be used to inspect the container
before it runs, or debug why a container is failing to run.

##  OPTIONS

#### **\--all**, **-a**

Initialize all containers. Containers that have already initialized
(including containers that have been started and are running) are
ignored.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

##  EXAMPLE

Initialize specified container with a given ID.

    podman init 35480fc9d568

Initialize specified container with a given name.

    podman init test1

Initialize the latest container. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

    podman init --latest

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-start(1)](podman-start.html)**

##  HISTORY

April 2019, Originally compiled by Matthew Heon <mheon@redhat.com>


---

<a id='podman-inspect'></a>

## podman-inspect - Display a container, image, volume, network, or
pod's configuration

##  NAME

podman-inspect - Display a container, image, volume, network, or pod\'s
configuration

##  SYNOPSIS

**podman inspect** \[*options*\] *name* \[\...\]

##  DESCRIPTION

This displays the low-level information on containers and images
identified by name or ID. By default, this renders all results in a JSON
array. If the inspect type is all, the order of inspection is:
containers, images, volumes, network, pods. If a container has the same
name as an image, then the container JSON is returned, and so on. If a
format is specified, the given template is executed for each result.

For more inspection options, see also
[podman-container-inspect(1)](podman-container-inspect.html),
[podman-image-inspect(1)](podman-image-inspect.html),
[podman-network-inspect(1)](podman-network-inspect.html),
[podman-pod-inspect(1)](podman-pod-inspect.html), and
[podman-volume-inspect(1)](podman-volume-inspect.html).

##  OPTIONS

#### **\--format**, **-f**=*format*

Format the output using the given Go template. The keys of the returned
JSON can be used as the values for the \--format flag (see examples
below).

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--size**, **-s**

In addition to normal output, display the total file size if the type is
a container.

#### **\--type**, **-t**=*type*

Return JSON for the specified type. Type can be \'container\',
\'image\', \'volume\', \'network\', \'pod\', or \'all\' (default: all)
(Only meaningful when invoked as *podman inspect*)

##  EXAMPLE

Inspect the fedora image:

    # podman inspect fedora
    [
        {
            "Id": "f0858ad3febdf45bb2e5501cb459affffacef081f79eaa436085c3b6d9bd46ca",
            "Digest": "sha256:d4f7df6b691d61af6cee7328f82f1d8afdef63bc38f58516858ae3045083924a",
            "RepoTags": [
                "docker.io/library/fedora:latest"
            ],
            "RepoDigests": [
                "docker.io/library/fedora@sha256:8fa60b88e2a7eac8460b9c0104b877f1aa0cea7fbc03c701b7e545dacccfb433",
                "docker.io/library/fedora@sha256:d4f7df6b691d61af6cee7328f82f1d8afdef63bc38f58516858ae3045083924a"
            ],
            "Parent": "",
            "Comment": "",
            "Created": "2019-10-29T03:23:37.695123423Z",
            "Config": {
                "Env": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "DISTTAG=f31-updates-candidatecontainer",
                    "FGC=f31-updates-candidate",
                    "FBR=f31-updates-candidate"
                ],
                "Cmd": [
                    "/bin/bash"
                ],
                "Labels": {
                    "maintainer": "Clement Verna \u003ccverna@fedoraproject.org\u003e"
                }
            },
            "Version": "18.06.1-ce",
            "Author": "",
            "Architecture": "amd64",
            "Os": "linux",
            "Size": 201096840,
            "VirtualSize": 201096840,
            "GraphDriver": {
                "Name": "overlay",
                "Data": {
                    "UpperDir": "/home/user/.local/share/containers/storage/overlay/2ae3cee18c8ef9e0d448649747dab81c4f1ca2714a8c4550eff49574cab262c9/diff",
                    "WorkDir": "/home/user/.local/share/containers/storage/overlay/2ae3cee18c8ef9e0d448649747dab81c4f1ca2714a8c4550eff49574cab262c9/work"
                }
            },
            "RootFS": {
                "Type": "layers",
                "Layers": [
                    "sha256:2ae3cee18c8ef9e0d448649747dab81c4f1ca2714a8c4550eff49574cab262c9"
                ]
            },
            "Labels": {
                "maintainer": "Clement Verna \u003ccverna@fedoraproject.org\u003e"
            },
            "Annotations": {},
            "ManifestType": "application/vnd.docker.distribution.manifest.v2+json",
            "User": "",
            "History": [
                {
                    "created": "2019-01-16T21:21:55.569693599Z",
                    "created_by": "/bin/sh -c #(nop)  LABEL maintainer=Clement Verna \u003ccverna@fedoraproject.org\u003e",
                    "empty_layer": true
                },
                {
                    "created": "2019-09-27T21:21:07.784469821Z",
                    "created_by": "/bin/sh -c #(nop)  ENV DISTTAG=f31-updates-candidatecontainer FGC=f31-updates-candidate FBR=f31-updates-candidate",
                    "empty_layer": true
                },
                {
                    "created": "2019-10-29T03:23:37.355187998Z",
                    "created_by": "/bin/sh -c #(nop) ADD file:298f828afc880ccde9205fc4418435d5e696ad165e283f0530d0b1a74326d6dc in / "
                },
                {
                    "created": "2019-10-29T03:23:37.695123423Z",
                    "created_by": "/bin/sh -c #(nop)  CMD [\"/bin/bash\"]",
                    "empty_layer": true
                }
            ],
            "NamesHistory": []
        }
    ]

Inspect the specified image with the `ImageName` format specifier:

    # podman inspect a04 --format "{{.ImageName}}"
    fedora

Inspect the specified image for `GraphDriver` format specifier:

    # podman inspect a04 --format "{{.GraphDriver.Name}}"
    overlay

Inspect the specified image for its `Size` format specifier:

    # podman image inspect --format "size: {{.Size}}" alpine
    size:   4405240

Inspect the latest container created for `EffectiveCaps` format
specifier. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines):

    podman container inspect --latest --format {{.EffectiveCaps}}
    [CAP_CHOWN CAP_DAC_OVERRIDE CAP_FSETID CAP_FOWNER CAP_SETGID CAP_SETUID CAP_SETFCAP CAP_SETPCAP CAP_NET_BIND_SERVICE CAP_KILL]

Inspect the specified pod for the `Name` format specifier:

    # podman inspect myPod --type pod --format "{{.Name}}"
    myPod

Inspect the specified volume for the `Name` format specifier:

    # podman inspect myVolume --type volume --format "{{.Name}}"
    myVolume

Inspect the specified network for the `Name` format specifier:

    # podman inspect nyNetwork --type network --format "{{.name}}"
    myNetwork

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-container-inspect(1)](podman-container-inspect.html)**,
**[podman-image-inspect(1)](podman-image-inspect.html)**,
**[podman-network-inspect(1)](podman-network-inspect.html)**,
**[podman-pod-inspect(1)](podman-pod-inspect.html)**,
**[podman-volume-inspect(1)](podman-volume-inspect.html)**

##  HISTORY

July 2017, Originally compiled by Dan Walsh <dwalsh@redhat.com>


---

<a id='podman-kill'></a>

## podman-kill - Kill the main process in one or more containers

##  NAME

podman-kill - Kill the main process in one or more containers

##  SYNOPSIS

**podman kill** \[*options*\] \[*container* \...\]

**podman container kill** \[*options*\] \[*container* \...\]

##  DESCRIPTION

The main process inside each container specified is sent SIGKILL or any
signal specified with the `--signal` option.

##  OPTIONS

#### **\--all**, **-a**

Signal all running and paused containers.

#### **\--cidfile**=*file*

Read container ID from the specified *file* and kill the container. Can
be specified multiple times.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--signal**, **-s**=**signal**

Signal to send to the container. For more information on Linux signals,
refer to *signal(7)*. The default is **SIGKILL**.

##  EXAMPLE

Kill container with a given name:

    podman kill mywebserver

Kill container with a given ID:

    podman kill 860a4b23

Terminate container by sending `TERM` signal:

    podman kill --signal TERM 860a4b23

Kill the latest container. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines):

    podman kill --latest

Terminate all containers by sending `KILL` signal:

    podman kill --signal KILL -a

Kill containers using ID specified in a given files:

    podman kill --cidfile /home/user/cidfile-1
    podman kill --cidfile /home/user/cidfile-1 --cidfile ./cidfile-2

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-stop(1)](podman-stop.html)**

##  HISTORY

September 2017, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-kube'></a>

## podman-kube - Play containers, pods or volumes based on a structured
input file

##  NAME

podman-kube - Play containers, pods or volumes based on a structured
input file

##  SYNOPSIS

**podman kube** *subcommand*

##  DESCRIPTION

The kube command recreates containers, pods or volumes based on the
input from a structured (like YAML) file input. Containers are
automatically started.

Note: The kube commands in podman focus on simplifying the process of
moving containers from podman to a Kubernetes environment and from a
Kubernetes environment back to podman. Podman is not replicating the
kubectl CLI. Once containers are deployed to a Kubernetes cluster from
podman, please use `kubectl` to manage the workloads in the cluster.

##  COMMANDS

  ---------------------------------------------------------------------------------------------------------
  Command    Man Page                                               Description
  ---------- ------------------------------------------------------ ---------------------------------------
  apply      [podman-kube-apply(1)](podman-kube-apply.html)         Apply Kubernetes YAML based on
                                                                    containers, pods, or volumes to a
                                                                    Kubernetes cluster

  down       [podman-kube-down(1)](podman-kube-down.html)           Remove containers and pods based on
                                                                    Kubernetes YAML.

  generate   [podman-kube-generate(1)](podman-kube-generate.html)   Generate Kubernetes YAML based on
                                                                    containers, pods or volumes.

  play       [podman-kube-play(1)](podman-kube-play.html)           Create containers, pods and volumes
                                                                    based on Kubernetes YAML.
  ---------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pod(1)](podman-pod.html)**,
**[podman-container(1)](podman-container.html)**,
**[podman-kube-play(1)](podman-kube-play.html)**,
**[podman-kube-down(1)](podman-kube-down.html)**,
**[podman-kube-generate(1)](podman-kube-generate.html)**,
**[podman-kube-apply(1)](podman-kube-apply.html)**

##  HISTORY

December 2018, Originally compiled by Brent Baude (bbaude at redhat dot
com)


---

<a id='podman-load'></a>

## podman-load - Load image(s) from a tar archive into container
storage

##  NAME

podman-load - Load image(s) from a tar archive into container storage

##  SYNOPSIS

**podman load** \[*options*\]

**podman image load** \[*options*\]

##  DESCRIPTION

**podman load** loads an image from either an **oci-archive** or a
**docker-archive** stored on the local machine into container storage.
**podman load** reads from stdin by default or a file if the **input**
option is set. **podman load** is used for loading from the archive
generated by **podman save**, that includes the image parent layers. To
load the archive of container\'s filesystem created by **podman
export**, use **podman import**.

The local client further supports loading an **oci-dir** or a
**docker-dir** as created with **podman save** (1).

The **quiet** option suppresses the progress output when set. Note: `:`
is a restricted character and cannot be part of the file name.

**podman \[GLOBAL OPTIONS\]**

**podman load \[GLOBAL OPTIONS\]**

**podman load [OPTIONS](#options)**

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

#### **\--input**, **-i**=*input*

Load the specified input file instead of from stdin. The file can be on
the local file system or on a server (e.g.,
https://server.com/archive.tar). Also supports loading in compressed
files.

The remote client, including Mac and Windows (excluding WSL2) machines,
requires the use of this option.

NOTE: Use the environment variable `TMPDIR` to change the temporary
storage location of container images. Podman defaults to use `/var/tmp`.

#### **\--quiet**, **-q**

Suppress the progress output

##  EXAMPLES

Create an image from a compressed tar file, without showing progress.

    $ podman load --quiet -i fedora.tar.gz

Create an image from the archive.tar file pulled from a URL, without
showing progress.

    $ podman load -q -i https://server.com/archive.tar

Create an image from stdin using bash redirection from a tar file.

    $ podman load < fedora.tar
    Getting image source signatures
    Copying blob sha256:5bef08742407efd622d243692b79ba0055383bbce12900324f75e56f589aedb0
     0 B / 4.03 MB [---------------------------------------------------------------]
    Copying config sha256:7328f6f8b41890597575cbaadc884e7386ae0acc53b747401ebce5cf0d624560
     0 B / 1.48 KB [---------------------------------------------------------------]
    Writing manifest to image destination
    Storing signatures
    Loaded image:  registry.fedoraproject.org/fedora:latest

Create an image from stdin using a pipe.

    $ cat fedora.tar | podman load
    Getting image source signatures
    Copying blob sha256:5bef08742407efd622d243692b79ba0055383bbce12900324f75e56f589aedb0
     0 B / 4.03 MB [---------------------------------------------------------------]
    Copying config sha256:7328f6f8b41890597575cbaadc884e7386ae0acc53b747401ebce5cf0d624560
     0 B / 1.48 KB [---------------------------------------------------------------]
    Writing manifest to image destination
    Storing signatures
    Loaded image:  registry.fedoraproject.org/fedora:latest

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-save(1)](podman-save.html)**

##  HISTORY

July 2017, Originally compiled by Urvashi Mohnani <umohnani@redhat.com>


---

<a id='podman-login'></a>

## podman-login - Log in to a container registry

##  NAME

podman-login - Log in to a container registry

##  SYNOPSIS

**podman login** \[*options*\] \[*registry*\]

##  DESCRIPTION

**podman login** logs into a specified registry server with the correct
username and password. If the registry is not specified, the first
registry under \[registries.search\] from registries.conf is used.
**podman login** reads in the username and password from STDIN. The
username and password can also be set using the **username** and
**password** flags. The path of the authentication file can be specified
by the user by setting the **authfile** flag. The default path for
reading and writing credentials is
**[*XDG*\_*RUNTIME*\_*DIR*/*containers*/*auth*.*json* \*  \* .*Podmanusesexistingcredentialsiftheuserdoesnotpassinausername*.*Podmanfirstsearchesfortheusernameandpasswordinthe* \* \*]{.math
.inline}{XDG_RUNTIME_DIR}/containers/auth.json**, if they are not valid,
Podman then uses any existing credentials found in
**[*HOME*/.*docker*/*config*.*json* \*  \* .*Ifthosecredentialsarenotpresent*, *Podmancreates* \* \*]{.math
.inline}{XDG_RUNTIME_DIR}/containers/auth.json** (if the file does not
exist) and then stores the username and password from STDIN as a base64
encoded string in it. For more details about format and configurations
of the auth.json file, see containers-auth.json(5)

**podman \[GLOBAL OPTIONS\]**

**podman login \[GLOBAL OPTIONS\]**

**podman login [OPTIONS](#options) \[REGISTRY\] \[GLOBAL OPTIONS\]**

##  OPTIONS

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--cert-dir**=*path*

Use certificates at *path* (\*.crt, \*.cert, \*.key) to connect to the
registry. (Default: /etc/containers/certs.d) For details, see
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--compat-auth-file**=*path*

Instead of updating the default credentials file, update the one at
*path*, and use a Docker-compatible format.

#### **\--get-login**

Return the logged-in user for the registry. Return error if no login is
found.

#### **\--help**, **-h**

Print usage statement

#### **\--password**, **-p**=*password*

Password for registry

#### **\--password-stdin**

Take the password from stdin

#### **\--secret**=*name*

Read the password for the registry from the podman secret `name`. If
\--username is not specified \--username=`name` is used.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

#### **\--username**, **-u**=*username*

Username for registry

#### **\--verbose**, **-v**

print detailed information about credential store

##  EXAMPLES

Add login credentials for specified registry to default authentication
file; note that unlike the `docker` default, the default credentials are
under `$XDG_RUNTIME_DIR` which is a subdirectory of `/run` (an
emphemeral directory) and hence do not persist across reboot.

    $ podman login quay.io
    Username: umohnani
    Password:
    Login Succeeded!

To explicitly preserve credentials across reboot, you will need to
specify the default persistent path:

    $ podman login --authfile ~/.config/containers/auth.json quay.io
    Username: umohnani
    Password:
    Login Succeeded!

Add login credentials using specified username and password for local
registry to default authentication file.

    $ podman login -u testuser -p testpassword localhost:5000
    Login Succeeded!

Add login credentials for alternate authfile path for the specified
registry.

    $ podman login --authfile authdir/myauths.json quay.io
    Username: umohnani
    Password:
    Login Succeeded!

Add login credentials using a Podman secret for the password.

    $ echo -n MySecret! | podman secret create secretname -
    a0ad54df3c97cf89d5ca6193c
    $ podman login --secret secretname -u testuser quay.io
    Login Succeeded!

Add login credentials for user test with password test to localhost:5000
registry disabling tls verification requirement.

    $ podman login --tls-verify=false -u test -p test localhost:5000
    Login Succeeded!

Add login credentials for user foo with password bar to localhost:5000
registry using the certificate directory /etc/containers/certs.d.

    $ podman login --cert-dir /etc/containers/certs.d/ -u foo -p bar localhost:5000
    Login Succeeded!

Add login credentials for specified registries to default authentication
file for given user with password information provided via stdin from a
file on disk.

    $ podman login -u testuser  --password-stdin < testpassword.txt docker.io
    Login Succeeded!

Add login credentials for specified registry to default authentication
file for given user with password information provided via stdin from a
pipe.

    $ echo $testpassword | podman login -u testuser --password-stdin quay.io
    Login Succeeded!

Add login credentials for specified registry to default authentication
file in verbose mode.

    $ podman login quay.io --verbose
    Username: myusername
    Password:
    Used: /run/user/1000/containers/auth.json
    Login Succeeded!

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-logout(1)](podman-logout.html)**,
**[containers-auth.json(5)](https://github.com/containers/image/blob/main/docs/containers-auth.json.5.md)**,
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**,
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**,
**[podman-secret(1)](podman-secret.html)**,
**[podman-secret-create(1)](podman-secret-create.html)**

##  HISTORY

August 2017, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-logout'></a>

## podman-logout - Log out of a container registry

##  NAME

podman-logout - Log out of a container registry

##  SYNOPSIS

**podman logout** \[*options*\] *registry*

##  DESCRIPTION

**podman logout** logs out of a specified registry server by deleting
the cached credentials stored in the **auth.json** file. If the registry
is not specified, the first registry under \[registries.search\] from
registries.conf is used. The path of the authentication file can be
overridden by the user by setting the **authfile** flag. The default
path used is **\${XDG_RUNTIME_DIR}/containers/auth.json**. For more
details about format and configurations of the auth,json file, see
containers-auth.json(5) All the cached credentials can be removed by
setting the **all** flag.

**podman \[GLOBAL OPTIONS\]**

**podman logout \[GLOBAL OPTIONS\]**

**podman logout [OPTIONS](#options) REGISTRY \[GLOBAL OPTIONS\]**

##  OPTIONS

#### **\--all**, **-a**

Remove the cached credentials for all registries in the auth file

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--compat-auth-file**=*path*

Instead of updating the default credentials file, update the one at
*path*, and use a Docker-compatible format.

#### **\--help**, **-h**

Print usage statement

##  EXAMPLES

Remove login credentials for the docker.io registry from the
authentication file:

    $ podman logout docker.io

Remove login credentials for the docker.io registry from the
authdir/myauths.json file:

    $ podman logout --authfile authdir/myauths.json docker.io

Remove login credentials for all registries:

    $ podman logout --all

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-login(1)](podman-login.html)**,
**[containers-auth.json(5)](https://github.com/containers/image/blob/main/docs/containers-auth.json.5.md)**

##  HISTORY

August 2017, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-logs'></a>

## podman-logs - Display the logs of one or more containers

##  NAME

podman-logs - Display the logs of one or more containers

##  SYNOPSIS

**podman logs** \[*options*\] *container* \[*container\...*\]

**podman container logs** \[*options*\] *container* \[*container\...*\]

##  DESCRIPTION

The podman logs command batch-retrieves whatever logs are present for
one or more containers at the time of execution. This does not guarantee
execution order when combined with podman run (i.e. the run may not have
generated any logs at the time podman logs was executed).

##  OPTIONS

#### **\--color**

Output the containers with different colors in the log.

#### **\--follow**, **-f**

Follow log output. Default is false.

Note: When following a container which is removed by
`podman container rm` or removed on exit (`podman run --rm ...`), there
is a chance that the log file is removed before `podman logs` reads the
final content.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--names**, **-n**

Output the container names instead of the container IDs in the log.

#### **\--since**=*TIMESTAMP*

Show logs since TIMESTAMP. The \--since option can be Unix timestamps,
date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m)
computed relative to the client machine\'s time. Supported formats for
date formatted time stamps include RFC3339Nano, RFC3339,
2006-01-02T15:04:05, 2006-01-02T15:04:05.999999999, 2006-01-02Z07:00,
and 2006-01-02.

#### **\--tail**=*LINES*

Output the specified number of LINES at the end of the logs. LINES must
be an integer. Defaults to -1, which prints all lines

#### **\--timestamps**, **-t**

Show timestamps in the log outputs. The default is false

#### **\--until**=*TIMESTAMP*

Show logs until TIMESTAMP. The \--until option can be Unix timestamps,
date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m)
computed relative to the client machine\'s time. Supported formats for
date formatted time stamps include RFC3339Nano, RFC3339,
2006-01-02T15:04:05, 2006-01-02T15:04:05.999999999, 2006-01-02Z07:00,
and 2006-01-02.

##  EXAMPLE

To view a container\'s logs:

    podman logs -t b3f2436bdb978c1d33b1387afb5d7ba7e3243ed2ce908db431ac0069da86cb45

    2017/08/07 10:16:21 Seeked /var/log/crio/pods/eb296bd56fab164d4d3cc46e5776b54414af3bf543d138746b25832c816b933b/c49f49788da14f776b7aa93fb97a2a71f9912f4e5a3e30397fca7dfe0ee0367b.log - &{Offset:0 Whence:0}
    1:C 07 Aug 14:10:09.055 # oO0OoO0OoO0Oo Redis is starting oO0OoO0OoO0Oo
    1:C 07 Aug 14:10:09.055 # Redis version=4.0.1, bits=64, commit=00000000, modified=0, pid=1, just started
    1:C 07 Aug 14:10:09.055 # Warning: no config file specified, using the default config. In order to specify a config file use redis-server /path/to/redis.conf
    1:M 07 Aug 14:10:09.055 # You requested maxclients of 10000 requiring at least 10032 max file descriptors.
    1:M 07 Aug 14:10:09.055 # Server can't set maximum open files to 10032 because of OS error: Operation not permitted.
    1:M 07 Aug 14:10:09.055 # Current maximum open files is 4096. maxclients has been reduced to 4064 to compensate for low ulimit. If you need higher maxclients increase 'ulimit -n'.
    1:M 07 Aug 14:10:09.056 * Running mode=standalone, port=6379.
    1:M 07 Aug 14:10:09.056 # WARNING: The TCP backlog setting of 511 cannot be enforced because /proc/sys/net/core/somaxconn is set to the lower value of 128.
    1:M 07 Aug 14:10:09.056 # Server initialized

To view only the last two lines in container\'s log:

    podman logs --tail 2 b3f2436bdb97

    # WARNING: The TCP backlog setting of 511 cannot be enforced because /proc/sys/net/core/somaxconn is set to the lower value of 128.
    # Server initialized

To view all containers logs:

    podman logs -t --since 0 myserver

    1:M 07 Aug 14:10:09.055 # Server can't set maximum open files to 10032 because of OS error: Operation not permitted.
    1:M 07 Aug 14:10:09.055 # Current maximum open files is 4096. maxclients has been reduced to 4064 to compensate for low ulimit. If you need higher maxclients increase 'ulimit -n'.
    1:M 07 Aug 14:10:09.056 * Running mode=standalone, port=6379.
    1:M 07 Aug 14:10:09.056 # WARNING: The TCP backlog setting of 511 cannot be enforced because /proc/sys/net/core/somaxconn is set to the lower value of 128.
    1:M 07 Aug 14:10:09.056 # Server initialized

To view a container\'s logs since a certain time:

    podman logs -t --since 2017-08-07T10:10:09.055837383-04:00 myserver

    1:M 07 Aug 14:10:09.055 # Server can't set maximum open files to 10032 because of OS error: Operation not permitted.
    1:M 07 Aug 14:10:09.055 # Current maximum open files is 4096. maxclients has been reduced to 4064 to compensate for low ulimit. If you need higher maxclients increase 'ulimit -n'.
    1:M 07 Aug 14:10:09.056 * Running mode=standalone, port=6379.
    1:M 07 Aug 14:10:09.056 # WARNING: The TCP backlog setting of 511 cannot be enforced because /proc/sys/net/core/somaxconn is set to the lower value of 128.
    1:M 07 Aug 14:10:09.056 # Server initialized

To view a container\'s logs generated in the last 10 minutes:

    podman logs --since 10m myserver

    # Server can't set maximum open files to 10032 because of OS error: Operation not permitted.
    # Current maximum open files is 4096. maxclients has been reduced to 4064 to compensate for low ulimit, Increase 'ulimit -n' when higher maxclients are required.

To view a container\'s logs until 30 minutes ago:

    podman logs --until 30m myserver

    AH00558: httpd: Could not reliably determine the server's fully qualified domain name, using 10.0.2.100. Set the 'ServerName' directive globally to suppress this message
    AH00558: httpd: Could not reliably determine the server's fully qualified domain name, using 10.0.2.100. Set the 'ServerName' directive globally to suppress this message
    [Tue Jul 20 13:18:14.223727 2021] [mpm_event:notice] [pid 1:tid 140021067187328] AH00489: Apache/2.4.48 (Unix) configured -- resuming normal operations
    [Tue Jul 20 13:18:14.223819 2021] [core:notice] [pid 1:tid 140021067187328] AH00094: Command line: 'httpd -D FOREGROUND'

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-run(1)](podman-run.html)**,
**[podman-rm(1)](podman-rm.html)**

##  HISTORY

February 2018, Updated by Brent Baude <bbaude@redhat.com>

August 2017, Originally compiled by Ryan Cole <rycole@redhat.com>


---

<a id='podman-machine'></a>

## podman-machine - Manage Podman's virtual machine

##  NAME

podman-machine - Manage Podman\'s virtual machine

##  SYNOPSIS

**podman machine** *subcommand*

##  DESCRIPTION

`podman machine` is a set of subcommands that manage Podman\'s virtual
machine.

Podman on MacOS and Windows requires a virtual machine. This is because
containers are Linux - containers do not run on any other OS because
containers\' core functionality are tied to the Linux kernel. Podman
machine must be used to manage MacOS and Windows machines, but can be
optionally used on Linux.

All `podman machine` commands are rootless only.

NOTE: The podman-machine configuration file is managed under the
`$XDG_CONFIG_HOME/containers/podman/machine/` directory. Changing the
`$XDG_CONFIG_HOME` environment variable while the machines are running
can lead to unexpected behavior.

Podman machine behaviour can be modified via the \[machine\] section in
the containers.conf(5) file.

##  SUBCOMMANDS

  ----------------------------------------------------------------------------------------------
  Command   Man Page                                                   Description
  --------- ---------------------------------------------------------- -------------------------
  info      [podman-machine-info(1)](podman-machine-info.html)         Display machine host info

  init      [podman-machine-init(1)](podman-machine-init.html)         Initialize a new virtual
                                                                       machine

  inspect   [podman-machine-inspect(1)](podman-machine-inspect.html)   Inspect one or more
                                                                       virtual machines

  list      [podman-machine-list(1)](podman-machine-list.html)         List virtual machines

  os        [podman-machine-os(1)](podman-machine-os.html)             Manage a Podman virtual
                                                                       machine\'s OS

  reset     [podman-machine-reset(1)](podman-machine-reset.html)       Reset Podman machines and
                                                                       environment

  rm        [podman-machine-rm(1)](podman-machine-rm.html)             Remove a virtual machine

  set       [podman-machine-set(1)](podman-machine-set.html)           Set a virtual machine
                                                                       setting

  ssh       [podman-machine-ssh(1)](podman-machine-ssh.html)           SSH into a virtual
                                                                       machine

  start     [podman-machine-start(1)](podman-machine-start.html)       Start a virtual machine

  stop      [podman-machine-stop(1)](podman-machine-stop.html)         Stop a virtual machine
  ----------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-machine-info(1)](podman-machine-info.html)**,
**[podman-machine-init(1)](podman-machine-init.html)**,
**[podman-machine-list(1)](podman-machine-list.html)**,
**[podman-machine-os(1)](podman-machine-os.html)**,
**[podman-machine-rm(1)](podman-machine-rm.html)**,
**[podman-machine-ssh(1)](podman-machine-ssh.html)**,
**[podman-machine-start(1)](podman-machine-start.html)**,
**[podman-machine-stop(1)](podman-machine-stop.html)**,
**[podman-machine-inspect(1)](podman-machine-inspect.html)**,
**[podman-machine-reset(1)](podman-machine-reset.html)**,
**containers.conf(5)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

##  HISTORY

March 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-manifest'></a>

## podman-manifest - Create and manipulate manifest lists and image
indexes

##  NAME

podman-manifest - Create and manipulate manifest lists and image indexes

##  SYNOPSIS

**podman manifest** *subcommand*

##  DESCRIPTION

The `podman manifest` command provides subcommands which can be used to:

    * Create a working Docker manifest list or OCI image index.

##  SUBCOMMANDS

  -----------------------------------------------------------------------------------------------------------------
  Command    Man Page                                                       Description
  ---------- -------------------------------------------------------------- ---------------------------------------
  add        [podman-manifest-add(1)](podman-manifest-add.html)             Add an image or artifact to a manifest
                                                                            list or image index.

  annotate   [podman-manifest-annotate(1)](podman-manifest-annotate.html)   Add and update information about an
                                                                            image or artifact in a manifest list or
                                                                            image index.

  create     [podman-manifest-create(1)](podman-manifest-create.html)       Create a manifest list or image index.

  exists     [podman-manifest-exists(1)](podman-manifest-exists.html)       Check if the given manifest list exists
                                                                            in local storage

  inspect    [podman-manifest-inspect(1)](podman-manifest-inspect.html)     Display a manifest list or image index.

  push       [podman-manifest-push(1)](podman-manifest-push.html)           Push a manifest list or image index to
                                                                            a registry.

  remove     [podman-manifest-remove(1)](podman-manifest-remove.html)       Remove an item from a manifest list or
                                                                            image index.

  rm         [podman-manifest-rm(1)](podman-manifest-rm.html)               Remove manifest list or image index
                                                                            from local storage.
  -----------------------------------------------------------------------------------------------------------------

##  EXAMPLES

### Building a multi-arch manifest list from a Containerfile

Assuming the `Containerfile` uses `RUN` instructions, the host needs a
way to execute non-native binaries. Configuring this is beyond the scope
of this example. Building a multi-arch manifest list `shazam` in
parallel across 4-threads can be done like this:

        $ platarch=linux/amd64,linux/ppc64le,linux/arm64,linux/s390x
        $ podman build --jobs=4 --platform=$platarch --manifest shazam .

**Note:** The `--jobs` argument is optional. Do not use the
`podman build` command\'s `--tag` (or `-t`) option when building a
multi-arch manifest list.

### Assembling a multi-arch manifest from separately built images

Assuming `example.com/example/shazam:$arch` images are built separately
on other hosts and pushed to the `example.com` registry. They may be
combined into a manifest list, and pushed using a simple loop:

        $ REPO=example.com/example/shazam
        $ podman manifest create $REPO:latest
        $ for IMGTAG in amd64 s390x ppc64le arm64; do \
                  podman manifest add $REPO:latest docker://$REPO:IMGTAG; \
              done
        $ podman manifest push --all $REPO:latest

**Note:** The `add` instruction argument order is `<manifest>` then
`<image>`. Also, the `--all` push option is required to ensure all
contents are pushed, not just the native platform/arch.

### Removing and tagging a manifest list before pushing

Special care is needed when removing and pushing manifest lists, as
opposed to the contents. You almost always want to use the `manifest rm`
and `manifest push --all` subcommands. For example, a rename and push
can be performed like this:

        $ podman tag localhost/shazam example.com/example/shazam
        $ podman manifest rm localhost/shazam
        $ podman manifest push --all example.com/example/shazam

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest-add(1)](podman-manifest-add.html)**,
**[podman-manifest-annotate(1)](podman-manifest-annotate.html)**,
**[podman-manifest-create(1)](podman-manifest-create.html)**,
**[podman-manifest-inspect(1)](podman-manifest-inspect.html)**,
**[podman-manifest-push(1)](podman-manifest-push.html)**,
**[podman-manifest-remove(1)](podman-manifest-remove.html)**


---

<a id='podman-network'></a>

## podman-network - Manage Podman networks

##  NAME

podman-network - Manage Podman networks

##  SYNOPSIS

**podman network** *subcommand*

##  DESCRIPTION

The network command manages networks for Podman.

Podman supports two network backends
[Netavark](https://github.com/containers/netavark) and
[CNI](https://www.cni.dev/). Netavark is the default network backend and
was added in Podman version 4.0. CNI is deprecated and will be removed
in the next major Podman version 5.0, in preference of Netavark. To
configure the network backend use the `network_backend` key under the
`[Network]` in
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**.
New systems use netavark by default, to check what backend is used run
`podman info --format {{.Host.NetworkBackend}}`.

All network commands work for both backends but CNI and Netavark use
different config files so networks have to be created again after a
backend change.

##  COMMANDS

  --------------------------------------------------------------------------------------------------------------
  Command      Man Page                                                         Description
  ------------ ---------------------------------------------------------------- --------------------------------
  connect      [podman-network-connect(1)](podman-network-connect.html)         Connect a container to a network

  create       [podman-network-create(1)](podman-network-create.html)           Create a Podman network

  disconnect   [podman-network-disconnect(1)](podman-network-disconnect.html)   Disconnect a container from a
                                                                                network

  exists       [podman-network-exists(1)](podman-network-exists.html)           Check if the given network
                                                                                exists

  inspect      [podman-network-inspect(1)](podman-network-inspect.html)         Display the network
                                                                                configuration for one or more
                                                                                networks

  ls           [podman-network-ls(1)](podman-network-ls.html)                   Display a summary of networks

  prune        [podman-network-prune(1)](podman-network-prune.html)             Remove all unused networks

  reload       [podman-network-reload(1)](podman-network-reload.html)           Reload network configuration for
                                                                                containers

  rm           [podman-network-rm(1)](podman-network-rm.html)                   Remove one or more networks

  update       [podman-network-update(1)](podman-network-update.html)           Update an existing Podman
                                                                                network
  --------------------------------------------------------------------------------------------------------------

##  SUBNET NOTES

Podman requires specific default IPs and, thus, network subnets. The
default values used by Podman can be modified in the
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
file.

### Podman network

The default bridge network (called `podman`) uses 10.88.0.0/16 as a
subnet. When Podman runs as root, the `podman` network is used as
default. It is the same as adding the option `--network bridge` or
`--network podman`. This subnet can be changed in
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
under the \[network\] section. Set the `default_subnet` to any subnet
that is free in the environment. The name of the default network can
also be changed from `podman` to another name using the default network
key. Note that this is only done when no containers are running.

### Pasta

Pasta by default performs no Network Address Translation (NAT) and
copies the IPs from your main interface into the container namespace. If
pasta cannot find an interface with the default route, it will select an
interface if there is only one interface with a valid route. If you do
not have a default route and several interfaces have defined routes,
pasta will be unable to figure out the correct interface and it will
fail to start. To specify the interface, use `-i` option to pasta. A
default set of pasta options can be set in
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
under the `[network]` section with the `pasta_options` key.

The default rootless networking tool can be selected in
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
under the `[network]` section with `default_rootless_network_cmd`, which
can be set to `pasta` (default) or `slirp4netns`.

### Slirp4netns

Slirp4nents uses 10.0.2.0/24 for its default network. This can also be
changed in
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
but under the `[engine]` section. Use the `network_cmd_options` key and
add `["cidr=X.X.X.X/24"]` as a value. Note that slirp4netns needs a
network prefix size between 1 and 25. This option accepts an array, so
more options can be added in a comma-separated string as described on
the **[podman-network-create(1)](podman-network-create.html)** man page.
To change the CIDR for just one container, specify it on the cli using
the `--network` option like this:
`--network slirp4netns:cidr=192.168.1.0/24`.

### Podman network create

When a new network is created with a `podman network create` command,
and no subnet is given with the \--subnet option, Podman starts picking
a free subnet from 10.89.0.0/24 to 10.255.255.0/24. Use the
`default_subnet_pools` option under the `[network]` section in
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
to change the range and/or size that is assigned by default.

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network-create(1)](podman-network-create.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**


---

<a id='podman-pause'></a>

## podman-pause - Pause one or more containers

##  NAME

podman-pause - Pause one or more containers

##  SYNOPSIS

**podman pause** \[*options*\] \[*container*\...\]

**podman container pause** \[*options*\] \[*container*\...\]

##  DESCRIPTION

Pauses all the processes in one or more containers. You may use
container IDs or names as input.

##  OPTIONS

#### **\--all**, **-a**

Pause all running containers.

#### **\--cidfile**=*file*

Read container ID from the specified *file* and pause the container. Can
be specified multiple times.

#### **\--filter**, **-f**=*filter*

Filter what containers pause. Multiple filters can be given with
multiple uses of the \--filter flag. Filters with the same key work
inclusive with the only exception being `label` which is exclusive.
Filters with different keys always work exclusive.

Valid filters are listed below:

  --------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ -------------------------------------------------------------
  id           \[ID\] Container\'s ID (CID prefix match by default; accepts
               regex)

  name         [Name](#name) Container\'s name (accepts regex)

  label        \[Key\] or \[Key=Value\] Label assigned to a container

  exited       \[Int\] Container\'s exit code

  status       \[Status\] Container\'s status: \'created\', \'exited\',
               \'paused\', \'running\', \'unknown\'

  ancestor     \[ImageName\] Image or descendant used to create container

  before       \[ID\] or [Name](#name) Containers created before this
               container

  since        \[ID\] or [Name](#name) Containers created since this
               container

  volume       \[VolumeName\] or \[MountpointDestination\] Volume mounted in
               container

  health       \[Status\] healthy or unhealthy

  pod          \[Pod\] name or full or partial ID of pod

  network      \[Network\] name or full ID of network

  until        \[DateTime\] container created before the given duration or
               time.
  --------------------------------------------------------------------------

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

##  EXAMPLE

Pause specified container:

    podman pause mywebserver

Pause container by partial container ID:

    podman pause 860a4b23

Pause all **running** containers:

    podman pause --all

Pause container using ID specified in given files:

    podman pause --cidfile /home/user/cidfile-1
    podman pause --cidfile /home/user/cidfile-1 --cidfile ./cidfile-2

Pause the latest container. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines):

    podman pause --latest

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-unpause(1)](podman-unpause.html)**

##  HISTORY

September 2017, Originally compiled by Dan Walsh <dwalsh@redhat.com>


---

<a id='podman-pod'></a>

## podman-pod - Management tool for groups of containers, called
pods

##  NAME

podman-pod - Management tool for groups of containers, called pods

##  SYNOPSIS

**podman pod** *subcommand*

##  DESCRIPTION

podman pod is a set of subcommands that manage pods, or groups of
containers.

##  SUBCOMMANDS

  ------------------------------------------------------------------------------------------------------
  Command   Man Page                                           Description
  --------- -------------------------------------------------- -----------------------------------------
  clone     [podman-pod-clone(1)](podman-pod-clone.html)       Create a copy of an existing pod.

  create    [podman-pod-create(1)](podman-pod-create.html)     Create a new pod.

  exists    [podman-pod-exists(1)](podman-pod-exists.html)     Check if a pod exists in local storage.

  inspect   [podman-pod-inspect(1)](podman-pod-inspect.html)   Display information describing a pod.

  kill      [podman-pod-kill(1)](podman-pod-kill.html)         Kill the main process of each container
                                                               in one or more pods.

  logs      [podman-pod-logs(1)](podman-pod-logs.html)         Display logs for pod with one or more
                                                               containers.

  pause     [podman-pod-pause(1)](podman-pod-pause.html)       Pause one or more pods.

  prune     [podman-pod-prune(1)](podman-pod-prune.html)       Remove all stopped pods and their
                                                               containers.

  ps        [podman-pod-ps(1)](podman-pod-ps.html)             Print out information about pods.

  restart   [podman-pod-restart(1)](podman-pod-restart.html)   Restart one or more pods.

  rm        [podman-pod-rm(1)](podman-pod-rm.html)             Remove one or more stopped pods and
                                                               containers.

  start     [podman-pod-start(1)](podman-pod-start.html)       Start one or more pods.

  stats     [podman-pod-stats(1)](podman-pod-stats.html)       Display a live stream of resource usage
                                                               stats for containers in one or more pods.

  stop      [podman-pod-stop(1)](podman-pod-stop.html)         Stop one or more pods.

  top       [podman-pod-top(1)](podman-pod-top.html)           Display the running processes of
                                                               containers in a pod.

  unpause   [podman-pod-unpause(1)](podman-pod-unpause.html)   Unpause one or more pods.
  ------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

July 2018, Originally compiled by Peter Hunt <pehunt@redhat.com>


---

<a id='podman-port'></a>

## podman-port - List port mappings for a container

##  NAME

podman-port - List port mappings for a container

##  SYNOPSIS

**podman port** \[*options*\] *container* \[*private-port*\[/*proto*\]\]

**podman container port** \[*options*\] *container*
\[*private-port*\[/*proto*\]\]

##  DESCRIPTION

List port mappings for the *container* or look up the public-facing port
that is NAT-ed to the *private-port*.

##  OPTIONS

#### **\--all**, **-a**

List all known port mappings for running containers; when using this
option, container names or private ports/protocols filters cannot be
used.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

##  EXAMPLE

List all port mappings:

    # podman port -a
    b4d2f05432e482e017b1a4b2eae15fa7b4f6fb7e9f65c1bde46294fdef285906
    80/udp -> 0.0.0.0:44327
    80/tcp -> 0.0.0.0:44327
    #

List port mappings for a specific container:

    # podman port b4d2f054
    80/udp -> 0.0.0.0:44327
    80/tcp -> 0.0.0.0:44327
    #

List the specified port mappings for a specific container:

    # podman port b4d2f054 80
     0.0.0.0:44327
    #

List the port mappings for a specific container for port 80 and the tcp
protocol:

    # podman port b4d2f054 80/tcp
    0.0.0.0:44327
    #

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-inspect(1)](podman-inspect.html)**

##  HISTORY

January 2018, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-ps'></a>

## podman-ps - Print out information about containers

##  NAME

podman-ps - Print out information about containers

##  SYNOPSIS

**podman ps** \[*options*\]

**podman container ps** \[*options*\]

**podman container list** \[*options*\]

**podman container ls** \[*options*\]

##  DESCRIPTION

**podman ps** lists the running containers on the system. Use the
**\--all** flag to view all the containers information. By default it
lists:

-   container id
-   the name of the image the container is using
-   the COMMAND the container is executing
-   the time the container was created
-   the status of the container
-   port mappings the container is using
-   alternative names for the container

##  OPTIONS

#### **\--all**, **-a**

Show all the containers, default is only running containers.

Note: Podman shares containers storage with other tools such as Buildah
and CRI-O. In some cases these `external` containers might also exist in
the same storage. Use the `--external` option to see these external
containers. External containers show the \'storage\' status.

#### **\--external**

Display external containers that are not controlled by Podman but are
stored in containers storage. These external containers are generally
created via other container technology such as Buildah or CRI-O and may
depend on the same container images that Podman is also using. External
containers are denoted with either a \'buildah\' or \'storage\' in the
COMMAND and STATUS column of the ps output.

#### **\--filter**, **-f**

Filter what containers are shown in the output. Multiple filters can be
given with multiple uses of the \--filter flag. Filters with the same
key work inclusive with the only exception being `label` which is
exclusive. Filters with different keys always work exclusive.

Valid filters are listed below:

  --------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ -------------------------------------------------------------
  id           \[ID\] Container\'s ID (CID prefix match by default; accepts
               regex)

  name         [Name](#name) Container\'s name (accepts regex)

  label        \[Key\] or \[Key=Value\] Label assigned to a container

  label!       \[Key\] or \[Key=Value\] Label NOT assigned to a container

  exited       \[Int\] Container\'s exit code

  status       \[Status\] Container\'s status: \'created\', \'exited\',
               \'paused\', \'running\', \'unknown\'

  ancestor     \[ImageName\] Image or descendant used to create container
               (accepts regex)

  before       \[ID\] or [Name](#name) Containers created before this
               container

  since        \[ID\] or [Name](#name) Containers created since this
               container

  volume       \[VolumeName\] or \[MountpointDestination\] Volume mounted in
               container

  health       \[Status\] healthy or unhealthy

  pod          \[Pod\] name or full or partial ID of pod

  network      \[Network\] name or full ID of network

  until        \[DateTime\] container created before the given duration or
               time.
  --------------------------------------------------------------------------

#### **\--format**=*format*

Pretty-print containers to JSON or using a Go template

Valid placeholders for the Go template are listed below:

  **Placeholder**      **Description**
  -------------------- ----------------------------------------------
  .AutoRemove          If true, containers are removed on exit
  .CIDFile             Container ID File
  .Command             Quoted command used
  .Created \...        Creation time for container, Y-M-D H:M:S
  .CreatedAt           Creation time for container (same as above)
  .CreatedHuman        Creation time, relative
  .ExitCode            Container exit code
  .Exited              \"true\" if container has exited
  .ExitedAt            Time (epoch seconds) that container exited
  .ExposedPorts \...   Map of exposed ports on this container
  .ID                  Container ID
  .Image               Image Name/ID
  .ImageID             Image ID
  .IsInfra             \"true\" if infra container
  .Label *string*      Specified label of the container
  .Labels \...         All the labels assigned to the container
  .Mounts              Volumes mounted in the container
  .Names               Name of container
  .Networks            Show all networks connected to the container
  .Pid                 Process ID on host system
  .Pod                 Pod the container is associated with (SHA)
  .PodName             PodName of the container
  .Ports               Forwarded and exposed ports
  .Restarts            Display the container restart count
  .RunningFor          Time elapsed since container was started
  .Size                Size of container
  .StartedAt           Time (epoch seconds) the container started
  .State               Human-friendly description of ctr state
  .Status              Status of container

#### **\--help**, **-h**

Print usage statement

#### **\--last**, **-n**

Print the n last created containers (all states)

#### **\--latest**, **-l**

Show the latest container created (all states) (This option is not
available with the remote Podman client, including Mac and Windows
(excluding WSL2) machines)

#### **\--namespace**, **\--ns**

Display namespace information

#### **\--no-trunc**

Do not truncate the output (default *false*).

#### **\--noheading**

Omit the table headings from the listing of containers.

#### **\--pod**, **-p**

Display the pods the containers are associated with

#### **\--quiet**, **-q**

Print the numeric IDs of the containers only

#### **\--size**, **-s**

Display the total file size

#### **\--sort**=*created*

Sort by command, created, id, image, names, runningfor, size, or
status\", Note: Choosing size sorts by size of rootFs, not
alphabetically like the rest of the options

#### **\--sync**

Force a sync of container state with the OCI runtime. In some cases, a
container\'s state in the runtime can become out of sync with Podman\'s
state. This updates Podman\'s state based on what the OCI runtime
reports. Forcibly syncing is much slower, but can resolve inconsistent
state issues.

#### **\--watch**, **-w**

Refresh the output with current containers on an interval in seconds.

##  EXAMPLES

List running containers.

    $ podman ps
    CONTAINER ID  IMAGE                            COMMAND    CREATED        STATUS        PORTS                                                   NAMES
    4089df24d4f3  docker.io/library/centos:latest  /bin/bash  2 minutes ago  Up 2 minutes  0.0.0.0:80->8080/tcp, 0.0.0.0:2000-2006->2000-2006/tcp  manyports
    92f58933c28c  docker.io/library/centos:latest  /bin/bash  3 minutes ago  Up 3 minutes  192.168.99.100:1000-1006->1000-1006/tcp                 zen_sanderson

List all containers.

    $ podman ps -a
    CONTAINER ID   IMAGE         COMMAND         CREATED       STATUS                    PORTS     NAMES
    02f65160e14ca  redis:alpine  "redis-server"  19 hours ago  Exited (-1) 19 hours ago  6379/tcp  k8s_podsandbox1-redis_podsandbox1_redhat.test.crio_redhat-test-crio_0
    69ed779d8ef9f  redis:alpine  "redis-server"  25 hours ago  Created                   6379/tcp  k8s_container1_podsandbox1_redhat.test.crio_redhat-test-crio_1

List all containers including their size. Note: this can take longer
since Podman needs to calculate the size from the file system.

    $ podman ps -a -s
    CONTAINER ID   IMAGE         COMMAND         CREATED       STATUS                    PORTS     NAMES                                                                  SIZE
    02f65160e14ca  redis:alpine  "redis-server"  20 hours ago  Exited (-1) 20 hours ago  6379/tcp  k8s_podsandbox1-redis_podsandbox1_redhat.test.crio_redhat-test-crio_0  27.49 MB
    69ed779d8ef9f  redis:alpine  "redis-server"  25 hours ago  Created                   6379/tcp  k8s_container1_podsandbox1_redhat.test.crio_redhat-test-crio_1         27.49 MB

List all containers, running or not, using a custom Go format.

    $ podman ps -a --format "{{.ID}}  {{.Image}}  {{.Labels}}  {{.Mounts}}"
    02f65160e14ca  redis:alpine  tier=backend  proc,tmpfs,devpts,shm,mqueue,sysfs,cgroup,/var/run/,/var/run/
    69ed779d8ef9f  redis:alpine  batch=no,type=small  proc,tmpfs,devpts,shm,mqueue,sysfs,cgroup,/var/run/,/var/run/

List all containers and display their namespaces.

    $ podman ps --ns -a
    CONTAINER ID    NAMES                                                                   PID     CGROUP       IPC          MNT          NET          PIDNS        USER         UTS
    3557d882a82e3   k8s_container2_podsandbox1_redhat.test.crio_redhat-test-crio_1          29910   4026531835   4026532585   4026532593   4026532508   4026532595   4026531837   4026532594
    09564cdae0bec   k8s_container1_podsandbox1_redhat.test.crio_redhat-test-crio_1          29851   4026531835   4026532585   4026532590   4026532508   4026532592   4026531837   4026532591
    a31ebbee9cee7   k8s_podsandbox1-redis_podsandbox1_redhat.test.crio_redhat-test-crio_0   29717   4026531835   4026532585   4026532587   4026532508   4026532589   4026531837   4026532588

List all containers including size sorted by names.

    $ podman ps -a --size --sort names
    CONTAINER ID   IMAGE         COMMAND         CREATED       STATUS                    PORTS     NAMES
    69ed779d8ef9f  redis:alpine  "redis-server"  25 hours ago  Created                   6379/tcp  k8s_container1_podsandbox1_redhat.test.crio_redhat-test-crio_1
    02f65160e14ca  redis:alpine  "redis-server"  19 hours ago  Exited (-1) 19 hours ago  6379/tcp  k8s_podsandbox1-redis_podsandbox1_redhat.test.crio_redhat-test-crio_0

List all external containers created by tools other than Podman.

    $ podman ps --external -a
    CONTAINER ID  IMAGE                             COMMAND  CREATED      STATUS  PORTS  NAMES
    69ed779d8ef9f  redis:alpine  "redis-server"  25 hours ago  Created                   6379/tcp  k8s_container1_podsandbox1_redhat.test.crio_redhat-test-crio_1
    38a8a78596f9  docker.io/library/busybox:latest  buildah  2 hours ago  storage        busybox-working-container
    fd7b786b5c32  docker.io/library/alpine:latest   buildah  2 hours ago  storage        alpine-working-container
    f78620804e00  scratch                           buildah  2 hours ago  storage        working-container

##  ps

Print a list of containers

##  SEE ALSO

**[podman(1)](podman.html)**,
**[buildah(1)](https://github.com/containers/buildah/blob/main/docs/buildah.html)**,
**[crio(8)](https://github.com/cri-o/cri-o/blob/main/docs/crio.8.md)**

##  HISTORY

August 2017, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-pull'></a>

## podman-pull - Pull an image from a registry

##  NAME

podman-pull - Pull an image from a registry

##  SYNOPSIS

**podman pull** \[*options*\] *source* \[*source*\...\]

**podman image pull** \[*options*\] *source* \[*source*\...\]

**podman pull** \[*options*\]
\[*transport*\]*name*\[:*tag*\|[@\*digest]{.citation
cites="*digest"}\*\]

**podman image pull** \[*options*\]
\[*transport*\]*name*\[:*tag*\|[@\*digest]{.citation
cites="*digest"}\*\]

##  DESCRIPTION

podman pull copies an image from a registry onto the local machine. The
command can pull one or more images. If the image reference in the
command line argument does not contain a registry, it is referred to as
a `short-name` reference. If the image is a \'short-name\' reference,
Podman prompts the user for the specific container registry to pull the
image from, if an alias for the short-name has not been specified in the
`short-name-aliases.conf`. If an image tag is not specified, **podman
pull** defaults to the image with the **latest** tag (if it exists) and
pulls it. After the image is pulled, podman prints the full image ID.
**podman pull** can also pull images using a digest **podman pull**
*image*@*digest* and can also be used to pull images from archives and
local storage using different transports. *IMPORTANT: Images are stored
in local image storage.*

##  SOURCE

SOURCE is the location from which the container image is pulled from. It
supports all transports from
**[containers-transports(5)](https://github.com/containers/image/blob/main/docs/containers-transports.5.md)**.
If no transport is specified, the input is subject to short-name
resolution and the `docker` (i.e., container registry) transport is
used. For remote clients, including Mac and Windows (excluding WSL2)
machines, `docker` is the only supported transport.

    # Pull from a container registry
    $ podman pull quay.io/username/myimage

    # Pull from a container registry with short-name resolution
    $ podman pull fedora

    # Pull from a container registry via the docker transport
    $ podman pull docker://quay.io/username/myimage

    # Pull from a local directory
    $ podman pull dir:/tmp/myimage

    # Pull from a tarball in the docker-archive format
    $ podman pull docker-archive:/tmp/myimage

    # Pull from a local docker daemon
    $ sudo podman pull docker-daemon:docker.io/library/myimage:33

    # Pull from a tarball in the OCI-archive format
    $ podman pull oci-archive:/tmp/myimage

##  OPTIONS

#### **\--all-tags**, **-a**

All tagged images in the repository are pulled.

*IMPORTANT: When using the all-tags flag, Podman does not iterate over
the search registries in the
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**
but always uses docker.io for unqualified image names.*

#### **\--arch**=*ARCH*

Override the architecture, defaults to hosts, of the image to be pulled.
For example, `arm`. Unless overridden, subsequent lookups of the same
image in the local storage matches this architecture, regardless of the
host.

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--cert-dir**=*path*

Use certificates at *path* (\*.crt, \*.cert, \*.key) to connect to the
registry. (Default: /etc/containers/certs.d) For details, see
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--creds**=*\[username\[:password\]\]*

The \[username\[:password\]\] to use to authenticate with the registry,
if required. If one or both values are not supplied, a command line
prompt appears and the value can be entered. The password is entered
without echo.

Note that the specified credentials are only used to authenticate
against target registries. They are not used for mirrors or when the
registry gets rewritten (see `containers-registries.conf(5)`); to
authenticate against those consider using a `containers-auth.json(5)`
file.

#### **\--decryption-key**=*key\[:passphrase\]*

The \[key\[:passphrase\]\] to be used for decryption of images. Key can
point to keys and/or certificates. Decryption is tried with all keys. If
the key is protected by a passphrase, it is required to be passed in the
argument and omitted otherwise.

#### **\--disable-content-trust**

This is a Docker-specific option to disable image verification to a
container registry and is not supported by Podman. This option is a NOOP
and provided solely for scripting compatibility.

#### **\--help**, **-h**

Print the usage statement.

#### **\--os**=*OS*

Override the OS, defaults to hosts, of the image to be pulled. For
example, `windows`. Unless overridden, subsequent lookups of the same
image in the local storage matches this OS, regardless of the host.

#### **\--platform**=*OS/ARCH*

Specify the platform for selecting the image. (Conflicts with \--arch
and \--os) The `--platform` option can be used to override the current
architecture and operating system. Unless overridden, subsequent lookups
of the same image in the local storage matches this platform, regardless
of the host.

#### **\--quiet**, **-q**

Suppress output information when pulling images

#### **\--retry**=*attempts*

Number of times to retry pulling or pushing images between the registry
and local storage in case of failure. Default is **3**.

#### **\--retry-delay**=*duration*

Duration of delay between retry attempts when pulling or pushing images
between the registry and local storage in case of failure. The default
is to start at two seconds and then exponentially back off. The delay is
used when this value is set, and no exponential back off occurs.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

#### **\--variant**=*VARIANT*

Use *VARIANT* instead of the default architecture variant of the
container image. Some images can use multiple variants of the arm
architectures, such as arm/v5 and arm/v7.

##  FILES

**short-name-aliases.conf**
(`/var/cache/containers/short-name-aliases.conf`,
`$HOME/.cache/containers/short-name-aliases.conf`)

When users specify images that do not include the container registry
where the image is stored, this is called a short name. The use of
unqualified-search registries entails an ambiguity as it is unclear from
which registry a given image, referenced by a short name, may be pulled
from.

Using short names is subject to the risk of hitting squatted registry
namespaces. If the unqualified-search registries are set to
\[\"public-registry.com\", \"my-private-registry.com\"\] an attacker may
take over a namespace of `public-registry.com` such that an image may be
pulled from `public-registry.com` instead of the intended source
`my-private-registry.com`.

While it is highly recommended to always use fully-qualified image
references, existing deployments using short names may not be easily
changed. To circumvent the aforementioned ambiguity, so called
short-name aliases can be configured that point to a fully-qualified
image reference. Distributions often ship a default shortnames.conf
expansion file in /etc/containers/registries.conf.d/ directory.
Administrators can use this directory to add their own local short-name
expansion files.

When pulling an image, if the user does not specify the complete
registry, container engines attempt to expand the short-name into a full
name. If the command is executed with a tty, the user is prompted to
select a registry from the default list unqualified registries defined
in registries.conf. The user\'s selection is then stored in a cache file
to be used in all future short-name expansions. Rootful short-names are
stored in /var/cache/containers/short-name-aliases.conf. Rootless
short-names are stored in the
\$HOME/.cache/containers/short-name-aliases.conf file.

For more information on short-names, see `containers-registries.conf(5)`

**registries.conf** (`/etc/containers/registries.conf`)

registries.conf is the configuration file which specifies which
container registries is consulted when completing image names which do
not include a registry or domain portion.

NOTE: Use the environment variable `TMPDIR` to change the temporary
storage location of downloaded container images. Podman defaults to use
`/var/tmp`.

##  EXAMPLES

Pull a single image with short name resolution.

    $ podman pull alpine:latest
    Resolved "alpine" as an alias (/etc/containers/registries.conf.d/000-shortnames.conf)
    Trying to pull docker.io/library/alpine:latest...
    Getting image source signatures
    Copying blob 5843afab3874 done
    Copying config d4ff818577 done
    Writing manifest to image destination
    Storing signatures
    d4ff818577bc193b309b355b02ebc9220427090057b54a59e73b79bdfe139b83

Pull multiple images with/without short name resolution.

    podman pull busybox:musl alpine quay.io/libpod/cirros
    Trying to pull docker.io/library/busybox:musl...
    Getting image source signatures
    Copying blob 0c52b060233b [--------------------------------------] 0.0b / 0.0b
    Copying config 9ad2c435a8 done
    Writing manifest to image destination
    Storing signatures
    9ad2c435a887e3f723654e09b48563de44aa3c7950246b2e9305ec85dd3422db
    Trying to pull docker.io/library/alpine:latest...
    Getting image source signatures
    Copying blob 5843afab3874 [--------------------------------------] 0.0b / 0.0b
    Copying config d4ff818577 done
    Writing manifest to image destination
    Storing signatures
    d4ff818577bc193b309b355b02ebc9220427090057b54a59e73b79bdfe139b83
    Trying to pull quay.io/libpod/cirros:latest...
    Getting image source signatures
    Copying blob 8da581cc9286 done
    Copying blob 856628d95d17 done
    Copying blob f513001ba4ab done
    Copying config 3c82e4d066 done
    Writing manifest to image destination
    Storing signatures
    3c82e4d066cf6f9e50efaead6e3ff7fddddf5527826afd68e5a969579fc4db4a

Pull an image using its digest.

    $ podman pull alpine@sha256:d7342993700f8cd7aba8496c2d0e57be0666e80b4c441925fc6f9361fa81d10e
    Trying to pull docker.io/library/alpine@sha256:d7342993700f8cd7aba8496c2d0e57be0666e80b4c441925fc6f9361fa81d10e...
    Getting image source signatures
    Copying blob 188c0c94c7c5 done
    Copying config d6e46aa247 done
    Writing manifest to image destination
    Storing signatures
    d6e46aa2470df1d32034c6707c8041158b652f38d2a9ae3d7ad7e7532d22ebe0

Pull an image by specifying an authentication file.

    $ podman pull --authfile temp-auths/myauths.json docker://docker.io/umohnani/finaltest
    Trying to pull docker.io/umohnani/finaltest:latest...Getting image source signatures
    Copying blob sha256:6d987f6f42797d81a318c40d442369ba3dc124883a0964d40b0c8f4f7561d913
     1.90 MB / 1.90 MB [========================================================] 0s
    Copying config sha256:ad4686094d8f0186ec8249fc4917b71faa2c1030d7b5a025c29f26e19d95c156
     1.41 KB / 1.41 KB [========================================================] 0s
    Writing manifest to image destination
    Storing signatures
    03290064078cb797f3e0a530e78c20c13dd22a3dd3adf84a5da2127b48df0438

Pull an image by authenticating to a registry.

    $ podman pull --creds testuser:testpassword docker.io/umohnani/finaltest
    Trying to pull docker.io/umohnani/finaltest:latest...Getting image source signatures
    Copying blob sha256:6d987f6f42797d81a318c40d442369ba3dc124883a0964d40b0c8f4f7561d913
     1.90 MB / 1.90 MB [========================================================] 0s
    Copying config sha256:ad4686094d8f0186ec8249fc4917b71faa2c1030d7b5a025c29f26e19d95c156
     1.41 KB / 1.41 KB [========================================================] 0s
    Writing manifest to image destination
    Storing signatures
    03290064078cb797f3e0a530e78c20c13dd22a3dd3adf84a5da2127b48df0438

Pull an image using tls verification.

    $ podman pull --tls-verify=false --cert-dir image/certs docker.io/umohnani/finaltest
    Trying to pull docker.io/umohnani/finaltest:latest...Getting image source signatures
    Copying blob sha256:6d987f6f42797d81a318c40d442369ba3dc124883a0964d40b0c8f4f7561d913
     1.90 MB / 1.90 MB [========================================================] 0s
    Copying config sha256:ad4686094d8f0186ec8249fc4917b71faa2c1030d7b5a025c29f26e19d95c156
     1.41 KB / 1.41 KB [========================================================] 0s
    Writing manifest to image destination
    Storing signatures
    03290064078cb797f3e0a530e78c20c13dd22a3dd3adf84a5da2127b48df0438

Pull an image by overriding the host architecture.

    $ podman pull --arch=arm arm32v7/debian:stretch
    Trying to pull docker.io/arm32v7/debian:stretch...
    Getting image source signatures
    Copying blob b531ae4a3925 done
    Copying config 3cba58dad5 done
    Writing manifest to image destination
    Storing signatures
    3cba58dad5d9b35e755b48b634acb3fdd185ab1c996ac11510cc72c17780e13c

Pull an image with up to 6 retries, delaying 10 seconds between retries
in quet mode.

    $ podman --remote pull -q --retry 6 --retry-delay 10s ubi9
    4d6addf62a90e392ff6d3f470259eb5667eab5b9a8e03d20b41d0ab910f92170

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-push(1)](podman-push.html)**,
**[podman-login(1)](podman-login.html)**,
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**,
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**,
**[containers-transports(5)](https://github.com/containers/image/blob/main/docs/containers-transports.5.md)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

##  HISTORY

July 2017, Originally compiled by Urvashi Mohnani <umohnani@redhat.com>


---

<a id='podman-push'></a>

## podman-push - Push an image, manifest list or image index from local
storage to elsewhere

##  NAME

podman-push - Push an image, manifest list or image index from local
storage to elsewhere

##  SYNOPSIS

**podman push** \[*options*\] *image* \[*destination*\]

**podman image push** \[*options*\] *image* \[*destination*\]

##  DESCRIPTION

Pushes an image, manifest list or image index from local storage to a
specified destination.

##  Image storage

Images are pushed from those stored in local image storage.

##  DESTINATION

DESTINATION is the location the container image is pushed to. It
supports all transports from `containers-transports(5)`. If no transport
is specified, the `docker` (i.e., container registry) transport is used.
For remote clients, including Mac and Windows (excluding WSL2) machines,
`docker` is the only supported transport.

    # Push to a container registry
    $ podman push quay.io/podman/stable

    # Push to a container registry via the docker transport
    $ podman push docker://quay.io/podman/stable

    # Push to a container registry with another tag
    $ podman push myimage quay.io/username/myimage

    # Push to a local directory
    $ podman push myimage dir:/tmp/myimage

    # Push to a tarball in the docker-archive format
    $ podman push myimage docker-archive:/tmp/myimage

    # Push to a local docker daemon
    $ sudo podman push myimage docker-daemon:docker.io/library/myimage:33

    # Push to a tarball in the OCI format
    $ podman push myimage oci-archive:/tmp/myimage

##  OPTIONS

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--cert-dir**=*path*

Use certificates at *path* (\*.crt, \*.cert, \*.key) to connect to the
registry. (Default: /etc/containers/certs.d) For details, see
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--compress**

Compress tarball image layers when pushing to a directory using the
\'dir\' transport. (default is same compression type, compressed or
uncompressed, as source)

Note: This flag can only be set when using the **dir** transport

#### **\--compression-format**=**gzip** \| *zstd* \| *zstd:chunked*

Specifies the compression format to use. Supported values are: `gzip`,
`zstd` and `zstd:chunked`. The default is `gzip` unless overridden in
the containers.conf file. `zstd:chunked` is incompatible with encrypting
images, and will be treated as `zstd` with a warning in that case.

#### **\--compression-level**=*level*

Specifies the compression level to use. The value is specific to the
compression algorithm used, e.g. for zstd the accepted values are in the
range 1-20 (inclusive) with a default of 3, while for gzip it is 1-9
(inclusive) and has a default of 5.

#### **\--creds**=*\[username\[:password\]\]*

The \[username\[:password\]\] to use to authenticate with the registry,
if required. If one or both values are not supplied, a command line
prompt appears and the value can be entered. The password is entered
without echo.

Note that the specified credentials are only used to authenticate
against target registries. They are not used for mirrors or when the
registry gets rewritten (see `containers-registries.conf(5)`); to
authenticate against those consider using a `containers-auth.json(5)`
file.

#### **\--digestfile**=*Digestfile*

After copying the image, write the digest of the resulting image to the
file.

#### **\--disable-content-trust**

This is a Docker-specific option to disable image verification to a
container registry and is not supported by Podman. This option is a NOOP
and provided solely for scripting compatibility.

#### **\--encrypt-layer**=*layer(s)*

Layer(s) to encrypt: 0-indexed layer indices with support for negative
indexing (e.g. 0 is the first layer, -1 is the last layer). If not
defined, encrypts all layers if encryption-key flag is specified.

#### **\--encryption-key**=*key*

The \[protocol:keyfile\] specifies the encryption protocol, which can be
JWE (RFC7516), PGP (RFC4880), and PKCS7 (RFC2315) and the key material
required for image encryption. For instance, jwe:/path/to/key.pem or
pgp:admin@example.com or pkcs7:/path/to/x509-file.

#### **\--force-compression**

If set, push uses the specified compression algorithm even if the
destination contains a differently-compressed variant already. Defaults
to `true` if `--compression-format` is explicitly specified on the
command-line, `false` otherwise.

#### **\--format**, **-f**=*format*

Manifest Type (oci, v2s2, or v2s1) to use when pushing an image.

#### **\--quiet**, **-q**

When writing the output image, suppress progress output

#### **\--remove-signatures**

Discard any pre-existing signatures in the image.

#### **\--retry**=*attempts*

Number of times to retry pulling or pushing images between the registry
and local storage in case of failure. Default is **3**.

#### **\--retry-delay**=*duration*

Duration of delay between retry attempts when pulling or pushing images
between the registry and local storage in case of failure. The default
is to start at two seconds and then exponentially back off. The delay is
used when this value is set, and no exponential back off occurs.

#### **\--sign-by**=*key*

Add a "simple signing" signature at the destination using the specified
key. (This option is not available with the remote Podman client,
including Mac and Windows (excluding WSL2) machines)

#### **\--sign-by-sigstore**=*param-file*

Add a sigstore signature based on further options specified in a
container\'s sigstore signing parameter file *param-file*. See
containers-sigstore-signing-params.yaml(5) for details about the file
format.

#### **\--sign-by-sigstore-private-key**=*path*

Add a sigstore signature at the destination using a private key at the
specified path. (This option is not available with the remote Podman
client, including Mac and Windows (excluding WSL2) machines)

#### **\--sign-passphrase-file**=*path*

If signing the image (using either **\--sign-by** or
**\--sign-by-sigstore-private-key**), read the passphrase to use from
the specified path.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

##  EXAMPLE

Push the specified image to a local directory:

    # podman push imageID dir:/path/to/image

Push the specified image to a local directory in OCI format:

    # podman push imageID oci-archive:/path/to/layout:image:tag

Push the specified image to a container registry:

    # podman push imageID docker://registry.example.com/repository:tag

Push the specified image to a container registry and save the digest in
the specified file:

    # podman push --digestfile=/tmp/mydigest imageID docker://registry.example.com/repository:tag

Push the specified image into the local Docker daemon container store:

    # podman push imageID docker-daemon:image:tag

Push the specified image with a different image name using credentials
from an alternate authfile path:

    # podman push --authfile temp-auths/myauths.json alpine docker://docker.io/umohnani/alpine
    Getting image source signatures
    Copying blob sha256:5bef08742407efd622d243692b79ba0055383bbce12900324f75e56f589aedb0
     4.03 MB / 4.03 MB [========================================================] 1s
    Copying config sha256:ad4686094d8f0186ec8249fc4917b71faa2c1030d7b5a025c29f26e19d95c156
     1.41 KB / 1.41 KB [========================================================] 1s
    Writing manifest to image destination
    Storing signatures

Push the specified image to a local directory as an OCI image:

    # podman push --format oci registry.access.redhat.com/rhel7 dir:rhel7-dir
    Getting image source signatures
    Copying blob sha256:9cadd93b16ff2a0c51ac967ea2abfadfac50cfa3af8b5bf983d89b8f8647f3e4
     71.41 MB / 71.41 MB [======================================================] 9s
    Copying blob sha256:4aa565ad8b7a87248163ce7dba1dd3894821aac97e846b932ff6b8ef9a8a508a
     1.21 KB / 1.21 KB [========================================================] 0s
    Copying config sha256:f1b09a81455c351eaa484b61aacd048ab613c08e4c5d1da80c4c46301b03cf3b
     3.01 KB / 3.01 KB [========================================================] 0s
    Writing manifest to image destination
    Storing signatures

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pull(1)](podman-pull.html)**,
**[podman-login(1)](podman-login.html)**,
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**,
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**,
**[containers-transports(5)](https://github.com/containers/image/blob/main/docs/containers-transports.5.md)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.


---

<a id='podman-rename'></a>

## podman-rename - Rename an existing container

##  NAME

podman-rename - Rename an existing container

##  SYNOPSIS

**podman rename** *container* *newname*

**podman container rename** *container* *newname*

##  DESCRIPTION

Rename changes the name of an existing container. The old name is freed,
and is available for use. This command can be run on containers in any
state. However, running containers may not fully receive the effects
until they are restarted - for example, a running container may still
use the old name in its logs. At present, only containers are supported;
pods and volumes cannot be renamed.

##  OPTIONS

##  EXAMPLES

Rename container with a given name.

    $ podman rename oldContainer aNewName

Rename container with a given ID.

    $ podman rename 717716c00a6b testcontainer

Create an alias for container with a given ID.

    $ podman container rename 6e7514b47180 databaseCtr

##  SEE ALSO

**[podman(1)](podman.html)**


---

<a id='podman-restart'></a>

## podman-restart - Restart one or more containers

##  NAME

podman-restart - Restart one or more containers

##  SYNOPSIS

**podman restart** \[*options*\] *container* \...

**podman container restart** \[*options*\] *container* \...

##  DESCRIPTION

The restart command allows containers to be restarted using their ID or
name. Running containers are stopped and restarted. Stopped containers
are started.

##  OPTIONS

#### **\--all**, **-a**

Restart all containers regardless of their current state.

#### **\--cidfile**

Read container ID from the specified file and restart the container. Can
be specified multiple times.

#### **\--filter**, **-f**=*filter*

Filter what containers restart. Multiple filters can be given with
multiple uses of the \--filter flag. Filters with the same key work
inclusive with the only exception being `label` which is exclusive.
Filters with different keys always work exclusive.

Valid filters are listed below:

  --------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ -------------------------------------------------------------
  id           \[ID\] Container\'s ID (CID prefix match by default; accepts
               regex)

  name         [Name](#name) Container\'s name (accepts regex)

  label        \[Key\] or \[Key=Value\] Label assigned to a container

  exited       \[Int\] Container\'s exit code

  status       \[Status\] Container\'s status: \'created\', \'exited\',
               \'paused\', \'running\', \'unknown\'

  ancestor     \[ImageName\] Image or descendant used to create container

  before       \[ID\] or [Name](#name) Containers created before this
               container

  since        \[ID\] or [Name](#name) Containers created since this
               container

  volume       \[VolumeName\] or \[MountpointDestination\] Volume mounted in
               container

  health       \[Status\] healthy or unhealthy

  pod          \[Pod\] name or full or partial ID of pod

  network      \[Network\] name or full ID of network

  until        \[DateTime\] Containers created before the given duration or
               time.
  --------------------------------------------------------------------------

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--running**

Restart all containers that are already in the *running* state.

#### **\--time**, **-t**=*seconds*

Seconds to wait before forcibly stopping the container. Use -1 for
infinite wait.

##  EXAMPLES

Restart the latest container.

    $ podman restart -l
    ec588fc80b05e19d3006bf2e8aa325f0a2e2ff1f609b7afb39176ca8e3e13467

Restart a specific container by partial container ID.

    $ podman restart ff6cf1
    ff6cf1e5e77e6dba1efc7f3fcdb20e8b89ad8947bc0518be1fcb2c78681f226f

Restart two containers by name with a timeout of 4 seconds.

    $ podman restart --time 4 test1 test2
    c3bb026838c30e5097f079fa365c9a4769d52e1017588278fa00d5c68ebc1502
    17e13a63081a995136f907024bcfe50ff532917988a152da229db9d894c5a9ec

Restart all running containers.

    $ podman restart --running

Restart all containers.

    $ podman restart --all

Restart container using ID specified in a given files.

    $ podman restart --cidfile /home/user/cidfile-1
    $ podman restart --cidfile /home/user/cidfile-1 --cidfile ./cidfile-2

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

March 2018, Originally compiled by Matt Heon <mheon@redhat.com>


---

<a id='podman-rm'></a>

## podman-rm - Remove one or more containers

##  NAME

podman-rm - Remove one or more containers

##  SYNOPSIS

**podman rm** \[*options*\] *container*

**podman container rm** \[*options*\] *container*

##  DESCRIPTION

**podman rm** removes one or more containers from the host. The
container name or ID can be used. This does not remove images. Running
or unusable containers are not removed without the **-f** option.

##  OPTIONS

#### **\--all**, **-a**

Remove all containers. Can be used in conjunction with **-f** as well.

#### **\--cidfile**=*file*

Read container ID from the specified *file* and rm the container. Can be
specified multiple times.

Command does not fail when *file* is missing and user specified
\--ignore.

#### **\--depend**

Remove selected container and recursively remove all containers that
depend on it.

#### **\--filter**=*filter*

Filter what containers remove. Multiple filters can be given with
multiple uses of the \--filter flag. Filters with the same key work
inclusive with the only exception being `label` which is exclusive.
Filters with different keys always work exclusive.

Valid filters are listed below:

  --------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ -------------------------------------------------------------
  id           \[ID\] Container\'s ID (CID prefix match by default; accepts
               regex)

  name         [Name](#name) Container\'s name (accepts regex)

  label        \[Key\] or \[Key=Value\] Label assigned to a container

  exited       \[Int\] Container\'s exit code

  status       \[Status\] Container\'s status: \'created\', \'exited\',
               \'paused\', \'running\', \'unknown\'

  ancestor     \[ImageName\] Image or descendant used to create container

  before       \[ID\] or [Name](#name) Containers created before this
               container

  since        \[ID\] or [Name](#name) Containers created since this
               container

  volume       \[VolumeName\] or \[MountpointDestination\] Volume mounted in
               container

  health       \[Status\] healthy or unhealthy

  pod          \[Pod\] name or full or partial ID of pod

  network      \[Network\] name or full ID of network

  until        \[DateTime\] Containers created before the given duration or
               time.
  --------------------------------------------------------------------------

#### **\--force**, **-f**

Force the removal of running and paused containers. Forcing a container
removal also removes containers from container storage even if the
container is not known to Podman. For example, containers that are
created by different container engines like Buildah. In addition,
forcing can be used to remove unusable containers, e.g. containers whose
OCI runtime has become unavailable.

#### **\--ignore**, **-i**

Ignore errors when specified containers are not in the container store.
A user might have decided to manually remove a container which leads to
a failure during the ExecStop directive of a systemd service referencing
that container.

Further ignore when the specified `--cidfile` does not exist as it may
have already been removed along with the container.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--time**, **-t**=*seconds*

Seconds to wait before forcibly stopping the container. Use -1 for
infinite wait.

The \--force option must be specified to use the \--time option.

#### **\--volumes**, **-v**

Remove anonymous volumes associated with the container. This does not
include named volumes created with **podman volume create**, or the
**\--volume** option of **podman run** and **podman create**.

##  EXAMPLE

Remove container with a given name:

    $ podman rm mywebserver

Remove container with a given name and all of the containers that depend
on it:

    $ podman rm --depend mywebserver

Remove multiple containers with given names or IDs:

    $ podman rm mywebserver myflaskserver 860a4b23

Remove multiple containers with IDs read from files:

    $ podman rm --cidfile ./cidfile-1 --cidfile /home/user/cidfile-2

Forcibly remove container with a given ID:

    $ podman rm -f 860a4b23

Remove all containers regardless of the run state:

    $ podman rm -f -a

Forcibly remove the last created container. (This option is not
available with the remote Podman client, including Mac and Windows
(excluding WSL2) machines):

    $ podman rm -f --latest

##  Exit Status

**0** All specified containers removed

**1** One of the specified containers did not exist, and no other
failures

**2** One of the specified containers is paused or running

**125** The command fails for any other reason

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

August 2017, Originally compiled by Ryan Cole <rycole@redhat.com>


---

<a id='podman-rmi'></a>

## podman-rmi - Remove one or more locally stored images

##  NAME

podman-rmi - Remove one or more locally stored images

##  SYNOPSIS

**podman rmi** \[*options*\] *image* \[\...\]

**podman image rm** \[*options*\] *image* \[\...\]

##  DESCRIPTION

Removes one or more locally stored images. Passing an argument *image*
deletes it, along with any of its dangling parent images. A dangling
image is an image without a tag and without being referenced by another
image.

Note: To delete an image from a remote registry, use the [**skopeo
delete**](https://github.com/containers/skopeo/blob/main/docs/skopeo-delete.html)
command. Some registries do not allow users to delete an image via a CLI
remotely.

##  OPTIONS

#### **\--all**, **-a**

Remove all images in the local storage.

#### **\--force**, **-f**

This option causes Podman to remove all containers that are using the
image before removing the image from the system.

#### **\--ignore**, **-i**

If a specified image does not exist in the local storage, ignore it and
do not throw an error.

#### **\--no-prune**

This option does not remove dangling parents of the specified image.

Remove an image by its short ID

    $ podman rmi c0ed59d05ff7

Remove an image and its associated containers.

    $ podman rmi --force imageID

Remove multiple images by their shortened IDs.

    $ podman rmi c4dfb1609ee2 93fd78260bd1 c0ed59d05ff7

Remove all images and containers.

    $ podman rmi -a -f

Remove an absent image with and without the `--ignore` flag.

    $ podman rmi --ignore nothing
    $ podman rmi nothing
    Error: nothing: image not known

##  Exit Status

**0** All specified images removed

**1** One of the specified images did not exist, and no other failures

**2** One of the specified images has child images or is being used by a
container

**125** The command fails for any other reason

##  SEE ALSO

**[podman(1)](podman.html)**,
**[skopeo-delete(1)](https://github.com/containers/skopeo/blob/main/docs/skopeo-delete.html)**

##  HISTORY

March 2017, Originally compiled by Dan Walsh <dwalsh@redhat.com>


---

<a id='podman-run'></a>

## podman-run - Run a command in a new container

##  NAME

podman-run - Run a command in a new container

##  SYNOPSIS

**podman run** \[*options*\] *image* \[*command* \[*arg* \...\]\]

**podman container run** \[*options*\] *image* \[*command* \[*arg*
\...\]\]

##  DESCRIPTION

Run a process in a new container. **podman run** starts a process with
its own file system, its own networking, and its own isolated process
tree. The *image* which starts the process may define defaults related
to the process that will be run in the container, the networking to
expose, and more, but **podman run** gives final control to the operator
or administrator who starts the container from the image. For that
reason **podman run** has more options than any other Podman command.

If the *image* is not already loaded then **podman run** will pull the
*image*, and all image dependencies, from the repository in the same way
running **podman pull** *image*, before it starts the container from
that image.

Several files will be automatically created within the container. These
include */etc/hosts*, */etc/hostname*, and */etc/resolv.conf* to manage
networking. These will be based on the host\'s version of the files,
though they can be customized with options (for example, **\--dns** will
override the host\'s DNS servers in the created *resolv.conf*).
Additionally, a container environment file is created in each container
to indicate to programs they are running in a container. This file is
located at */run/.containerenv* (or */var/run/.containerenv* for FreeBSD
containers). When using the \--privileged flag the .containerenv
contains name/value pairs indicating the container engine version,
whether the engine is running in rootless mode, the container name and
ID, as well as the image name and ID that the container is based on.
Note: */run/.containerenv* will not be created when a volume is mounted
on /run.

When running from a user defined network namespace, the
*/etc/netns/NSNAME/resolv.conf* will be used if it exists, otherwise
*/etc/resolv.conf* will be used.

Default settings are defined in `containers.conf`. Most settings for
remote connections use the servers containers.conf, except when
documented in man pages.

##  IMAGE

The image is specified using transport:path format. If no transport is
specified, the `docker` (container registry) transport is used by
default. For remote Podman, including Mac and Windows (excluding WSL2)
machines, `docker` is the only allowed transport.

**dir:**\_path\_ An existing local directory *path* storing the
manifest, layer tarballs and signatures as individual files. This is a
non-standardized format, primarily useful for debugging or noninvasive
container inspection.

    $ podman save --format docker-dir fedora -o /tmp/fedora
    $ podman run dir:/tmp/fedora echo hello

**docker://**\_docker-reference\_ (Default) An image reference stored in
a remote container image registry. Example:
\"quay.io/podman/stable:latest\". The reference can include a path to a
specific registry; if it does not, the registries listed in
registries.conf are queried to find a matching image. By default,
credentials from `podman login` (stored at
\$XDG_RUNTIME_DIR/containers/auth.json by default) are used to
authenticate; otherwise it falls back to using credentials in
\$HOME/.docker/config.json.

    $ podman run registry.fedoraproject.org/fedora:latest echo hello

**docker-archive:**\_path\_\[**:**\_docker-reference\_\] An image stored
in the `docker save` formatted file. *docker-reference* is only used
when creating such a file, and it must not contain a digest.

    $ podman save --format docker-archive fedora -o /tmp/fedora
    $ podman run docker-archive:/tmp/fedora echo hello

**docker-daemon:**\_docker-reference\_ An image in *docker-reference*
format stored in the docker daemon internal storage. The
*docker-reference* can also be an image ID (docker-daemon:algo:digest).

    $ sudo docker pull fedora
    $ sudo podman run docker-daemon:docker.io/library/fedora echo hello

**oci-archive:**\_path\_**:**\_tag\_ An image in a directory compliant
with the \"Open Container Image Layout Specification\" at the specified
*path* and specified with a *tag*.

    $ podman save --format oci-archive fedora -o /tmp/fedora
    $ podman run oci-archive:/tmp/fedora echo hello

##  OPTIONS

#### **\--add-host**=*hostname\[;hostname\[;\...\]\]*:*ip*

Add a custom host-to-IP mapping to the container\'s `/etc/hosts` file.

The option takes one or multiple semicolon-separated hostnames to be
mapped to a single IPv4 or IPv6 address, separated by a colon. It can
also be used to overwrite the IP addresses of hostnames Podman adds to
`/etc/hosts` by default (also see the **\--name** and **\--hostname**
options). This option can be specified multiple times to add additional
mappings to `/etc/hosts`. It conflicts with the **\--no-hosts** option
and conflicts with *no_hosts=true* in `containers.conf`.

Instead of an IP address, the special flag *host-gateway* can be given.
This resolves to an IP address the container can use to connect to the
host. The IP address chosen depends on your network setup, thus there\'s
no guarantee that Podman can determine the *host-gateway* address
automatically, which will then cause Podman to fail with an error
message. You can overwrite this IP address using the
*host_containers_internal_ip* option in *containers.conf*.

The *host-gateway* address is also used by Podman to automatically add
the `host.containers.internal` and `host.docker.internal` hostnames to
`/etc/hosts`. You can prevent that by either giving the **\--no-hosts**
option, or by setting *host_containers_internal_ip=\"none\"* in
*containers.conf*. If no *host-gateway* address was configured manually
and Podman fails to determine the IP address automatically, Podman will
silently skip adding these internal hostnames to `/etc/hosts`. If Podman
is running in a virtual machine using `podman machine` (this includes
Mac and Windows hosts), Podman will silently skip adding the internal
hostnames to `/etc/hosts`, unless an IP address was configured manually;
the internal hostnames are resolved by the gvproxy DNS resolver instead.

Podman will use the `/etc/hosts` file of the host as a basis by default,
i.e. any hostname present in this file will also be present in the
`/etc/hosts` file of the container. A different base file can be
configured using the *base_hosts_file* config in `containers.conf`.

#### **\--annotation**=*key=value*

Add an annotation to the container. This option can be set multiple
times.

#### **\--arch**=*ARCH*

Override the architecture, defaults to hosts, of the image to be pulled.
For example, `arm`. Unless overridden, subsequent lookups of the same
image in the local storage matches this architecture, regardless of the
host.

#### **\--attach**, **-a**=*stdin* \| *stdout* \| *stderr*

Attach to STDIN, STDOUT or STDERR.

In foreground mode (the default when **-d** is not specified), **podman
run** can start the process in the container and attach the console to
the process\'s standard input, output, and error. It can even pretend to
be a TTY (this is what most command-line executables expect) and pass
along signals. The **-a** option can be set for each of **stdin**,
**stdout**, and **stderr**.

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--blkio-weight**=*weight*

Block IO relative weight. The *weight* is a value between **10** and
**1000**.

This option is not supported on cgroups V1 rootless systems.

#### **\--blkio-weight-device**=*device:weight*

Block IO relative device weight.

#### **\--cap-add**=*capability*

Add Linux capabilities.

#### **\--cap-drop**=*capability*

Drop Linux capabilities.

#### **\--cgroup-conf**=*KEY=VALUE*

When running on cgroup v2, specify the cgroup file to write to and its
value. For example **\--cgroup-conf=memory.high=1073741824** sets the
memory.high limit to 1GB.

#### **\--cgroup-parent**=*path*

Path to cgroups under which the cgroup for the container is created. If
the path is not absolute, the path is considered to be relative to the
cgroups path of the init process. Cgroups are created if they do not
already exist.

#### **\--cgroupns**=*mode*

Set the cgroup namespace mode for the container.

-   **host**: use the host\'s cgroup namespace inside the container.
-   **container:**\_id\_: join the namespace of the specified container.
-   **private**: create a new cgroup namespace.
-   **ns:**\_path\_: join the namespace at the specified path.

If the host uses cgroups v1, the default is set to **host**. On cgroups
v2, the default is **private**.

#### **\--cgroups**=*how*

Determines whether the container creates CGroups.

Default is **enabled**.

The **enabled** option creates a new cgroup under the cgroup-parent. The
**disabled** option forces the container to not create CGroups, and thus
conflicts with CGroup options (**\--cgroupns** and
**\--cgroup-parent**). The **no-conmon** option disables a new CGroup
only for the **conmon** process. The **split** option splits the current
CGroup in two sub-cgroups: one for conmon and one for the container
payload. It is not possible to set **\--cgroup-parent** with **split**.

#### **\--chrootdirs**=*path*

Path to a directory inside the container that is treated as a `chroot`
directory. Any Podman managed file (e.g., /etc/resolv.conf, /etc/hosts,
etc/hostname) that is mounted into the root directory is mounted into
that location as well. Multiple directories are separated with a comma.

#### **\--cidfile**=*file*

Write the container ID to *file*. The file is removed along with the
container, except when used with podman \--remote run on detached
containers.

#### **\--conmon-pidfile**=*file*

Write the pid of the **conmon** process to a file. As **conmon** runs in
a separate process than Podman, this is necessary when using systemd to
restart Podman containers. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines)

#### **\--cpu-period**=*limit*

Set the CPU period for the Completely Fair Scheduler (CFS), which is a
duration in microseconds. Once the container\'s CPU quota is used up, it
will not be scheduled to run until the current period ends. Defaults to
100000 microseconds.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-quota**=*limit*

Limit the CPU Completely Fair Scheduler (CFS) quota.

Limit the container\'s CPU usage. By default, containers run with the
full CPU resource. The limit is a number in microseconds. If a number is
provided, the container is allowed to use that much CPU time until the
CPU period ends (controllable via **\--cpu-period**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-rt-period**=*microseconds*

Limit the CPU real-time period in microseconds.

Limit the container\'s Real Time CPU usage. This option tells the kernel
to restrict the container\'s Real Time CPU usage to the period
specified.

This option is only supported on cgroups V1 rootful systems.

#### **\--cpu-rt-runtime**=*microseconds*

Limit the CPU real-time runtime in microseconds.

Limit the containers Real Time CPU usage. This option tells the kernel
to limit the amount of time in a given CPU period Real Time tasks may
consume. Ex: Period of 1,000,000us and Runtime of 950,000us means that
this container can consume 95% of available CPU and leave the remaining
5% to normal priority tasks.

The sum of all runtimes across containers cannot exceed the amount
allotted to the parent cgroup.

This option is only supported on cgroups V1 rootful systems.

#### **\--cpu-shares**, **-c**=*shares*

CPU shares (relative weight).

By default, all containers get the same proportion of CPU cycles. This
proportion can be modified by changing the container\'s CPU share
weighting relative to the combined weight of all the running containers.
Default weight is **1024**.

The proportion only applies when CPU-intensive processes are running.
When tasks in one container are idle, other containers can use the
left-over CPU time. The actual amount of CPU time varies depending on
the number of containers running on the system.

For example, consider three containers, one has a cpu-share of 1024 and
two others have a cpu-share setting of 512. When processes in all three
containers attempt to use 100% of CPU, the first container receives 50%
of the total CPU time. If a fourth container is added with a cpu-share
of 1024, the first container only gets 33% of the CPU. The remaining
containers receive 16.5%, 16.5% and 33% of the CPU.

On a multi-core system, the shares of CPU time are distributed over all
CPU cores. Even if a container is limited to less than 100% of CPU time,
it can use 100% of each individual CPU core.

For example, consider a system with more than three cores. If the
container *C0* is started with **\--cpu-shares=512** running one
process, and another container *C1* with **\--cpu-shares=1024** running
two processes, this can result in the following division of CPU shares:

  PID   container   CPU   CPU share
  ----- ----------- ----- --------------
  100   C0          0     100% of CPU0
  101   C1          1     100% of CPU1
  102   C1          2     100% of CPU2

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpus**=*number*

Number of CPUs. The default is *0.0* which means no limit. This is
shorthand for **\--cpu-period** and **\--cpu-quota**, therefore the
option cannot be specified with **\--cpu-period** or **\--cpu-quota**.

On some systems, changing the CPU limits may not be allowed for non-root
users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-cpus**=*number*

CPUs in which to allow execution. Can be specified as a comma-separated
list (e.g. **0,1**), as a range (e.g. **0-3**), or any combination
thereof (e.g. **0-3,7,11-15**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-mems**=*nodes*

Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
effective on NUMA systems.

If there are four memory nodes on the system (0-3), use
**\--cpuset-mems=0,1** then processes in the container only uses memory
from the first two memory nodes.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--decryption-key**=*key\[:passphrase\]*

The \[key\[:passphrase\]\] to be used for decryption of images. Key can
point to keys and/or certificates. Decryption is tried with all keys. If
the key is protected by a passphrase, it is required to be passed in the
argument and omitted otherwise.

#### **\--detach**, **-d**

Detached mode: run the container in the background and print the new
container ID. The default is *false*.

At any time run **podman ps** in the other shell to view a list of the
running containers. Reattach to a detached container with **podman
attach** command.

When attached via tty mode, detach from the container (and leave it
running) using a configurable key sequence. The default sequence is
`ctrl-p,ctrl-q`. Specify the key sequence using the **\--detach-keys**
option, or configure it in the **containers.conf** file: see
**containers.conf(5)** for more information.

#### **\--detach-keys**=*sequence*

Specify the key sequence for detaching a container. Format is a single
character `[a-Z]` or one or more `ctrl-<value>` characters where
`<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`. Specifying \"\"
disables this feature. The default is *ctrl-p,ctrl-q*.

This option can also be set in **containers.conf**(5) file.

#### **\--device**=*host-device\[:container-device\]\[:permissions\]*

Add a host device to the container. Optional *permissions* parameter can
be used to specify device permissions by combining **r** for read, **w**
for write, and **m** for **mknod**(2).

Example: **\--device=/dev/sdc:/dev/xvdc:rwm**.

Note: if *host-device* is a symbolic link then it is resolved first. The
container only stores the major and minor numbers of the host device.

Podman may load kernel modules required for using the specified device.
The devices that Podman loads modules for when necessary are: /dev/fuse.

In rootless mode, the new device is bind mounted in the container from
the host rather than Podman creating it within the container space.
Because the bind mount retains its SELinux label on SELinux systems, the
container can get permission denied when accessing the mounted device.
Modify SELinux settings to allow containers to use all device labels via
the following command:

\$ sudo setsebool -P container_use_devices=true

Note: if the user only has access rights via a group, accessing the
device from inside a rootless container fails. Use the
`--group-add keep-groups` flag to pass the user\'s supplementary group
access into the container.

#### **\--device-cgroup-rule**=*\"type major:minor mode\"*

Add a rule to the cgroup allowed devices list. The rule is expected to
be in the format specified in the Linux kernel documentation
[admin-guide/cgroup-v1/devices](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v1/devices.html): -
*type*: `a` (all), `c` (char), or `b` (block); - *major* and *minor*:
either a number, or `*` for all; - *mode*: a composition of `r` (read),
`w` (write), and `m` (mknod(2)).

#### **\--device-read-bps**=*path:rate*

Limit read rate (in bytes per second) from a device (e.g.
**\--device-read-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-read-iops**=*path:rate*

Limit read rate (in IO operations per second) from a device (e.g.
**\--device-read-iops=/dev/sda:1000**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-bps**=*path:rate*

Limit write rate (in bytes per second) to a device (e.g.
**\--device-write-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-iops**=*path:rate*

Limit write rate (in IO operations per second) to a device (e.g.
**\--device-write-iops=/dev/sda:1000**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--disable-content-trust**

This is a Docker-specific option to disable image verification to a
container registry and is not supported by Podman. This option is a NOOP
and provided solely for scripting compatibility.

#### **\--dns**=*ipaddr*

Set custom DNS servers.

This option can be used to override the DNS configuration passed to the
container. Typically this is necessary when the host DNS configuration
is invalid for the container (e.g., **127.0.0.1**). When this is the
case the **\--dns** flag is necessary for every run.

The special value **none** can be specified to disable creation of
*/etc/resolv.conf* in the container by Podman. The */etc/resolv.conf*
file in the image is used without changes.

This option cannot be combined with **\--network** that is set to
**none** or **container:**\_id\_.

#### **\--dns-option**=*option*

Set custom DNS options. Invalid if using **\--dns-option** with
**\--network** that is set to **none** or **container:**\_id\_.

#### **\--dns-search**=*domain*

Set custom DNS search domains. Invalid if using **\--dns-search** with
**\--network** that is set to **none** or **container:**\_id\_. Use
**\--dns-search=.** to remove the search domain.

#### **\--entrypoint**=*\"command\"* \| *\'\[\"command\", \"arg1\", \...\]\'*

Override the default ENTRYPOINT from the image.

The ENTRYPOINT of an image is similar to a COMMAND because it specifies
what executable to run when the container starts, but it is (purposely)
more difficult to override. The ENTRYPOINT gives a container its default
nature or behavior. When the ENTRYPOINT is set, the container runs as if
it were that binary, complete with default options. More options can be
passed in via the COMMAND. But, if a user wants to run something else
inside the container, the **\--entrypoint** option allows a new
ENTRYPOINT to be specified.

Specify multi option commands in the form of a json string.

#### **\--env**, **-e**=*env*

Set environment variables.

This option allows arbitrary environment variables that are available
for the process to be launched inside of the container. If an
environment variable is specified without a value, Podman checks the
host environment for a value and set the variable only if it is set on
the host. As a special case, if an environment variable ending in
\_\_\*\_\_ is specified without a value, Podman searches the host
environment for variables starting with the prefix and adds those
variables to the container.

See [**Environment**](#environment) note below for precedence and
examples.

#### **\--env-file**=*file*

Read in a line-delimited file of environment variables.

See [**Environment**](#environment) note below for precedence and
examples.

#### **\--env-host**

Use host environment inside of the container. See **Environment** note
below for precedence. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines)

#### **\--env-merge**=*env*

Preprocess default environment variables for the containers. For example
if image contains environment variable `hello=world` user can preprocess
it using `--env-merge hello=${hello}-some` so new value is
`hello=world-some`.

Please note that if the environment variable `hello` is not present in
the image, then it\'ll be replaced by an empty string and so using
`--env-merge hello=${hello}-some` would result in the new value of
`hello=-some`, notice the leading `-` delimiter.

#### **\--expose**=*port\[/protocol\]*

Expose a port or a range of ports (e.g. **\--expose=3300-3310**). The
protocol can be `tcp`, `udp` or `sctp` and if not given `tcp` is
assumed. This option matches the EXPOSE instruction for image builds and
has no effect on the actual networking rules unless
**-P/\--publish-all** is used to forward to all exposed ports from
random host ports. To forward specific ports from the host into the
container use the **-p/\--publish** option instead.

#### **\--gidmap**=*\[flags\]container_uid:from_uid\[:amount\]*

Run the container in a new user namespace using the supplied GID
mapping. This option conflicts with the **\--userns** and
**\--subgidname** options. This option provides a way to map host GIDs
to container GIDs in the same way as **\--uidmap** maps host UIDs to
container UIDs. For details see **\--uidmap**.

Note: the **\--gidmap** option cannot be called in conjunction with the
**\--pod** option as a gidmap cannot be set on the container level when
in a pod.

#### **\--gpus**=*ENTRY*

GPU devices to add to the container (\'all\' to pass all GPUs) Currently
only Nvidia devices are supported.

#### **\--group-add**=*group* \| *keep-groups*

Assign additional groups to the primary user running within the
container process.

-   `keep-groups` is a special flag that tells Podman to keep the
    supplementary group access.

Allows container to use the user\'s supplementary group access. If file
systems or devices are only accessible by the rootless user\'s group,
this flag tells the OCI runtime to pass the group access into the
container. Currently only available with the `crun` OCI runtime. Note:
`keep-groups` is exclusive, other groups cannot be specified with this
flag. (Not available for remote commands, including Mac and Windows
(excluding WSL2) machines)

#### **\--group-entry**=*ENTRY*

Customize the entry that is written to the `/etc/group` file within the
container when `--user` is used.

The variables \$GROUPNAME, \$GID, and \$USERLIST are automatically
replaced with their value at runtime if present.

#### **\--health-cmd**=*\"command\"* \| *\'\[\"command\", \"arg1\", \...\]\'*

Set or alter a healthcheck command for a container. The command is a
command to be executed inside the container that determines the
container health. The command is required for other healthcheck options
to be applied. A value of **none** disables existing healthchecks.

Multiple options can be passed in the form of a JSON array; otherwise,
the command is interpreted as an argument to **/bin/sh -c**.

#### **\--health-interval**=*interval*

Set an interval for the healthchecks. An *interval* of **disable**
results in no automatic timer setup. The default is **30s**.

#### **\--health-log-destination**=*directory_path*

Set the destination of the HealthCheck log. Directory path, local or
events_logger (local use container state file) (Default: local)

-   `local`: (default) HealthCheck logs are stored in overlay
    containers. (For example: `$runroot/healthcheck.log`)
-   `directory`: creates a log file named
    `<container-ID>-healthcheck.log` with HealthCheck logs in the
    specified directory.
-   `events_logger`: The log will be written with logging mechanism set
    by events_logger. It also saves the log to a default directory, for
    performance on a system with a large number of logs.

#### **\--health-max-log-count**=*number of stored logs*

Set maximum number of attempts in the HealthCheck log file. (\'0\' value
means an infinite number of attempts in the log file) (Default: 5
attempts)

#### **\--health-max-log-size**=*size of stored logs*

Set maximum length in characters of stored HealthCheck log. (\"0\" value
means an infinite log length) (Default: 500 characters)

#### **\--health-on-failure**=*action*

Action to take once the container transitions to an unhealthy state. The
default is **none**.

-   **none**: Take no action.
-   **kill**: Kill the container.
-   **restart**: Restart the container. Do not combine the `restart`
    action with the `--restart` flag. When running inside of a systemd
    unit, consider using the `kill` or `stop` action instead to make use
    of systemd\'s restart policy.
-   **stop**: Stop the container.

#### **\--health-retries**=*retries*

The number of retries allowed before a healthcheck is considered to be
unhealthy. The default value is **3**.

#### **\--health-start-period**=*period*

The initialization time needed for a container to bootstrap. The value
can be expressed in time format like **2m3s**. The default value is
**0s**.

Note: The health check command is executed as soon as a container is
started, if the health check is successful the container\'s health state
will be updated to `healthy`. However, if the health check fails, the
health state will stay as `starting` until either the health check is
successful or until the `--health-start-period` time is over. If the
health check command fails after the `--health-start-period` time is
over, the health state will be updated to `unhealthy`. The health check
command is executed periodically based on the value of
`--health-interval`.

#### **\--health-startup-cmd**=*\"command\"* \| *\'\[\"command\", \"arg1\", \...\]\'*

Set a startup healthcheck command for a container. This command is
executed inside the container and is used to gate the regular
healthcheck. When the startup command succeeds, the regular healthcheck
begins and the startup healthcheck ceases. Optionally, if the command
fails for a set number of attempts, the container is restarted. A
startup healthcheck can be used to ensure that containers with an
extended startup period are not marked as unhealthy until they are fully
started. Startup healthchecks can only be used when a regular
healthcheck (from the container\'s image or the **\--health-cmd**
option) is also set.

#### **\--health-startup-interval**=*interval*

Set an interval for the startup healthcheck. An *interval* of
**disable** results in no automatic timer setup. The default is **30s**.

#### **\--health-startup-retries**=*retries*

The number of attempts allowed before the startup healthcheck restarts
the container. If set to **0**, the container is never restarted. The
default is **0**.

#### **\--health-startup-success**=*retries*

The number of successful runs required before the startup healthcheck
succeeds and the regular healthcheck begins. A value of **0** means that
any success begins the regular healthcheck. The default is **0**.

#### **\--health-startup-timeout**=*timeout*

The maximum time a startup healthcheck command has to complete before it
is marked as failed. The value can be expressed in a time format like
**2m3s**. The default value is **30s**.

#### **\--health-timeout**=*timeout*

The maximum time allowed to complete the healthcheck before an interval
is considered failed. Like start-period, the value can be expressed in a
time format such as **1m22s**. The default value is **30s**.

#### **\--help**

Print usage statement

#### **\--hostname**, **-h**=*name*

Set the container\'s hostname inside the container.

This option can only be used with a private UTS namespace
`--uts=private` (default). If `--pod` is given and the pod shares the
same UTS namespace (default), the pod\'s hostname is used. The given
hostname is also added to the `/etc/hosts` file using the container\'s
primary IP address (also see the **\--add-host** option).

#### **\--hostuser**=*name*

Add a user account to /etc/passwd from the host to the container. The
Username or UID must exist on the host system.

#### **\--http-proxy**

By default proxy environment variables are passed into the container if
set for the Podman process. This can be disabled by setting the value to
**false**. The environment variables passed in include **http_proxy**,
**https_proxy**, **ftp_proxy**, **no_proxy**, and also the upper case
versions of those. This option is only needed when the host system must
use a proxy but the container does not use any proxy. Proxy environment
variables specified for the container in any other way overrides the
values that have been passed through from the host. (Other ways to
specify the proxy for the container include passing the values with the
**\--env** flag, or hard coding the proxy environment at container build
time.) When used with the remote client it uses the proxy environment
variables that are set on the server process.

Defaults to **true**.

#### **\--image-volume**=**bind** \| *tmpfs* \| *ignore*

Tells Podman how to handle the builtin image volumes. Default is
**bind**.

-   **bind**: An anonymous named volume is created and mounted into the
    container.
-   **tmpfs**: The volume is mounted onto the container as a tmpfs,
    which allows the users to create content that disappears when the
    container is stopped.
-   **ignore**: All volumes are just ignored and no action is taken.

#### **\--init**

Run an init inside the container that forwards signals and reaps
processes. The container-init binary is mounted at `/run/podman-init`.
Mounting over `/run` breaks container execution.

#### **\--init-path**=*path*

Path to the container-init binary.

#### **\--interactive**, **-i**

When set to **true**, make stdin available to the contained process. If
**false**, the stdin of the contained process is empty and immediately
closed.

If attached, stdin is piped to the contained process. If detached,
reading stdin will block until later attached.

**Caveat:** Podman will consume input from stdin as soon as it becomes
available, even if the contained process doesn\'t request it.

#### **\--ip**=*ipv4*

Specify a static IPv4 address for the container, for example
**10.88.64.128**. This option can only be used if the container is
joined to only a single network - i.e., **\--network=network-name** is
used at most once - and if the container is not joining another
container\'s network namespace via **\--network=container:*id***. The
address must be within the network\'s IP address pool (default
**10.88.0.0/16**).

To specify multiple static IP addresses per container, set multiple
networks using the **\--network** option with a static IP address
specified for each using the `ip` mode for that option.

#### **\--ip6**=*ipv6*

Specify a static IPv6 address for the container, for example
**fd46:db93:aa76:ac37::10**. This option can only be used if the
container is joined to only a single network - i.e.,
**\--network=network-name** is used at most once - and if the container
is not joining another container\'s network namespace via
**\--network=container:*id***. The address must be within the network\'s
IPv6 address pool.

To specify multiple static IPv6 addresses per container, set multiple
networks using the **\--network** option with a static IPv6 address
specified for each using the `ip6` mode for that option.

#### **\--ipc**=*ipc*

Set the IPC namespace mode for a container. The default is to create a
private IPC namespace.

-   \"\": Use Podman\'s default, defined in containers.conf.
-   **container:**\_id\_: reuses another container\'s shared memory,
    semaphores, and message queues
-   **host**: use the host\'s shared memory, semaphores, and message
    queues inside the container. Note: the host mode gives the container
    full access to local shared memory and is therefore considered
    insecure.
-   **none**: private IPC namespace, with /dev/shm not mounted.
-   **ns:**\_path\_: path to an IPC namespace to join.
-   **private**: private IPC namespace.
-   **shareable**: private IPC namespace with a possibility to share it
    with other containers.

#### **\--label**, **-l**=*key=value*

Add metadata to a container.

#### **\--label-file**=*file*

Read in a line-delimited file of labels.

#### **\--link-local-ip**=*ip*

Not implemented.

#### **\--log-driver**=*driver*

Logging driver for the container. Currently available options are
**k8s-file**, **journald**, **none**, **passthrough** and
**passthrough-tty**, with **json-file** aliased to **k8s-file** for
scripting compatibility. (Default **journald**).

The podman info command below displays the default log-driver for the
system.

    $ podman info --format '{{ .Host.LogDriver }}'
    journald

The **passthrough** driver passes down the standard streams (stdin,
stdout, stderr) to the container. It is not allowed with the remote
Podman client, including Mac and Windows (excluding WSL2) machines, and
on a tty, since it is vulnerable to attacks via TIOCSTI.

The **passthrough-tty** driver is the same as **passthrough** except
that it also allows it to be used on a TTY if the user really wants it.

#### **\--log-opt**=*name=value*

Logging driver specific options.

Set custom logging configuration. The following *name*s are supported:

**path**: specify a path to the log file (e.g. **\--log-opt
path=/var/log/container/mycontainer.json**);

**max-size**: specify a max size of the log file (e.g. **\--log-opt
max-size=10mb**);

**tag**: specify a custom log tag for the container (e.g. **\--log-opt
tag=\"{{.ImageName}}\"**. It supports the same keys as **podman inspect
\--format**. This option is currently supported only by the **journald**
log driver.

#### **\--mac-address**=*address*

Container network interface MAC address (e.g. 92:d0:c6:0a:29:33) This
option can only be used if the container is joined to only a single
network - i.e., **\--network=*network-name*** is used at most once - and
if the container is not joining another container\'s network namespace
via **\--network=container:*id***.

Remember that the MAC address in an Ethernet network must be unique. The
IPv6 link-local address is based on the device\'s MAC address according
to RFC4862.

To specify multiple static MAC addresses per container, set multiple
networks using the **\--network** option with a static MAC address
specified for each using the `mac` mode for that option.

#### **\--memory**, **-m**=*number\[unit\]*

Memory limit. A *unit* can be **b** (bytes), **k** (kibibytes), **m**
(mebibytes), or **g** (gibibytes).

Allows the memory available to a container to be constrained. If the
host supports swap memory, then the **-m** memory setting can be larger
than physical RAM. If a limit of 0 is specified (not using **-m**), the
container\'s memory is not limited. The actual limit may be rounded up
to a multiple of the operating system\'s page size (the value is very
large, that\'s millions of trillions).

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-reservation**=*number\[unit\]*

Memory soft limit. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes).

After setting memory reservation, when the system detects memory
contention or low memory, containers are forced to restrict their
consumption to their reservation. So always set the value below
**\--memory**, otherwise the hard limit takes precedence. By default,
memory reservation is the same as memory limit.

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swap**=*number\[unit\]*

A limit value equal to memory plus swap. A *unit* can be **b** (bytes),
**k** (kibibytes), **m** (mebibytes), or **g** (gibibytes).

Must be used with the **-m** (**\--memory**) flag. The argument value
must be larger than that of **-m** (**\--memory**) By default, it is set
to double the value of **\--memory**.

Set *number* to **-1** to enable unlimited swap.

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swappiness**=*number*

Tune a container\'s memory swappiness behavior. Accepts an integer
between *0* and *100*.

This flag is only supported on cgroups V1 rootful systems.

#### **\--mount**=*type=TYPE,TYPE-SPECIFIC-OPTION\[,\...\]*

Attach a filesystem mount to the container

Current supported mount TYPEs are **bind**, **devpts**, **glob**,
**image**, **ramfs**, **tmpfs** and **volume**.

Options common to all mount types:

-   *src*, *source*: mount source spec for **bind**, **glob**, and
    **volume**. Mandatory for **bind** and **glob**.

-   *dst*, *destination*, *target*: mount destination spec.

When source globs are specified without the destination directory, the
files and directories are mounted with their complete path within the
container. When the destination is specified, the files and directories
matching the glob on the base file name on the destination directory are
mounted. The option `type=glob,src=/foo*,destination=/tmp/bar` tells
container engines to mount host files matching /foo\* to the /tmp/bar/
directory in the container.

Options specific to type=**volume**:

-   *ro*, *readonly*: *true* or *false* (default if unspecified:
    *false*).

-   *U*, *chown*: *true* or *false* (default if unspecified: *false*).
    Recursively change the owner and group of the source volume based on
    the UID and GID of the container.

-   *idmap*: If specified, create an idmapped mount to the target user
    namespace in the container. The idmap option supports a custom
    mapping that can be different than the user namespace used by the
    container. The mapping can be specified after the idmap option like:
    `idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the
    first value is the start of the backing file system IDs that are
    mapped to the second value on the host. The length of this mapping
    is given in the third value. Multiple ranges are separated with #.
    If the specified mapping is prepended with a \'@\' then the mapping
    is considered relative to the container user namespace. The host ID
    for the mapping is changed to account for the relative position of
    the container user in the container user namespace.

Options specific to type=**image**:

-   *rw*, *readwrite*: *true* or *false* (default if unspecified:
    *false*).

-   *subpath*: Mount only a specific path within the image, instead of
    the whole image.

Options specific to **bind** and **glob**:

-   *ro*, *readonly*: *true* or *false* (default if unspecified:
    *false*).

-   *bind-propagation*: *shared*, *slave*, *private*, *unbindable*,
    *rshared*, *rslave*, *runbindable*, or **rprivate**
    (default).^[\[1\]](#Footnote1)^ See also mount(2).

-   *bind-nonrecursive*: do not set up a recursive bind mount. By
    default it is recursive.

-   *relabel*: *shared*, *private*.

-   *idmap*: *true* or *false* (default if unspecified: *false*). If
    true, create an idmapped mount to the target user namespace in the
    container.

-   *U*, *chown*: *true* or *false* (default if unspecified: *false*).
    Recursively change the owner and group of the source volume based on
    the UID and GID of the container.

-   *no-dereference*: do not dereference symlinks but copy the link
    source into the mount destination.

Options specific to type=**tmpfs** and **ramfs**:

-   *ro*, *readonly*: *true* or *false* (default if unspecified:
    *false*).

-   *tmpfs-size*: Size of the tmpfs/ramfs mount, in bytes. Unlimited by
    default in Linux.

-   *tmpfs-mode*: Octal file mode of the tmpfs/ramfs (e.g. 700 or
    0700.).

-   *tmpcopyup*: Enable copyup from the image directory at the same
    location to the tmpfs/ramfs. Used by default.

-   *notmpcopyup*: Disable copying files from the image to the
    tmpfs/ramfs.

-   *U*, *chown*: *true* or *false* (default if unspecified: *false*).
    Recursively change the owner and group of the source volume based on
    the UID and GID of the container.

Options specific to type=**devpts**:

-   *uid*: numeric UID of the file owner (default: 0).

-   *gid*: numeric GID of the file owner (default: 0).

-   *mode*: octal permission mask for the file (default: 600).

-   *max*: maximum number of PTYs (default: 1048576).

Examples:

-   `type=bind,source=/path/on/host,destination=/path/in/container`

-   `type=bind,src=/path/on/host,dst=/path/in/container,relabel=shared`

-   `type=bind,src=/path/on/host,dst=/path/in/container,relabel=shared,U=true`

-   `type=devpts,destination=/dev/pts`

-   `type=glob,src=/usr/lib/libfoo*,destination=/usr/lib,ro=true`

-   `type=image,source=fedora,destination=/fedora-image,rw=true`

-   `type=ramfs,tmpfs-size=512M,destination=/path/in/container`

-   `type=tmpfs,tmpfs-size=512M,destination=/path/in/container`

-   `type=tmpfs,destination=/path/in/container,noswap`

-   `type=volume,source=vol1,destination=/path/in/container,ro=true`

#### **\--name**=*name*

Assign a name to the container.

The operator can identify a container in three ways:

-   UUID long identifier
    ("f78375b1c487e03c9438c729345e54db9d20cfa2ac1fc3494b6eb60872e74778");
-   UUID short identifier ("f78375b1c487");
-   Name ("jonah").

Podman generates a UUID for each container, and if no name is assigned
to the container using **\--name**, Podman generates a random string
name. The name can be useful as a more human-friendly way to identify
containers. This works for both background and foreground containers.
The container\'s name is also added to the `/etc/hosts` file using the
container\'s primary IP address (also see the **\--add-host** option).

#### **\--network**=*mode*, **\--net**

Set the network mode for the container.

Valid *mode* values are:

-   **bridge\[:OPTIONS,\...\]**: Create a network stack on the default
    bridge. This is the default for rootful containers. It is possible
    to specify these additional options:

    -   **alias=**\_name\_: Add network-scoped alias for the container.
    -   **ip=**\_IPv4\_: Specify a static IPv4 address for this
        container.
    -   **ip6=**\_IPv6\_: Specify a static IPv6 address for this
        container.
    -   **mac=**\_MAC\_: Specify a static MAC address for this
        container.
    -   **interface_name=**\_name\_: Specify a name for the created
        network interface inside the container.

    For example, to set a static ipv4 address and a static mac address,
    use `--network bridge:ip=10.88.0.10,mac=44:33:22:11:00:99`.

-   *\<network name or ID\>***\[:OPTIONS,\...\]**: Connect to a
    user-defined network; this is the network name or ID from a network
    created by **[podman network create](podman-network-create.html)**.
    It is possible to specify the same options described under the
    bridge mode above. Use the **\--network** option multiple times to
    specify additional networks.\
    For backwards compatibility it is also possible to specify
    comma-separated networks on the first **\--network** argument,
    however this prevents you from using the options described under the
    bridge section above.

-   **none**: Create a network namespace for the container but do not
    configure network interfaces for it, thus the container has no
    network connectivity.

-   **container:**\_id\_: Reuse another container\'s network stack.

-   **host**: Do not create a network namespace, the container uses the
    host\'s network. Note: The host mode gives the container full access
    to local system services such as D-bus and is therefore considered
    insecure.

-   **ns:**\_path\_: Path to a network namespace to join.

-   **private**: Create a new namespace for the container. This uses the
    **bridge** mode for rootful containers and **slirp4netns** for
    rootless ones.

-   **slirp4netns\[:OPTIONS,\...\]**: use **slirp4netns**(1) to create a
    user network stack. It is possible to specify these additional
    options, they can also be set with `network_cmd_options` in
    containers.conf:

    -   **allow_host_loopback=true\|false**: Allow slirp4netns to reach
        the host loopback IP (default is 10.0.2.2 or the second IP from
        slirp4netns cidr subnet when changed, see the cidr option
        below). The default is false.
    -   **mtu=**\_MTU\_: Specify the MTU to use for this network.
        (Default is `65520`).
    -   **cidr=**\_CIDR\_: Specify ip range to use for this network.
        (Default is `10.0.2.0/24`).
    -   **enable_ipv6=true\|false**: Enable IPv6. Default is true.
        (Required for `outbound_addr6`).
    -   **outbound_addr=**\_INTERFACE\_: Specify the outbound interface
        slirp binds to (ipv4 traffic only).
    -   **outbound_addr=**\_IPv4\_: Specify the outbound ipv4 address
        slirp binds to.
    -   **outbound_addr6=**\_INTERFACE\_: Specify the outbound interface
        slirp binds to (ipv6 traffic only).
    -   **outbound_addr6=**\_IPv6\_: Specify the outbound ipv6 address
        slirp binds to.
    -   **port_handler=rootlesskit**: Use rootlesskit for port
        forwarding. Default.\
        Note: Rootlesskit changes the source IP address of incoming
        packets to an IP address in the container network namespace,
        usually `10.0.2.100`. If the application requires the real
        source IP address, e.g. web server logs, use the slirp4netns
        port handler. The rootlesskit port handler is also used for
        rootless containers when connected to user-defined networks.
    -   **port_handler=slirp4netns**: Use the slirp4netns port
        forwarding, it is slower than rootlesskit but preserves the
        correct source IP address. This port handler cannot be used for
        user-defined networks.

-   **pasta\[:OPTIONS,\...\]**: use **pasta**(1) to create a user-mode
    networking stack.\
    This is the default for rootless containers and only supported in
    rootless mode.\
    By default, IPv4 and IPv6 addresses and routes, as well as the pod
    interface name, are copied from the host. If port forwarding isn\'t
    configured, ports are forwarded dynamically as services are bound on
    either side (init namespace or container namespace). Port forwarding
    preserves the original source IP address. Options described in
    pasta(1) can be specified as comma-separated arguments.\
    In terms of pasta(1) options, **\--config-net** is given by default,
    in order to configure networking when the container is started, and
    **\--no-map-gw** is also assumed by default, to avoid direct access
    from container to host using the gateway address. The latter can be
    overridden by passing **\--map-gw** in the pasta-specific options
    (despite not being an actual pasta(1) option).\
    Also, **-t none** and **-u none** are passed if, respectively, no
    TCP or UDP port forwarding from host to container is configured, to
    disable automatic port forwarding based on bound ports. Similarly,
    **-T none** and **-U none** are given to disable the same
    functionality from container to host.\
    Some examples:

    -   **pasta:\--map-gw**: Allow the container to directly reach the
        host using the gateway address.
    -   **pasta:\--mtu,1500**: Specify a 1500 bytes MTU for the *tap*
        interface in the container.
    -   **pasta:\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,-m,1500,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options: disable IPv6,
        assign `10.0.2.0/24` to the `tap0` interface in the container,
        with gateway `10.0.2.3`, enable DNS forwarder reachable at
        `10.0.2.3`, set MTU to 1500 bytes, disable NDP, DHCPv6 and DHCP
        support.
    -   **pasta:-I,tap0,\--ipv4-only,-a,10.0.2.0,-n,24,-g,10.0.2.2,\--dns-forward,10.0.2.3,\--no-ndp,\--no-dhcpv6,\--no-dhcp**,
        equivalent to default slirp4netns(1) options with Podman
        overrides: same as above, but leave the MTU to 65520 bytes
    -   **pasta:-t,auto,-u,auto,-T,auto,-U,auto**: enable automatic port
        forwarding based on observed bound ports from both host and
        container sides
    -   **pasta:-T,5201**: enable forwarding of TCP port 5201 from
        container to host, using the loopback interface instead of the
        tap interface for improved performance

Invalid if using **\--dns**, **\--dns-option**, or **\--dns-search**
with **\--network** set to **none** or **container:**\_id\_.

If used together with **\--pod**, the container joins the pod\'s network
namespace.

#### **\--network-alias**=*alias*

Add a network-scoped alias for the container, setting the alias for all
networks that the container joins. To set a name only for a specific
network, use the alias option as described under the **\--network**
option. If the network has DNS enabled
(`podman network inspect -f {{.DNSEnabled}} <name>`), these aliases can
be used for name resolution on the given network. This option can be
specified multiple times. NOTE: When using CNI a container only has
access to aliases on the first network that it joins. This limitation
does not exist with netavark/aardvark-dns.

#### **\--no-healthcheck**

Disable any defined healthchecks for container.

#### **\--no-hosts**

Do not modify the `/etc/hosts` file in the container.

Podman assumes control over the container\'s `/etc/hosts` file by
default and adds entries for the container\'s name (see **\--name**
option) and hostname (see **\--hostname** option), the internal
`host.containers.internal` and `host.docker.internal` hosts, as well as
any hostname added using the **\--add-host** option. Refer to the
**\--add-host** option for details. Passing **\--no-hosts** disables
this, so that the image\'s `/etc/hosts` file is kept unmodified. The
same can be achieved globally by setting *no_hosts=true* in
`containers.conf`.

This option conflicts with **\--add-host**.

#### **\--oom-kill-disable**

Whether to disable OOM Killer for the container or not.

This flag is not supported on cgroups V2 systems.

#### **\--oom-score-adj**=*num*

Tune the host\'s OOM preferences for containers (accepts values from
**-1000** to **1000**).

When running in rootless mode, the specified value can\'t be lower than
the oom_score_adj for the current process. In this case, the
oom-score-adj is clamped to the current process value.

#### **\--os**=*OS*

Override the OS, defaults to hosts, of the image to be pulled. For
example, `windows`. Unless overridden, subsequent lookups of the same
image in the local storage matches this OS, regardless of the host.

#### **\--passwd**

Allow Podman to add entries to /etc/passwd and /etc/group when used in
conjunction with the \--user option. This is used to override the Podman
provided user setup in favor of entrypoint configurations such as
libnss-extrausers.

#### **\--passwd-entry**=*ENTRY*

Customize the entry that is written to the `/etc/passwd` file within the
container when `--passwd` is used.

The variables \$USERNAME, \$UID, \$GID, \$NAME, \$HOME are automatically
replaced with their value at runtime.

#### **\--personality**=*persona*

Personality sets the execution domain via Linux personality(2).

#### **\--pid**=*mode*

Set the PID namespace mode for the container. The default is to create a
private PID namespace for the container.

-   **container:**\_id\_: join another container\'s PID namespace;
-   **host**: use the host\'s PID namespace for the container. Note the
    host mode gives the container full access to local PID and is
    therefore considered insecure;
-   **ns:**\_path\_: join the specified PID namespace;
-   **private**: create a new namespace for the container (default).

#### **\--pidfile**=*path*

When the pidfile location is specified, the container process\' PID is
written to the pidfile. (This option is not available with the remote
Podman client, including Mac and Windows (excluding WSL2) machines) If
the pidfile option is not specified, the container process\' PID is
written to
/run/containers/storage/[*storage* − *driver* − *containers*/]{.math
.inline}CID/userdata/pidfile.

After the container is started, the location for the pidfile can be
discovered with the following `podman inspect` command:

    $ podman inspect --format '{{ .PidFile }}' $CID
    /run/containers/storage/${storage-driver}-containers/$CID/userdata/pidfile

#### **\--pids-limit**=*limit*

Tune the container\'s pids limit. Set to **-1** to have unlimited pids
for the container. The default is **2048** on systems that support
\"pids\" cgroup controller.

#### **\--platform**=*OS/ARCH*

Specify the platform for selecting the image. (Conflicts with \--arch
and \--os) The `--platform` option can be used to override the current
architecture and operating system. Unless overridden, subsequent lookups
of the same image in the local storage matches this platform, regardless
of the host.

#### **\--pod**=*name*

Run container in an existing pod. Podman makes the pod automatically if
the pod name is prefixed with **new:**. To make a pod with more granular
options, use the **podman pod create** command before creating a
container. When a container is run with a pod with an infra-container,
the infra-container is started first.

#### **\--pod-id-file**=*file*

Run container in an existing pod and read the pod\'s ID from the
specified *file*. When a container is run within a pod which has an
infra-container, the infra-container starts first.

#### **\--preserve-fd**=*FD1\[,FD2,\...\]*

Pass down to the process the additional file descriptors specified in
the comma separated list. It can be specified multiple times. This
option is only supported with the crun OCI runtime. It might be a
security risk to use this option with other OCI runtimes.

(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--preserve-fds**=*N*

Pass down to the process N additional file descriptors (in addition to
0, 1, 2). The total FDs are 3+N. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--privileged**

Give extended privileges to this container. The default is **false**.

By default, Podman containers are unprivileged (**=false**) and cannot,
for example, modify parts of the operating system. This is because by
default a container is only allowed limited access to devices. A
\"privileged\" container is given the same access to devices as the user
launching the container, with the exception of virtual consoles
(*/dev/tty*) when running in systemd mode (**\--systemd=always**).

A privileged container turns off the security features that isolate the
container from the host. Dropped Capabilities, limited devices,
read-only mount points, Apparmor/SELinux separation, and Seccomp filters
are all disabled. Due to the disabled security features, the privileged
field should almost never be set as containers can easily break out of
confinement.

Containers running in a user namespace (e.g., rootless containers)
cannot have more privileges than the user that launched them.

#### **\--publish**, **-p**=*\[\[ip:\]\[hostPort\]:\]containerPort\[/protocol\]*

Publish a container\'s port, or range of ports, to the host.

Both *hostPort* and *containerPort* can be specified as a range of
ports. When specifying ranges for both, the number of container ports in
the range must match the number of host ports in the range.

If host IP is set to 0.0.0.0 or not set at all, the port is bound on all
IPs on the host.

By default, Podman publishes TCP ports. To publish a UDP port instead,
give `udp` as protocol. To publish both TCP and UDP ports, set
`--publish` twice, with `tcp`, and `udp` as protocols respectively.
Rootful containers can also publish ports using the `sctp` protocol.

Host port does not have to be specified (e.g.
`podman run -p 127.0.0.1::80`). If it is not, the container port is
randomly assigned a port on the host.

Use **podman port** to see the actual mapping:
`podman port $CONTAINER $CONTAINERPORT`.

Note that the network drivers `macvlan` and `ipvlan` do not support port
forwarding, it will have no effect on these networks.

**Note:** If a container runs within a pod, it is not necessary to
publish the port for the containers in the pod. The port must only be
published by the pod itself. Pod network stacks act like the network
stack on the host - meaning a variety of containers in the pod and
programs in the container all share a single interface, IP address, and
associated ports. If one container binds to a port, no other container
can use that port within the pod while it is in use. Containers in the
pod can also communicate over localhost by having one container bind to
localhost in the pod, and another connect to that port.

#### **\--publish-all**, **-P**

Publish all exposed ports to random ports on the host interfaces. The
default is **false**.

When set to **true**, publish all exposed ports to the host interfaces.
If the operator uses **-P** (or **-p**) then Podman makes the exposed
port accessible on the host and the ports are available to any client
that can reach the host.

When using this option, Podman binds any exposed port to a random port
on the host within an ephemeral port range defined by
*/proc/sys/net/ipv4/ip_local_port_range*. To find the mapping between
the host ports and the exposed ports, use **podman port**.

#### **\--pull**=*policy*

Pull image policy. The default is **missing**.

-   **always**: Always pull the image and throw an error if the pull
    fails.
-   **missing**: Pull the image only when the image is not in the local
    containers storage. Throw an error if no image is found and the pull
    fails.
-   **never**: Never pull the image but use the one from the local
    containers storage. Throw an error if no image is found.
-   **newer**: Pull if the image on the registry is newer than the one
    in the local containers storage. An image is considered to be newer
    when the digests are different. Comparing the time stamps is prone
    to errors. Pull errors are suppressed if a local image was found.

#### **\--quiet**, **-q**

Suppress output information when pulling images

#### **\--rdt-class**=*intel-rdt-class-of-service*

Rdt-class sets the class of service (CLOS or COS) for the container to
run in. Based on the Cache Allocation Technology (CAT) feature that is
part of Intel\'s Resource Director Technology (RDT) feature set, all
container processes will run within the pre-configured COS, representing
a part of the cache. The COS has to be created and configured using a
pseudo file system (usually mounted at `/sys/fs/resctrl`) that the
resctrl kernel driver provides. Assigning the container to a COS
requires root privileges and thus doesn\'t work in a rootless
environment. Currently, the feature is only supported using `runc` as a
runtime. See <https://docs.kernel.org/arch/x86/resctrl.html> for more
details on creating a COS before a container can be assigned to it.

#### **\--read-only**

Mount the container\'s root filesystem as read-only.

By default, container root filesystems are writable, allowing processes
to write files anywhere. By specifying the **\--read-only** flag, the
containers root filesystem are mounted read-only prohibiting any writes.

#### **\--read-only-tmpfs**

When running \--read-only containers, mount a read-write tmpfs on
*/dev*, */dev/shm*, */run*, */tmp*, and */var/tmp*. The default is
**true**.

  ------------------------------------------------------------------------------
  \--read-only   \--read-only-tmpfs   /     /run, /tmp, /var/tmp
  -------------- -------------------- ----- ------------------------------------
  true           true                 r/o   r/w

  true           false                r/o   r/o

  false          false                r/w   r/w

  false          true                 r/w   r/w
  ------------------------------------------------------------------------------

When **\--read-only=true** and **\--read-only-tmpfs=true** additional
tmpfs are mounted on the /tmp, /run, and /var/tmp directories.

When **\--read-only=true** and **\--read-only-tmpfs=false** /dev and
/dev/shm are marked Read/Only and no tmpfs are mounted on /tmp, /run and
/var/tmp. The directories are exposed from the underlying image, meaning
they are read-only by default. This makes the container totally
read-only. No writable directories exist within the container. In this
mode writable directories need to be added via external volumes or
mounts.

By default, when **\--read-only=false**, the /dev and /dev/shm are
read/write, and the /tmp, /run, and /var/tmp are read/write directories
from the container image.

#### **\--replace**

If another container with the same name already exists, replace and
remove it. The default is **false**.

#### **\--requires**=*container*

Specify one or more requirements. A requirement is a dependency
container that is started before this container. Containers can be
specified by name or ID, with multiple containers being separated by
commas.

#### **\--restart**=*policy*

Restart policy to follow when containers exit. Restart policy does not
take effect if a container is stopped via the **podman kill** or
**podman stop** commands.

Valid *policy* values are:

-   `no` : Do not restart containers on exit
-   `never` : Synonym for **no**; do not restart containers on exit
-   `on-failure[:max_retries]` : Restart containers when they exit with
    a non-zero exit code, retrying indefinitely or until the optional
    *max_retries* count is hit
-   `always` : Restart containers when they exit, regardless of status,
    retrying indefinitely
-   `unless-stopped` : Identical to **always**

Podman provides a systemd unit file, podman-restart.service, which
restarts containers after a system reboot.

When running containers in systemd services, use the restart
functionality provided by systemd. In other words, do not use this
option in a container unit, instead set the `Restart=` systemd directive
in the `[Service]` section. See **podman-systemd.unit**(5) and
**systemd.service**(5).

#### **\--retry**=*attempts*

Number of times to retry pulling or pushing images between the registry
and local storage in case of failure. Default is **3**.

#### **\--retry-delay**=*duration*

Duration of delay between retry attempts when pulling or pushing images
between the registry and local storage in case of failure. The default
is to start at two seconds and then exponentially back off. The delay is
used when this value is set, and no exponential back off occurs.

#### **\--rm**

Automatically remove the container and any anonymous unnamed volume
associated with the container when it exits. The default is **false**.

#### **\--rmi**

After exit of the container, remove the image unless another container
is using it. Implies \--rm on the new container. The default is *false*.

#### **\--rootfs**

If specified, the first argument refers to an exploded container on the
file system.

This is useful to run a container without requiring any image
management, the rootfs of the container is assumed to be managed
externally.

`Overlay Rootfs Mounts`

The `:O` flag tells Podman to mount the directory from the rootfs path
as storage using the `overlay file system`. The container processes can
modify content within the mount point which is stored in the container
storage in a separate directory. In overlay terms, the source directory
is the lower, and the container storage directory is the upper.
Modifications to the mount point are destroyed when the container
finishes executing, similar to a tmpfs mount point being unmounted.

Note: On **SELinux** systems, the rootfs needs the correct label, which
is by default **unconfined_u:object_r:container_file_t:s0**.

`idmap`

If `idmap` is specified, create an idmapped mount to the target user
namespace in the container. The idmap option supports a custom mapping
that can be different than the user namespace used by the container. The
mapping can be specified after the idmap option like:
`idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the first
value is the start of the backing file system IDs that are mapped to the
second value on the host. The length of this mapping is given in the
third value. Multiple ranges are separated with #.

#### **\--sdnotify**=**container** \| *conmon* \| *healthy* \| *ignore*

Determines how to use the NOTIFY_SOCKET, as passed with systemd and
Type=notify.

Default is **container**, which means allow the OCI runtime to proxy the
socket into the container to receive ready notification. Podman sets the
MAINPID to conmon\'s pid. The **conmon** option sets MAINPID to
conmon\'s pid, and sends READY when the container has started. The
socket is never passed to the runtime or the container. The **healthy**
option sets MAINPID to conmon\'s pid, and sends READY when the container
has turned healthy; requires a healthcheck to be set. The socket is
never passed to the runtime or the container. The **ignore** option
removes NOTIFY_SOCKET from the environment for itself and child
processes, for the case where some other process above Podman uses
NOTIFY_SOCKET and Podman does not use it.

#### **\--seccomp-policy**=*policy*

Specify the policy to select the seccomp profile. If set to *image*,
Podman looks for a \"io.containers.seccomp.profile\" label in the
container-image config and use its value as a seccomp profile.
Otherwise, Podman follows the *default* policy by applying the default
profile unless specified otherwise via *\--security-opt seccomp* as
described below.

Note that this feature is experimental and may change in the future.

#### **\--secret**=*secret\[,opt=opt \...\]*

Give the container access to a secret. Can be specified multiple times.

A secret is a blob of sensitive data which a container needs at runtime
but is not stored in the image or in source control, such as usernames
and passwords, TLS certificates and keys, SSH keys or other important
generic strings or binary content (up to 500 kb in size).

When secrets are specified as type `mount`, the secrets are copied and
mounted into the container when a container is created. When secrets are
specified as type `env`, the secret is set as an environment variable
within the container. Secrets are written in the container at the time
of container creation, and modifying the secret using `podman secret`
commands after the container is created affects the secret inside the
container.

Secrets and its storage are managed using the `podman secret` command.

Secret Options

-   `type=mount|env` : How the secret is exposed to the container.
    `mount` mounts the secret into the container as a file. `env`
    exposes the secret as an environment variable. Defaults to `mount`.
-   `target=target` : Target of secret. For mounted secrets, this is the
    path to the secret inside the container. If a fully qualified path
    is provided, the secret is mounted at that location. Otherwise, the
    secret is mounted to `/run/secrets/target` for linux containers or
    `/var/run/secrets/target` for freebsd containers. If the target is
    not set, the secret is mounted to `/run/secrets/secretname` by
    default. For env secrets, this is the environment variable key.
    Defaults to `secretname`.
-   `uid=0` : UID of secret. Defaults to 0. Mount secret type only.
-   `gid=0` : GID of secret. Defaults to 0. Mount secret type only.
-   `mode=0` : Mode of secret. Defaults to 0444. Mount secret type only.

Examples

Mount at `/my/location/mysecret` with UID 1:

    --secret mysecret,target=/my/location/mysecret,uid=1

Mount at `/run/secrets/customtarget` with mode 0777:

    --secret mysecret,target=customtarget,mode=0777

Create a secret environment variable called `ENVSEC`:

    --secret mysecret,type=env,target=ENVSEC

#### **\--security-opt**=*option*

Security Options

-   **apparmor=unconfined** : Turn off apparmor confinement for the
    container

-   **apparmor**=*alternate-profile* : Set the apparmor confinement
    profile for the container

-   **label=user:**\_USER\_: Set the label user for the container
    processes

-   **label=role:**\_ROLE\_: Set the label role for the container
    processes

-   **label=type:**\_TYPE\_: Set the label process type for the
    container processes

-   **label=level:**\_LEVEL\_: Set the label level for the container
    processes

-   **label=filetype:**\_TYPE\_: Set the label file type for the
    container files

-   **label=disable**: Turn off label separation for the container

Note: Labeling can be disabled for all containers by setting label=false
in the **containers.conf** (`/etc/containers/containers.conf` or
`$HOME/.config/containers/containers.conf`) file.

-   **label=nested**: Allows SELinux modifications within the container.
    Containers are allowed to modify SELinux labels on files and
    processes, as long as SELinux policy allows. Without **nested**,
    containers view SELinux as disabled, even when it is enabled on the
    host. Containers are prevented from setting any labels.

-   **mask**=*/path/1:/path/2*: The paths to mask separated by a colon.
    A masked path cannot be accessed inside the container.

-   **no-new-privileges**: Disable container processes from gaining
    additional privileges.

-   **seccomp=unconfined**: Turn off seccomp confinement for the
    container.

-   **seccomp=profile.json**: JSON file to be used as a seccomp filter.
    Note that the `io.podman.annotations.seccomp` annotation is set with
    the specified value as shown in `podman inspect`.

-   **proc-opts**=*OPTIONS* : Comma-separated list of options to use for
    the /proc mount. More details for the possible mount options are
    specified in the **proc(5)** man page.

-   **unmask**=*ALL* or */path/1:/path/2*, or shell expanded paths
    (/proc/\*): Paths to unmask separated by a colon. If set to **ALL**,
    it unmasks all the paths that are masked or made read-only by
    default. The default masked paths are **/proc/acpi, /proc/kcore,
    /proc/keys, /proc/latency_stats, /proc/sched_debug, /proc/scsi,
    /proc/timer_list, /proc/timer_stats, /sys/firmware, and
    /sys/fs/selinux**, **/sys/devices/virtual/powercap**. The default
    paths that are read-only are **/proc/asound**, **/proc/bus**,
    **/proc/fs**, **/proc/irq**, **/proc/sys**, **/proc/sysrq-trigger**,
    **/sys/fs/cgroup**.

Note: Labeling can be disabled for all containers by setting
**label=false** in the **containers.conf**(5) file.

#### **\--shm-size**=*number\[unit\]*

Size of */dev/shm*. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes). If the unit is omitted, the
system uses bytes. If the size is omitted, the default is **64m**. When
*size* is **0**, there is no limit on the amount of memory used for IPC
by the container. This option conflicts with **\--ipc=host**.

#### **\--shm-size-systemd**=*number\[unit\]*

Size of systemd-specific tmpfs mounts such as /run, /run/lock,
/var/log/journal and /tmp. A *unit* can be **b** (bytes), **k**
(kibibytes), **m** (mebibytes), or **g** (gibibytes). If the unit is
omitted, the system uses bytes. If the size is omitted, the default is
**64m**. When *size* is **0**, the usage is limited to 50% of the
host\'s available memory.

#### **\--sig-proxy**

Proxy received signals to the container process. SIGCHLD, SIGURG,
SIGSTOP, and SIGKILL are not proxied.

The default is **true**.

#### **\--stop-signal**=*signal*

Signal to stop a container. Default is **SIGTERM**.

#### **\--stop-timeout**=*seconds*

Timeout to stop a container. Default is **10**. Remote connections use
local containers.conf for defaults.

#### **\--subgidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subgid* file. If running rootless, the user needs to have the
right to use the mapping. See **subgid**(5). This flag conflicts with
**\--userns** and **\--gidmap**.

#### **\--subuidname**=*name*

Run the container in a new user namespace using the map with *name* in
the */etc/subuid* file. If running rootless, the user needs to have the
right to use the mapping. See **subuid**(5). This flag conflicts with
**\--userns** and **\--uidmap**.

#### **\--sysctl**=*name=value*

Configure namespaced kernel parameters at runtime.

For the IPC namespace, the following sysctls are allowed:

-   kernel.msgmax
-   kernel.msgmnb
-   kernel.msgmni
-   kernel.sem
-   kernel.shmall
-   kernel.shmmax
-   kernel.shmmni
-   kernel.shm_rmid_forced
-   Sysctls beginning with fs.mqueue.\*

Note: if using the **\--ipc=host** option, the above sysctls are not
allowed.

For the network namespace, only sysctls beginning with net.\* are
allowed.

Note: if using the **\--network=host** option, the above sysctls are not
allowed.

#### **\--systemd**=*true* \| *false* \| *always*

Run container in systemd mode. The default is **true**.

-   **true** enables systemd mode only when the command executed inside
    the container is *systemd*, */usr/sbin/init*, */sbin/init* or
    */usr/local/sbin/init*.

-   **false** disables systemd mode.

-   **always** enforces the systemd mode to be enabled.

Running the container in systemd mode causes the following changes:

-   Podman mounts tmpfs file systems on the following directories
    -   */run*
    -   */run/lock*
    -   */tmp*
    -   */sys/fs/cgroup/systemd* (on a cgroup v1 system)
    -   */var/lib/journal*
-   Podman sets the default stop signal to **SIGRTMIN+3**.
-   Podman sets **container_uuid** environment variable in the container
    to the first 32 characters of the container ID.
-   Podman does not mount virtual consoles (*/dev/tty*) when running
    with **\--privileged**.
-   On cgroup v2, */sys/fs/cgroup* is mounted writeable.

This allows systemd to run in a confined container without any
modifications.

Note that on **SELinux** systems, systemd attempts to write to the
cgroup file system. Containers writing to the cgroup file system are
denied by default. The **container_manage_cgroup** boolean must be
enabled for this to be allowed on an SELinux separated system.

    setsebool -P container_manage_cgroup true

#### **\--timeout**=*seconds*

Maximum time a container is allowed to run before conmon sends it the
kill signal. By default containers run until they exit or are stopped by
`podman stop`.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

#### **\--tmpfs**=*fs*

Create a tmpfs mount.

Mount a temporary filesystem (**tmpfs**) mount into a container, for
example:

    $ podman run -d --tmpfs /tmp:rw,size=787448k,mode=1777 my_image

This command mounts a **tmpfs** at */tmp* within the container. The
supported mount options are the same as the Linux default mount flags.
If no options are specified, the system uses the following options:
**rw,noexec,nosuid,nodev**.

#### **\--tty**, **-t**

Allocate a pseudo-TTY. The default is **false**.

When set to **true**, Podman allocates a pseudo-tty and attach to the
standard input of the container. This can be used, for example, to run a
throwaway interactive shell.

**NOTE**: The \--tty flag prevents redirection of standard output. It
combines STDOUT and STDERR, it can insert control characters, and it can
hang pipes. This option is only used when run interactively in a
terminal. When feeding input to Podman, use -i only, not -it.

    echo "asdf" | podman run --rm -i someimage /bin/cat

#### **\--tz**=*timezone*

Set timezone in container. This flag takes area-based timezones, GMT
time, as well as `local`, which sets the timezone in the container to
match the host machine. See `/usr/share/zoneinfo/` for valid timezones.
Remote connections use local containers.conf for defaults

#### **\--uidmap**=*\[flags\]container_uid:from_uid\[:amount\]*

Run the container in a new user namespace using the supplied UID
mapping. This option conflicts with the **\--userns** and
**\--subuidname** options. This option provides a way to map host UIDs
to container UIDs. It can be passed several times to map different
ranges.

The possible values of the optional *flags* are discussed further down
on this page. The *amount* value is optional and assumed to be **1** if
not given.

The *from_uid* value is based upon the user running the command, either
rootful or rootless users.

-   rootful user: \[*flags*\]*container_uid*:*host_uid*\[:*amount*\]

-   rootless user:
    \[*flags*\]*container_uid*:*intermediate_uid*\[:*amount*\]

    `Rootful mappings`

When **podman run** is called by a privileged user, the option
**\--uidmap** works as a direct mapping between host UIDs and container
UIDs.

host UID -\> container UID

The *amount* specifies the number of consecutive UIDs that is mapped. If
for example *amount* is **4** the mapping looks like:

  host UID         container UID
  ---------------- ---------------------
  *from_uid*       *container_uid*
  *from_uid* + 1   *container_uid* + 1
  *from_uid* + 2   *container_uid* + 2
  *from_uid* + 3   *container_uid* + 3

`Rootless mappings`

When **podman run** is called by an unprivileged user (i.e. running
rootless), the value *from_uid* is interpreted as an \"intermediate
UID\". In the rootless case, host UIDs are not mapped directly to
container UIDs. Instead the mapping happens over two mapping steps:

host UID -\> intermediate UID -\> container UID

The **\--uidmap** option only influences the second mapping step.

The first mapping step is derived by Podman from the contents of the
file */etc/subuid* and the UID of the user calling Podman.

First mapping step:

  host UID              intermediate UID
  --------------------- ------------------
  UID for Podman user   0
  1st subordinate UID   1
  2nd subordinate UID   2
  3rd subordinate UID   3
  nth subordinate UID   n

To be able to use intermediate UIDs greater than zero, the user needs to
have subordinate UIDs configured in */etc/subuid*. See **subuid**(5).

The second mapping step is configured with **\--uidmap**.

If for example *amount* is **5** the second mapping step looks like:

  intermediate UID   container UID
  ------------------ ---------------------
  *from_uid*         *container_uid*
  *from_uid* + 1     *container_uid* + 1
  *from_uid* + 2     *container_uid* + 2
  *from_uid* + 3     *container_uid* + 3
  *from_uid* + 4     *container_uid* + 4

When running as rootless, Podman uses all the ranges configured in the
*/etc/subuid* file.

The current user ID is mapped to UID=0 in the rootless user namespace.
Every additional range is added sequentially afterward:

  ----------------------------------------------------------------------------------------------------------
  host         rootless user namespace                        length
  ------------ ---------------------------------------------- ----------------------------------------------
  \$UID        0                                              1

  1            \$FIRST_RANGE_ID                               [*FIRST*~*R*~*ANGE*~*L*~*ENGTH*\|\|1+]{.math
                                                              .inline}FIRST_RANGE_LENGTH
  ----------------------------------------------------------------------------------------------------------

`Referencing a host ID from the parent namespace`

As a rootless user, the given host ID in **\--uidmap** or **\--gidmap**
is mapped from the *intermediate namespace* generated by Podman.
Sometimes it is desirable to refer directly at the *host namespace*. It
is possible to manually do so, by running
`podman unshare cat /proc/self/gid_map`, finding the desired host id at
the second column of the output, and getting the corresponding
intermediate id from the first column.

Podman can perform all that by preceding the host id in the mapping with
the `@` symbol. For instance, by specifying `--gidmap 100000:@2000:1`,
podman will look up the intermediate id corresponding to host id `2000`
and it will map the found intermediate id to the container id `100000`.
The given host id must have been subordinated (otherwise it would not be
mapped into the intermediate space in the first place).

If the length is greater than one, for instance with
`--gidmap 100000:@2000:2`, Podman will map host ids `2000` and `2001` to
`100000` and `100001`, respectively, regardless of how the intermediate
mapping is defined.

`Extending previous mappings`

Some mapping modifications may be cumbersome. For instance, a user
starts with a mapping such as `--gidmap="0:0:65000"`, that needs to be
changed such as the parent id `1000` is mapped to container id `100000`
instead, leaving container id `1` unassigned. The corresponding
`--gidmap` becomes
`--gidmap="0:0:1" --gidmap="2:2:65534" --gidmap="100000:1:1"`.

This notation can be simplified using the `+` flag, that takes care of
breaking previous mappings removing any conflicting assignment with the
given mapping. The flag is given before the container id as follows:
`--gidmap="0:0:65000" --gidmap="+100000:1:1"`

  Flag   Example         Description
  ------ --------------- -----------------------------
  `+`    `+100000:1:1`   Extend the previous mapping

This notation leads to gaps in the assignment, so it may be convenient
to fill those gaps afterwards:
`--gidmap="0:0:65000" --gidmap="+100000:1:1" --gidmap="1:65001:1"`

One specific use case for this flag is in the context of rootless users.
A rootless user may specify mappings with the `+` flag as in
`--gidmap="+100000:1:1"`. Podman will then \"fill the gaps\" starting
from zero with all the remaining intermediate ids. This is convenient
when a user wants to map a specific intermediate id to a container id,
leaving the rest of subordinate ids to be mapped by Podman at will.

`Passing only one of --uidmap or --gidmap`

Usually, subordinated user and group ids are assigned simultaneously,
and for any user the subordinated user ids match the subordinated group
ids. For convenience, if only one of **\--uidmap** or **\--gidmap** is
given, podman assumes the mapping refers to both UIDs and GIDs and
applies the given mapping to both. If only one value of the two needs to
be changed, the mappings should include the `u` or the `g` flags to
specify that they only apply to UIDs or GIDs and should not be copied
over.

  flag   Example           Description
  ------ ----------------- ----------------------------------
  `u`    `u20000:2000:1`   The mapping only applies to UIDs
  `g`    `g10000:1000:1`   The mapping only applies to GIDs

For instance given the command

    podman run --gidmap "0:0:1000" --gidmap "g2000:2000:1"

Since no **\--uidmap** is given, the **\--gidmap** is copied to
**\--uidmap**, giving a command equivalent to

    podman run --gidmap "0:0:1000" --gidmap "2000:2000:1" --uidmap "0:0:1000"

The `--gidmap "g2000:2000:1"` used the `g` flag and therefore it was not
copied to **\--uidmap**.

`Rootless mapping of additional host GIDs`

A rootless user may desire to map a specific host group that has already
been subordinated within */etc/subgid* without specifying the rest of
the mapping.

This can be done with \*\*\--gidmap
\"+g*container_gid*:[@\*host_gid]{.citation cites="*host_gid"}\*\"\*\*

Where:

-   The host GID is given through the `@` symbol
-   The mapping of this GID is not copied over to **\--usermap** thanks
    to the `g` flag.
-   The rest of the container IDs will be mapped starting from 0 to n,
    with all the remaining subordinated GIDs, thanks to the `+` flag.

For instance, if a user belongs to the group `2000` and that group is
subordinated to that user (with
`usermod --add-subgids 2000-2000 $USER`), the user can map the group
into the container with: **\--gidmap=+g100000:[\@2000]{.citation
cites="2000"}**.

If this mapping is combined with the option,
**\--group-add=keep-groups**, the process in the container will belong
to group `100000`, and files belonging to group `2000` in the host will
appear as being owned by group `100000` inside the container.

    podman run --group-add=keep-groups --gidmap="+g100000:@2000" ...

`No subordinate UIDs`

Even if a user does not have any subordinate UIDs in */etc/subuid*,
**\--uidmap** can be used to map the normal UID of the user to a
container UID by running
`podman run --uidmap $container_uid:0:1 --user $container_uid ...`.

`Pods`

The **\--uidmap** option cannot be called in conjunction with the
**\--pod** option as a uidmap cannot be set on the container level when
in a pod.

#### **\--ulimit**=*option*

Ulimit options. Sets the ulimits values inside of the container.

\--ulimit with a soft and hard limit in the format =\[:\]. For example:

\$ podman run \--ulimit nofile=1024:1024 \--rm ubi9 ulimit -n 1024

Set -1 for the soft or hard limit to set the limit to the maximum limit
of the current process. In rootful mode this is often unlimited.

If nofile and nproc are unset, a default value of 1048576 will be used,
unless overridden in containers.conf(5). However, if the default value
exceeds the hard limit for the current rootless user, the current hard
limit will be applied instead.

Use **host** to copy the current configuration from the host.

Don\'t use nproc with the ulimit flag as Linux uses nproc to set the
maximum number of processes available to a user, not to a container.

Use the \--pids-limit option to modify the cgroup control to limit the
number of processes within a container.

#### **\--umask**=*umask*

Set the umask inside the container. Defaults to `0022`. Remote
connections use local containers.conf for defaults

#### **\--unsetenv**=*env*

Unset default environment variables for the container. Default
environment variables include variables provided natively by Podman,
environment variables configured by the image, and environment variables
from containers.conf.

#### **\--unsetenv-all**

Unset all default environment variables for the container. Default
environment variables include variables provided natively by Podman,
environment variables configured by the image, and environment variables
from containers.conf.

#### **\--user**, **-u**=*user\[:group\]*

Sets the username or UID used and, optionally, the groupname or GID for
the specified command. Both *user* and *group* may be symbolic or
numeric.

Without this argument, the command runs as the user specified in the
container image. Unless overridden by a `USER` command in the
Containerfile or by a value passed to this option, this user generally
defaults to root.

When a user namespace is not in use, the UID and GID used within the
container and on the host match. When user namespaces are in use,
however, the UID and GID in the container may correspond to another UID
and GID on the host. In rootless containers, for example, a user
namespace is always used, and root in the container by default
corresponds to the UID and GID of the user invoking Podman.

#### **\--userns**=*mode*

Set the user namespace mode for the container.

If `--userns` is not set, the default value is determined as follows. -
If `--pod` is set, `--userns` is ignored and the user namespace of the
pod is used. - If the environment variable **PODMAN_USERNS** is set its
value is used. - If `userns` is specified in `containers.conf` this
value is used. - Otherwise, `--userns=host` is assumed.

`--userns=""` (i.e., an empty string) is an alias for `--userns=host`.

This option is incompatible with **\--gidmap**, **\--uidmap**,
**\--subuidname** and **\--subgidname**.

Rootless user \--userns=Key mappings:

  ----------------------------------------------------------------------
  Key                           Host User     Container User
  ----------------------------- ------------- --------------------------
  auto                          \$UID         nil (Host User UID is not
                                              mapped into container.)

  host                          \$UID         0 (Default User account
                                              mapped to root user in
                                              container.)

  keep-id                       \$UID         \$UID (Map user account to
                                              same UID within
                                              container.)

  keep-id:uid=200,gid=210       \$UID         200:210 (Map user account
                                              to specified UID, GID
                                              value within container.)

  nomap                         \$UID         nil (Host User UID is not
                                              mapped into container.)
  ----------------------------------------------------------------------

Valid *mode* values are:

**auto**\[:*OPTIONS,\...*\]: automatically create a unique user
namespace.

-   `rootful mode`: The `--userns=auto` flag requires that the user name
    **containers** be specified in the /etc/subuid and /etc/subgid
    files, with an unused range of subordinate user IDs that Podman
    containers are allowed to allocate.

         Example: `containers:2147483647:2147483648`.

-   `rootless mode`: The users range from the /etc/subuid and
    /etc/subgid files will be used. Note running a single container
    without using \--userns=auto will use the entire range of UIDs and
    not allow further subdividing. See subuid(5).

Podman allocates unique ranges of UIDs and GIDs from the `containers`
subordinate user IDs. The size of the ranges is based on the number of
UIDs required in the image. The number of UIDs and GIDs can be
overridden with the `size` option.

The option `--userns=keep-id` uses all the subuids and subgids of the
user. The option `--userns=nomap` uses all the subuids and subgids of
the user except the user\'s own ID. Using `--userns=auto` when starting
new containers does not work as long as any containers exist that were
started with `--userns=keep-id` or `--userns=nomap`.

Valid `auto` options:

-   *gidmapping*=*CONTAINER_GID:HOST_GID:SIZE*: to force a GID mapping
    to be present in the user namespace.
-   *size*=*SIZE*: to specify an explicit size for the automatic user
    namespace. e.g. `--userns=auto:size=8192`. If `size` is not
    specified, `auto` estimates a size for the user namespace.
-   *uidmapping*=*CONTAINER_UID:HOST_UID:SIZE*: to force a UID mapping
    to be present in the user namespace.

The host UID and GID in *gidmapping* and *uidmapping* can optionally be
prefixed with the `@` symbol. In this case, podman will look up the
intermediate ID corresponding to host ID and it will map the found
intermediate ID to the container id. For details see **\--uidmap**.

**container:**\_id\_: join the user namespace of the specified
container.

**host** or **\"\"** (empty string): run in the user namespace of the
caller. The processes running in the container have the same privileges
on the host as any other process launched by the calling user.

**keep-id**: creates a user namespace where the current user\'s UID:GID
are mapped to the same values in the container. For containers created
by root, the current mapping is created into a new user namespace.

Valid `keep-id` options:

-   *uid*=UID: override the UID inside the container that is used to map
    the current user to.
-   *gid*=GID: override the GID inside the container that is used to map
    the current user to.

**nomap**: creates a user namespace where the current rootless user\'s
UID:GID are not mapped into the container. This option is not allowed
for containers created by the root user.

**ns:**\_namespace\_: run the container in the given existing user
namespace.

#### **\--uts**=*mode*

Set the UTS namespace mode for the container. The following values are
supported:

-   **host**: use the host\'s UTS namespace inside the container.
-   **private**: create a new namespace for the container (default).
-   **ns:\[path\]**: run the container in the given existing UTS
    namespace.
-   **container:\[container\]**: join the UTS namespace of the specified
    container.

#### **\--variant**=*VARIANT*

Use *VARIANT* instead of the default architecture variant of the
container image. Some images can use multiple variants of the arm
architectures, such as arm/v5 and arm/v7.

#### **\--volume**, **-v**=*\[\[SOURCE-VOLUME\|HOST-DIR:\]CONTAINER-DIR\[:OPTIONS\]\]*

Create a bind mount. If `-v /HOST-DIR:/CONTAINER-DIR` is specified,
Podman bind mounts `/HOST-DIR` from the host into `/CONTAINER-DIR` in
the Podman container. Similarly, `-v SOURCE-VOLUME:/CONTAINER-DIR`
mounts the named volume from the host into the container. If no such
named volume exists, Podman creates one. If no source is given, the
volume is created as an anonymously named volume with a randomly
generated name, and is removed when the container is removed via the
`--rm` flag or the `podman rm --volumes` command.

(Note when using the remote client, including Mac and Windows (excluding
WSL2) machines, the volumes are mounted from the remote server, not
necessarily the client machine.)

The *OPTIONS* is a comma-separated list and can be one or more of:

-   **rw**\|**ro**
-   **z**\|**Z**
-   \[**O**\]
-   \[**U**\]
-   \[**no**\]**copy**
-   \[**no**\]**dev**
-   \[**no**\]**exec**
-   \[**no**\]**suid**
-   \[**r**\]**bind**
-   \[**r**\]**shared**\|\[**r**\]**slave**\|\[**r**\]**private**\[**r**\]**unbindable**
    ^[\[1\]](#Footnote1)^
-   **idmap**\[=**options**\]

The `CONTAINER-DIR` must be an absolute path such as `/src/docs`. The
volume is mounted into the container at this directory.

If a volume source is specified, it must be a path on the host or the
name of a named volume. Host paths are allowed to be absolute or
relative; relative paths are resolved relative to the directory Podman
is run in. If the source does not exist, Podman returns an error. Users
must pre-create the source files or directories.

Any source that does not begin with a `.` or `/` is treated as the name
of a named volume. If a volume with that name does not exist, it is
created. Volumes created with names are not anonymous, and they are not
removed by the `--rm` option and the `podman rm --volumes` command.

Specify multiple **-v** options to mount one or more volumes into a
container.

`Write Protected Volume Mounts`

Add **:ro** or **:rw** option to mount a volume in read-only or
read-write mode, respectively. By default, the volumes are mounted
read-write. See examples.

`Chowning Volume Mounts`

By default, Podman does not change the owner and group of source volume
directories mounted into containers. If a container is created in a new
user namespace, the UID and GID in the container may correspond to
another UID and GID on the host.

The `:U` suffix tells Podman to use the correct host UID and GID based
on the UID and GID within the container, to change recursively the owner
and group of the source volume. Chowning walks the file system under the
volume and changes the UID/GID on each file. If the volume has thousands
of inodes, this process takes a long time, delaying the start of the
container.

**Warning** use with caution since this modifies the host filesystem.

`Labeling Volume Mounts`

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a container. Without a label, the security
system might prevent the processes running inside the container from
using the content. By default, Podman does not change the labels set by
the OS.

To change a label in the container context, add either of two suffixes
**:z** or **:Z** to the volume mount. These suffixes tell Podman to
relabel file objects on the shared volumes. The **z** option tells
Podman that two or more containers share the volume content. As a
result, Podman labels the content with a shared content label. Shared
volume labels allow all containers to read/write content. The **Z**
option tells Podman to label the content with a private unshared label
Only the current container can use a private volume. Note: all
containers within a `pod` share the same SELinux label. This means all
containers within said pod can read/write volumes shared into the
container created with the `:Z` on any of one the containers. Relabeling
walks the file system under the volume and changes the label on each
file, if the volume has thousands of inodes, this process takes a long
time, delaying the start of the container. If the volume was previously
relabeled with the `z` option, Podman is optimized to not relabel a
second time. If files are moved into the volume, then the labels can be
manually change with the `chcon -Rt container_file_t PATH` command.

Note: Do not relabel system files and directories. Relabeling system
content might cause other confined services on the machine to fail. For
these types of containers we recommend disabling SELinux separation. The
option **\--security-opt label=disable** disables SELinux separation for
the container. For example if a user wanted to volume mount their entire
home directory into a container, they need to disable SELinux
separation.

    $ podman run --security-opt label=disable -v $HOME:/home/user fedora touch /home/user/file

`Overlay Volume Mounts`

The `:O` flag tells Podman to mount the directory from the host as a
temporary storage using the `overlay file system`. The container
processes can modify content within the mountpoint which is stored in
the container storage in a separate directory. In overlay terms, the
source directory is the lower, and the container storage directory is
the upper. Modifications to the mount point are destroyed when the
container finishes executing, similar to a tmpfs mount point being
unmounted.

For advanced users, the **overlay** option also supports custom
non-volatile **upperdir** and **workdir** for the overlay mount. Custom
**upperdir** and **workdir** can be fully managed by the users
themselves, and Podman does not remove it on lifecycle completion.
Example **:O,upperdir=/some/upper,workdir=/some/work**

Subsequent executions of the container sees the original source
directory content, any changes from previous container executions no
longer exist.

One use case of the overlay mount is sharing the package cache from the
host into the container to allow speeding up builds.

Note: The `O` flag conflicts with other options listed above.

Content mounted into the container is labeled with the private label. On
SELinux systems, labels in the source directory must be readable by the
container label. Usually containers can read/execute `container_share_t`
and can read/write `container_file_t`. If unable to change the labels on
a source volume, SELinux container separation must be disabled for the
container to work.

Do not modify the source directory mounted into the container with an
overlay mount, it can cause unexpected failures. Only modify the
directory after the container finishes running.

`Mounts propagation`

By default, bind-mounted volumes are `private`. That means any mounts
done inside the container are not visible on the host and vice versa.
One can change this behavior by specifying a volume mount propagation
property. When a volume is `shared`, mounts done under that volume
inside the container are visible on host and vice versa. Making a volume
**slave**^[\[1\]](#Footnote1)^ enables only one-way mount propagation:
mounts done on the host under that volume are visible inside the
container but not the other way around.

To control mount propagation property of a volume one can use the
\[**r**\]**shared**, \[**r**\]**slave**, \[**r**\]**private** or the
\[**r**\]**unbindable** propagation flag. Propagation property can be
specified only for bind mounted volumes and not for internal volumes or
named volumes. For mount propagation to work the source mount point (the
mount point where source dir is mounted on) has to have the right
propagation properties. For shared volumes, the source mount point has
to be shared. And for slave volumes, the source mount point has to be
either shared or slave. ^[\[1\]](#Footnote1)^

To recursively mount a volume and all of its submounts into a container,
use the **rbind** option. By default the bind option is used, and
submounts of the source directory is not mounted into the container.

Mounting the volume with a **copy** option tells podman to copy content
from the underlying destination directory onto newly created internal
volumes. The **copy** only happens on the initial creation of the
volume. Content is not copied up when the volume is subsequently used on
different containers. The **copy** option is ignored on bind mounts and
has no effect.

Mounting volumes with the **nosuid** options means that SUID executables
on the volume can not be used by applications to change their privilege.
By default volumes are mounted with **nosuid**.

Mounting the volume with the **noexec** option means that no executables
on the volume can be executed within the container.

Mounting the volume with the **nodev** option means that no devices on
the volume can be used by processes within the container. By default
volumes are mounted with **nodev**.

If the *HOST-DIR* is a mount point, then **dev**, **suid**, and **exec**
options are ignored by the kernel.

Use **df HOST-DIR** to figure out the source mount, then use **findmnt
-o TARGET,PROPAGATION *source-mount-dir*** to figure out propagation
properties of source mount. If **findmnt**(1) utility is not available,
then one can look at the mount entry for the source mount point in
*/proc/self/mountinfo*. Look at the \"optional fields\" and see if any
propagation properties are specified. In there, **shared:N** means the
mount is shared, **master:N** means mount is slave, and if nothing is
there, the mount is private. ^[\[1\]](#Footnote1)^

To change propagation properties of a mount point, use **mount**(8)
command. For example, if one wants to bind mount source directory
*/foo*, one can do **mount \--bind /foo /foo** and **mount
\--make-private \--make-shared /foo**. This converts /foo into a shared
mount point. Alternatively, one can directly change propagation
properties of source mount. Say */* is source mount for */foo*, then use
**mount \--make-shared /** to convert */* into a shared mount.

Note: if the user only has access rights via a group, accessing the
volume from inside a rootless container fails.

`Idmapped mount`

If `idmap` is specified, create an idmapped mount to the target user
namespace in the container. The idmap option supports a custom mapping
that can be different than the user namespace used by the container. The
mapping can be specified after the idmap option like:
`idmap=uids=0-1-10#10-11-10;gids=0-100-10`. For each triplet, the first
value is the start of the backing file system IDs that are mapped to the
second value on the host. The length of this mapping is given in the
third value. Multiple ranges are separated with #.

Use the **\--group-add keep-groups** option to pass the user\'s
supplementary group access into the container.

#### **\--volumes-from**=*CONTAINER\[:OPTIONS\]*

Mount volumes from the specified container(s). Used to share volumes
between containers. The *options* is a comma-separated list with the
following available elements:

-   **rw**\|**ro**
-   **z**

Mounts already mounted volumes from a source container onto another
container. *CONTAINER* may be a name or ID. To share a volume, use the
\--volumes-from option when running the target container. Volumes can be
shared even if the source container is not running.

By default, Podman mounts the volumes in the same mode (read-write or
read-only) as it is mounted in the source container. This can be changed
by adding a `ro` or `rw` *option*.

Labeling systems like SELinux require that proper labels are placed on
volume content mounted into a container. Without a label, the security
system might prevent the processes running inside the container from
using the content. By default, Podman does not change the labels set by
the OS.

To change a label in the container context, add `z` to the volume mount.
This suffix tells Podman to relabel file objects on the shared volumes.
The `z` option tells Podman that two entities share the volume content.
As a result, Podman labels the content with a shared content label.
Shared volume labels allow all containers to read/write content.

If the location of the volume from the source container overlaps with
data residing on a target container, then the volume hides that data on
the target.

#### **\--workdir**, **-w**=*dir*

Working directory inside the container.

The default working directory for running binaries within a container is
the root directory (**/**). The image developer can set a different
default with the WORKDIR instruction. The operator can override the
working directory by using the **-w** option.

##  Exit Status

The exit code from **podman run** gives information about why the
container failed to run or why it exited. When **podman run** exits with
a non-zero code, the exit codes follow the **chroot**(1) standard, see
below:

**125** The error is with Podman itself

    $ podman run --foo busybox; echo $?
    Error: unknown flag: --foo
    125

**126** The *contained command* cannot be invoked

    $ podman run busybox /etc; echo $?
    Error: container_linux.go:346: starting container process caused "exec: \"/etc\": permission denied": OCI runtime error
    126

**127** The *contained command* cannot be found

    $ podman run busybox foo; echo $?
    Error: container_linux.go:346: starting container process caused "exec: \"foo\": executable file not found in $PATH": OCI runtime error
    127

**Exit code** *contained command* exit code

    $ podman run busybox /bin/sh -c 'exit 3'; echo $?
    3

##  EXAMPLES

### Running container in read-only mode

During container image development, containers often need to write to
the image content. Installing packages into */usr*, for example. In
production, applications seldom need to write to the image. Container
applications write to volumes if they need to write to file systems at
all. Applications can be made more secure by running them in read-only
mode using the **\--read-only** switch. This protects the container\'s
image from modification. By default read-only containers can write to
temporary data. Podman mounts a tmpfs on */run* and */tmp* within the
container.

    $ podman run --read-only -i -t fedora /bin/bash

If the container does not write to any file system within the container,
including tmpfs, set \--read-only-tmpfs=false.

    $ podman run --read-only --read-only-tmpfs=false --tmpfs /run -i -t fedora /bin/bash

### Exposing shared libraries inside of container as read-only using a glob

    $ podman run --mount type=glob,src=/usr/lib64/libnvidia\*,ro=true -i -t fedora /bin/bash

### Exposing log messages from the container to the host\'s log

Bind mount the */dev/log* directory to have messages that are logged in
the container show up in the host\'s syslog/journal.

    $ podman run -v /dev/log:/dev/log -i -t fedora /bin/bash

From inside the container test this by sending a message to the log.

    (bash)# logger "Hello from my container"

Then exit and check the journal.

    (bash)# exit

    $ journalctl -b | grep Hello

This lists the message sent to the logger.

### Attaching to one or more from STDIN, STDOUT, STDERR

Without specifying the **-a** option, Podman attaches everything (stdin,
stdout, stderr). Override the default by specifying -a (stdin, stdout,
stderr), as in:

    $ podman run -a stdin -a stdout -i -t fedora /bin/bash

### Sharing IPC between containers

Using **shm_server.c** available here:
https://www.cs.cf.ac.uk/Dave/C/node27.html

Testing **\--ipc=host** mode:

Host shows a shared memory segment with 7 pids attached, happens to be
from httpd:

    $ sudo ipcs -m

    ------ Shared Memory Segments --------
    key        shmid      owner      perms      bytes      nattch     status
    0x01128e25 0          root       600        1000       7

Now run a regular container, and it correctly does NOT see the shared
memory segment from the host:

    $ podman run -it shm ipcs -m

    ------ Shared Memory Segments --------
    key        shmid      owner      perms      bytes      nattch     status

Run a container with the new **\--ipc=host** option, and it now sees the
shared memory segment from the host httpd:

    $ podman run -it --ipc=host shm ipcs -m

    ------ Shared Memory Segments --------
    key        shmid      owner      perms      bytes      nattch     status
    0x01128e25 0          root       600        1000       7

Testing **\--ipc=container:**\_id\_ mode:

Start a container with a program to create a shared memory segment:

    $ podman run -it shm bash
    $ sudo shm/shm_server &
    $ sudo ipcs -m

    ------ Shared Memory Segments --------
    key        shmid      owner      perms      bytes      nattch     status
    0x0000162e 0          root       666        27         1

Create a 2nd container correctly shows no shared memory segment from 1st
container:

    $ podman run shm ipcs -m

    ------ Shared Memory Segments --------
    key        shmid      owner      perms      bytes      nattch     status

Create a 3rd container using the **\--ipc=container:**\_id\_ option, now
it shows the shared memory segment from the first:

    $ podman run -it --ipc=container:ed735b2264ac shm ipcs -m
    $ sudo ipcs -m

    ------ Shared Memory Segments --------
    key        shmid      owner      perms      bytes      nattch     status
    0x0000162e 0          root       666        27         1

### Mapping Ports for External Usage

The exposed port of an application can be mapped to a host port using
the **-p** flag. For example, an httpd port 80 can be mapped to the host
port 8080 using the following:

    $ podman run -p 8080:80 -d -i -t fedora/httpd

### Mounting External Volumes

To mount a host directory as a container volume, specify the absolute
path to the directory and the absolute path for the container directory
separated by a colon. If the source is a named volume maintained by
Podman, it is recommended to use its name rather than the path to the
volume. Otherwise the volume is considered an orphan and wiped by the
**podman volume prune** command:

    $ podman run -v /var/db:/data1 -i -t fedora bash

    $ podman run -v data:/data2 -i -t fedora bash

    $ podman run -v /var/cache/dnf:/var/cache/dnf:O -ti fedora dnf -y update

If the container needs a writeable mounted volume by a non root user
inside the container, use the **U** option. This option tells Podman to
chown the source volume to match the default UID and GID used within the
container.

    $ podman run -d -e MARIADB_ROOT_PASSWORD=root --user mysql --userns=keep-id -v ~/data:/var/lib/mysql:Z,U mariadb

Alternatively if the container needs a writable volume by a non root
user inside of the container, the \--userns=keep-id option allows users
to specify the UID and GID of the user executing Podman to specific UIDs
and GIDs within the container. Since the processes running in the
container run as the user\'s UID, they can read/write files owned by the
user.

    $ podman run -d -e MARIADB_ROOT_PASSWORD=root --user mysql --userns=keep-id:uid=999,gid=999 -v ~/data:/var/lib/mysql:Z mariadb

Using **\--mount** flags to mount a host directory as a container
folder, specify the absolute path to the directory or the volume name,
and the absolute path within the container directory:

    $ podman run --mount type=bind,src=/var/db,target=/data1 busybox sh

    $ podman run --mount type=bind,src=volume-name,target=/data1 busybox sh

When using SELinux, be aware that the host has no knowledge of container
SELinux policy. Therefore, in the above example, if SELinux policy is
enforced, the */var/db* directory is not writable to the container. A
\"Permission Denied\" message occurs, and an **avc:** message is added
to the host\'s syslog.

To work around this, at time of writing this man page, the following
command needs to be run in order for the proper SELinux policy type
label to be attached to the host directory:

    $ chcon -Rt svirt_sandbox_file_t /var/db

Now, writing to the */data1* volume in the container is allowed and the
changes are reflected on the host in */var/db*.

### Using alternative security labeling

Override the default labeling scheme for each container by specifying
the **\--security-opt** flag. For example, specify the MCS/MLS level, a
requirement for MLS systems. Specifying the level in the following
command allows the same content to be shared between containers.

    podman run --security-opt label=level:s0:c100,c200 -i -t fedora bash

An MLS example might be:

    $ podman run --security-opt label=level:TopSecret -i -t rhel7 bash

To disable the security labeling for this container versus running with
the \#### **\--permissive** flag, use the following command:

    $ podman run --security-opt label=disable -i -t fedora bash

Tighten the security policy on the processes within a container by
specifying an alternate type for the container. For example, run a
container that is only allowed to listen on Apache ports by executing
the following command:

    $ podman run --security-opt label=type:svirt_apache_t -i -t centos bash

Note that an SELinux policy defining a **svirt_apache_t** type must be
written.

To mask additional specific paths in the container, specify the paths
separated by a colon using the **mask** option with the
**\--security-opt** flag.

    $ podman run --security-opt mask=/foo/bar:/second/path fedora bash

To unmask all the paths that are masked by default, set the **unmask**
option to **ALL**. Or to only unmask specific paths, specify the paths
as shown above with the **mask** option.

    $ podman run --security-opt unmask=ALL fedora bash

To unmask all the paths that start with /proc, set the **unmask** option
to **/proc/**\*.

    $ podman run --security-opt unmask=/proc/* fedora bash

    $ podman run --security-opt unmask=/foo/bar:/sys/firmware fedora bash

### Setting device weight via **\--blkio-weight-device** flag.

    $ podman run -it --blkio-weight-device "/dev/sda:200" ubuntu

### Using a podman container with input from a pipe

    $ echo "asdf" | podman run --rm -i --entrypoint /bin/cat someimage
    asdf

### Setting automatic user namespace separated containers

    # podman run --userns=auto:size=65536 ubi8-micro cat /proc/self/uid_map
    0 2147483647      65536
    # podman run --userns=auto:size=65536 ubi8-micro cat /proc/self/uid_map
    0 2147549183      65536

### Setting Namespaced Kernel Parameters (Sysctls)

The **\--sysctl** sets namespaced kernel parameters (sysctls) in the
container. For example, to turn on IP forwarding in the containers
network namespace, run this command:

    $ podman run --sysctl net.ipv4.ip_forward=1 someimage

Note that not all sysctls are namespaced. Podman does not support
changing sysctls inside of a container that also modify the host system.
As the kernel evolves we expect to see more sysctls become namespaced.

See the definition of the **\--sysctl** option above for the current
list of supported sysctls.

### Set UID/GID mapping in a new user namespace

Running a container in a new user namespace requires a mapping of the
UIDs and GIDs from the host.

    $ podman run --uidmap 0:30000:7000 --gidmap 0:30000:7000 fedora echo hello

### Configuring Storage Options from the command line

Podman allows for the configuration of storage by changing the values in
the */etc/container/storage.conf* or by using global options. This shows
how to set up and use fuse-overlayfs for a one-time run of busybox using
global options.

    podman --log-level=debug --storage-driver overlay --storage-opt "overlay.mount_program=/usr/bin/fuse-overlayfs" run busybox /bin/sh

### Configure timezone in a container

    $ podman run --tz=local alpine date
    $ podman run --tz=Asia/Shanghai alpine date
    $ podman run --tz=US/Eastern alpine date

### Adding dependency containers

The first container, container1, is not started initially, but must be
running before container2 starts. The `podman run` command starts the
container automatically before starting container2.

    $ podman create --name container1 -t -i fedora bash
    $ podman run --name container2 --requires container1 -t -i fedora bash

Multiple containers can be required.

    $ podman create --name container1 -t -i fedora bash
    $ podman create --name container2 -t -i fedora bash
    $ podman run --name container3 --requires container1,container2 -t -i fedora bash

### Configure keep supplemental groups for access to volume

    $ podman run -v /var/lib/design:/var/lib/design --group-add keep-groups ubi8

### Configure execution domain for containers using personality flag

    $ podman run --name container1 --personality=LINUX32 fedora bash

### Run a container with external rootfs mounted as an overlay

    $ podman run --name container1 --rootfs /path/to/rootfs:O bash

### Handling Timezones in java applications in a container.

In order to use a timezone other than UTC when running a Java
application within a container, the `TZ` environment variable must be
set within the container. Java applications ignores the value set with
the `--tz` option.

    # Example run
    podman run -ti --rm  -e TZ=EST mytzimage
    lrwxrwxrwx. 1 root root 29 Nov  3 08:51 /etc/localtime -> ../usr/share/zoneinfo/Etc/UTC
    Now with default timezone:
    Fri Nov 19 18:10:55 EST 2021
    Java default sees the following timezone:
    2021-11-19T18:10:55.651130-05:00
    Forcing UTC:
    Fri Nov 19 23:10:55 UTC 2021

### Run a container connected to two networks (called net1 and net2) with a static ip

    $ podman run --network net1:ip=10.89.1.5 --network net2:ip=10.89.10.10 alpine ip addr

### Rootless Containers

Podman runs as a non-root user on most systems. This feature requires
that a new enough version of **shadow-utils** be installed. The
**shadow-utils** package must include the **newuidmap**(1) and
**newgidmap**(1) executables.

In order for users to run rootless, there must be an entry for their
username in */etc/subuid* and */etc/subgid* which lists the UIDs for
their user namespace.

Rootless Podman works better if the fuse-overlayfs and slirp4netns
packages are installed. The **fuse-overlayfs** package provides a
userspace overlay storage driver, otherwise users need to use the
**vfs** storage driver, which can be disk space expensive and less
performant than other drivers.

To enable VPN on the container, slirp4netns or pasta needs to be
specified; without either, containers need to be run with the
\--network=host flag.

##  ENVIRONMENT

Environment variables within containers can be set using multiple
different options, in the following order of precedence (later entries
override earlier entries):

-   Container image: Any environment variables specified in the
    container image.
-   **\--http-proxy**: By default, several environment variables are
    passed in from the host, such as **http_proxy** and **no_proxy**.
    See **\--http-proxy** for details.
-   **\--env-host**: Host environment of the process executing Podman is
    added.
-   **\--env-file**: Any environment variables specified via env-files.
    If multiple files are specified, then they override each other in
    order of entry.
-   **\--env**: Any environment variables specified overrides previous
    settings.

Run containers and set the environment ending with a *****. The
trailing***** glob functionality is only active when no value is
specified:

    $ export ENV1=a
    $ podman run --env 'ENV*' alpine env | grep ENV
    ENV1=a
    $ podman run --env 'ENV*=b' alpine env | grep ENV
    ENV*=b

##  CONMON

When Podman starts a container it actually executes the conmon program,
which then executes the OCI Runtime. Conmon is the container monitor. It
is a small program whose job is to watch the primary process of the
container, and if the container dies, save the exit code. It also holds
open the tty of the container, so that it can be attached to later. This
is what allows Podman to run in detached mode (backgrounded), so Podman
can exit but conmon continues to run. Each container has their own
instance of conmon. Conmon waits for the container to exit, gathers and
saves the exit code, and then launches a Podman process to complete the
container cleanup, by shutting down the network and storage. For more
information about conmon, see the conmon(8) man page.

##  FILES

**/etc/subuid**

**/etc/subgid**

NOTE: Use the environment variable `TMPDIR` to change the temporary
storage location of downloaded container images. Podman defaults to use
`/var/tmp`.

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-save(1)](podman-save.html)**,
**[podman-ps(1)](podman-ps.html)**,
**[podman-attach(1)](podman-attach.html)**,
**[podman-pod-create(1)](podman-pod-create.html)**,
**[podman-port(1)](podman-port.html)**,
**[podman-start(1)](podman-start.html)**,
**[podman-kill(1)](podman-kill.html)**,
**[podman-stop(1)](podman-stop.html)**,
**[podman-generate-systemd(1)](podman-generate-systemd.html)**,
**[podman-rm(1)](podman-rm.html)**,
**[subgid(5)](https://www.unix.com/man-page/linux/5/subgid)**,
**[subuid(5)](https://www.unix.com/man-page/linux/5/subuid)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**,
**[systemd.unit(5)](https://www.freedesktop.org/software/systemd/man/systemd.unit.html)**,
**[setsebool(8)](https://man7.org/linux/man-pages/man8/setsebool.8.html)**,
**[slirp4netns(1)](https://github.com/rootless-containers/slirp4netns/blob/master/slirp4netns.html)**,
**[pasta(1)](https://passt.top/builds/latest/web/passt.1.html)**,
**[fuse-overlayfs(1)](https://github.com/containers/fuse-overlayfs/blob/main/fuse-overlayfs.html)**,
**proc(5)**,
**[conmon(8)](https://github.com/containers/conmon/blob/main/docs/conmon.8.md)**,
**personality(2)**

### Troubleshooting

See
[podman-troubleshooting(7)](https://github.com/containers/podman/blob/main/troubleshooting.md)
for solutions to common issues.

See
[podman-rootless(7)](https://github.com/containers/podman/blob/main/rootless.md)
for rootless issues.

##  HISTORY

September 2018, updated by Kunal Kushwaha
`<kushwaha_kunal_v7@lab.ntt.co.jp>`

October 2017, converted from Docker documentation to Podman by Dan Walsh
for Podman `<dwalsh@redhat.com>`

November 2015, updated by Sally O\'Malley `<somalley@redhat.com>`

June 2014, updated by Sven Dowideit `<SvenDowideit@home.org.au>`

April 2014, Originally compiled by William Henry `<whenry@redhat.com>`
based on docker.com source material and internal work.

##  FOOTNOTES

[1]{#Footnote1}: The Podman project is committed to inclusivity, a core
value of open source. The `master` and `slave` mount propagation
terminology used here is problematic and divisive, and needs to be
changed. However, these terms are currently used within the Linux kernel
and must be used as-is at this time. When the kernel maintainers rectify
this usage, Podman will follow suit immediately.


---

<a id='podman-save'></a>

## podman-save - Save image(s) to an archive

##  NAME

podman-save - Save image(s) to an archive

##  SYNOPSIS

**podman save** \[*options*\] *name*\[:*tag*\]

**podman image save** \[*options*\] *name*\[:*tag*\]

##  DESCRIPTION

**podman save** saves an image to a local file or directory. **podman
save** writes to STDOUT by default and can be redirected to a file using
the **output** flag. The **quiet** flag suppresses the output when set.
**podman save** saves parent layers of the image(s) and the image(s) can
be loaded using **podman load**. To export the containers, use the
**podman export**. Note: `:` is a restricted character and cannot be
part of the file name.

**podman \[GLOBAL OPTIONS\]**

**podman save \[GLOBAL OPTIONS\]**

**podman save [OPTIONS](#options) NAME\[:TAG\]**

##  OPTIONS

#### **\--compress**

Compress tarball image layers when pushing to a directory using the
\'dir\' transport. (default is same compression type, compressed or
uncompressed, as source)

Note: This flag can only be set with **\--format=docker-dir**.

#### **\--format**=*format*

An image format to produce, one of:

  -----------------------------------------------------------------------------
  Format               Description
  -------------------- --------------------------------------------------------
  **docker-archive**   A tar archive interoperable with **docker load(1)** (the
                       default)

  **oci-archive**      A tar archive using the OCI Image Format

  **oci-dir**          A directory using the OCI Image Format

  **docker-dir**       **dir** transport (see **containers-transports(5)**)
                       with v2s2 manifest type
  -----------------------------------------------------------------------------

#### **\--help**, **-h**

Print usage statement

#### **\--multi-image-archive**, **-m**

Allow for creating archives with more than one image. Additional names
are interpreted as images instead of tags. Only supported for
**\--format=docker-archive**. The default for this option can be
modified via the `multi_image_archive="true"|"false"` flag in
containers.conf.

#### **\--output**, **-o**=*file*

Write to a file, default is STDOUT

#### **\--quiet**, **-q**

Suppress the output

#### **\--uncompressed**

Accept uncompressed layers when using one of the OCI formats.

##  EXAMPLES

Save image to a local file without displaying progress.

    $ podman save --quiet -o alpine.tar alpine:2.6

Save image to stdout and redirect content via shell.

    $ podman save alpine > alpine-all.tar

Save image in oci-archive format to the local file.

    $ podman save -o oci-alpine.tar --format oci-archive alpine

Save image compressed in docker-dir format.

    $ podman save --compress --format docker-dir -o alp-dir alpine
    Getting image source signatures
    Copying blob sha256:2fdfe1cd78c20d05774f0919be19bc1a3e4729bce219968e4188e7e0f1af679d
     1.97 MB / 1.97 MB [========================================================] 0s
    Copying config sha256:501d1a8f0487e93128df34ea349795bc324d5e0c0d5112e08386a9dfaff620be
     584 B / 584 B [============================================================] 0s
    Writing manifest to image destination
    Storing signatures

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-load(1)](podman-load.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**,
**[containers-transports(5)](https://github.com/containers/image/blob/main/docs/containers-transports.5.md)**

##  HISTORY

July 2017, Originally compiled by Urvashi Mohnani <umohnani@redhat.com>


---

<a id='podman-search'></a>

## podman-search - Search a registry for an image

##  NAME

podman-search - Search a registry for an image

##  SYNOPSIS

**podman search** \[*options*\] *term*

##  DESCRIPTION

**podman search** searches a registry or a list of registries for a
matching image. The user can specify which registry to search by
prefixing the registry in the search term (e.g.,
**registry.fedoraproject.org/fedora**). By default, all
unqualified-search registries in `containers-registries.conf(5)` are
used.

The default number of results is 25. The number of results can be
limited using the **\--limit** flag. If more than one registry is being
searched, the limit is applied to each registry. The output can be
filtered using the **\--filter** flag. To get all available images in a
registry without a specific search term, the user can just enter the
registry name with a trailing \"/\" (example
**registry.fedoraproject.org/**).

Note that **podman search** is not a reliable way to determine the
presence or existence of an image. The search behavior of the v1 and v2
Docker distribution API is specific to the implementation of each
registry. Some registries may not support searching at all. Further note
that searching without a search term only works for registries that
implement the v2 API.

**podman \[GLOBAL OPTIONS\]**

**podman search \[GLOBAL OPTIONS\]**

**podman search [OPTIONS](#options) TERM**

##  OPTIONS

#### **\--authfile**=*path*

Path of the authentication file. Default is
`${XDG_RUNTIME_DIR}/containers/auth.json` on Linux, and
`$HOME/.config/containers/auth.json` on Windows/macOS. The file is
created by **[podman login](podman-login.html)**. If the authorization
state is not found there, `$HOME/.docker/config.json` is checked, which
is set using **docker login**.

Note: There is also the option to override the default path of the
authentication file by setting the `REGISTRY_AUTH_FILE` environment
variable. This can be done with **export REGISTRY_AUTH_FILE=*path***.

#### **\--cert-dir**=*path*

Use certificates at *path* (\*.crt, \*.cert, \*.key) to connect to the
registry. (Default: /etc/containers/certs.d) For details, see
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--compatible**

After the name and the description, also show the stars, official and
automated descriptors as Docker does. Podman does not show these
descriptors by default since they are not supported by most public
container registries.

#### **\--creds**=*\[username\[:password\]\]*

The \[username\[:password\]\] to use to authenticate with the registry,
if required. If one or both values are not supplied, a command line
prompt appears and the value can be entered. The password is entered
without echo.

Note that the specified credentials are only used to authenticate
against target registries. They are not used for mirrors or when the
registry gets rewritten (see `containers-registries.conf(5)`); to
authenticate against those consider using a `containers-auth.json(5)`
file.

#### **\--filter**, **-f**=*filter*

Filter output based on conditions provided (default \[\])

Supported filters are:

-   stars (int) - minimum number of stars required for images to show
-   is-automated (boolean - true \| false) - is the image automated or
    not
-   is-official (boolean - true \| false) - is the image official or not

#### **\--format**=*format*

Change the output format to a Go template

Valid placeholders for the Go template are listed below:

  **Placeholder**   **Description**
  ----------------- ----------------------------------
  .Automated        \"\[OK\]\" if image is automated
  .Description      Image description
  .Index            Registry
  .Name             Image name
  .Official         \"\[OK\]\" if image is official
  .Stars            Star count of image
  .Tag              Repository tag

Note: use .Tag only if the \--list-tags is set.

#### **\--help**, **-h**

Print usage statement

#### **\--limit**=*limit*

Limit the number of results (default 25). Note: The results from each
registry is limited to this value. Example if limit is 10 and two
registries are being searched, the total number of results is 20, 10
from each (if there are at least 10 matches in each). The order of the
search results is the order in which the API endpoint returns the
results.

#### **\--list-tags**

List the available tags in the repository for the specified image.
**Note:** \--list-tags requires the search term to be a fully specified
image name. The result contains the Image name and its tag, one line for
every tag associated with the image.

#### **\--no-trunc**

Do not truncate the output (default *false*).

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

##  EXAMPLES

Search for images containing the specified name, returning the first
three images from each defined registry.

    $ podman search --limit 3 fedora
    NAME                                     DESCRIPTION
    NAME                                           DESCRIPTION
    registry.fedoraproject.org/f29/fedora-toolbox
    registry.fedoraproject.org/f30/fedora-toolbox
    registry.fedoraproject.org/f31/fedora-toolbox
    docker.io/library/fedora                       Official Docker builds of Fedora
    docker.io/kasmweb/fedora-37-desktop            Fedora 37 desktop for Kasm Workspaces
    docker.io/kasmweb/fedora-38-desktop            Fedora 38 desktop for Kasm Workspaces
    quay.io/fedora/fedora
    quay.io/containerdisks/fedora                  # Fedora Containerdisk Images  <img src="htt...
    quay.io/fedora/fedora-minimal

Note that the Stars, Official and Automated descriptors are only
available on Docker Hub and are hence not displayed by default.

    $ podman search --format "{{.Name}}\t{{.Stars}}\t{{.Official}}" alpine --limit 3
    docker.io/library/alpine       7956        [OK]
    docker.io/alpine/git           192
    docker.io/anapsix/alpine-java  474
    quay.io/libpod/alpine          0
    quay.io/vqcomms/alpine-tools   0
    quay.io/wire/alpine-deps       0

Search and list tags for the specified image returning the first four
images from each defined registry.

    $ podman search --list-tags registry.access.redhat.com/ubi8 --limit 4
    NAME                             TAG
    registry.access.redhat.com/ubi8  8.4-211
    registry.access.redhat.com/ubi8  8.4-206.1626828523-source
    registry.access.redhat.com/ubi8  8.4-199
    registry.access.redhat.com/ubi8  8.4-211-source

Note: This works only with registries that implement the v2 API. If
tried with a v1 registry an error is returned.

##  FILES

**registries.conf** (`/etc/containers/registries.conf`)

registries.conf is the configuration file which specifies which
container registries is consulted when completing image names which do
not include a registry or domain portion.

##  SEE ALSO

**[podman(1)](podman.html)**,
**[containers-registries(5)](https://github.com/containers/image/blob/main/docs/containers-registries.5.md)**

##  HISTORY

January 2018, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-secret'></a>

## podman-secret - Manage podman secrets

##  NAME

podman-secret - Manage podman secrets

##  SYNOPSIS

**podman secret** *subcommand*

##  DESCRIPTION

podman secret is a set of subcommands that manage secrets.

##  SUBCOMMANDS

  ---------------------------------------------------------------------------------------------------
  Command   Man Page                                                 Description
  --------- -------------------------------------------------------- --------------------------------
  create    [podman-secret-create(1)](podman-secret-create.html)     Create a new secret

  exists    [podman-secret-exists(1)](podman-secret-exists.html)     Check if the given secret exists

  inspect   [podman-secret-inspect(1)](podman-secret-inspect.html)   Display detailed information on
                                                                     one or more secrets

  ls        [podman-secret-ls(1)](podman-secret-ls.html)             List all available secrets

  rm        [podman-secret-rm(1)](podman-secret-rm.html)             Remove one or more secrets
  ---------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

January 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-start'></a>

## podman-start - Start one or more containers

##  NAME

podman-start - Start one or more containers

##  SYNOPSIS

**podman start** \[*options*\] *container* \...

**podman container start** \[*options*\] *container* \...

##  DESCRIPTION

Start one or more containers using container IDs or names as input. The
*attach* and *interactive* options cannot be used to override the
*\--tty* and *\--interactive* options from when the container was
created. Starting an already running container with the *\--attach*
option, Podman simply attaches to the container.

##  OPTIONS

#### **\--all**

Start all the containers, default is only running containers.

#### **\--attach**, **-a**

Attach container\'s STDOUT and STDERR. The default is false. This option
cannot be used when starting multiple containers.

#### **\--detach-keys**=*sequence*

Specify the key sequence for detaching a container. Format is a single
character `[a-Z]` or one or more `ctrl-<value>` characters where
`<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`. Specifying \"\"
disables this feature. The default is *ctrl-p,ctrl-q*.

This option can also be set in **containers.conf**(5) file.

#### **\--filter**, **-f**

Filter what containers are going to be started from the given arguments.
Multiple filters can be given with multiple uses of the \--filter flag.
Filters with the same key work inclusive with the only exception being
`label` which is exclusive. Filters with different keys always work
exclusive.

Valid filters are listed below:

  --------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ -------------------------------------------------------------
  id           \[ID\] Container\'s ID (CID prefix match by default; accepts
               regex)

  name         [Name](#name) Container\'s name (accepts regex)

  label        \[Key\] or \[Key=Value\] Label assigned to a container

  exited       \[Int\] Container\'s exit code

  status       \[Status\] Container\'s status: \'created\', \'exited\',
               \'paused\', \'running\', \'unknown\'

  ancestor     \[ImageName\] Image or descendant used to create container

  before       \[ID\] or [Name](#name) Containers created before this
               container

  since        \[ID\] or [Name](#name) Containers created since this
               container

  volume       \[VolumeName\] or \[MountpointDestination\] Volume mounted in
               container

  health       \[Status\] healthy or unhealthy

  pod          \[Pod\] name or full or partial ID of pod

  network      \[Network\] name or full ID of network

  until        \[DateTime\] Containers created before the given duration or
               time.
  --------------------------------------------------------------------------

#### **\--interactive**, **-i**

When set to **true**, make stdin available to the contained process. If
**false**, the stdin of the contained process is empty and immediately
closed.

If attached, stdin is piped to the contained process. If detached,
reading stdin will block until later attached.

**Caveat:** Podman will consume input from stdin as soon as it becomes
available, even if the contained process doesn\'t request it.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--sig-proxy**

Proxy received signals to the container process. SIGCHLD, SIGURG,
SIGSTOP, and SIGKILL are not proxied.

The default is **true** when attaching, **false** otherwise.

##  EXAMPLE

Start specified container:

    podman start mywebserver

Start multiple containers:

    podman start 860a4b231279 5421ab43b45

Start specified container in interactive mode with terminal attached:

    podman start --interactive --attach 860a4b231279

Start last created container in interactive mode (This option is not
available with the remote Podman client, including Mac and Windows
(excluding WSL2) machines):

    podman start -i -l

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

November 2018, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-stats'></a>

## podman-stats - Display a live stream of one or more container's
resource usage statistics

##  NAME

podman-stats - Display a live stream of one or more container\'s
resource usage statistics

##  SYNOPSIS

**podman stats** \[*options*\] \[*container*\]

**podman container stats** \[*options*\] \[*container*\]

##  DESCRIPTION

Display a live stream of one or more containers\' resource usage
statistics

Note: Podman stats does not work in rootless environments that use
CGroups V1. Podman stats relies on CGroup information for statistics,
and CGroup v1 is not supported for rootless use cases.

Note: Rootless environments that use CGroups V2 are not able to report
statistics about their networking usage.

##  OPTIONS

#### **\--all**, **-a**

Show all containers. Only running containers are shown by default

#### **\--format**=*template*

Pretty-print container statistics to JSON or using a Go template

Valid placeholders for the Go template are listed below:

  ----------------------------------------------------------------------
  **Placeholder**      **Description**
  -------------------- -------------------------------------------------
  .AvgCPU              Average CPU, full precision float

  .AVGCPU              Average CPU, formatted as a percent

  .BlockInput          Total data read from block device

  .BlockIO             Total data read/total data written to block
                       device

  .BlockOutput         Total data written to block device

  .ContainerID         Container ID, full (untruncated) hash

  .ContainerStats \... Nested structure, for experts only

  .CPU                 Percent CPU, full precision float

  .CPUNano             CPU Usage, total, in nanoseconds

  .CPUPerc             Percentage of CPU used

  .CPUSystemNano       CPU Usage, kernel, in nanoseconds

  .Duration            Same as CPUNano

  .ID                  Container ID, truncated

  .MemLimit            Memory limit, in bytes

  .MemPerc             Memory percentage used

  .MemUsage            Memory usage

  .MemUsageBytes       Memory usage (IEC)

  .Name                Container Name

  .NetIO               Network IO

  .Network \...        Network I/O, separated by network interface

  .PerCPU              CPU time consumed by all tasks \[1\]

  .PIDs                Number of PIDs

  .PIDS                Number of PIDs (yes, we know this is a dup)

  .SystemNano          Current system datetime, nanoseconds since epoch

  .Up                  Duration (CPUNano), in human-readable form

  .UpTime              Same as Up
  ----------------------------------------------------------------------

\[1\] Cgroups V1 only

When using a Go template, precede the format with `table` to print
headers.

#### **\--interval**, **-i**=*seconds*

Time in seconds between stats reports, defaults to 5 seconds.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--no-reset**

Do not clear the terminal/screen in between reporting intervals

#### **\--no-stream**

Disable streaming stats and only pull the first result, default setting
is false

#### **\--no-trunc**

Do not truncate output

##  EXAMPLE

List statistics about all running containers without streaming mode:

    # podman stats -a --no-stream
    ID             NAME              CPU %   MEM USAGE / LIMIT   MEM %   NET IO    BLOCK IO   PIDS
    a9f807ffaacd   frosty_hodgkin    --      3.092MB / 16.7GB    0.02%   -- / --   -- / --    2
    3b33001239ee   sleepy_stallman   --      -- / --             --      -- / --   -- / --    --

List the specified container\'s statistics in streaming mode:

    # podman stats a9f80
    ID             NAME             CPU %   MEM USAGE / LIMIT   MEM %   NET IO    BLOCK IO   PIDS
    a9f807ffaacd   frosty_hodgkin   --      3.092MB / 16.7GB    0.02%   -- / --   -- / --    2

List the specified statistics about the specified container in table
format:

    $ podman stats --no-trunc 3667 --format 'table {{ .ID }} {{ .MemUsage }}'
    ID                                                                MEM USAGE / LIMIT
    3667c6aacb06aac2eaffce914c01736420023d56ef9b0f4cfe58b6d6a78b7503  49.15kB / 67.17GB

List the specified container statistics in JSON format:

    # podman stats --no-stream --format=json a9f80
    [
        {
        "id": "a9f807ffaacd",
        "name": "frosty_hodgkin",
        "cpu_percent": "--",
        "mem_usage": "3.092MB / 16.7GB",
        "mem_percent": "0.02%",
        "netio": "-- / --",
        "blocki": "-- / --",
        "pids": "2"
        }
    ]

List the specified container statistics in table format:

    # podman stats --no-stream --format "table {{.ID}} {{.Name}} {{.MemUsage}}" 6eae
    ID             NAME           MEM USAGE / LIMIT
    6eae9e25a564   clever_bassi   3.031MB / 16.7GB

Note: When using a slirp4netns network with the rootlesskit port
handler, the traffic sent via the port forwarding is accounted to the
`lo` device. Traffic accounted to `lo` is not accounted in the stats
output.

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

July 2017, Originally compiled by Ryan Cole <rycole@redhat.com>


---

<a id='podman-stop'></a>

## podman-stop - Stop one or more running containers

##  NAME

podman-stop - Stop one or more running containers

##  SYNOPSIS

**podman stop** \[*options*\] *container* \...

**podman container stop** \[*options*\] *container* \...

##  DESCRIPTION

Stops one or more containers using container IDs or names as input. The
**\--time** option specifies the number of seconds to wait before
forcibly stopping the container after the stop command is issued to the
container. The default is 10 seconds. By default, containers are stopped
with SIGTERM and then SIGKILL after the timeout. The SIGTERM default can
be overridden by the image used to create the container and also via
command line when creating the container.

##  OPTIONS

#### **\--all**, **-a**

Stop all running containers. This does not include paused containers.

#### **\--cidfile**=*file*

Read container ID from the specified *file* and stop the container. Can
be specified multiple times.

Command does not fail when *file* is missing and user specified
\--ignore.

#### **\--filter**, **-f**=*filter*

Filter what containers are going to be stopped. Multiple filters can be
given with multiple uses of the \--filter flag. Filters with the same
key work inclusive with the only exception being `label` which is
exclusive. Filters with different keys always work exclusive.

Valid filters are listed below:

  --------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ -------------------------------------------------------------
  id           \[ID\] Container\'s ID (CID prefix match by default; accepts
               regex)

  name         [Name](#name) Container\'s name (accepts regex)

  label        \[Key\] or \[Key=Value\] Label assigned to a container

  exited       \[Int\] Container\'s exit code

  status       \[Status\] Container\'s status: \'created\', \'exited\',
               \'paused\', \'running\', \'unknown\'

  ancestor     \[ImageName\] Image or descendant used to create container

  before       \[ID\] or [Name](#name) Containers created before this
               container

  since        \[ID\] or [Name](#name) Containers created since this
               container

  volume       \[VolumeName\] or \[MountpointDestination\] Volume mounted in
               container

  health       \[Status\] healthy or unhealthy

  pod          \[Pod\] name or full or partial ID of pod

  network      \[Network\] name or full ID of network

  until        \[DateTime\] Containers created before the given duration or
               time.
  --------------------------------------------------------------------------

#### **\--ignore**, **-i**

Ignore errors when specified containers are not in the container store.
A user might have decided to manually remove a container which leads to
a failure during the ExecStop directive of a systemd service referencing
that container.

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--time**, **-t**=*seconds*

Seconds to wait before forcibly stopping the container. Use -1 for
infinite wait.

##  EXAMPLES

Stop the specified container via its name.

    $ podman stop mywebserver

Stop the container via its id.

    $ podman stop 860a4b235279

Stop multiple containers.

    $ podman stop mywebserver 860a4b235279

Stop the container identified in the cidfile.

    $ podman stop --cidfile /home/user/cidfile-1

Stop the containers identified in the cidfiles.

    $ podman stop --cidfile /home/user/cidfile-1 --cidfile ./cidfile-2

Stop the specified container in 2 seconds.

    $ podman stop --time 2 860a4b235279

Stop all running containers.

    $ podman stop -a

Stop the last created container (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

    $ podman stop --latest

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-rm(1)](podman-rm.html)**

##  HISTORY

September 2018, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-system'></a>

## podman-system - Manage podman

##  NAME

podman-system - Manage podman

##  SYNOPSIS

**podman system** *subcommand*

##  DESCRIPTION

The system command allows management of the podman systems

##  COMMANDS

  ----------------------------------------------------------------------------------------------------------------
  Command      Man Page                                                       Description
  ------------ -------------------------------------------------------------- ------------------------------------
  check        [podman-system-check(1)](podman-system-check.html)             Perform consistency checks on image
                                                                              and container storage.

  connection   [podman-system-connection(1)](podman-system-connection.html)   Manage the destination(s) for Podman
                                                                              service(s)

  df           [podman-system-df(1)](podman-system-df.html)                   Show podman disk usage.

  events       [podman-events(1)](podman-events.html)                         Monitor Podman events

  info         [podman-info(1)](podman-info.html)                             Display Podman related system
                                                                              information.

  migrate      [podman-system-migrate(1)](podman-system-migrate.html)         Migrate existing containers to a new
                                                                              podman version.

  prune        [podman-system-prune(1)](podman-system-prune.html)             Remove all unused pods, containers,
                                                                              images, networks, and volume data.

  renumber     [podman-system-renumber(1)](podman-system-renumber.html)       Migrate lock numbers to handle a
                                                                              change in maximum number of locks.

  reset        [podman-system-reset(1)](podman-system-reset.html)             Reset storage back to initial state.

  service      [podman-system-service(1)](podman-system-service.html)         Run an API service
  ----------------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**


---

<a id='podman-tag'></a>

## podman-tag - Add an additional name to a local image

##  NAME

podman-tag - Add an additional name to a local image

##  SYNOPSIS

**podman tag** *image*\[:*tag*\] \[*target-name*\[:*tag*\]\...\]
\[*options*\]

**podman image tag** *image*\[:*tag*\] \[*target-name*\[:*tag*\]\...\]
\[*options*\]

##  DESCRIPTION

Assigns a new image name to an existing image. A full name refers to the
entire image name, including the optional *tag* after the `:`. If there
is no *tag* provided, then Podman defaults to `latest` for both the
*image* and the *target-name*.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

##  EXAMPLES

Tag specified image with an image name defaulting to :latest.

    $ podman tag 0e3bbc2 fedora:latest

Tag specified image with fully specified image name.

    $ podman tag httpd myregistryhost:5000/fedora/httpd:v2

Tag specified image with multiple tags.

    $ podman tag mymariadb mycontainerregistry.io/namespace/mariadb:10 mycontainerregistry.io/namespace/mariadb:10.11 mycontainerregistry.io/namespace/mariadb:10.11.12

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

December 2019, Update description to refer to \'name\' instead of
\'alias\' by Sascha Grunert <sgrunert@suse.com> July 2017, Originally
compiled by Ryan Cole <rycole@redhat.com>


---

<a id='podman-top'></a>

## podman-top - Display the running processes of a container

##  NAME

podman-top - Display the running processes of a container

##  SYNOPSIS

**podman top** \[*options*\] *container* \[*format-descriptors*\]

**podman container top** \[*options*\] *container*
\[*format-descriptors*\]

##  DESCRIPTION

Display the running processes of the container. The *format-descriptors*
are ps (1) compatible AIX format descriptors but extended to print
additional information, such as the seccomp mode or the effective
capabilities of a given process. The descriptors can either be passed as
separated arguments or as a single comma-separated argument. Note that
options and or flags of ps(1) can also be specified; in this case,
Podman falls back to executing ps(1) from the host with the specified
arguments and flags in the container namespace. If the container has the
`CAP_SYS_PTRACE` capability then we will execute ps(1) in the container
so it must be installed there. To extract host-related information, use
the \"h\*\" descriptors. For instance, `podman top $name hpid huser` to
display the PID and user of the processes in the host context.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

##  FORMAT DESCRIPTORS

The following descriptors are supported in addition to the AIX format
descriptors mentioned in ps (1):

**args, capbnd, capeff, capinh, capprm, comm, etime, group, hgroup,
hpid, huser, label, nice, pcpu, pgid, pid, ppid, rgroup, ruser, seccomp,
state, time, tty, user, vsz**

**capbnd**

Set of bounding capabilities. See capabilities (7) for more information.

**capeff**

Set of effective capabilities. See capabilities (7) for more
information.

**capinh**

Set of inheritable capabilities. See capabilities (7) for more
information.

**capprm**

Set of permitted capabilities. See capabilities (7) for more
information.

**hgroup**

The corresponding effective group of a container process on the host.

**hpid**

The corresponding host PID of a container process.

**huser**

The corresponding effective user of a container process on the host.

**label**

Current security attributes of the process.

**seccomp**

Seccomp mode of the process (i.e., disabled, strict or filter). See
seccomp (2) for more information.

**state**

Process state codes (e.g, **R** for *running*, **S** for *sleeping*).
See proc(5) for more information.

**stime**

Process start time (e.g, \"2019-12-09 10:50:36 +0100 CET).

##  EXAMPLES

By default, `podman-top` prints data similar to `ps -ef`.

    $ podman top f5a62a71b07
    USER   PID   PPID   %CPU    ELAPSED         TTY     TIME   COMMAND
    root   1     0      0.000   20.386825206s   pts/0   0s     sh
    root   7     1      0.000   16.386882887s   pts/0   0s     sleep
    root   8     1      0.000   11.386886562s   pts/0   0s     vi

The output can be controlled by specifying format descriptors as
arguments after the container.

    $ podman top -l pid seccomp args %C
    PID   SECCOMP   COMMAND     %CPU
    1     filter    sh          0.000
    8     filter    vi /etc/    0.000

Podman falls back to executing ps(1) from the host in the container
namespace if an unknown descriptor is specified.

    $ podman top -l -- aux
    USER   PID   PPID   %CPU    ELAPSED             TTY   TIME   COMMAND
    root   1     0      0.000   1h2m12.497061672s   ?     0s     sleep 100000

##  SEE ALSO

**[podman(1)](podman.html)**, **ps(1)**, **seccomp(2)**, **proc(5)**,
**capabilities(7)**

##  HISTORY

July 2018, Introduce format descriptors by Valentin Rothberg
<vrothberg@suse.com>

December 2017, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-unpause'></a>

## podman-unpause - Unpause one or more containers

##  NAME

podman-unpause - Unpause one or more containers

##  SYNOPSIS

**podman unpause** \[*options*\]\|\[*container* \...\]

**podman container unpause** \[*options*\]\|\[*container* \...\]

##  DESCRIPTION

Unpauses the processes in one or more containers. Container IDs or names
can be used as input.

##  OPTIONS

#### **\--all**, **-a**

Unpause all paused containers.

#### **\--cidfile**=*file*

Read container ID from the specified *file* and unpause the container.
Can be specified multiple times.

#### **\--filter**, **-f**=*filter*

Filter what containers unpause. Multiple filters can be given with
multiple uses of the \--filter flag. Filters with the same key work
inclusive with the only exception being `label` which is exclusive.
Filters with different keys always work exclusive.

Valid filters are listed below:

  --------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ -------------------------------------------------------------
  id           \[ID\] Container\'s ID (CID prefix match by default; accepts
               regex)

  name         [Name](#name) Container\'s name (accepts regex)

  label        \[Key\] or \[Key=Value\] Label assigned to a container

  exited       \[Int\] Container\'s exit code

  status       \[Status\] Container\'s status: \'created\', \'exited\',
               \'paused\', \'running\', \'unknown\'

  ancestor     \[ImageName\] Image or descendant used to create container

  before       \[ID\] or [Name](#name) Containers created before this
               container

  since        \[ID\] or [Name](#name) Containers created since this
               container

  volume       \[VolumeName\] or \[MountpointDestination\] Volume mounted in
               container

  health       \[Status\] healthy or unhealthy

  pod          \[Pod\] name or full or partial ID of pod

  network      \[Network\] name or full ID of network

  until        \[DateTime\] Containers created before the given duration or
               time.
  --------------------------------------------------------------------------

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

##  EXAMPLE

Unpause specified container:

    podman unpause mywebserver

Unpause container by a partial container ID:

    podman unpause 860a4b23

Unpause all **paused** containers:

    podman unpause --all

Unpause container using ID specified in given files:

    podman unpause --cidfile /home/user/cidfile-1
    podman unpause --cidfile /home/user/cidfile-1 --cidfile ./cidfile-2

Unpause the latest container. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines):

    podman unpause --latest

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-pause(1)](podman-pause.html)**

##  HISTORY

September 2017, Originally compiled by Dan Walsh <dwalsh@redhat.com>


---

<a id='podman-untag'></a>

## podman-untag - Remove one or more names from a locally-stored
image

##  NAME

podman-untag - Remove one or more names from a locally-stored image

##  SYNOPSIS

**podman untag** *image* \[*name*\[:*tag*\]\...\]

**podman image untag** *image* \[*name*\[:*tag*\]\...\]

##  DESCRIPTION

Remove one or more names from an image in the local storage. The image
can be referred to by ID or reference. If no name is specified, all
names are removed from the image. If a specified name is a short name
and does not include a registry, `localhost/` is prefixed (e.g.,
`fedora` -\> `localhost/fedora`). If a specified name does not include a
tag, `:latest` is appended (e.g., `localhost/fedora` -\>
`localhost/fedora:latest`).

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

##  EXAMPLES

Remove all tags from the specified image.

    $ podman untag 0e3bbc2

Remove tag from specified image.

    $ podman untag imageName:latest otherImageName:latest

Remove multiple tags from the specified image.

    $ podman untag httpd myhttpd myregistryhost:5000/fedora/httpd:v2

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

December 2019, Originally compiled by Sascha Grunert <sgrunert@suse.com>


---

<a id='podman-update'></a>

## podman-update - Update the configuration of a given container

##  NAME

podman-update - Update the configuration of a given container

##  SYNOPSIS

**podman update** \[*options*\] *container*

**podman container update** \[*options*\] *container*

##  DESCRIPTION

Updates the configuration of an already existing container, allowing
different resource limits to be set. The currently supported options are
a subset of the podman create/run resource limit options.

##  OPTIONS

#### **\--blkio-weight**=*weight*

Block IO relative weight. The *weight* is a value between **10** and
**1000**.

This option is not supported on cgroups V1 rootless systems.

#### **\--blkio-weight-device**=*device:weight*

Block IO relative device weight.

#### **\--cpu-period**=*limit*

Set the CPU period for the Completely Fair Scheduler (CFS), which is a
duration in microseconds. Once the container\'s CPU quota is used up, it
will not be scheduled to run until the current period ends. Defaults to
100000 microseconds.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-quota**=*limit*

Limit the CPU Completely Fair Scheduler (CFS) quota.

Limit the container\'s CPU usage. By default, containers run with the
full CPU resource. The limit is a number in microseconds. If a number is
provided, the container is allowed to use that much CPU time until the
CPU period ends (controllable via **\--cpu-period**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpu-rt-period**=*microseconds*

Limit the CPU real-time period in microseconds.

Limit the container\'s Real Time CPU usage. This option tells the kernel
to restrict the container\'s Real Time CPU usage to the period
specified.

This option is only supported on cgroups V1 rootful systems.

#### **\--cpu-rt-runtime**=*microseconds*

Limit the CPU real-time runtime in microseconds.

Limit the containers Real Time CPU usage. This option tells the kernel
to limit the amount of time in a given CPU period Real Time tasks may
consume. Ex: Period of 1,000,000us and Runtime of 950,000us means that
this container can consume 95% of available CPU and leave the remaining
5% to normal priority tasks.

The sum of all runtimes across containers cannot exceed the amount
allotted to the parent cgroup.

This option is only supported on cgroups V1 rootful systems.

#### **\--cpu-shares**, **-c**=*shares*

CPU shares (relative weight).

By default, all containers get the same proportion of CPU cycles. This
proportion can be modified by changing the container\'s CPU share
weighting relative to the combined weight of all the running containers.
Default weight is **1024**.

The proportion only applies when CPU-intensive processes are running.
When tasks in one container are idle, other containers can use the
left-over CPU time. The actual amount of CPU time varies depending on
the number of containers running on the system.

For example, consider three containers, one has a cpu-share of 1024 and
two others have a cpu-share setting of 512. When processes in all three
containers attempt to use 100% of CPU, the first container receives 50%
of the total CPU time. If a fourth container is added with a cpu-share
of 1024, the first container only gets 33% of the CPU. The remaining
containers receive 16.5%, 16.5% and 33% of the CPU.

On a multi-core system, the shares of CPU time are distributed over all
CPU cores. Even if a container is limited to less than 100% of CPU time,
it can use 100% of each individual CPU core.

For example, consider a system with more than three cores. If the
container *C0* is started with **\--cpu-shares=512** running one
process, and another container *C1* with **\--cpu-shares=1024** running
two processes, this can result in the following division of CPU shares:

  PID   container   CPU   CPU share
  ----- ----------- ----- --------------
  100   C0          0     100% of CPU0
  101   C1          1     100% of CPU1
  102   C1          2     100% of CPU2

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpus**=*number*

Number of CPUs. The default is *0.0* which means no limit. This is
shorthand for **\--cpu-period** and **\--cpu-quota**, therefore the
option cannot be specified with **\--cpu-period** or **\--cpu-quota**.

On some systems, changing the CPU limits may not be allowed for non-root
users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-cpus**=*number*

CPUs in which to allow execution. Can be specified as a comma-separated
list (e.g. **0,1**), as a range (e.g. **0-3**), or any combination
thereof (e.g. **0-3,7,11-15**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--cpuset-mems**=*nodes*

Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
effective on NUMA systems.

If there are four memory nodes on the system (0-3), use
**\--cpuset-mems=0,1** then processes in the container only uses memory
from the first two memory nodes.

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-read-bps**=*path:rate*

Limit read rate (in bytes per second) from a device (e.g.
**\--device-read-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-read-iops**=*path:rate*

Limit read rate (in IO operations per second) from a device (e.g.
**\--device-read-iops=/dev/sda:1000**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-bps**=*path:rate*

Limit write rate (in bytes per second) to a device (e.g.
**\--device-write-bps=/dev/sda:1mb**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--device-write-iops**=*path:rate*

Limit write rate (in IO operations per second) to a device (e.g.
**\--device-write-iops=/dev/sda:1000**).

On some systems, changing the resource limits may not be allowed for
non-root users. For more details, see
https://github.com/containers/podman/blob/main/troubleshooting.md#26-running-containers-with-resource-limits-fails-with-a-permissions-error

This option is not supported on cgroups V1 rootless systems.

#### **\--memory**, **-m**=*number\[unit\]*

Memory limit. A *unit* can be **b** (bytes), **k** (kibibytes), **m**
(mebibytes), or **g** (gibibytes).

Allows the memory available to a container to be constrained. If the
host supports swap memory, then the **-m** memory setting can be larger
than physical RAM. If a limit of 0 is specified (not using **-m**), the
container\'s memory is not limited. The actual limit may be rounded up
to a multiple of the operating system\'s page size (the value is very
large, that\'s millions of trillions).

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-reservation**=*number\[unit\]*

Memory soft limit. A *unit* can be **b** (bytes), **k** (kibibytes),
**m** (mebibytes), or **g** (gibibytes).

After setting memory reservation, when the system detects memory
contention or low memory, containers are forced to restrict their
consumption to their reservation. So always set the value below
**\--memory**, otherwise the hard limit takes precedence. By default,
memory reservation is the same as memory limit.

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swap**=*number\[unit\]*

A limit value equal to memory plus swap. A *unit* can be **b** (bytes),
**k** (kibibytes), **m** (mebibytes), or **g** (gibibytes).

Must be used with the **-m** (**\--memory**) flag. The argument value
must be larger than that of **-m** (**\--memory**) By default, it is set
to double the value of **\--memory**.

Set *number* to **-1** to enable unlimited swap.

This option is not supported on cgroups V1 rootless systems.

#### **\--memory-swappiness**=*number*

Tune a container\'s memory swappiness behavior. Accepts an integer
between *0* and *100*.

This flag is only supported on cgroups V1 rootful systems.

#### **\--pids-limit**=*limit*

Tune the container\'s pids limit. Set to **-1** to have unlimited pids
for the container. The default is **2048** on systems that support
\"pids\" cgroup controller.

#### **\--restart**=*policy*

Restart policy to follow when containers exit. Restart policy does not
take effect if a container is stopped via the **podman kill** or
**podman stop** commands.

Valid *policy* values are:

-   `no` : Do not restart containers on exit
-   `never` : Synonym for **no**; do not restart containers on exit
-   `on-failure[:max_retries]` : Restart containers when they exit with
    a non-zero exit code, retrying indefinitely or until the optional
    *max_retries* count is hit
-   `always` : Restart containers when they exit, regardless of status,
    retrying indefinitely
-   `unless-stopped` : Identical to **always**

Podman provides a systemd unit file, podman-restart.service, which
restarts containers after a system reboot.

When running containers in systemd services, use the restart
functionality provided by systemd. In other words, do not use this
option in a container unit, instead set the `Restart=` systemd directive
in the `[Service]` section. See **podman-systemd.unit**(5) and
**systemd.service**(5).

##  EXAMPLEs

Update a container with a new cpu quota and period.

    podman update --cpus=5 myCtr

Update a container with all available options for cgroups v2.

    podman update --cpus 5 --cpuset-cpus 0 --cpu-shares 123 --cpuset-mems 0 --memory 1G --memory-swap 2G --memory-reservation 2G --blkio-weight-device /dev/zero:123 --blkio-weight 123 --device-read-bps /dev/zero:10mb --device-write-bps /dev/zero:10mb --device-read-iops /dev/zero:1000 --device-write-iops /dev/zero:1000 --pids-limit 123 ctrID

Update a container with all available options for cgroups v1.

    podman update --cpus 5 --cpuset-cpus 0 --cpu-shares 123 --cpuset-mems 0 --memory 1G --memory-swap 2G --memory-reservation 2G --memory-swappiness 50 --pids-limit 123 ctrID

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-create(1)](podman-create.html)**,
**[podman-run(1)](podman-run.html)**

##  HISTORY

August 2022, Originally written by Charlie Doern <cdoern@redhat.com>


---

<a id='podman-version'></a>

## podman-version - Display the Podman version information

##  NAME

podman-version - Display the Podman version information

##  SYNOPSIS

**podman version** \[*options*\]

##  DESCRIPTION

Shows the following information: Remote API Version, Version, Go
Version, Git Commit, Build Time, OS, and Architecture.

##  OPTIONS

#### **\--format**, **-f**=*format*

Change output format to \"json\" or a Go template.

  **Placeholder**   **Description**
  ----------------- --------------------------
  .Client \...      Version of local podman
  .Server \...      Version of remote podman

Each of the above fields branch deeper into further subfields such as
.Version, .APIVersion, .GoVersion, and more.

##  Example

A sample output of the `version` command:

    $ podman version
    Version:      2.0.0
    API Version:  1
    Go Version:   go1.14.2
    Git Commit:   4520664f63c3a7f9a80227715359e20069d95542
    Built:        Tue May 19 10:48:59 2020
    OS/Arch:      linux/amd64

Filtering out only the version:

    $ podman version --format '{{.Client.Version}}'
    2.0.0

#### **\--help**, **-h**

Print usage statement

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

November 2018, Added \--format flag by Tomas Tomecek
<ttomecek@redhat.com> July 2017, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-volume'></a>

## podman-volume - Simple management tool for volumes

##  NAME

podman-volume - Simple management tool for volumes

##  SYNOPSIS

**podman volume** *subcommand*

##  DESCRIPTION

podman volume is a set of subcommands that manage volumes.

##  SUBCOMMANDS

  ----------------------------------------------------------------------------------------------------------
  Command   Man Page                                                 Description
  --------- -------------------------------------------------------- ---------------------------------------
  create    [podman-volume-create(1)](podman-volume-create.html)     Create a new volume.

  exists    [podman-volume-exists(1)](podman-volume-exists.html)     Check if the given volume exists.

  export    [podman-volume-export(1)](podman-volume-export.html)     Export volume to external tar.

  import    [podman-volume-import(1)](podman-volume-import.html)     Import tarball contents into an
                                                                     existing podman volume.

  inspect   [podman-volume-inspect(1)](podman-volume-inspect.html)   Get detailed information on one or more
                                                                     volumes.

  ls        [podman-volume-ls(1)](podman-volume-ls.html)             List all the available volumes.

  mount     [podman-volume-mount(1)](podman-volume-mount.html)       Mount a volume filesystem.

  prune     [podman-volume-prune(1)](podman-volume-prune.html)       Remove all unused volumes.

  reload    [podman-volume-reload(1)](podman-volume-reload.html)     Reload all volumes from volumes
                                                                     plugins.

  rm        [podman-volume-rm(1)](podman-volume-rm.html)             Remove one or more volumes.

  unmount   [podman-volume-unmount(1)](podman-volume-unmount.html)   Unmount a volume.
  ----------------------------------------------------------------------------------------------------------

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

November 2018, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-wait'></a>

## podman-wait - Wait on one or more containers to stop and print their
exit codes

##  NAME

podman-wait - Wait on one or more containers to stop and print their
exit codes

##  SYNOPSIS

**podman wait** \[*options*\] *container* \[\...\]

**podman container wait** \[*options*\] *container* \[\...\]

##  DESCRIPTION

Waits on one or more containers to stop. The container can be referred
to by its name or ID. In the case of multiple containers, Podman waits
on each consecutively. After all conditions are satisfied, the
containers\' return codes are printed separated by newline in the same
order as they were given to the command. An exit code of -1 is emitted
for all conditions other than \"stopped\" and \"exited\".

When waiting for containers with a restart policy of `always` or
`on-failure`, such as those created by `podman kube play`, the
containers may be repeatedly exiting and restarting, possibly with
different exit codes. `podman wait` will only display and detect the
first exit after the wait command was started.

When running a container with podman run \--rm wait does not wait for
the container to be fully removed. To wait for the removal of a
container use `--condition=removing`.

##  OPTIONS

#### **\--condition**=*state*

Container state or condition to wait for. Can be specified multiple
times where at least one condition must match for the command to return.
Supported values are \"configured\", \"created\", \"exited\",
\"healthy\", \"initialized\", \"paused\", \"removing\", \"running\",
\"stopped\", \"stopping\", \"unhealthy\". The default condition is
\"stopped\".

#### **\--help**, **-h**

Print usage statement

#### **\--ignore**

Ignore errors when a specified container is missing and mark its return
code as -1.

#### **\--interval**, **-i**=*duration*

Time interval to wait before polling for completion. A duration string
is a sequence of decimal numbers, each with optional fraction and a unit
suffix, such as \"300ms\", \"-1.5h\" or \"2h45m\". Valid time units are
\"ns\", \"us\" (or \"µs\"), \"ms\", \"s\", \"m\", \"h\". Time unit
defaults to \"ms\".

#### **\--latest**, **-l**

Instead of providing the container name or ID, use the last created
container. Note: the last started container can be from other users of
Podman on the host machine. (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

##  EXAMPLES

Wait for the specified container to exit.

    $ podman wait mywebserver
    0

Wait for the latest container to exit. (This option is not available
with the remote Podman client, including Mac and Windows (excluding
WSL2) machines)

    $ podman wait --latest
    0

Wait for the container to exit, checking every two seconds.

    $ podman wait --interval 2s mywebserver
    0

Wait for the container by ID. This container exits with error status 1:

    $ podman wait 860a4b23
    1

Wait for both specified containers to exit.

    $ podman wait mywebserver myftpserver
    0
    125

Wait for the named container to exit, but do not fail if the container
does not exist.

    $ podman wait --ignore does-not-exist
    -1

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

September 2017, Originally compiled by Brent Baude<bbaude@redhat.com>


---

