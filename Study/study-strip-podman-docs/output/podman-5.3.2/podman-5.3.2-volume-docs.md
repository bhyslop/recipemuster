# podman-5.3.2 Volume Commands

*This document contains Volume commands from the Podman documentation.*

## Table of Contents

- [podman-volume-create - Create a new volume](#podman-volume-create)
- [podman-volume-exists - Check if the given volume exists](#podman-volume-exists)
- [podman-volume-inspect - Get detailed information on one or more
volumes](#podman-volume-inspect)
- [podman-volume-ls - List all the available volumes](#podman-volume-ls)
- [podman-volume-prune - Remove all unused volumes](#podman-volume-prune)
- [podman-volume-reload - Reload all volumes from volumes plugins](#podman-volume-reload)
- [podman-volume-rm - Remove one or more volumes](#podman-volume-rm)

<a id='podman-volume-create'></a>

## podman-volume-create - Create a new volume

##  NAME

podman-volume-create - Create a new volume

##  SYNOPSIS

**podman volume create** \[*options*\] \[*name*\]

##  DESCRIPTION

Creates an empty volume and prepares it to be used by containers. The
volume can be created with a specific name, if a name is not given a
random name is generated. You can add metadata to the volume by using
the **\--label** flag and driver options can be set using the **\--opt**
flag.

##  OPTIONS

#### **\--driver**, **-d**=*driver*

Specify the volume driver name (default **local**). There are two
drivers supported by Podman itself: **local** and **image**.

The **local** driver uses a directory on disk as the backend by default,
but can also use the **mount(8)** command to mount a filesystem as the
volume if **\--opt** is specified.

The **image** driver uses an image as the backing store of for the
volume. An overlay filesystem is created, which allows changes to the
volume to be committed as a new layer on top of the image.

Using a value other than **local** or **image**, Podman attempts to
create the volume using a volume plugin with the given name. Such
plugins must be defined in the **volume_plugins** section of the
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
configuration file.

#### **\--help**

Print usage statement

#### **\--ignore**

Don\'t fail if the named volume already exists, instead just print the
name. Note that the new options are not applied to the existing volume.

#### **\--label**, **-l**=*label*

Set metadata for a volume (e.g., \--label mykey=value).

#### **\--opt**, **-o**=*option*

Set driver specific options. For the default driver, **local**, this
allows a volume to be configured to mount a filesystem on the host.

For the `local` driver the following options are supported: `type`,
`device`, `o`, and `[no]copy`.

-   The `type` option sets the type of the filesystem to be mounted, and
    is equivalent to the `-t` flag to **mount(8)**.
-   The `device` option sets the device to be mounted, and is equivalent
    to the `device` argument to **mount(8)**.
-   The `copy` option enables copying files from the container image
    path where the mount is created to the newly created volume on the
    first run. `copy` is the default.

The `o` option sets options for the mount, and is equivalent to the
filesystem options (also `-o`) passed to **mount(8)** with the following
exceptions:

-   The `o` option supports `uid` and `gid` options to set the UID and
    GID of the created volume that are not normally supported by
    **mount(8)**.
-   The `o` option supports the `size` option to set the maximum size of
    the created volume, the `inodes` option to set the maximum number of
    inodes for the volume, and `noquota` to completely disable quota
    support even for tracking of disk usage. The `size` option is
    supported on the \"tmpfs\" and \"xfs\[note\]\" file systems. The
    `inodes` option is supported on the \"xfs\[note\]\" file systems.
    Note: xfs filesystems must be mounted with the `prjquota` flag
    described in the **xfs_quota(8)** man page. Podman will throw an
    error if they\'re not.
-   The `o` option supports using volume options other than the UID/GID
    options with the **local** driver and requires root privileges.
-   The `o` options supports the `timeout` option which allows users to
    set a driver specific timeout in seconds before volume creation
    fails. For example, **\--opt=o=timeout=10** sets a driver timeout of
    10 seconds.

***Note*** Do not confuse the `--opt,-o` create option with the `-o`
mount option. For example, with `podman volume create`, use
`-o=o=uid=1000` *not* `-o=uid=1000`.

For the **image** driver, the only supported option is `image`, which
specifies the image the volume is based on. This option is mandatory
when using the **image** driver.

When not using the **local** and **image** drivers, the given options
are passed directly to the volume plugin. In this case, supported
options are dictated by the plugin in question, not Podman.

##  EXAMPLES

Create empty volume.

    $ podman volume create

Create empty named volume.

    $ podman volume create myvol

Create empty named volume with specified label.

    $ podman volume create --label foo=bar myvol

Create tmpfs named volume with specified size and mount options.

    # podman volume create --opt device=tmpfs --opt type=tmpfs --opt o=size=2M,nodev,noexec myvol

Create tmpfs named volume testvol with specified options.

    # podman volume create --opt device=tmpfs --opt type=tmpfs --opt o=uid=1000,gid=1000 testvol

Create image named volume using the specified local image in
containers/storage.

    # podman volume create --driver image --opt image=fedora:latest fedoraVol

##  QUOTAS

`podman volume create` uses `XFS project quota controls` for controlling
the size and the number of inodes of builtin volumes. The directory used
to store the volumes must be an `XFS` file system and be mounted with
the `pquota` option.

Example /etc/fstab entry:

    /dev/podman/podman-var /var xfs defaults,x-systemd.device-timeout=0,pquota 1 2

Podman generates project IDs for each builtin volume, but these project
IDs need to be unique for the XFS file system. These project IDs by
default are generated randomly, with a potential for overlap with other
quotas on the same file system.

The xfs_quota tool can be used to assign a project ID to the storage
driver directory, e.g.:

    echo 100000:/var/lib/containers/storage/overlay >> /etc/projects
    echo 200000:/var/lib/containers/storage/volumes >> /etc/projects
    echo storage:100000 >> /etc/projid
    echo volumes:200000 >> /etc/projid
    xfs_quota -x -c 'project -s storage volumes' /<xfs mount point>

In the example above we are configuring the overlay storage driver for
newly created containers as well as volumes to use project IDs with a
**start offset**. All containers are assigned larger project IDs (e.g.
\>= 100000). All volume assigned project IDs larger project IDs starting
with 200000. This prevents xfs_quota management conflicts with
containers/storage.

##  MOUNT EXAMPLES

`podman volume create` allows the `type`, `device`, and `o` options to
be passed to `mount(8)` when using the `local` driver.

##  [s3fs-fuse](https://github.com/s3fs-fuse/s3fs-fuse)

[s3fs-fuse](https://github.com/s3fs-fuse/s3fs-fuse) or just `s3fs`, is a
[fuse](https://github.com/libfuse/libfuse) filesystem that allows s3
prefixes to be mounted as filesystem mounts.

**Installing:**

``` shell
$ doas dnf install s3fs-fuse
```

**Simple usage:**

``` shell
$ s3fs --help
$ s3fs -o use_xattr,endpoint=aq-central-1 bucket:/prefix /mnt
```

**Equivalent through `mount(8)`**

``` shell
$ mount -t fuse.s3fs -o use_xattr,endpoint=aq-central-1 bucket:/prefix /mnt
```

**Equivalent through `podman volume create`**

``` shell
$ podman volume create s3fs-fuse-volume -o type=fuse.s3fs -o device=bucket:/prefix -o o=use_xattr,endpoint=aq-central-1
```

**The volume can then be mounted in a container with**

``` shell
$ podman run -v s3fs-fuse-volume:/s3:z --rm -it fedora:latest
```

Please see the available
[options](https://github.com/s3fs-fuse/s3fs-fuse/wiki/Fuse-Over-Amazon#options)
on their wiki.

### Using with other container users

The above example works because the volume is mounted as the host user
and inside the container `root` is mapped to the user in the host.

If the mount is accessed by a different user inside the container, a
\"Permission denied\" error will be raised.

``` shell
$ podman run --user bin:bin -v s3fs-fuse-volume:/s3:z,U --rm -it fedora:latest
$ ls /s3
# ls: /s3: Permission denied
```

In FUSE-land, mounts are protected for the user who mounted them;
specify the `allow_other` mount option if other users need access. \>
Note: This will remove the normal fuse [security
measures](https://github.com/libfuse/libfuse/wiki/FAQ#why-dont-other-users-have-access-to-the-mounted-filesystem)
on the mount point, after which, the normal filesystem permissions will
have to protect it

``` shell
$ podman volume create s3fs-fuse-other-volume -o type=fuse.s3fs -o device=bucket:/prefix -o o=allow_other,use_xattr,endpoint=aq-central-1
$ podman run --user bin:bin -v s3fs-fuse-volume:/s3:z,U --rm -it fedora:latest
$ ls /s3
```

### The Prefix must exist

`s3fs` will fail to mount if the prefix does not exist in the bucket.

Create a s3-directory by putting an empty object at the desired
`prefix/` key

``` shell
$ aws s3api put-object --bucket bucket --key prefix/
```

If performance is the priority, please check out the more performant
[goofys](https://github.com/kahing/goofys).

> FUSE filesystems exist for [Google Cloud
> Storage](https://github.com/GoogleCloudPlatform/gcsfuse) and [Azure
> Blob Storage](https://github.com/Azure/azure-storage-fuse)

##  SEE ALSO

**[podman(1)](podman.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**,
**[podman-volume(1)](podman-volume.html)**, **mount(8)**,
**xfs_quota(8)**, **xfs_quota(8)**, **projects(5)**, **projid(5)**

##  HISTORY

January 2020, updated with information on volume plugins by Matthew Heon
<mheon@redhat.com> November 2018, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-volume-exists'></a>

## podman-volume-exists - Check if the given volume exists

##  NAME

podman-volume-exists - Check if the given volume exists

##  SYNOPSIS

**podman volume exists** *volume*

##  DESCRIPTION

**podman volume exists** checks if a volume exists. Podman returns an
exit code of `0` when the volume is found. A `1` is returned otherwise.
An exit code of `125` indicates there was another issue.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

##  EXAMPLE

Check if a volume called `myvol` exists (the volume does actually
exist).

    $ podman volume exists myvol
    $ echo $?
    0
    $

Check if a volume called `mysql` exists (the volume does not actually
exist).

    $ podman volume exists mysql
    $ echo $?
    1
    $

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-volume(1)](podman-volume.html)**

##  HISTORY

January 2021, Originally compiled by Paul Holzinger
`<paul.holzinger@web.de>`


---

<a id='podman-volume-inspect'></a>

## podman-volume-inspect - Get detailed information on one or more
volumes

##  NAME

podman-volume-inspect - Get detailed information on one or more volumes

##  SYNOPSIS

**podman volume inspect** \[*options*\] *volume* \[\...\]

##  DESCRIPTION

Display detailed information on one or more volumes. The output can be
formatted using the **\--format** flag and a Go template. To get
detailed information about all the existing volumes, use the **\--all**
flag. Volumes can be queried individually by providing their full name
or a unique partial name.

##  OPTIONS

#### **\--all**, **-a**

Inspect all volumes.

#### **\--format**, **-f**=*format*

Format volume output using Go template

Valid placeholders for the Go template are listed below:

  --------------------------------------------------------------------------
  **Placeholder**   **Description**
  ----------------- --------------------------------------------------------
  .Anonymous        Indicates whether volume is anonymous

  .CreatedAt \...   Volume creation time

  .Driver           Volume driver

  .GID              GID the volume was created with

  .Labels \...      Label information associated with the volume

  .LockNumber       Number of the volume\'s Libpod lock

  .MountCount       Number of times the volume is mounted

  .Mountpoint       Source of volume mount point

  .Name             Volume name

  .NeedsChown       Indicates volume will be chowned on next use

  .NeedsCopyUp      Indicates data at the destination will be copied into
                    the volume on next use

  .Options \...     Volume options

  .Scope            Volume scope

  .Status \...      Status of the volume

  .StorageID        StorageID of the volume

  .Timeout          Timeout of the volume

  .UID              UID the volume was created with
  --------------------------------------------------------------------------

#### **\--help**

Print usage statement

##  EXAMPLES

Inspect named volume.

    $ podman volume inspect myvol
    [
         {
              "Name": "myvol",
              "Driver": "local",
              "Mountpoint": "/home/myusername/.local/share/containers/storage/volumes/myvol/_data",
              "CreatedAt": "2023-03-13T16:26:48.423069028-04:00",
              "Labels": {},
              "Scope": "local",
              "Options": {},
              "MountCount": 0,
              "NeedsCopyUp": true,
              "NeedsChown": true
         }
    ]

Inspect all volumes.

    $ podman volume inspect --all
    [
         {
              "Name": "myvol",
              "Driver": "local",
              "Mountpoint": "/home/myusername/.local/share/containers/storage/volumes/myvol/_data",
              "CreatedAt": "2023-03-13T16:26:48.423069028-04:00",
              "Labels": {},
              "Scope": "local",
              "Options": {},
              "MountCount": 0,
              "NeedsCopyUp": true,
              "NeedsChown": true
         }
    ]

Inspect named volume and display its Driver and Scope field.

    $ podman volume inspect --format "{{.Driver}} {{.Scope}}" myvol
    local local

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-volume(1)](podman-volume.html)**,
**[podman-inspect(1)](podman-inspect.html)**

##  HISTORY

November 2018, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-volume-ls'></a>

## podman-volume-ls - List all the available volumes

##  NAME

podman-volume-ls - List all the available volumes

##  SYNOPSIS

**podman volume ls** \[*options*\]

##  DESCRIPTION

Lists all the volumes that exist. The output can be filtered using the
**\--filter** flag and can be formatted to either JSON or a Go template
using the **\--format** flag. Use the **\--quiet** flag to print only
the volume names.

##  OPTIONS

#### **\--filter**, **-f**=*filter*

Filter what volumes are shown in the output. Multiple filters can be
given with multiple uses of the \--filter flag. Filters with the same
key work inclusive, with the only exception being `label` which is
exclusive. Filters with different keys always work exclusive.

Volumes can be filtered by the following attributes:

  -----------------------------------------------------------------------------
  **Filter**    **Description**
  ------------- ---------------------------------------------------------------
  dangling      \[Dangling\] Matches all volumes not referenced by any
                containers

  driver        \[Driver\] Matches volumes based on their driver

  label         \[Key\] or \[Key=Value\] Label assigned to a volume

  name          [Name](#name) Volume name (accepts regex)

  opt           Matches a storage driver options

  scope         Filters volume by scope

  after/since   Filter by volumes created after the given VOLUME (name or tag)

  until         Only remove volumes created before given timestamp
  -----------------------------------------------------------------------------

#### **\--format**=*format*

Format volume output using Go template.

Valid placeholders for the Go template are listed below:

  ----------------------------------------------------------------------
  **Placeholder**           **Description**
  ------------------------- --------------------------------------------
  .Anonymous                Indicates whether volume is anonymous

  .CreatedAt \...           Volume creation time

  .Driver                   Volume driver

  .GID                      GID of volume

  .InspectVolumeData \...   Don\'t use

  .Labels \...              Label information associated with the volume

  .LockNumber               Number of the volume\'s Libpod lock

  .MountCount               Number of times the volume is mounted

  .Mountpoint               Source of volume mount point

  .Name                     Volume name

  .NeedsChown               Indicates whether volume needs to be chowned

  .NeedsCopyUp              Indicates if volume needs to be copied up to

  .Options \...             Volume options

  .Scope                    Volume scope

  .Status \...              Status of the volume

  .StorageID                StorageID of the volume

  .Timeout                  Timeout of the volume

  .UID                      UID of volume

  .VolumeConfigResponse     Don\'t use
  \...                      
  ----------------------------------------------------------------------

#### **\--help**

Print usage statement.

#### **\--noheading**, **-n**

Omit the table headings from the listing.

#### **\--quiet**, **-q**

Print volume output in quiet mode. Only print the volume names.

##  EXAMPLES

List all volumes.

    $ podman volume ls

List all volumes and display content as json format.

    $ podman volume ls --format json

List all volumes and display their Driver and Scope fields

    $ podman volume ls --format "{{.Driver}} {{.Scope}}"

List volumes with the name foo and label blue.

    $ podman volume ls --filter name=foo,label=blue

List volumes with the label key=value.

    $ podman volume ls --filter label=key=value

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-volume(1)](podman-volume.html)**

##  HISTORY

November 2018, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-volume-prune'></a>

## podman-volume-prune - Remove all unused volumes

##  NAME

podman-volume-prune - Remove all unused volumes

##  SYNOPSIS

**podman volume prune** \[*options*\]

##  DESCRIPTION

Removes unused volumes. By default all unused volumes are removed, the
**\--filter** flag can be used to filter specific volumes. Users are
prompted to confirm the removal of all the unused volumes. To bypass the
confirmation, use the **\--force** flag.

##  OPTIONS

#### **\--filter**

Provide filter values.

The *filters* argument format is of `key=value`. If there is more than
one *filter*, then pass multiple OPTIONS: **\--filter** *foo=bar*
**\--filter** *bif=baz*.

Supported filters:

  -----------------------------------------------------------------------------
     Filter     Description
  ------------- ---------------------------------------------------------------
    dangling    \[Bool\] Only remove volumes not referenced by any containers

     driver     \[String\] Only remove volumes with the given driver

      label     \[String\] Only remove volumes, with (or without, in the case
                of label!=\[\...\] is used) the specified labels.

      name      \[String\] Only remove volume with the given name

       opt      \[String\] Only remove volumes created with the given options

      scope     \[String\] Only remove volumes with the given scope

      until     \[DateTime\] Only remove volumes created before given
                timestamp.

   after/since  \[Volume\] Filter by volumes created after the given VOLUME
                (name or tag)
  -----------------------------------------------------------------------------

The `label` *filter* accepts two formats. One is the `label`=*key* or
`label`=*key*=*value*, which removes volumes with the specified labels.
The other format is the `label!`=*key* or `label!`=*key*=*value*, which
removes volumes without the specified labels.

The `until` *filter* can be Unix timestamps, date formatted timestamps,
or Go duration strings (e.g. 10m, 1h30m) computed relative to the
machine's time.

#### **\--force**, **-f**

Do not prompt for confirmation.

#### **\--help**

Print usage statement

##  EXAMPLES

Prune all unused volumes.

    $ podman volume prune

Prune all volumes. Note: this command will also remove all containers
that are using a volume.

    $ podman volume prune --force

Prune all volumes that contain the specified label.

    $ podman volume prune --filter label=mylabel=mylabelvalue

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-volume(1)](podman-volume.html)**

##  HISTORY

November 2018, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

<a id='podman-volume-reload'></a>

## podman-volume-reload - Reload all volumes from volumes plugins

##  NAME

podman-volume-reload - Reload all volumes from volumes plugins

##  SYNOPSIS

**podman volume reload**

##  DESCRIPTION

**podman volume reload** checks all configured volume plugins and
updates the libpod database with all available volumes. Existing volumes
are also removed from the database when they are no longer present in
the plugin.

This command it is best effort and cannot guarantee a perfect state
because plugins can be modified from the outside at any time.

Note: This command is not supported with podman-remote.

##  EXAMPLES

Reload the volume plugins.

    $ podman volume reload
    Added:
    vol6
    Removed:
    t3

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-volume(1)](podman-volume.html)**


---

<a id='podman-volume-rm'></a>

## podman-volume-rm - Remove one or more volumes

##  NAME

podman-volume-rm - Remove one or more volumes

##  SYNOPSIS

**podman volume rm** \[*options*\] *volume* \[\...\]

##  DESCRIPTION

Removes one or more volumes. Only volumes that are not being used are
removed. If a volume is being used by a container, an error is returned
unless the **\--force** flag is being used. To remove all volumes, use
the **\--all** flag. Volumes can be removed individually by providing
their full name or a unique partial name.

##  OPTIONS

#### **\--all**, **-a**

Remove all volumes.

#### **\--force**, **-f**

Remove a volume by force. If it is being used by containers, the
containers are removed first.

#### **\--help**

Print usage statement

#### **\--time**, **-t**=*seconds*

Seconds to wait before forcibly stopping running containers that are
using the specified volume. The \--force option must be specified to use
the \--time option. Use -1 for infinite wait.

##  EXAMPLES

Remove multiple specified volumes.

    $ podman volume rm myvol1 myvol2

Remove all volumes.

    $ podman volume rm --all

Remove the specified volume even if it is in use. Note, this removes all
containers using the volume.

    $ podman volume rm --force myvol

##  Exit Status

**0** All specified volumes removed

**1** One of the specified volumes did not exist, and no other failures

**2** One of the specified volumes is being used by a container

**125** The command fails for any other reason

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-volume(1)](podman-volume.html)**

##  HISTORY

November 2018, Originally compiled by Urvashi Mohnani
<umohnani@redhat.com>


---

