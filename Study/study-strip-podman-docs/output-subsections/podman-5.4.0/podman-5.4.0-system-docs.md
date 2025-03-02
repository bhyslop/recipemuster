# podman-5.4.0 System Commands

*This document contains System commands from the Podman documentation.*

## Table of Contents

- [podman-system-check - Perform consistency checks on image and
container storage](#podman-system-check)
- [podman-system-connection - Manage the destination(s) for Podman
service(s)](#podman-system-connection)
- [podman-system-df - Show podman disk usage](#podman-system-df)
- [podman-events - Monitor Podman events](#podman-system-events)
- [podman-info - Display Podman related system information](#podman-system-info)
- [podman-system-prune - Remove all unused pods, containers, images,
networks, and volume data](#podman-system-prune)

<a id='podman-system-check'></a>

## podman-system-check - Perform consistency checks on image and
container storage

##  NAME

podman-system-check - Perform consistency checks on image and container
storage

##  SYNOPSIS

**podman system check** \[*options*\]

##  DESCRIPTION

Perform consistency checks on image and container storage, reporting
images and containers which have identified issues.

##  OPTIONS

#### **\--force**, **-f**

When attempting to remove damaged images, also remove containers which
depend on those images. By default, damaged images which are being used
by containers are left alone.

Containers which depend on damaged images do so regardless of which
engine created them, but because podman only \"knows\" how to shut down
containers that it started, the effect on still-running containers which
were started by other engines is difficult to predict.

#### **\--max**, **-m**=*duration*

When considering layers which are not used by any images or containers,
assume that any layers which are more than *duration* old are the
results of canceled attempts to pull images, and should be treated as
though they are damaged.

#### **\--quick**, **-q**

Skip checks which are known to be time-consuming. This will prevent some
types of errors from being detected.

#### **\--repair**, **-r**

Remove any images which are determined to have been damaged in some way,
unless they are in use by containers. Use **\--force** to remove
containers which depend on damaged images, and those damaged images, as
well.

##  EXAMPLE

A reasonably quick check:

    podman system check --quick --repair --force

A more thorough check:

    podman system check --repair --max=1h --force

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-system(1)](podman-system.html)**

##  HISTORY

April 2024


---

<a id='podman-system-connection'></a>

## podman-system-connection - Manage the destination(s) for Podman
service(s)

##  NAME

podman-system-connection - Manage the destination(s) for Podman
service(s)

##  SYNOPSIS

**podman system connection** *subcommand*

##  DESCRIPTION

Manage the destination(s) for Podman service(s).

The user is prompted for the ssh login password or key file passphrase
as required. The `ssh-agent` is supported if it is running.

Podman manages the system connection by writing and reading the
`podman-connections.json` file located under
`$XDG_CONFIG_HOME/containers` or if the env is not set it defaults to
`$HOME/.config/containers`. Or the `PODMAN_CONNECTIONS_CONF` environment
variable can be set to a full file path which podman will use instead.
This file is managed by the podman commands and should never be edited
by users directly. To manually configure the connections use
`service_destinations` in containers.conf.

If the ReadWrite column in the **podman system connection list** output
is set to true the connection is stored in the `podman-connections.json`
file otherwise it is stored in containers.conf and can therefore not be
edited with the **podman system connection** commands.

##  COMMANDS

  ---------------------------------------------------------------------------------------------------------------------
  Command   Man Page                                                                       Description
  --------- ------------------------------------------------------------------------------ ----------------------------
  add       [podman-system-connection-add(1)](podman-system-connection-add.html)           Record destination for the
                                                                                           Podman service

  default   [podman-system-connection-default(1)](podman-system-connection-default.html)   Set named destination as
                                                                                           default for the Podman
                                                                                           service

  list      [podman-system-connection-list(1)](podman-system-connection-list.html)         List the destination for the
                                                                                           Podman service(s)

  remove    [podman-system-connection-remove(1)](podman-system-connection-remove.html)     Delete named destination

  rename    [podman-system-connection-rename(1)](podman-system-connection-rename.html)     Rename the destination for
                                                                                           Podman service
  ---------------------------------------------------------------------------------------------------------------------

##  EXAMPLE

List system connections:

    $ podman system connection list
    Name URI                                           Identity       Default  ReadWrite
    devl ssh://root@example.com/run/podman/podman.sock ~/.ssh/id_rsa  true     true

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-system(1)](podman-system.html)**

##  HISTORY

June 2020, Originally compiled by Jhon Honce (jhonce at redhat dot com)


---

<a id='podman-system-df'></a>

## podman-system-df - Show podman disk usage

##  NAME

podman-system-df - Show podman disk usage

##  SYNOPSIS

**podman system df** \[*options*\]

##  DESCRIPTION

Show podman disk usage

##  OPTIONS

#### **\--format**=*format*

Pretty-print images using a Go template or JSON. This flag is not
allowed in combination with **\--verbose**

Valid placeholders for the Go template are listed below:

  -----------------------------------------------------------------------
  **Placeholder**          **Description**
  ------------------------ ----------------------------------------------
  .Active                  Indicates whether volume is in use

  .RawReclaimable          Raw reclaimable size of each Type

  .RawSize                 Raw size of each type

  .Reclaimable             Reclaimable size or each type (human-readable)

  .Size                    Size of each type (human-readable)

  .Total                   Total items for each type

  .Type                    Type of data
  -----------------------------------------------------------------------

#### **\--verbose**, **-v**

Show detailed information on space usage

##  EXAMPLE

Show disk usage:

    $ podman system df
    TYPE            TOTAL   ACTIVE   SIZE    RECLAIMABLE
    Images          6       2        281MB   168MB (59%)
    Containers      3       1        0B      0B (0%)
    Local Volumes   1       1        22B     0B (0%)

Show disk usage in verbose mode:

    $ podman system df -v
    Images space usage:

    REPOSITORY                 TAG      IMAGE ID       CREATED       SIZE     SHARED SIZE   UNIQUE SIZE   CONTAINERS
    docker.io/library/alpine   latest   5cb3aa00f899   2 weeks ago   5.79MB   0B            5.79MB       5

    Containers space usage:

    CONTAINER ID    IMAGE   COMMAND       LOCAL VOLUMES   SIZE     CREATED        STATUS       NAMES
    073f7e62812d    5cb3    sleep 100     1               0B       20 hours ago   exited       zen_joliot
    3f19f5bba242    5cb3    sleep 100     0               5.52kB   22 hours ago   exited       pedantic_archimedes
    8cd89bf645cc    5cb3    ls foodir     0               58B      21 hours ago   configured   agitated_hamilton
    a1d948a4b61d    5cb3    ls foodir     0               12B      21 hours ago   exited       laughing_wing
    eafe3e3c5bb3    5cb3    sleep 10000   0               72B      21 hours ago   exited       priceless_liskov

    Local Volumes space usage:

    VOLUME NAME   LINKS   SIZE
    data          1       0B

    $ podman system df --format "{{.Type}}\t{{.Total}}"
    Images          1
    Containers      5
    Local Volumes   1

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-system(1)](podman-system.html)**

##  HISTORY

March 2019, Originally compiled by Qi Wang (qiwan at redhat dot com)


---

<a id='podman-system-events'></a>

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

<a id='podman-system-info'></a>

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

<a id='podman-system-prune'></a>

## podman-system-prune - Remove all unused pods, containers, images,
networks, and volume data

##  NAME

podman-system-prune - Remove all unused pods, containers, images,
networks, and volume data

##  SYNOPSIS

**podman system prune** \[*options*\]

##  DESCRIPTION

**podman system prune** removes all unused containers (both dangling and
unreferenced), build containers, pods, networks, and optionally, volumes
from local storage.

Use the **\--all** option to delete all unused images. Unused images are
dangling images as well as any image that does not have any containers
based on it.

By default, volumes are not removed to prevent important data from being
deleted if there is currently no container using the volume. Use the
**\--volumes** flag when running the command to prune volumes as well.

By default, build containers are not removed to prevent interference
with builds in progress. Use the **\--build** flag when running the
command to remove build containers as well.

##  OPTIONS

#### **\--all**, **-a**

Recursively remove all unused pods, containers, images, networks, and
volume data. (Maximum 50 iterations.)

#### **\--build**

Removes any build containers that were created during the build, but
were not removed because the build was unexpectedly terminated.

Note: **This is not safe operation and should be executed only when no
builds are in progress. It can interfere with builds in progress.**

#### **\--external**

Tries to clean up remainders of previous containers or layers that are
not references in the storage json files. These can happen in the case
of unclean shutdowns or regular restarts in transient storage mode.

However, when using transient storage mode, the Podman database does not
persist. This means containers leave the writable layers on disk after a
reboot. When using a transient store, it is recommended that the
**podman system prune \--external** command is run during boot.

This option is incompatible with **\--all** and **\--filter** and drops
the default behaviour of removing unused resources.

#### **\--filter**=*filters*

Provide filter values.

The *filters* argument format is of `key=value`. If there is more than
one *filter*, then pass multiple OPTIONS: **\--filter** *foo=bar*
**\--filter** *bif=baz*.

Supported filters:

  ---------------------------------------------------------------------------
   Filter  Description
  -------- ------------------------------------------------------------------
   label   Only remove containers and images, with (or without, in the case
           of label!=\[\...\] is used) the specified labels.

   until   Only remove containers and images created before given timestamp.
  ---------------------------------------------------------------------------

The `label` *filter* accepts two formats. One is the `label`=*key* or
`label`=*key*=*value*, which removes containers and images with the
specified labels. The other format is the `label!`=*key* or
`label!`=*key*=*value*, which removes containers and images without the
specified labels.

The `until` *filter* can be Unix timestamps, date formatted timestamps,
or Go duration strings (e.g. 10m, 1h30m) computed relative to the
machine's time.

#### **\--force**, **-f**

Do not prompt for confirmation

#### **\--help**, **-h**

Print usage statement

#### **\--volumes**

Prune volumes currently unused by any container

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-system(1)](podman-system.html)**

##  HISTORY

February 2019, Originally compiled by Dan Walsh (dwalsh at redhat dot
com) December 2020, converted filter information from docs.docker.com
documentation by Dan Walsh (dwalsh at redhat dot com)


---

