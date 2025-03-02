# podman-5.4.0 Kube Commands

*This document contains Kube commands from the Podman documentation.*

## Table of Contents

- [podman-kube-apply - Apply Kubernetes YAML based on containers, pods,
or volumes to a Kubernetes cluster](#podman-kube-apply)
- [podman-kube-down - Remove containers and pods based on Kubernetes
YAML](#podman-kube-down)
- [podman-kube-generate - Generate Kubernetes YAML based on containers,
pods or volumes](#podman-kube-generate)
- [podman-kube-play - Create containers, pods and volumes based on
Kubernetes YAML](#podman-kube-play)

<a id='podman-kube-apply'></a>

## podman-kube-apply - Apply Kubernetes YAML based on containers, pods,
or volumes to a Kubernetes cluster

##  NAME

podman-kube-apply - Apply Kubernetes YAML based on containers, pods, or
volumes to a Kubernetes cluster

##  SYNOPSIS

**podman kube apply** \[*options*\] \[*container\...* \| *pod\...* \|
*volume\...*\]

##  DESCRIPTION

**podman kube apply** deploys a podman container, pod, or volume to a
Kubernetes cluster. Use the `--file` option to deploy a Kubernetes YAML
(v1 specification) to a Kubernetes cluster as well.

Note that the Kubernetes YAML file can be used to run the deployment in
Podman via podman-play-kube(1).

##  OPTIONS

#### **\--ca-cert-file**=*ca cert file path \| \"insecure\"*

The path to the CA cert file for the Kubernetes cluster. Usually the
kubeconfig has the CA cert file data and `generate kube` automatically
picks that up if it is available in the kubeconfig. If no CA cert file
data is available, set this to `insecure` to bypass the certificate
verification.

#### **\--file**, **-f**=*kube yaml filepath*

Path to the kubernetes yaml file to deploy onto the kubernetes cluster.
This file can be generated using the `podman kube generate` command. The
input may be in the form of a yaml file, or stdin. For stdin, use
`--file=-`.

#### **\--kubeconfig**, **-k**=*kubeconfig filepath*

Path to the kubeconfig file to be used when deploying the generated kube
yaml to the Kubernetes cluster. The environment variable `KUBECONFIG`
can be used to set the path for the kubeconfig file as well. Note: A
kubeconfig can have multiple cluster configurations, but `kube generate`
always picks the first cluster configuration in the given kubeconfig.

#### **\--ns**=*namespace*

The namespace or project to deploy the workloads of the generated kube
yaml to in the Kubernetes cluster.

#### **\--service**, **-s**

Used to create a service for the corresponding container or pod being
deployed to the cluster. In particular, if the container or pod has
portmap bindings, the service specification includes a NodePort
declaration to expose the service. A random port is assigned by Podman
in the service specification that is deployed to the cluster.

##  EXAMPLES

Apply a podman volume and container to the \"default\" namespace in a
Kubernetes cluster.

    $ podman kube apply --kubeconfig /tmp/kubeconfig myvol vol-test-1
    Deploying to cluster...
    Successfully deployed workloads to cluster!
    $ kubectl get pods
    NAME             READY   STATUS    RESTARTS   AGE
    vol-test-1-pod   1/1     Running   0          9m

Apply a Kubernetes YAML file to the \"default\" namespace in a
Kubernetes cluster.

    $ podman kube apply --kubeconfig /tmp/kubeconfig -f vol.yaml
    Deploying to cluster...
    Successfully deployed workloads to cluster!
    $ kubectl get pods
    NAME             READY   STATUS    RESTARTS   AGE
    vol-test-2-pod   1/1     Running   0          9m

Apply a Kubernetes YAML file to the \"test1\" namespace in a Kubernetes
cluster.

    $ podman kube apply --kubeconfig /tmp/kubeconfig --ns test1 vol-test-3
    Deploying to cluster...
    Successfully deployed workloads to cluster!
    $ kubectl get pods --namespace test1
    NAME             READY   STATUS    RESTARTS   AGE
    vol-test-3-pod   1/1     Running   0          9m

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-container(1)](podman-container.html)**,
**[podman-pod(1)](podman-pod.html)**,
**[podman-kube-play(1)](podman-kube-play.html)**,
**[podman-kube-generate(1)](podman-kube-generate.html)**

##  HISTORY

September 2022, Originally compiled by Urvashi Mohnani (umohnani at
redhat dot com)


---

<a id='podman-kube-down'></a>

## podman-kube-down - Remove containers and pods based on Kubernetes
YAML

##  NAME

podman-kube-down - Remove containers and pods based on Kubernetes YAML

##  SYNOPSIS

**podman kube down** \[*options*\]
*file.yml\|-\|https://website.io/file.yml*

##  DESCRIPTION

**podman kube down** reads a specified Kubernetes YAML file, tearing
down pods that were created by the `podman kube play` command via the
same Kubernetes YAML file. Any volumes that were created by the previous
`podman kube play` command remain intact unless the `--force` options is
used. If the YAML file is specified as `-`, `podman kube down` reads the
YAML from stdin. The input can also be a URL that points to a YAML file
such as https://podman.io/demo.yml. `podman kube down` tears down the
pods and containers created by `podman kube play` via the same
Kubernetes YAML from the URL. However, `podman kube down` does not work
with a URL if the YAML file the URL points to has been changed or
altered since the creation of the pods and containers using
`podman kube play`.

##  OPTIONS

#### **\--force**

Tear down the volumes linked to the PersistentVolumeClaims as part
\--down

##  EXAMPLES

Example YAML file `demo.yml`:

    apiVersion: v1
    kind: Pod
    metadata:
    ...
    spec:
      containers:
      - command:
        - top
        - name: container
          value: podman
        image: foobar
    ...

Remove the pod and containers as described in the `demo.yml` file

    $ podman kube down demo.yml
    Pods stopped:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6
    Pods removed:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

Remove the pod and containers as described in the `demo.yml` file YAML
sent to stdin

    $ cat demo.yml | podman kube play -
    Pods stopped:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6
    Pods removed:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

Remove the pods and containers as described in the `demo.yml` file YAML
read from a URL

    $ podman kube down https://podman.io/demo.yml
    Pods stopped:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6
    Pods removed:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

`podman kube down` does not work with a URL if the YAML file the URL
points to has been changed or altered since it was used to create the
pods and containers.

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-kube(1)](podman-kube.html)**,
**[podman-kube-play(1)](podman-kube-play.html)**,
**[podman-kube-generate(1)](podman-kube-generate.html)**,
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**


---

<a id='podman-kube-generate'></a>

## podman-kube-generate - Generate Kubernetes YAML based on containers,
pods or volumes

##  NAME

podman-kube-generate - Generate Kubernetes YAML based on containers,
pods or volumes

##  SYNOPSIS

**podman kube generate** \[*options*\] *container\...* \| *pod\...* \|
*volume\...*

##  DESCRIPTION

**podman kube generate** generates Kubernetes YAML (v1 specification)
from Podman containers, pods or volumes. Regardless of whether the input
is for containers or pods, Podman generates the specification as a Pod
by default. The input may be in the form of one or more containers, pods
or volumes names or IDs.

`Podman Containers or Pods`

Volumes appear in the generated YAML according to two different volume
types. Bind-mounted volumes become *hostPath* volume types and named
volumes become *persistentVolumeClaim* volume types. Generated
*hostPath* volume types are one of three subtypes depending on the state
of the host path: *DirectoryOrCreate* when no file or directory exists
at the host, *Directory* when host path is a directory, or *File* when
host path is a file. The value for *claimName* for a
*persistentVolumeClaim* is the name of the named volume registered in
Podman.

Potential name conflicts between volumes are avoided by using a standard
naming scheme for each volume type. The *hostPath* volume types are
named according to the path on the host machine, replacing forward
slashes with hyphens less any leading and trailing forward slashes. The
special case of the filesystem root, `/`, translates to the name `root`.
Additionally, the name is suffixed with `-host` to avoid naming
conflicts with *persistentVolumeClaim* volumes. Each
*persistentVolumeClaim* volume type uses the name of its associated
named volume suffixed with `-pvc`.

Note that if an init container is created with type `once` and the pod
has been started, it does not show up in the generated kube YAML as
`once` type init containers are deleted after they are run. If the pod
has only been created and not started, it is in the generated kube YAML.
Init containers created with type `always` are always generated in the
kube YAML as they are never deleted, even after running to completion.

*Note*: When using volumes and generating a Kubernetes YAML for an
unprivileged and rootless podman container on an **SELinux enabled
system**, one of the following options must be completed: \* Add the
\"privileged: true\" option to the pod spec \* Add `type: spc_t` under
the `securityContext` `seLinuxOptions` in the pod spec \* Relabel the
volume via the CLI command `chcon -t container_file_t -R <directory>`

Once completed, the correct permissions are in place to access the
volume when the pod/container is created in a Kubernetes cluster.

Note that the generated Kubernetes YAML file can be used to re-run the
deployment via podman-play-kube(1).

Note that if the pod being generated was created with the
**\--infra-name** flag set, then the generated kube yaml will have the
**io.podman.annotations.infra.name** set where the value is the name of
the infra container set by the user.

Note that both Deployment and DaemonSet can only have `restartPolicy`
set to `Always`.

Note that Job can only have `restartPolicy` set to `OnFailure` or
`Never`. By default, podman sets it to `Never` when generating a kube
yaml using `kube generate`.

##  OPTIONS

#### **\--filename**, **-f**=*filename*

Output to the given file instead of STDOUT. If the file already exists,
`kube generate` refuses to replace it and returns an error.

#### **\--podman-only**

Add podman-only reserved annotations in generated YAML file (Cannot be
used by Kubernetes)

#### **\--replicas**, **-r**=*replica count*

The value to set `replicas` to when generating a **Deployment** kind.
Note: this can only be set with the option `--type=deployment`.

#### **\--service**, **-s**

Generate a Kubernetes service object in addition to the Pods. Used to
generate a Service specification for the corresponding Pod output. In
particular, if the object has portmap bindings, the service
specification includes a NodePort declaration to expose the service. A
random port is assigned by Podman in the specification.

#### **\--type**, **-t**=*pod* \| *deployment* \| *daemonset* \| *job*

The Kubernetes kind to generate in the YAML file. Currently, the only
supported Kubernetes specifications are `Pod`, `Deployment`, `Job`, and
`DaemonSet`. By default, the `Pod` specification is generated.

##  EXAMPLES

Create Kubernetes Pod YAML for the specified container.

    $ podman kube generate some-mariadb
    # Save the output of this file and use kubectl create -f to import
    # it into Kubernetes.
    #
    # Created with podman-4.8.2

    # NOTE: If you generated this yaml from an unprivileged and rootless podman container on an SELinux
    # enabled system, check the podman generate kube man page for steps to follow to ensure that your pod/container
    # has the right permissions to access the volumes added.
    ---
    apiVersion: v1
    kind: Pod
    metadata:
      creationTimestamp: "2024-01-09T02:24:55Z"
      labels:
        app: some-mariadb-pod
      name: some-mariadb-pod
    spec:
      containers:
      - args:
        - mariadbd
        env:
        - name: MARIADB_ROOT_PASSWORD
          value: x
        image: docker.io/library/mariadb:10.11
        name: some-mariadb
        ports:
        - containerPort: 3306
          hostPort: 34891
        volumeMounts:
        - mountPath: /var/lib/mysql
          name: mariadb_data-pvc
      volumes:
      - name: mariadb_data-pvc
        persistentVolumeClaim:
          claimName: mariadb_data

Create Kubernetes Deployment YAML with 3 replicas for the specified
container.

    $ podman kube generate --type deployment --replicas 3 dep-ct
    r
    # Save the output of this file and use kubectl create -f to import
    # it into Kubernetes.
    #
    # Created with podman-4.5.0-dev
    apiVersion: apps/v1
    kind: Deployment
    metadata:
      creationTimestamp: "2023-03-27T20:45:08Z"
      labels:
        app: dep-ctr-pod
      name: dep-ctr-pod-deployment
    spec:
      replicas: 3
      selector:
        matchLabels:
          app: dep-ctr-pod
      template:
        metadata:
          annotations:
            io.podman.annotations.ulimit: nofile=524288:524288,nproc=127332:127332
          creationTimestamp: "2023-03-27T20:45:08Z"
          labels:
            app: dep-ctr-pod
          name: dep-ctr-pod
        spec:
          containers:
          - command:
            - top
            image: docker.io/library/alpine:latest
            name: dep-ctr

Create Kubernetes Pod YAML for the specified container with the host
directory `/home/user/my-data` bind-mounted onto the container path
`/volume`.

    $ podman kube generate my-container-with-bind-mounted-data
    # Save the output of this file and use kubectl create -f to import
    # it into Kubernetes.
    #
    # Created with podman-3.1.0-dev
    apiVersion: v1
    kind: Pod
    metadata:
      creationTimestamp: "2021-03-18T16:26:08Z"
      labels:
        app: my-container-with-bind-mounted-data
      name: my-container-with-bind-mounted-data
    spec:
      containers:
      - command:
        - /bin/sh
        image: docker.io/library/alpine:latest
        name: test-bind-mount
        volumeMounts:
        - mountPath: /volume
          name: home-user-my-data-host
      restartPolicy: Never
      volumes:
      - hostPath:
          path: /home/user/my-data
          type: Directory
        name: home-user-my-data-host

Create Kubernetes Pod YAML for the specified container with named volume
`priceless-data` mounted onto the container path `/volume`.

    $ podman kube generate my-container-using-priceless-data
    # Save the output of this file and use kubectl create -f to import
    # it into Kubernetes.
    #
    # Created with podman-3.1.0-dev
    apiVersion: v1
    kind: Pod
    metadata:
      creationTimestamp: "2021-03-18T16:26:08Z"
      labels:
        app: my-container-using-priceless-data
      name: my-container-using-priceless-data
    spec:
      containers:
      - command:
        - /bin/sh
        image: docker.io/library/alpine:latest
        name: test-bind-mount
        volumeMounts:
        - mountPath: /volume
          name: priceless-data-pvc
      restartPolicy: Never
      volumes:
      - name: priceless-data-pvc
        persistentVolumeClaim:
          claimName: priceless-data

Create Kubernetes Pod YAML for the specified pod and include a service.

    $ sudo podman kube generate -s demoweb
    # Save the output of this file and use kubectl create -f to import
    # it into Kubernetes.
    #
    # Created with podman-0.12.2-dev
    apiVersion: v1
    kind: Pod
    metadata:
      creationTimestamp: 2018-12-18T15:16:06Z
      labels:
        app: demoweb
      name: demoweb-libpod
    spec:
      containers:
      - command:
        - python3
        - /root/code/graph.py
        image: quay.io/baude/demoweb:latest
        name: practicalarchimedes
        tty: true
        workingDir: /root/code
    ---
    apiVersion: v1
    kind: Service
    metadata:
      creationTimestamp: 2018-12-18T15:16:06Z
      labels:
        app: demoweb
      name: demoweb-libpod
    spec:
      ports:
      - name: "8050"
        nodePort: 31269
        port: 8050
        targetPort: 0
      selector:
        app: demoweb
      type: NodePort
    status:
      loadBalancer: {}

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-container(1)](podman-container.html)**,
**[podman-pod(1)](podman-pod.html)**,
**[podman-kube-play(1)](podman-kube-play.html)**,
**[podman-kube-down(1)](podman-kube-down.html)**

##  HISTORY

December 2018, Originally compiled by Brent Baude (bbaude at redhat dot
com)


---

<a id='podman-kube-play'></a>

## podman-kube-play - Create containers, pods and volumes based on
Kubernetes YAML

##  NAME

podman-kube-play - Create containers, pods and volumes based on
Kubernetes YAML

##  SYNOPSIS

**podman kube play** \[*options*\]
*file.yml\|-\|https://website.io/file.yml*

##  DESCRIPTION

**podman kube play** reads in a structured file of Kubernetes YAML. It
recreates the containers, pods, or volumes described in the YAML.
Containers within a pod are then started, and the ID of the new Pod or
the name of the new Volume is output. If the YAML file is specified as
\"-\", then `podman kube play` reads the YAML file from stdin. The input
can also be a URL that points to a YAML file such as
https://podman.io/demo.yml. `podman kube play` reads the YAML from the
URL and create pods and containers from it.

Using the `--down` command line option, it is also capable of tearing
down the pods created by a previous run of `podman kube play`.

Using the `--replace` command line option, it tears down the pods(if
any) created by a previous run of `podman kube play` and recreate the
pods with the Kubernetes YAML file.

Ideally the input file is created by the Podman command (see
podman-kube-generate(1)). This guarantees a smooth import and expected
results.

Currently, the supported Kubernetes kinds are:

-   Pod
-   Deployment
-   PersistentVolumeClaim
-   ConfigMap
-   Secret
-   DaemonSet
-   Job

`Kubernetes Pods or Deployments`

Only four volume types are supported by kube play, the *hostPath*,
*emptyDir*, *persistentVolumeClaim*, and *image* volume types.

-   When using the *hostPath* volume type, only the *default (empty)*,
    *DirectoryOrCreate*, *Directory*, *FileOrCreate*, *File*, *Socket*,
    *CharDevice* and *BlockDevice* subtypes are supported. Podman
    interprets the value of *hostPath* *path* as a file path when it
    contains at least one forward slash, otherwise Podman treats the
    value as the name of a named volume.
-   When using a *persistentVolumeClaim*, the value for *claimName* is
    the name for the Podman named volume.
-   When using an *emptyDir* volume, Podman creates an anonymous volume
    that is attached the containers running inside the pod and is
    deleted once the pod is removed.
-   When using an *image* volume, Podman creates a read-only image
    volume with an empty subpath (the whole image is mounted). The image
    must already exist locally. It is supported in rootful mode only.

Note: The default restart policy for containers is `always`. You can
change the default by setting the `restartPolicy` field in the spec.

Note: When playing a kube YAML with init containers, the init container
is created with init type value `once`. To change the default type, use
the `io.podman.annotations.init.container.type` annotation to set the
type to `always`.

Note: *hostPath* volume types created by kube play is given an SELinux
shared label (z), bind mounts are not relabeled (use
`chcon -t container_file_t -R <directory>`).

Note: To set userns of a pod, use the **io.podman.annotations.userns**
annotation in the pod/deployment definition. For example,
**io.podman.annotations.userns=keep-id** annotation tells Podman to
create a user namespace where the current rootless user\'s UID:GID are
mapped to the same values in the container. This can be overridden with
the `--userns` flag.

Note: Use the **io.podman.annotations.volumes-from** annotation to bind
mount volumes of one container to another. You can mount volumes from
multiple source containers to a target container. The source containers
that belong to the same pod must be defined before the source container
in the kube YAML. The annotation format is
`io.podman.annotations.volumes-from/targetContainer: "sourceContainer1:mountOpts1;sourceContainer2:mountOpts2"`.

Note: If the `:latest` tag is used, Podman attempts to pull the image
from a registry. If the image was built locally with Podman or Buildah,
it has `localhost` as the domain, in that case, Podman uses the image
from the local store even if it has the `:latest` tag.

Note: The command `podman play kube` is an alias of `podman kube play`,
and performs the same function.

Note: The command `podman kube down` can be used to stop and remove pods
or containers based on the same Kubernetes YAML used by
`podman kube play` to create them.

Note: To customize the name of the infra container created during
`podman kube play`, use the **io.podman.annotations.infra.name**
annotation in the pod definition. This annotation is automatically set
when generating a kube yaml from a pod that was created with the
`--infra-name` flag set.

`Kubernetes PersistentVolumeClaims`

A Kubernetes PersistentVolumeClaim represents a Podman named volume.
Only the PersistentVolumeClaim name is required by Podman to create a
volume. Kubernetes annotations can be used to make use of the available
options for Podman volumes.

-   volume.podman.io/driver
-   volume.podman.io/device
-   volume.podman.io/type
-   volume.podman.io/uid
-   volume.podman.io/gid
-   volume.podman.io/mount-options
-   volume.podman.io/import-source
-   volume.podman.io/image

Use `volume.podman.io/import-source` to import the contents of the
tarball (.tar, .tar.gz, .tgz, .bzip, .tar.xz, .txz) specified in the
annotation\'s value into the created Podman volume

Kube play is capable of building images on the fly given the correct
directory layout and Containerfiles. This option is not available for
remote clients, including Mac and Windows (excluding WSL2) machines,
yet. Consider the following excerpt from a YAML file:

    apiVersion: v1
    kind: Pod
    metadata:
    ...
    spec:
      containers:
      - name: container
        image: foobar
    ...

If there is a directory named `foobar` in the current working directory
with a file named `Containerfile` or `Dockerfile`, Podman kube play
builds that image and name it `foobar`. An example directory structure
for this example looks like:

    |- mykubefiles
        |- myplayfile.yaml
        |- foobar
             |- Containerfile

The build considers `foobar` to be the context directory for the build.
If there is an image in local storage called `foobar`, the image is not
built unless the `--build` flag is used. Use `--build=false` to
completely disable builds.

Kube play supports CDI (Container Device Interface) device selectors to
share host devices (e.g. GPUs) with containers. The configuration format
follows Kubernetes extended resource management:

    apiVersion: v1
    kind: Pod
    spec:
      containers:
      - name: container
        resources:
          limits:
            nvidia.com/gpu=all: 1

To enable sharing host devices, analogous to using the `--device` flag
Podman kube supports a custom CDI selector:
`podman.io/device=<host device path>`.

`Kubernetes ConfigMap`

Kubernetes ConfigMap can be referred as a source of environment
variables or volumes in Pods or Deployments. ConfigMaps aren\'t a
standalone object in Podman; instead, when a container uses a ConfigMap,
Podman creates environment variables or volumes as needed.

For example, the following YAML document defines a ConfigMap and then
uses it in a Pod:

    apiVersion: v1
    kind: ConfigMap
    metadata:
      name: foo
    data:
        FOO: bar
    ---
    apiVersion: v1
    kind: Pod
    metadata:
      name: foobar
    spec:
      containers:
      - name: container-1
        image: foobar
        envFrom:
        - configMapRef:
            name: foo
            optional: false

and as a result environment variable `FOO` is set to `bar` for container
`container-1`.

`Kubernetes Secret`

Kubernetes Secret represents a Podman named secret. The Kubernetes
Secret is saved as a whole and may be referred to as a source of
environment variables or volumes in Pods or Deployments.

For example, the following YAML document defines a Secret and then uses
it in a Pod:

    kind: Secret
    apiVersion: v1
    metadata:
      name: foo
    data:
      foo: YmFy # base64 for bar
    ---
    apiVersion: v1
    kind: Pod
    metadata:
      name: foobar
    spec:
      containers:
      - name: container-1
        image: foobar
        env:
        - name: FOO
          valueFrom:
            secretKeyRef:
              name: foo
              key: foo

and as a result environment variable `FOO` is set to `bar` for container
`container-1`.

`Automounting Volumes (deprecated)`

Note: The automounting annotation is deprecated. Kubernetes has [native
support for image
volumes](https://kubernetes.io/docs/tasks/configure-pod-container/image-volumes/)
and that should be used rather than this podman-specific annotation.

An image can be automatically mounted into a container if the annotation
`io.podman.annotations.kube.image.automount/$ctrname` is given. The
following rules apply:

-   The image must already exist locally.
-   The image must have at least 1 volume directive.
-   The path given by the volume directive will be mounted from the
    image into the container. For example, an image with a volume at
    `/test/test_dir` will have `/test/test_dir` in the image mounted to
    `/test/test_dir` in the container.
-   Multiple images can be specified. If multiple images have a volume
    at a specific path, the last image specified trumps.
-   The images are always mounted read-only.
-   Images to mount are defined in the annotation
    \"io.podman.annotations.kube.image.automount/\$ctrname\" as a
    semicolon-separated list. They are mounted into a single container
    in the pod, not the whole pod. The annotation can be specified for
    additional containers if additional mounts are required.

##  OPTIONS

#### **\--annotation**=*key=value*

Add an annotation to the container or pod. This option can be set
multiple times.

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

#### **\--build**

Build images even if they are found in the local storage. Use
`--build=false` to completely disable builds. (This option is not
available with the remote Podman client)

Note: You can also override the default isolation type by setting the
BUILDAH_ISOLATION environment variable. export BUILDAH_ISOLATION=oci.
See podman-build.1.md for more information.

#### **\--cert-dir**=*path*

Use certificates at *path* (\*.crt, \*.cert, \*.key) to connect to the
registry. (Default: /etc/containers/certs.d) For details, see
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**.
(This option is not available with the remote Podman client, including
Mac and Windows (excluding WSL2) machines)

#### **\--configmap**=*path*

Use Kubernetes configmap YAML at path to provide a source for
environment variable values within the containers of the pod. (This
option is not available with the remote Podman client)

Note: The *\--configmap* option can be used multiple times or a
comma-separated list of paths can be used to pass multiple Kubernetes
configmap YAMLs. The YAML file may be in a multi-doc YAML format. But,
it must container only configmaps

#### **\--context-dir**=*path*

Use *path* as the build context directory for each image. Requires
\--build option be true. (This option is not available with the remote
Podman client)

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

#### **\--force**

Tear down the volumes linked to the PersistentVolumeClaims as part of
\--down

#### **\--help**, **-h**

Print usage statement

#### **\--ip**=*IP address*

Assign a static ip address to the pod. This option can be specified
several times when kube play creates more than one pod. Note: When
joining multiple networks use the **\--network name:ip=\<ip\>** syntax.

#### **\--log-driver**=*driver*

Set logging driver for all created containers.

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

#### **\--mac-address**=*MAC address*

Assign a static mac address to the pod. This option can be specified
several times when kube play creates more than one pod. Note: When
joining multiple networks use the **\--network name:mac=\<mac\>**
syntax.

#### **\--network**=*mode*, **\--net**

Set the network mode for the pod.

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
    -   **host_interface_name=**\_name\_: Specify a name for the created
        network interface outside the container.

    Any other options will be passed through to netavark without
    validation. This can be useful to pass arguments to netavark
    plugins.

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

When no network option is specified and *host* network mode is not
configured in the YAML file, a new network stack is created and pods are
attached to it making possible pod to pod communication.

#### **\--no-hostname**

Do not create the */etc/hostname* file in the containers.

By default, Podman manages the */etc/hostname* file, adding the
container\'s own hostname. When the **\--no-hostname** option is set,
the image\'s */etc/hostname* will be preserved unmodified if it exists.

#### **\--no-hosts**

Do not modify the `/etc/hosts` file in the pod.

Podman assumes control over the pod\'s `/etc/hosts` file by default and
adds entries for the container\'s name (see **\--name** option) and
hostname (see **\--hostname** option), the internal
`host.containers.internal` and `host.docker.internal` hosts, as well as
any hostname added using the **\--add-host** option. Refer to the
**\--add-host** option for details. Passing **\--no-hosts** disables
this, so that the image\'s `/etc/hosts` file is kept unmodified. The
same can be achieved globally by setting *no_hosts=true* in
`containers.conf`.

This option conflicts with host added in the Kubernetes YAML.

#### **\--publish**=*\[\[ip:\]\[hostPort\]:\]containerPort\[/protocol\]*

Define or override a port definition in the YAML file.

The lists of ports in the YAML file and the command line are merged.
Matching is done by using the **containerPort** field. If
**containerPort** exists in both the YAML file and the option, the
latter takes precedence.

#### **\--publish-all**

Setting this option to `true` will expose all ports to the host, even if
only specified via **containerPort** in the K8 YAML. In terms of which
port will be exposed, **\--publish** has higher priority than
**hostPort**, has higher priority than **containerPort**.

If set to `false` (which is the default), only ports defined via
**hostPort** or **\--publish** are published on the host.

#### **\--quiet**, **-q**

Suppress output information when pulling images

#### **\--replace**

Tears down the pods created by a previous run of `kube play` and
recreates the pods. This option is used to keep the existing pods up to
date based upon the Kubernetes YAML.

#### **\--seccomp-profile-root**=*path*

Directory path for seccomp profiles (default:
\"/var/lib/kubelet/seccomp\"). (This option is not available with the
remote Podman client, including Mac and Windows (excluding WSL2)
machines)

#### **\--start**

Start the pod after creating it, set to false to only create it.

#### **\--tls-verify**

Require HTTPS and verify certificates when contacting registries
(default: **true**). If explicitly set to **true**, TLS verification is
used. If set to **false**, TLS verification is not used. If not
specified, TLS verification is used unless the target registry is listed
as an insecure registry in
**[containers-registries.conf(5)](https://github.com/containers/image/blob/main/docs/containers-registries.conf.5.md)**

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
started with `--userns=nomap` or `--userns=keep-id` without limiting the
user namespace size.

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
-   *size*=SIZE: override the size of the configured user namespace. It
    is useful to not saturate all the available IDs. Not supported when
    running as root.

**nomap**: creates a user namespace where the current rootless user\'s
UID:GID are not mapped into the container. This option is not allowed
for containers created by the root user.

**ns:**\_namespace\_: run the pod in the given existing user namespace.

#### **\--wait**, **-w**

Run pods and containers in the foreground. Default is false.

At any time you can run `podman pod ps` in another shell to view a list
of the running pods and containers.

When attached in the tty mode, you can kill the pods and containers by
pressing Ctrl-C or receiving any other interrupt signals.

All pods, containers, and volumes created with `podman kube play` is
removed upon exit.

##  EXAMPLES

Recreate the pod and containers described in the specified host YAML
file.

    $ podman kube play demo.yml
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

Recreate the pod and containers specified in a YAML file sent to stdin.

    $ cat demo.yml | podman kube play -
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

Tear down the pod and containers as described in the specified YAML
file.

    $  podman kube play --down demo.yml
    Pods stopped:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6
    Pods removed:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

Provide multiple configmap files as sources for environment variables
within the specified pods and containers.

    $ podman kube play demo.yml --configmap configmap-foo.yml,configmap-bar.yml
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

    $ podman kube play demo.yml --configmap configmap-foo.yml --configmap configmap-bar.yml
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

Create a pod connected to two networks with a static ip on each.

    $ podman kube play demo.yml --network net1:ip=10.89.1.5 --network net2:ip=10.89.10.10
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

Please take into account that networks must be created first using
podman-network-create(1).

Create and teardown from a URL pointing to a YAML file.

    $ podman kube play https://podman.io/demo.yml
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

    $ podman kube play --down https://podman.io/demo.yml
    Pods stopped:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6
    Pods removed:
    52182811df2b1e73f36476003a66ec872101ea59034ac0d4d3a7b40903b955a6

`podman kube play --down` does not work with a URL if the YAML file the
URL points to has been changed or altered.

# Podman Kube Play Support

This document outlines the kube yaml fields that are currently supported
by the **podman kube play** command.

Note: **N/A** means that the option cannot be supported in a single-node
Podman environment.

##  Pod Fields

  Field                                               Support
  --------------------------------------------------- ---------
  containers                                          ✅
  initContainers                                      ✅
  imagePullSecrets                                    no
  enableServiceLinks                                  no
  os.name                                             no
  volumes                                             ✅
  nodeSelector                                        N/A
  nodeName                                            N/A
  affinity.nodeAffinity                               N/A
  affinity.podAffinity                                N/A
  affinity.podAntiAffinity                            N/A
  tolerations.key                                     N/A
  tolerations.operator                                N/A
  tolerations.effect                                  N/A
  tolerations.tolerationSeconds                       N/A
  schedulerName                                       N/A
  runtimeClassName                                    no
  priorityClassName                                   no
  priority                                            no
  topologySpreadConstraints.maxSkew                   N/A
  topologySpreadConstraints.topologyKey               N/A
  topologySpreadConstraints.whenUnsatisfiable         N/A
  topologySpreadConstraints.labelSelector             N/A
  topologySpreadConstraints.minDomains                N/A
  restartPolicy                                       ✅
  terminationGracePeriodSeconds                       ✅
  activeDeadlineSeconds                               no
  readinessGates.conditionType                        no
  hostname                                            ✅
  setHostnameAsFQDN                                   no
  subdomain                                           no
  hostAliases.hostnames                               ✅
  hostAliases.ip                                      ✅
  dnsConfig.nameservers                               ✅
  dnsConfig.options.name                              ✅
  dnsConfig.options.value                             ✅
  dnsConfig.searches                                  ✅
  dnsPolicy                                           no
  hostNetwork                                         ✅
  hostPID                                             ✅
  hostIPC                                             ✅
  shareProcessNamespace                               ✅
  serviceAccountName                                  no
  automountServiceAccountToken                        no
  securityContext.runAsUser                           ✅
  securityContext.runAsNonRoot                        no
  securityContext.runAsGroup                          ✅
  securityContext.supplementalGroups                  ✅
  securityContext.fsGroup                             no
  securityContext.fsGroupChangePolicy                 no
  securityContext.seccompProfile.type                 no
  securityContext.seccompProfile.localhostProfile     no
  securityContext.seLinuxOptions.level                ✅
  securityContext.seLinuxOptions.role                 ✅
  securityContext.seLinuxOptions.type                 ✅
  securityContext.seLinuxOptions.user                 ✅
  securityContext.sysctls.name                        ✅
  securityContext.sysctls.value                       ✅
  securityContext.windowsOptions.gmsaCredentialSpec   no
  securityContext.windowsOptions.hostProcess          no
  securityContext.windowsOptions.runAsUserName        no

##  Container Fields

  Field                                               Support
  --------------------------------------------------- ---------
  name                                                ✅
  image                                               ✅
  imagePullPolicy                                     ✅
  command                                             ✅
  args                                                ✅
  workingDir                                          ✅
  ports.containerPort                                 ✅
  ports.hostIP                                        ✅
  ports.hostPort                                      ✅
  ports.name                                          ✅
  ports.protocol                                      ✅
  env.name                                            ✅
  env.value                                           ✅
  env.valueFrom.configMapKeyRef.key                   ✅
  env.valueFrom.configMapKeyRef.name                  ✅
  env.valueFrom.configMapKeyRef.optional              ✅
  env.valueFrom.fieldRef                              ✅
  env.valueFrom.resourceFieldRef                      ✅
  env.valueFrom.secretKeyRef.key                      ✅
  env.valueFrom.secretKeyRef.name                     ✅
  env.valueFrom.secretKeyRef.optional                 ✅
  envFrom.configMapRef.name                           ✅
  envFrom.configMapRef.optional                       ✅
  envFrom.prefix                                      no
  envFrom.secretRef.name                              ✅
  envFrom.secretRef.optional                          ✅
  volumeMounts.mountPath                              ✅
  volumeMounts.name                                   ✅
  volumeMounts.mountPropagation                       no
  volumeMounts.readOnly                               ✅
  volumeMounts.subPath                                ✅
  volumeMounts.subPathExpr                            no
  volumeDevices.devicePath                            no
  volumeDevices.name                                  no
  resources.limits                                    ✅
  resources.requests                                  ✅
  lifecycle.postStart                                 no
  lifecycle.preStop                                   no
  terminationMessagePath                              no
  terminationMessagePolicy                            no
  livenessProbe                                       ✅
  readinessProbe                                      no
  startupProbe                                        no
  securityContext.runAsUser                           ✅
  securityContext.runAsNonRoot                        no
  securityContext.runAsGroup                          ✅
  securityContext.readOnlyRootFilesystem              ✅
  securityContext.procMount                           ✅
  securityContext.privileged                          ✅
  securityContext.allowPrivilegeEscalation            ✅
  securityContext.capabilities.add                    ✅
  securityContext.capabilities.drop                   ✅
  securityContext.seccompProfile.type                 no
  securityContext.seccompProfile.localhostProfile     no
  securityContext.seLinuxOptions.level                ✅
  securityContext.seLinuxOptions.role                 ✅
  securityContext.seLinuxOptions.type                 ✅
  securityContext.seLinuxOptions.user                 ✅
  securityContext.windowsOptions.gmsaCredentialSpec   no
  securityContext.windowsOptions.hostProcess          no
  securityContext.windowsOptions.runAsUserName        no
  stdin                                               no
  stdinOnce                                           no
  tty                                                 no

##  PersistentVolumeClaim Fields

  Field                Support
  -------------------- ---------
  volumeName           no
  storageClassName     ✅
  volumeMode           no
  accessModes          ✅
  selector             no
  resources.limits     no
  resources.requests   ✅

##  ConfigMap Fields

  Field        Support
  ------------ ---------
  binaryData   ✅
  data         ✅
  immutable    no

##  Deployment Fields

  --------------------------------------------------------------------------------
  Field                                   Support
  --------------------------------------- ----------------------------------------
  replicas                                ✅ (the actual replica count is ignored
                                          and set to 1)

  selector                                ✅

  template                                ✅

  minReadySeconds                         no

  strategy.type                           no

  strategy.rollingUpdate.maxSurge         no

  strategy.rollingUpdate.maxUnavailable   no

  revisionHistoryLimit                    no

  progressDeadlineSeconds                 no

  paused                                  no
  --------------------------------------------------------------------------------

##  DaemonSet Fields

  Field                                   Support
  --------------------------------------- ---------
  selector                                ✅
  template                                ✅
  minReadySeconds                         no
  strategy.type                           no
  strategy.rollingUpdate.maxSurge         no
  strategy.rollingUpdate.maxUnavailable   no
  revisionHistoryLimit                    no

##  Job Fields

  Field                     Support
  ------------------------- ----------------------------------
  activeDeadlineSeconds     no
  selector                  no (automatically set by k8s)
  template                  ✅
  backoffLimit              no
  completionMode            no
  completions               no (set to 1 with kube generate)
  manualSelector            no
  parallelism               no (set to 1 with kube generate)
  podFailurePolicy          no
  suspend                   no
  ttlSecondsAfterFinished   no

##  SEE ALSO

**[podman(1)](podman.html)**, **[podman-kube(1)](podman-kube.html)**,
**[podman-kube-down(1)](podman-kube-down.html)**,
**[podman-network-create(1)](podman-network-create.html)**,
**[podman-kube-generate(1)](podman-kube-generate.html)**,
**[podman-build(1)](podman-build.html)**,
**[containers-certs.d(5)](https://github.com/containers/image/blob/main/docs/containers-certs.d.5.md)**


---

