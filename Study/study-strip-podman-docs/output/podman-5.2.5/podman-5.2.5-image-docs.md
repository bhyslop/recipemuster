# podman-5.2.5 Image Commands

*This document contains Image commands from the Podman documentation.*

## Table of Contents

- [podman-build - Build a container image using a Containerfile](#podman-image-build)
- [podman-image-diff - Inspect changes on an image's filesystem](#podman-image-diff)
- [podman-image-exists - Check if an image exists in local storage](#podman-image-exists)
- [podman-history - Show the history of an image](#podman-image-history)
- [podman-import - Import a tarball and save it as a filesystem image](#podman-image-import)
- [podman-image-inspect - Display an image's configuration](#podman-image-inspect)
- [podman-images - List images in local storage](#podman-image-list)
- [podman-load - Load image(s) from a tar archive into container storage](#podman-image-load)
- [podman-image-prune - Remove all unused images from the local store](#podman-image-prune)
- [podman-pull - Pull an image from a registry](#podman-image-pull)
- [podman-push - Push an image, manifest list or image index from local storage to elsewhere](#podman-image-push)
- [podman-rmi - Remove one or more locally stored images](#podman-image-rm)
- [podman-save - Save image(s) to an archive](#podman-image-save)
- [podman-image-scp - Securely copy an image from one host to another](#podman-image-scp)
- [podman-search - Search a registry for an image](#podman-image-search)
- [podman-tag - Add an additional name to a local image](#podman-image-tag)
- [podman-image-tree - Print layer hierarchy of an image in a tree format](#podman-image-tree)
- [podman-untag - Remove one or more names from a locally-stored image](#podman-image-untag)

<a id='podman-image-build'></a>

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
`podman ps --all --storage` command to see these containers. External
containers can be removed with the `podman rm --storage` command.

`podman buildx build` command is an alias of `podman build`. Not all
`buildx build` features are available in Podman. The `buildx build`
option is provided for scripting compatibility.

##  OPTIONS

#### **\--add-host**=*host:ip*

Add a custom host-to-IP mapping (host:ip)

Add a line to /etc/hosts. The format is hostname:ip. The **\--add-host**
option can be set multiple times. Conflicts with the **\--no-hosts**
option.

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
`--no-cache` in the implementation since this means that the user dones
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

Do not create */etc/hosts* for the container. By default, Podman manages
*/etc/hosts*, adding the container\'s own IP address and any hosts from
**\--add-host**. **\--no-hosts** disables this, and the image\'s
*/etc/hosts* is preserved unmodified.

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
any platform that exists, `RUN` instructions are able to succeed without
the help of emulation provided by packages like `qemu-user-static`.

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

<a id='podman-image-diff'></a>

## podman-image-diff - Inspect changes on an image's filesystem

##  NAME

podman-image-diff - Inspect changes on an image\'s filesystem

##  SYNOPSIS

**podman image diff** \[*options*\] *image* \[*image*\]

##  DESCRIPTION

Displays changes on an image\'s filesystem. The image is compared to its
parent layer or the second argument when given.

The output is prefixed with the following symbols:

  Symbol   Description
  -------- ----------------------------------
  A        A file or directory was added.
  D        A file or directory was deleted.
  C        A file or directory was changed.

##  OPTIONS

#### **\--format**

Alter the output into a different format. The only valid format for
**podman image diff** is `json`.

##  EXAMPLE

Display image differences from images parent layer:

    $ podman image diff redis:old
    C /usr
    C /usr/local
    C /usr/local/bin
    A /usr/local/bin/docker-entrypoint.sh

Display image differences between two different images in JSON format:

    $ podman image diff --format json redis:old redis:alpine
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

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-image(1)](podman-image.html)**

##  HISTORY

August 2017, Originally compiled by Ryan Cole <rycole@redhat.com>


---

<a id='podman-image-exists'></a>

## podman-image-exists - Check if an image exists in local storage

##  NAME

podman-image-exists - Check if an image exists in local storage

##  SYNOPSIS

**podman image exists** *image*

##  DESCRIPTION

**podman image exists** checks if an image exists in local storage. The
**ID** or **Name** of the image may be used as input. Podman returns an
exit code of `0` when the image is found. A `1` is returned otherwise.
An exit code of `125` indicates there was an issue accessing the local
storage.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

##  EXAMPLES

Check if an image called `webclient` exists in local storage (the image
does actually exist):

    $ podman image exists webclient
    $ echo $?
    0

Check if an image called `webbackend` exists in local storage (the image
does not actually exist):

    $ podman image exists webbackend
    $ echo $?
    1

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-image(1)](podman-image.html)**

##  HISTORY

November 2018, Originally compiled by Brent Baude (bbaude at redhat dot
com)


---

<a id='podman-image-history'></a>

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

<a id='podman-image-import'></a>

## podman-import - Import a tarball and save it as a filesystem image

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

<a id='podman-image-inspect'></a>

## podman-image-inspect - Display an image's configuration

##  NAME

podman-image-inspect - Display an image\'s configuration

##  SYNOPSIS

**podman image inspect** \[*options*\] *image* \[*image* \...\]

##  DESCRIPTION

This displays the low-level information on images identified by name or
ID. By default, this renders all results in a JSON array. If a format is
specified, the given template is executed for each result.

##  OPTIONS

#### **\--format**, **-f**=*format*

Format the output using the given Go template. The keys of the returned
JSON can be used as the values for the \--format flag (see examples
below).

Valid placeholders for the Go template are listed below:

  **Placeholder**     **Description**
  ------------------- ----------------------------------------------
  .Annotations \...   Annotation information included in the image
  .Architecture       Architecture of software in the image
  .Author             Image author
  .Comment            Image comment
  .Config \...        Structure with config info
  .Created \...       Image creation time (string, ISO3601)
  .Digest             Image digest (sha256:+64-char hash)
  .GraphDriver \...   Structure for the graph driver info
  .HealthCheck \...   Structure for the health check info
  .History            History information stored in image
  .ID                 Image ID (full 64-char hash)
  .Labels \...        Label information included in the image
  .ManifestType       Manifest type of the image
  .NamesHistory       Name history information stored in image
  .Os                 Operating system of software in the image
  .Parent             Parent image of the specified image
  .RepoDigests        Repository digests for the image
  .RepoTags           Repository tags for the image
  .RootFS \...        Structure for the root file system info
  .Size               Size of image, in bytes
  .User               Default user to execute the image as
  .Version            Image Version
  .VirtualSize        Virtual size of image, in bytes

##  EXAMPLE

Inspect information on the specified image:

    $ podman image inspect fedora
    [
        {
            "Id": "37e5619f4a8ca9dbc4d6c0ae7890625674a10dbcfb76201399e2aaddb40da17d",
            "Digest": "sha256:1b0d4ddd99b1a8c8a80e885aafe6034c95f266da44ead992aab388e6aa91611a",
            "RepoTags": [
                "registry.fedoraproject.org/fedora:latest"
            ],
            "RepoDigests": [
                "registry.fedoraproject.org/fedora@sha256:1b0d4ddd99b1a8c8a80e885aafe6034c95f266da44ead992aab388e6aa91611a",
                "registry.fedoraproject.org/fedora@sha256:b5290db40008aae9272ad3a6bd8070ef7ecd547c3bef014b894c327960acc582"
            ],
            "Parent": "",
            "Comment": "Created by Image Factory",
            "Created": "2021-08-09T05:48:47Z",
            "Config": {
                "Env": [
                    "DISTTAG=f34container",
                    "FGC=f34",
                    "container=oci"
                ],
                "Cmd": [
                    "/bin/bash"
                ],
                "Labels": {
                    "license": "MIT",
                    "name": "fedora",
                    "vendor": "Fedora Project",
                    "version": "34"
                }
            },
            "Version": "1.10.1",
            "Author": "",
            "Architecture": "amd64",
            "Os": "linux",
            "Size": 183852302,
            "VirtualSize": 183852302,
            "GraphDriver": {
                "Name": "overlay",
                "Data": {
                    "UpperDir": "/home/dwalsh/.local/share/containers/storage/overlay/0203e243f1ca4b6bb49371ecd21363212467ec6d7d3fa9f324cd4e78cc6b5fa2/diff",
                    "WorkDir": "/home/dwalsh/.local/share/containers/storage/overlay/0203e243f1ca4b6bb49371ecd21363212467ec6d7d3fa9f324cd4e78cc6b5fa2/work"
                }
            },
            "RootFS": {
                "Type": "layers",
                "Layers": [
                    "sha256:0203e243f1ca4b6bb49371ecd21363212467ec6d7d3fa9f324cd4e78cc6b5fa2"
                ]
            },
            "Labels": {
                "license": "MIT",
                "name": "fedora",
                "vendor": "Fedora Project",
                "version": "34"
            },
            "Annotations": {},
            "ManifestType": "application/vnd.docker.distribution.manifest.v2+json",
            "User": "",
            "History": [
                {
                    "created": "2021-08-09T05:48:47Z",
                    "comment": "Created by Image Factory"
                }
            ],
            "NamesHistory": [
                "registry.fedoraproject.org/fedora:latest"
            ]
        }
    ]

Inspect image ID for the specified image:

    $ podman image inspect --format '{{ .Id }}' fedora
    37e5619f4a8ca9dbc4d6c0ae7890625674a10dbcfb76201399e2aaddb40da17d

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-image(1)](podman-image.html)**,
**[podman-inspect(1)](podman-inspect.html)**

##  HISTORY

Sep 2021, Originally compiled by Dan Walsh <dwalsh@redhat.com>


---

<a id='podman-image-list'></a>

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

  **Placeholder**   **Description**
  ----------------- ------------------------------------------------------------
  .Containers       Number of containers using this image
  .Created          Elapsed time since the image was created
  .CreatedAt        Time when the image was created, YYYY-MM-DD HH:MM:SS +nnnn
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
**created**)

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

##  HISTORY

March 2017, Originally compiled by Dan Walsh `<dwalsh@redhat.com>`


---

<a id='podman-image-load'></a>

## podman-load - Load image(s) from a tar archive into container storage

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

<a id='podman-image-prune'></a>

## podman-image-prune - Remove all unused images from the local store

##  NAME

podman-image-prune - Remove all unused images from the local store

##  SYNOPSIS

**podman image prune** \[*options*\]

##  DESCRIPTION

**podman image prune** removes all dangling images from local storage.
With the `all` option, all unused images are deleted (i.e., images not
in use by any container).

The image prune command does not prune cache images that only use layers
that are necessary for other images.

##  OPTIONS

#### **\--all**, **-a**

Remove dangling images and images that have no associated containers.

#### **\--external**

Remove images even when they are used by external containers (e.g.,
build containers).

#### **\--filter**=*filters*

Provide filter values.

The *filters* argument format is of `key=value`. If there is more than
one *filter*, then pass multiple OPTIONS: **\--filter** *foo=bar*
**\--filter** *bif=baz*.

Supported filters:

  --------------------------------------------------------------------------
   Filter  Description
  -------- -----------------------------------------------------------------
   label   Only remove images, with (or without, in the case of
           label!=\[\...\] is used) the specified labels.

   until   Only remove images created before given timestamp.
  --------------------------------------------------------------------------

The `label` *filter* accepts two formats. One is the `label`=*key* or
`label`=*key*=*value*, which removes containers with the specified
labels. The other format is the `label!`=*key* or
`label!`=*key*=*value*, which removes containers without the specified
labels.

The `until` *filter* can be Unix timestamps, date formatted timestamps
or Go duration strings (e.g. 10m, 1h30m) computed relative to the
machine's time.

#### **\--force**, **-f**

Do not provide an interactive prompt for container removal.

#### **\--help**, **-h**

Print usage statement

##  EXAMPLES

Remove all dangling images from local storage:

    $ sudo podman image prune

    WARNING! This will remove all dangling images.
    Are you sure you want to continue? [y/N] y
    f3e20dc537fb04cb51672a5cb6fdf2292e61d411315549391a0d1f64e4e3097e
    324a7a3b2e0135f4226ffdd473e4099fd9e477a74230cdc35de69e84c0f9d907

Remove all unused images from local storage without confirming:

    $ sudo podman image prune -a -f
    f3e20dc537fb04cb51672a5cb6fdf2292e61d411315549391a0d1f64e4e3097e
    324a7a3b2e0135f4226ffdd473e4099fd9e477a74230cdc35de69e84c0f9d907
    6125002719feb1ddf3030acab1df6156da7ce0e78e571e9b6e9c250424d6220c
    91e732da5657264c6f4641b8d0c4001c218ae6c1adb9dcef33ad00cafd37d8b6
    e4e5109420323221f170627c138817770fb64832da7d8fe2babd863148287fca
    77a57fa8285e9656dbb7b23d9efa837a106957409ddd702f995605af27a45ebe

Remove all unused images from local storage since given time/hours:

    $ sudo podman image prune -a --filter until=2019-11-14T06:15:42.937792374Z

    WARNING! This will remove all dangling images.
    Are you sure you want to continue? [y/N] y
    e813d2135f17fadeffeea8159a34cfdd4c30b98d8111364b913a91fd930643e9
    5e6572320437022e2746467ddf5b3561bf06e099e8e6361df27e0b2a7ed0b17b
    58fda2abf5042b35dfe04e5f8ee458a3cc26375bf309efb42c078b551a2055c7
    6d2bd30fe924d3414b64bd3920760617e6ced872364bc3bc6959a623252da002
    33d1c829be64a1e1d379caf4feec1f05a892c3ef7aa82c0be53d3c08a96c59c5
    f9f0a8a58c9e02a2b3250b88cc5c95b1e10245ca2c4161d19376580aaa90f55c
    1ef14d5ede80db78978b25ad677fd3e897a578c3af614e1fda608d40c8809707
    45e1482040e441a521953a6da2eca9bafc769e15667a07c23720d6e0cafc3ab2

    $ sudo podman image prune -f --filter until=10h
    f3e20dc537fb04cb51672a5cb6fdf2292e61d411315549391a0d1f64e4e3097e
    324a7a3b2e0135f4226ffdd473e4099fd9e477a74230cdc35de69e84c0f9d907

Remove all unused images from local storage with label version 1.0:

    $ sudo podman image prune -a -f --filter label=version=1.0
    e813d2135f17fadeffeea8159a34cfdd4c30b98d8111364b913a91fd930643e9
    5e6572320437022e2746467ddf5b3561bf06e099e8e6361df27e0b2a7ed0b17b
    58fda2abf5042b35dfe04e5f8ee458a3cc26375bf309efb42c078b551a2055c7
    6d2bd30fe924d3414b64bd3920760617e6ced872364bc3bc6959a623252da002
    33d1c829be64a1e1d379caf4feec1f05a892c3ef7aa82c0be53d3c08a96c59c5
    f9f0a8a58c9e02a2b3250b88cc5c95b1e10245ca2c4161d19376580aaa90f55c
    1ef14d5ede80db78978b25ad677fd3e897a578c3af614e1fda608d40c8809707
    45e1482040e441a521953a6da2eca9bafc769e15667a07c23720d6e0cafc3ab2

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-images(1)](podman-images.html)**

##  HISTORY

December 2018, Originally compiled by Brent Baude (bbaude at redhat dot
com) December 2020, converted filter information from docs.docker.com
documentation by Dan Walsh (dwalsh at redhat dot com)


---

<a id='podman-image-pull'></a>

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
a`short-name` reference. If the image is a \'short-name\' reference,
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

##  HISTORY

July 2017, Originally compiled by Urvashi Mohnani <umohnani@redhat.com>


---

<a id='podman-image-push'></a>

## podman-push - Push an image, manifest list or image index from local storage to elsewhere

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
the containers.conf file.

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


---

<a id='podman-image-rm'></a>

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

<a id='podman-image-save'></a>

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

<a id='podman-image-scp'></a>

## podman-image-scp - Securely copy an image from one host to another

##  NAME

podman-image-scp - Securely copy an image from one host to another

##  SYNOPSIS

**podman image scp** \[*options*\] *name*\[:*tag*\]

##  DESCRIPTION

**podman image scp** copies container images between hosts on a network.
This command can copy images to the remote host or from the remote host
as well as between two remote hosts. Note: `::` is used to specify the
image name depending on Podman is saving or loading. Images can also be
transferred from rootful to rootless storage on the same machine without
using sshd. This feature is not supported on the remote client,
including Mac and Windows (excluding WSL2) machines.

**podman image scp \[GLOBAL OPTIONS\]**

**podman image** *scp [OPTIONS](#options) NAME\[:TAG\] \[HOSTNAME::\]*

**podman image** *scp [OPTIONS](#options) \[HOSTNAME::\]IMAGENAME*

**podman image** *scp [OPTIONS](#options) \[HOSTNAME::\]IMAGENAME
\[HOSTNAME::\]*

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

#### **\--quiet**, **-q**

Suppress the output

##  EXAMPLES

Copy specified image to local storage:

    $ podman image scp alpine
    Loaded image: docker.io/library/alpine:latest

Copy specified image from local storage to remote connection:

    $ podman image scp alpine Fedora::/home/charliedoern/Documents/alpine
    Getting image source signatures
    Copying blob 72e830a4dff5 done
    Copying config 85f9dc67c7 done
    Writing manifest to image destination
    Storing signatures
    Loaded image: docker.io/library/alpine:latest

Copy specified image from remote connection to remote connection:

    $ podman image scp Fedora::alpine RHEL::
    Loaded image: docker.io/library/alpine:latest

Copy specified image via ssh to local storage:

    $ podman image scp charliedoern@192.168.68.126:22/run/user/1000/podman/podman.sock::alpine
    WARN[0000] Unknown connection name given. Please use system connection add to specify the default remote socket location
    Getting image source signatures
    Copying blob 9450ef9feb15 [--------------------------------------] 0.0b / 0.0b
    Copying config 1f97f0559c done
    Writing manifest to image destination
    Storing signatures
    Loaded image: docker.io/library/alpine:latest

Copy specified image from root account to user accounts local storage:

    $ sudo podman image scp root@localhost::alpine username@localhost::
    Copying blob e2eb06d8af82 done
    Copying config 696d33ca15 done
    Writing manifest to image destination
    Storing signatures
    Getting image source signatures
    Copying blob 5eb901baf107 skipped: already exists
    Copying config 696d33ca15 done
    Writing manifest to image destination
    Storing signatures
    Loaded image: docker.io/library/alpine:latest

Copy specified image from root account to local storage:

    $ sudo podman image scp root@localhost::alpine
    Copying blob e2eb06d8af82 done
    Copying config 696d33ca15 done
    Writing manifest to image destination
    Storing signatures
    Getting image source signatures
    Copying blob 5eb901baf107
    Copying config 696d33ca15 done
    Writing manifest to image destination
    Storing signatures
    Loaded image: docker.io/library/alpine:latest

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-load(1)](podman-load.html)**,
**[podman-save(1)](podman-save.html)**,
**[podman-remote(1)](podman-remote.html)**,
**[podman-system-connection-add(1)](podman-system-connection-add.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**,
**[containers-transports(5)](https://github.com/containers/image/blob/main/docs/containers-transports.5.md)**

##  HISTORY

July 2021, Originally written by Charlie Doern <cdoern@redhat.com>


---

<a id='podman-image-search'></a>

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

<a id='podman-image-tag'></a>

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

<a id='podman-image-tree'></a>

## podman-image-tree - Print layer hierarchy of an image in a tree format

##  NAME

podman-image-tree - Print layer hierarchy of an image in a tree format

##  SYNOPSIS

**podman image tree** \[*options*\] *image:tag*\|*image-id*

##  DESCRIPTION

Prints layer hierarchy of an image in a tree format. If no *tag* is
provided, Podman defaults to `latest` for the *image*. Layers are
indicated with image tags as `Top Layer of`, when the tag is known
locally. \## OPTIONS

#### **\--help**, **-h**

Print usage statement

#### **\--whatrequires**

Show all child images and layers of the specified image

##  EXAMPLES

List image tree information on specified image:

    $ podman image tree docker.io/library/wordpress
    Image ID: 6e880d17852f
    Tags:    [docker.io/library/wordpress:latest]
    Size:    429.9MB
    Image Layers
    ├──  ID: 3c816b4ead84 Size: 58.47MB
    ├──  ID: e39dad2af72e Size: 3.584kB
    ├──  ID: b2d6a702383c Size: 213.6MB
    ├──  ID: 94609408badd Size: 3.584kB
    ├──  ID: f4dddbf86725 Size: 43.04MB
    ├──  ID: 8f695df43a4c Size: 11.78kB
    ├──  ID: c29d67bf8461 Size: 9.728kB
    ├──  ID: 23f4315918f8 Size:  7.68kB
    ├──  ID: d082f93a18b3 Size: 13.51MB
    ├──  ID: 7ea8bedcac69 Size: 4.096kB
    ├──  ID: dc3bbf7b3dc0 Size: 57.53MB
    ├──  ID: fdbbc6404531 Size: 11.78kB
    ├──  ID: 8d24785437c6 Size: 4.608kB
    ├──  ID: 80715f9e8880 Size: 4.608kB Top Layer of: [docker.io/library/php:7.2-apache]
    ├──  ID: c93cbcd6437e Size: 3.573MB
    ├──  ID: dece674f3cd1 Size: 4.608kB
    ├──  ID: 834f4497afda Size: 7.168kB
    ├──  ID: bfe2ce1263f8 Size: 40.06MB
    └──  ID: 748e99b214cf Size: 11.78kB Top Layer of: [docker.io/library/wordpress:latest]

Show all child images and layers of the specified image:

    $ podman image tree ae96a4ad4f3f --whatrequires
    Image ID: ae96a4ad4f3f
    Tags:    [docker.io/library/ruby:latest]
    Size:    894.2MB
    Image Layers
    └──  ID: 9c92106221c7 Size:  2.56kB Top Layer of: [docker.io/library/ruby:latest]
     ├──  ID: 1b90f2b80ba0 Size: 3.584kB
     │   ├──  ID: 42b7d43ae61c Size: 169.5MB
     │   ├──  ID: 26dc8ba99ec3 Size: 2.048kB
     │   ├──  ID: b4f822db8d95 Size: 3.957MB
     │   ├──  ID: 044e9616ef8a Size: 164.7MB
     │   ├──  ID: bf94b940200d Size: 11.75MB
     │   ├──  ID: 4938e71bfb3b Size: 8.532MB
     │   └──  ID: f513034bf553 Size: 1.141MB
     ├──  ID: 1e55901c3ea9 Size: 3.584kB
     ├──  ID: b62835a63f51 Size: 169.5MB
     ├──  ID: 9f4e8857f3fd Size: 2.048kB
     ├──  ID: c3b392020e8f Size: 3.957MB
     ├──  ID: 880163026a0a Size: 164.8MB
     ├──  ID: 8c78b2b14643 Size: 11.75MB
     ├──  ID: 830370cfa182 Size: 8.532MB
     └──  ID: 567fd7b7bd38 Size: 1.141MB Top Layer of: [docker.io/circleci/ruby:latest]

##  SEE ALSO

**[podman(1)](podman.html)**

##  HISTORY

Feb 2019, Originally compiled by Kunal Kushwaha
`<kushwaha_kunal_v7@lab.ntt.co.jp>`


---

<a id='podman-image-untag'></a>

## podman-untag - Remove one or more names from a locally-stored image

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

