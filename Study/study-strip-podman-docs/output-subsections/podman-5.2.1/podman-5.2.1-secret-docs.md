# podman-5.2.1 Secret Commands

*This document contains Secret commands from the Podman documentation.*

## Table of Contents

- [podman-secret-create - Create a new secret](#podman-secret-create)
- [podman-secret-exists - Check if the given secret exists](#podman-secret-exists)
- [podman-secret-inspect - Display detailed information on one or more secrets](#podman-secret-inspect)
- [podman-secret-ls - List all available secrets](#podman-secret-ls)
- [podman-secret-rm - Remove one or more secrets](#podman-secret-rm)

<a id='podman-secret-create'></a>

## podman-secret-create - Create a new secret

##  NAME

podman-secret-create - Create a new secret

##  SYNOPSIS

**podman secret create** \[*options*\] *name* *file\|-*

##  DESCRIPTION

Creates a secret using standard input or from a file for the secret
content.

Create accepts a path to a file, or `-`, which tells podman to read the
secret from stdin

A secret is a blob of sensitive data which a container needs at runtime
but is not stored in the image or in source control, such as usernames
and passwords, TLS certificates and keys, SSH keys or other important
generic strings or binary content (up to 500 kb in size).

Secrets are not committed to an image with `podman commit`, and does not
get committed in the archive created by a `podman export` command.

Secrets can also be used to store passwords for `podman login` to
authenticate against container registries.

##  OPTIONS

#### **\--driver**, **-d**=*driver*

Specify the secret driver (default **file**).

#### **\--driver-opts**=*key1=val1,key2=val2*

Specify driver specific options.

#### **\--env**=*false*

Read secret data from environment variable.

#### **\--help**

Print usage statement.

#### **\--label**, **-l**=*key=val1,key2=val2*

Add label to secret. These labels can be viewed in podman secrete
inspect or ls.

#### **\--replace**=*false*

If existing secret with the same name already exists, update the secret.
The `--replace` option does not change secrets within existing
containers, only newly created containers. The default is **false**.

##  SECRET DRIVERS

#### file

Secret resides in a read-protected file.

#### pass

Secret resides in a GPG-encrypted file.

#### shell

Secret is managed by custom scripts. An environment variable
**SECRET_ID** is passed to the scripts (except for **list**), and
secrets are communicated via stdin/stdout (where applicable). Driver
options **list**, **lookup**, **store**, and **delete** serve to install
the scripts:

    [secrets]
    driver = "shell"

    [secrets.opts]
    list =
    lookup =
    store =
    delete =

##  EXAMPLES

Create the specified secret based on local file.

    echo -n mysecret > ./secret.txt
    $ podman secret create my_secret ./secret.txt

Create the specified secret via stdin.

    $ printf <secret> | podman secret create my_secret -

Create gpg encrypted secret based on local file using the pass driver.

    $ podman secret create --driver=pass my_secret ./secret.txt.gpg

Create a secret from an environment variable called \'MYSECRET\'.

    $ podman secret create --env=true my_secret MYSECRET

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-secret(1)](podman-secret.html)**,
**[podman-login(1)](podman-login.html)**

##  HISTORY

January 2021, Originally compiled by Ashley Cui <acui@redhat.com>
February 2024, Added example showing secret creation from an environment
variable by Brett Calliss <brett@obligatory.email>


---

<a id='podman-secret-exists'></a>

## podman-secret-exists - Check if the given secret exists

##  NAME

podman-secret-exists - Check if the given secret exists

##  SYNOPSIS

**podman secret exists** *secret*

##  DESCRIPTION

**podman secret exists** checks if a secret exists. Podman returns an
exit code of `0` when the secret is found. A `1` is returned otherwise.
An exit code of `125` indicates there was another issue.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

##  EXAMPLE

Check if a secret called `mysecret` exists (the secret does actually
exist).

    $ podman secret exists mysecret
    $ echo $?
    0
    $

Check if a secret called `mypassword` exists (the secret does not
actually exist).

    $ podman secret exists mypassword
    $ echo $?
    1
    $

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-secret(1)](podman-secret.html)**

##  HISTORY

April 2023, Originally compiled by Ygal Blum `<ygal.blum@gmail.com>`


---

<a id='podman-secret-inspect'></a>

## podman-secret-inspect - Display detailed information on one or more secrets

##  NAME

podman-secret-inspect - Display detailed information on one or more
secrets

##  SYNOPSIS

**podman secret inspect** \[*options*\] *secret* \[\...\]

##  DESCRIPTION

Inspects the specified secret.

By default, this renders all results in a JSON array. If a format is
specified, the given template is executed for each result. Secrets can
be queried individually by providing their full name or a unique partial
name.

##  OPTIONS

#### **\--format**, **-f**=*format*

Format secret output using Go template.

  --------------------------------------------------------------------------
  **Placeholder**        **Description**
  ---------------------- ---------------------------------------------------
  .CreatedAt \...        When secret was created (relative timestamp,
                         human-readable)

  .ID                    ID of secret

  .SecretData            Secret Data (Displayed only with \--showsecret
                         option)

  .Spec \...             Details of secret

  .Spec.Driver \...      Driver info

  .Spec.Driver.Name      Driver name (string)

  .Spec.Driver.Options   Driver options (map of driver-specific options)
  \...                   

  .Spec.Labels \...      Labels for this secret

  .Spec.Name             Name of secret

  .UpdatedAt \...        When secret was last updated (relative timestamp,
                         human-readable)
  --------------------------------------------------------------------------

#### **\--help**

Print usage statement.

#### **\--pretty**

Print inspect output in human-readable format

#### **\--showsecret**

Display secret data

##  EXAMPLES

Inspect the secret mysecret.

    $ podman secret inspect mysecret

Inspect the secret mysecret and display the Name and Scope field.

    $ podman secret inspect --format "{{.Name} {{.Scope}}" mysecret

Inspect the secret mysecret and display the Name and SecretData fields.
Note this will display the secret data to the screen.

    $ podman secret inspect --showsecret --format "{{.Name} {{.SecretData}}" mysecret

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-secret(1)](podman-secret.html)**

##  HISTORY

January 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-secret-ls'></a>

## podman-secret-ls - List all available secrets

##  NAME

podman-secret-ls - List all available secrets

##  SYNOPSIS

**podman secret ls** \[*options*\]

##  DESCRIPTION

Lists all the secrets that exist. The output can be formatted to a Go
template using the **\--format** option.

##  OPTIONS

#### **\--filter**, **-f**=*filter=value*

Filter output based on conditions given. Multiple filters can be given
with multiple uses of the \--filter option.

Valid filters are listed below:

  **Filter**   **Description**
  ------------ -------------------------------------------
  name         [Name](#name) Secret name (accepts regex)
  id           \[ID\] Full or partial secret ID

#### **\--format**=*format*

Format secret output using Go template.

Valid placeholders for the Go template are listed below:

  --------------------------------------------------------------------------
  **Placeholder**        **Description**
  ---------------------- ---------------------------------------------------
  .CreatedAt \...        When secret was created (relative timestamp,
                         human-readable)

  .ID                    ID of secret

  .SecretData            Secret Data (Displayed only with \--showsecret
                         option)

  .Spec \...             Details of secret

  .Spec.Driver \...      Driver info

  .Spec.Driver.Name      Driver name (string)

  .Spec.Driver.Options   Driver options (map of driver-specific options)
  \...                   

  .Spec.Labels \...      Labels for this secret

  .Spec.Name             Name of secret

  .UpdatedAt \...        When secret was last updated (relative timestamp,
                         human-readable)
  --------------------------------------------------------------------------

#### **\--noheading**, **-n**

Omit the table headings from the listing.

#### **\--quiet**, **-q**

Print secret IDs only.

##  EXAMPLES

List all secrets.

    $ podman secret ls

List the name field of all secrets.

    $ podman secret ls --format "{{.Name}}"

List all secrets whose name includes the specified string.

    $ podman secret ls --filter name=confidential

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-secret(1)](podman-secret.html)**

##  HISTORY

January 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

<a id='podman-secret-rm'></a>

## podman-secret-rm - Remove one or more secrets

##  NAME

podman-secret-rm - Remove one or more secrets

##  SYNOPSIS

**podman secret rm** \[*options*\] *secret* \[\...\]

##  DESCRIPTION

Removes one or more secrets.

`podman secret rm` is safe to use on secrets that are in use by a
container. The created container has access to the secret data because
secrets are copied and mounted into the container when a container is
created. If a secret is deleted and another secret is created with the
same name, the secret inside the container does not change; the old
secret value still remains.

##  OPTIONS

#### **\--all**, **-a**

Remove all existing secrets.

#### **\--help**

Print usage statement.

#### **\--ignore**, **-i**

Ignore errors when specified secrets are not present.

##  EXAMPLES

Remove secrets mysecret1 and mysecret2.

    $ podman secret rm mysecret1 mysecret2

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-secret(1)](podman-secret.html)**

##  HISTORY

January 2021, Originally compiled by Ashley Cui <acui@redhat.com>


---

