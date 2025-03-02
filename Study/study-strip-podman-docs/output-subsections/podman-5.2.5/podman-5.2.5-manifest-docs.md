# podman-5.2.5 Manifest Commands

*This document contains Manifest commands from the Podman documentation.*

## Table of Contents

- [podman-manifest-add - Add an image or artifact to a manifest list or image index](#podman-manifest-add)
- [podman-manifest-annotate - Add and update information about an image or artifact in a manifest list or image index](#podman-manifest-annotate)
- [podman-manifest-create - Create a manifest list or image index](#podman-manifest-create)
- [podman-manifest-exists - Check if the given manifest list exists in local storage](#podman-manifest-exists)
- [podman-manifest-inspect - Display a manifest list or image index](#podman-manifest-inspect)
- [podman-manifest-push - Push a manifest list or image index to a registry](#podman-manifest-push)
- [podman-manifest-remove - Remove an image from a manifest list or image index](#podman-manifest-remove)
- [podman-manifest-rm - Remove manifest list or image index from local storage](#podman-manifest-rm)

<a id='podman-manifest-add'></a>

## podman-manifest-add - Add an image or artifact to a manifest list or image index

##  NAME

podman-manifest-add - Add an image or artifact to a manifest list or
image index

##  SYNOPSIS

**podman manifest add** \[*options*\] *listnameorindexname*
\[*transport*\]:*imagename* *imageorartifactname* \[\...\]

##  DESCRIPTION

Adds the specified image to the specified manifest list or image index,
or creates an artifact manifest and adds it to the specified image
index.

##  RETURN VALUE

The list image\'s ID.

##  OPTIONS

#### **\--all**

If the image which is added to the list or index is itself a list or
index, add all of the contents to the local list. By default, only one
image from such a list or index is added to the list or index. Combining
*\--all* with any of the other options described below is NOT
recommended.

#### **\--annotation**=*annotation=value*

Set an annotation on the entry for the specified image or artifact.

#### **\--arch**=*architecture*

Override the architecture which the list or index records as a
requirement for the image. If *imageName* refers to a manifest list or
image index, the architecture information is retrieved from it.
Otherwise, it is retrieved from the image\'s configuration information.

#### **\--artifact**

Create an artifact manifest and add it to the image index. Arguments
after the index name will be interpreted as file names rather than as
image references. In most scenarios, the **\--artifact-type** option
should also be specified.

#### **\--artifact-config**=*path*

When creating an artifact manifest and adding it to the image index, use
the specified file\'s contents as the configuration blob in the artifact
manifest. In most scenarios, leaving the default value, which signifies
an empty configuration, unchanged, is the preferred option.

#### **\--artifact-config-type**=*type*

When creating an artifact manifest and adding it to the image index, use
the specified MIME type as the `mediaType` associated with the
configuration blob in the artifact manifest. In most scenarios, leaving
the default value, which signifies either an empty configuration or the
standard OCI configuration type, unchanged, is the preferred option.

#### **\--artifact-exclude-titles**

When creating an artifact manifest and adding it to the image index, do
not set \"org.opencontainers.image.title\" annotations equal to the
file\'s basename for each file added to the artifact manifest. Tools
which retrieve artifacts from a registry may use these values to choose
names for files when saving artifacts to disk, so this option is not
recommended unless it is required for interoperability with a particular
registry.

#### **\--artifact-layer-type**=*type*

When creating an artifact manifest and adding it to the image index, use
the specified MIME type as the `mediaType` associated with the files\'
contents. If not specified, guesses based on either the files names or
their contents will be made and used, but the option should be specified
if certainty is needed.

#### **\--artifact-subject**=*imageName*

When creating an artifact manifest and adding it to the image index, set
the *subject* field in the artifact manifest to mark the artifact
manifest as being associated with the specified image in some way. An
artifact manifest can only be associated with, at most, one subject.

#### **\--artifact-type**=*type*

When creating an artifact manifest, use the specified MIME type as the
manifest\'s `artifactType` value instead of the less informative default
value.

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

#### **\--features**=*feature*

Specify the features list which the list or index records as
requirements for the image. This option is rarely used.

#### **\--os**=*OS*

Override the OS which the list or index records as a requirement for the
image. If *imagename* refers to a manifest list or image index, the OS
information is retrieved from it. Otherwise, it is retrieved from the
image\'s configuration information.

#### **\--os-version**=*version*

Specify the OS version which the list or index records as a requirement
for the image. This option is rarely used.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

#### **\--variant**

Specify the variant which the list or index records for the image. This
option is typically used to distinguish between multiple entries which
share the same architecture value, but which expect different versions
of its instruction set.

##  Transport

Multiple transports are supported:

**docker://**\_docker-reference\_ *(default)* An image in a registry
implementing the \"Docker Registry HTTP API V2\". By default, uses the
authorization state in `$XDG_RUNTIME_DIR/containers/auth.json`, which is
set using `(podman login)`. If the authorization state is not found
there, `$HOME/.docker/config.json` is checked, which is set using
`(docker login)`.

    $ podman manifest add mylist:v1.11 docker://quay.io/username/myimage

**containers-storage:**\_oci-reference\_ An image in *oci-reference*
format stored in the local container storage. *oci-reference* must
contain a tag.

    $ podman manifest add mylist:v1.11 containers-storage:quay.io/username/myimage

**dir:**\_path\_ An existing local directory *path* storing the
manifest, layer tarballs, and signatures as individual files. This is a
non-standardized format, primarily useful for debugging or noninvasive
container inspection.

    $ podman manifest add dir:/tmp/myimage

**docker-archive:**\_path\_\[**:**\_docker-reference\_\] An image is
stored in the `docker save` formatted file. *docker-reference* is only
used when creating such a file, and it must not contain a digest.

    $ podman manifest add docker-archive:/tmp/myimage

**docker-daemon:**\_docker-reference\_ An image in *docker-reference*
format stored in the docker daemon internal storage. The
*docker-reference* can also be an image ID (docker-daemon:algo:digest).

    $ sudo podman manifest add docker-daemon:docker.io/library/myimage:33

**oci-archive:**\_path\_**:**\_tag\_ An image *tag* in a directory
compliant with \"Open Container Image Layout Specification\" at *path*.

    $ podman manifest add oci-archive:/tmp/myimage

##  EXAMPLE

Add specified default image from source manifest list to destination
manifest list:

    podman manifest add mylist:v1.11 docker://fedora
    71c201d10fffdcac52968a000d85a0a016ca1c7d5473948000d3131c1773d965

Add all images from source manfest list to destination manifest list:

    podman manifest add --all mylist:v1.11 docker://fedora
    71c201d10fffdcac52968a000d85a0a016ca1c7d5473948000d3131c1773d965

Add selected image matching arch and variant from source manifest list
to destination manifest list:

    podman manifest add --arch arm64 --variant v8 mylist:v1.11 docker://71c201d10fffdcac52968a000d85a0a016ca1c7d5473948000d3131c1773d965

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**


---

<a id='podman-manifest-annotate'></a>

## podman-manifest-annotate - Add and update information about an image or artifact in a manifest list or image index

##  NAME

podman-manifest-annotate - Add and update information about an image or
artifact in a manifest list or image index

##  SYNOPSIS

**podman manifest annotate** \[*options*\] *listnameorindexname*
*imagemanifestdigestorimageorartifactname*

##  DESCRIPTION

Adds or updates information about an image or artifact included in a
manifest list or image index.

##  OPTIONS

#### **\--annotation**=*annotation=value*

Set an annotation on the entry for the specified image or artifact.

If **\--index** is also specified, sets the annotation on the entire
image index.

#### **\--arch**=*architecture*

Override the architecture which the list or index records as a
requirement for the image. This is usually automatically retrieved from
the image\'s configuration information, so it is rarely necessary to use
this option.

#### **\--features**=*feature*

Specify the features list which the list or index records as
requirements for the image. This option is rarely used.

#### **\--index**

Treats arguments to the **\--annotation** option as annotation values to
be set on the image index itself rather than on an entry in the image
index. Implied for **\--subject**.

#### **\--os**=*OS*

Override the OS which the list or index records as a requirement for the
image. This is usually automatically retrieved from the image\'s
configuration information, so it is rarely necessary to use this option.

#### **\--os-features**=*feature*

Specify the OS features list which the list or index records as
requirements for the image. This option is rarely used.

#### **\--os-version**=*version*

Specify the OS version which the list or index records as a requirement
for the image. This option is rarely used.

#### **\--subject**=*imageName*

Set the *subject* field in the image index to mark the image index as
being associated with the specified image in some way. An image index
can only be associated with, at most, one subject.

#### **\--variant**

Specify the variant which the list or index records for the image. This
option is typically used to distinguish between multiple entries which
share the same architecture value, but which expect different versions
of its instruction set.

##  EXAMPLE

Update arch and variant information to specified manifest list for
image:

    podman manifest annotate --arch arm64 --variant v8 mylist:v1.11 sha256:59eec8837a4d942cc19a52b8c09ea75121acc38114a2c68b98983ce9356b8610
    07ec8dc22b5dba3a33c60b68bce28bbd2b905e383fdb32a90708fa5eeac13a07: sha256:59eec8837a4d942cc19a52b8c09ea75121acc38114a2c68b98983ce9356b8610

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**


---

<a id='podman-manifest-create'></a>

## podman-manifest-create - Create a manifest list or image index

##  NAME

podman-manifest-create - Create a manifest list or image index

##  SYNOPSIS

**podman manifest create** \[*options*\] *listnameorindexname*
\[*imagename* \...\]

##  DESCRIPTION

Creates a new manifest list and stores it as an image in local storage
using the specified name.

If additional images are specified, they are added to the newly-created
list or index.

##  OPTIONS

#### **\--all**

If any of the images added to the new list or index are themselves lists
or indexes, add all of their contents. By default, only one image from
such a list is added to the newly-created list or index.

#### **\--amend**, **-a**

If a manifest list named *listnameorindexname* already exists, modify
the preexisting list instead of exiting with an error. The contents of
*listnameorindexname* are not modified if no *imagename*s are given.

#### **\--annotation**=*value*

Set an annotation on the newly-created image index.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

##  EXAMPLES

Create the specified manifest.

    podman manifest create mylist:v1.11
    9cfd24048d5fc80903f088f1531a21bff01172abe66effa8941a4c2308dc745f

Create the specified manifest manifest or modify it if it previously
exist.

    podman manifest create --amend mylist:v1.11
    9cfd24048d5fc80903f088f1531a21bff01172abe66effa8941a4c2308dc745f

Create the named manifest including the specified image matching the
current platform.

    podman manifest create mylist:v1.11 docker://fedora
    5c2bc76bfb4ba6665a7973f7e1c05ee0536b4580637f27adc9fa5a4b2bc03cf1

Create the named manifest including all images referred to with the
specified image reference.

    podman manifest create --all mylist:v1.11 docker://fedora
    30330571e79c65288a4fca421d9aed29b0210d57294d9c2056743fdcf6e3967b

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**


---

<a id='podman-manifest-exists'></a>

## podman-manifest-exists - Check if the given manifest list exists in local storage

##  NAME

podman-manifest-exists - Check if the given manifest list exists in
local storage

##  SYNOPSIS

**podman manifest exists** *manifest*

##  DESCRIPTION

**podman manifest exists** checks if a manifest list exists on local
storage. Podman returns an exit code of `0` when the manifest is found.
A `1` is returned otherwise. An exit code of `125` indicates there was
another issue.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement.

##  EXAMPLE

Check if a manifest list called `list1` exists (the manifest list does
actually exist):

    $ podman manifest exists list1
    $ echo $?
    0

Check if a manifest called `mylist` exists (the manifest list does not
actually exist):

    $ podman manifest exists mylist
    $ echo $?
    1

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**

##  HISTORY

January 2021, Originally compiled by Paul Holzinger
`<paul.holzinger@web.de>`


---

<a id='podman-manifest-inspect'></a>

## podman-manifest-inspect - Display a manifest list or image index

##  NAME

podman-manifest-inspect - Display a manifest list or image index

##  SYNOPSIS

**podman manifest inspect** \[*options*\] *listnameorindexname*

##  DESCRIPTION

Displays the manifest list or image index stored using the specified
image name. \## RETURN VALUE

A formatted JSON representation of the manifest list or image index.

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

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

##  EXAMPLES

Inspect the manifest of mylist:v1.11.

    podman manifest inspect mylist:v1.11

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**


---

<a id='podman-manifest-push'></a>

## podman-manifest-push - Push a manifest list or image index to a registry

##  NAME

podman-manifest-push - Push a manifest list or image index to a registry

##  SYNOPSIS

**podman manifest push** \[*options*\] *listnameorindexname*
\[*destination*\]

##  DESCRIPTION

Pushes a manifest list or image index to a registry.

##  RETURN VALUE

The list image\'s ID and the digest of the image\'s manifest.

##  OPTIONS

#### **\--add-compression**=*compression*

Makes sure that requested compression variant for each platform is added
to the manifest list keeping original instance intact in the same
manifest list. Supported values are (`gzip`, `zstd` and `zstd:chunked`).
Following flag can be used multiple times.

Note that `--compression-format` controls the compression format of each
instance in the manifest list. `--add-compression` will add another
variant for each instance in the list with the specified compressions.
`--compression-format` gzip `--add-compression` zstd will push a
manifest list with each instance being compressed with gzip plus an
additional variant of each instance being compressed with zstd.

#### **\--all**

Push the images mentioned in the manifest list or image index, in
addition to the list or index itself. (Default true)

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

#### **\--force-compression**

If set, push uses the specified compression algorithm even if the
destination contains a differently-compressed variant already. Defaults
to `true` if `--compression-format` is explicitly specified on the
command-line, `false` otherwise.

#### **\--format**, **-f**=*format*

Manifest list type (oci or v2s2) to use when pushing the list (default
is oci).

#### **\--quiet**, **-q**

When writing the manifest, suppress progress output

#### **\--remove-signatures**

Don\'t copy signatures when pushing images.

#### **\--rm**

Delete the manifest list or image index from local storage if pushing
succeeds.

#### **\--sign-by**=*fingerprint*

Sign the pushed images with a "simple signing" signature using the
specified key. (This option is not available with the remote Podman
client, including Mac and Windows (excluding WSL2) machines)

#### **\--sign-by-sigstore**=*param-file*

Add a sigstore signature based on further options specified in a
container\'s sigstore signing parameter file *param-file*. See
containers-sigstore-signing-params.yaml(5) for details about the file
format.

#### **\--sign-by-sigstore-private-key**=*path*

Sign the pushed images with a sigstore signature using a private key at
the specified path. (This option is not available with the remote Podman
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

##  DESTINATION

DESTINATION is the location the container image is pushed to. It
supports all transports from `containers-transports(5)`. If no transport
is specified, the `docker` (i.e., container registry) transport is used.
For remote clients, including Mac and Windows (excluding WSL2) machines,
`docker` is the only supported transport.

##  EXAMPLE

Push manifest list to container registry:

    podman manifest push mylist:v1.11 docker://registry.example.org/mylist:v1.11

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**,
**[containers-transports(5)](https://github.com/containers/image/blob/main/docs/containers-transports.5.md)**


---

<a id='podman-manifest-remove'></a>

## podman-manifest-remove - Remove an image from a manifest list or image index

##  NAME

podman-manifest-remove - Remove an image from a manifest list or image
index

##  SYNOPSIS

**podman manifest remove** *listnameorindexname* *transport:details*

##  DESCRIPTION

Removes the image with the specified digest from the specified manifest
list or image index.

##  RETURN VALUE

The list image\'s ID and the digest of the removed image\'s manifest.

##  EXAMPLE

Remove specified digest from the manifest list:

    podman manifest remove mylist:v1.11 sha256:cb8a924afdf0229ef7515d9e5b3024e23b3eb03ddbba287f4a19c6ac90b8d221
    e604eabaaee4858232761b4fef84e2316ed8f93e15eceafce845966ee3400036 :sha256:cb8a924afdf0229ef7515d9e5b3024e23b3eb03ddbba287f4a19c6ac90b8d221

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**


---

<a id='podman-manifest-rm'></a>

## podman-manifest-rm - Remove manifest list or image index from local storage

##  NAME

podman-manifest-rm - Remove manifest list or image index from local
storage

##  SYNOPSIS

**podman manifest rm** *list-or-index* \[\...\]

##  DESCRIPTION

Removes one or more locally stored manifest lists.

##  EXAMPLE

podman manifest rm `<list>`

podman manifest rm listid1 listid2

**storage.conf** (`/etc/containers/storage.conf`)

storage.conf is the storage configuration file for all tools using
containers/storage

The storage configuration file specifies all of the available container
storage options for tools using shared container storage.

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-manifest(1)](podman-manifest.html)**,
**[containers-storage.conf(5)](https://github.com/containers/storage/blob/main/docs/containers-storage.conf.5.md)**


---

