# podman-5.3.2 Network Commands

*This document contains Network commands from the Podman documentation.*

## Table of Contents

- [podman-network-connect - Connect a container to a network](#podman-network-connect)
- [podman-network-create - Create a Podman network](#podman-network-create)
- [podman-network-disconnect - Disconnect a container from a network](#podman-network-disconnect)
- [podman-network-exists - Check if the given network exists](#podman-network-exists)
- [podman-network-inspect - Display the network configuration for one or
more networks](#podman-network-inspect)
- [podman-network-ls - Display a summary of networks](#podman-network-ls)
- [podman-network-prune - Remove all unused networks](#podman-network-prune)
- [podman-network-rm - Remove one or more networks](#podman-network-rm)
- [podman-network-update - Update an existing Podman network](#podman-network-update)

<a id='podman-network-connect'></a>

## podman-network-connect - Connect a container to a network

##  NAME

podman-network-connect - Connect a container to a network

##  SYNOPSIS

**podman network connect** \[*options*\] network container

##  DESCRIPTION

Connects a container to a network. A container can be connected to a
network by name or by ID. Once connected, the container can communicate
with other containers in the same network.

##  OPTIONS

#### **\--alias**=*name*

Add network-scoped alias for the container. If the network has DNS
enabled (`podman network inspect -f {{.DNSEnabled}} <NAME>`), these
aliases can be used for name resolution on the given network. Multiple
*\--alias* options may be specified as input. NOTE: When using CNI, a
container only has access to aliases on the first network that it joins.
This limitation does not exist with netavark/aardvark-dns.

#### **\--ip**=*address*

Set a static ipv4 address for this container on this network.

#### **\--ip6**=*address*

Set a static ipv6 address for this container on this network.

#### **\--mac-address**=*address*

Set a static mac address for this container on this network.

##  EXAMPLE

Connect specified container to a named network:

    podman network connect test web

Connect specified container to named network with two aliases:

    podman network connect --alias web1 --alias web2 test web

Connect specified container to named network with a static ip:

    podman network connect --ip 10.89.1.13 test web

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**,
**[podman-network-inspect(1)](podman-network-inspect.html)**,
**[podman-network-disconnect(1)](podman-network-disconnect.html)**

##  HISTORY

November 2020, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-network-create'></a>

## podman-network-create - Create a Podman network

##  NAME

podman-network-create - Create a Podman network

##  SYNOPSIS

**podman network create** \[*options*\] \[*name*\]

##  DESCRIPTION

Create a network configuration for use with Podman. By default, Podman
creates a bridge connection. A *Macvlan* connection can be created with
the *-d macvlan* option. A parent device for macvlan or ipvlan can be
designated with the *-o parent=`<device>`* or
*\--network-interface=`<device>`* option.

If no options are provided, Podman assigns a free subnet and name for
the network.

Upon completion of creating the network, Podman displays the name of the
newly added network.

##  OPTIONS

#### **\--disable-dns**

Disables the DNS plugin for this network which if enabled, can perform
container to container name resolution. It is only supported with the
`bridge` driver, for other drivers it is always disabled.

#### **\--dns**=*ip*

Set network-scoped DNS resolver/nameserver for containers in this
network. If not set, the host servers from `/etc/resolv.conf` is used.
It can be overwritten on the container level with the
`podman run/create --dns` option. This option can be specified multiple
times to set more than one IP.

#### **\--driver**, **-d**=*driver*

Driver to manage the network. Currently `bridge`, `macvlan` and `ipvlan`
are supported. Defaults to `bridge`. As rootless the `macvlan` and
`ipvlan` driver have no access to the host network interfaces because
rootless networking requires a separate network namespace.

The netavark backend allows the use of so called *netavark plugins*, see
the
[plugin-API.md](https://github.com/containers/netavark/blob/main/plugin-API.md)
documentation in netavark. The binary must be placed in a specified
directory so podman can discover it, this list is set in
`netavark_plugin_dirs` in
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**
under the `[network]` section.

The name of the plugin can then be used as driver to create a network
for your plugin. The list of all supported drivers and plugins can be
seen with `podman info --format {{.Plugins.Network}}`.

Note that the `macvlan` and `ipvlan` drivers do not support port
forwarding. Support for port forwarding with a plugin depends on the
implementation of the plugin.

#### **\--gateway**=*ip*

Define a gateway for the subnet. To provide a gateway address, a
*subnet* option is required. Can be specified multiple times. The
argument order of the **\--subnet**, **\--gateway** and **\--ip-range**
options must match.

#### **\--ignore**

Ignore the create request if a network with the same name already exists
instead of failing. Note, trying to create a network with an existing
name and different parameters does not change the configuration of the
existing one.

#### **\--interface-name**=*name*

This option maps the *network_interface* option in the network config,
see **podman network inspect**. Depending on the driver, this can have
different effects; for `bridge`, it uses the bridge interface name. For
`macvlan` and `ipvlan`, it is the parent device on the host. It is the
same as `--opt parent=...`.

#### **\--internal**

Restrict external access of this network when using a `bridge` network.
Note when using the CNI backend DNS will be automatically disabled, see
**\--disable-dns**.

When using the `macvlan` or `ipvlan` driver with this option no default
route will be added to the container. Because it bypasses the host
network stack no additional restrictions can be set by podman and if a
privileged container is run it can set a default route themselves. If
this is a concern then the container connections should be blocked on
your actual network gateway.

#### **\--ip-range**=*range*

Allocate container IP from a range. The range must be a either a
complete subnet in CIDR notation or be in the `<startIP>-<endIP>` syntax
which allows for a more flexible range compared to the CIDR subnet. The
*ip-range* option must be used with a *subnet* option. Can be specified
multiple times. The argument order of the **\--subnet**, **\--gateway**
and **\--ip-range** options must match.

#### **\--ipam-driver**=*driver*

Set the ipam driver (IP Address Management Driver) for the network. When
unset podman chooses an ipam driver automatically based on the network
driver.

Valid values are:

-   `dhcp`: IP addresses are assigned from a dhcp server on the network.
    When using the netavark backend the `netavark-dhcp-proxy.socket`
    must be enabled in order to start the dhcp-proxy when a container is
    started, for CNI use the `cni-dhcp.socket` unit instead.
-   `host-local`: IP addresses are assigned locally.
-   `none`: No ip addresses are assigned to the interfaces.

View the driver in the **podman network inspect** output under the
`ipam_options` field.

#### **\--ipv6**

Enable IPv6 (Dual Stack) networking. If no subnets are given, it
allocates an ipv4 and an ipv6 subnet.

#### **\--label**=*label*

Set metadata for a network (e.g., \--label mykey=value).

#### **\--opt**, **-o**=*option*

Set driver specific options.

All drivers accept the `mtu`, `metric`, `no_default_route` and options.

-   `mtu`: Sets the Maximum Transmission Unit (MTU) and takes an integer
    value.
-   `metric` Sets the Route Metric for the default route created in
    every container joined to this network. Accepts a positive integer
    value. Can only be used with the Netavark network backend.
-   `no_default_route`: If set to 1, Podman will not automatically add a
    default route to subnets. Routes can still be added manually by
    creating a custom route using `--route`.

Additionally the `bridge` driver supports the following options:

-   `vlan`: This option assign VLAN tag and enables vlan_filtering.
    Defaults to none.
-   `isolate`: This option isolates networks by blocking traffic between
    those that have this option enabled.
-   `com.docker.network.bridge.name`: This option assigns the given name
    to the created Linux Bridge
-   `com.docker.network.driver.mtu`: Sets the Maximum Transmission Unit
    (MTU) and takes an integer value.
-   `vrf`: This option assigns a VRF to the bridge interface. It accepts
    the name of the VRF and defaults to none. Can only be used with the
    Netavark network backend.

The `macvlan` and `ipvlan` driver support the following options:

-   `parent`: The host device which is used for the macvlan interface.
    Defaults to the default route interface.
-   `mode`: This option sets the specified ip/macvlan mode on the
    interface.
    -   Supported values for `macvlan` are `bridge`, `private`, `vepa`,
        `passthru`. Defaults to `bridge`.
    -   Supported values for `ipvlan` are `l2`, `l3`, `l3s`. Defaults to
        `l2`.

Additionally the `macvlan` driver supports the `bclim` option:

-   `bclim`: Set the threshold for broadcast queueing. Must be a 32 bit
    integer. Setting this value to `-1` disables broadcast queueing
    altogether.

#### **\--route**=*route*

A static route in the format
`<destination in CIDR notation>,<gateway>,<route metric (optional)>`.
This route will be added to every container in this network. Only
available with the netavark backend. It can be specified multiple times
if more than one static route is desired.

#### **\--subnet**=*subnet*

The subnet in CIDR notation. Can be specified multiple times to allocate
more than one subnet for this network. The argument order of the
**\--subnet**, **\--gateway** and **\--ip-range** options must match.
This is useful to set a static ipv4 and ipv6 subnet.

##  EXAMPLE

Create a network with no options.

    $ podman network create
    podman2

Create a network named *newnet* that uses *192.5.0.0/16* for its subnet.

    $ podman network create --subnet 192.5.0.0/16 newnet
    newnet

Create an IPv6 network named *newnetv6* with a subnet of
*2001:db8::/64*.

    $ podman network create --subnet 2001:db8::/64 --ipv6 newnetv6
    newnetv6

Create a network named *newnet* that uses *192.168.33.0/24* and defines
a gateway as *192.168.33.3*.

    $ podman network create --subnet 192.168.33.0/24 --gateway 192.168.33.3 newnet
    newnet

Create a network that uses a *192.168.55.0/24* subnet and has an IP
address range of *192.168.55.129 - 192.168.55.254*.

    $ podman network create --subnet 192.168.55.0/24 --ip-range 192.168.55.128/25
    podman5

Create a network with a static ipv4 and ipv6 subnet and set a gateway.

    $ podman network create --subnet 192.168.55.0/24 --gateway 192.168.55.3 --subnet fd52:2a5a:747e:3acd::/64 --gateway fd52:2a5a:747e:3acd::10
    podman4

Create a network with a static subnet and a static route.

    $ podman network create --subnet 192.168.33.0/24 --route 10.1.0.0/24,192.168.33.10 newnet

Create a network with a static subnet and a static route without a
default route.

    $ podman network create --subnet 192.168.33.0/24 --route 10.1.0.0/24,192.168.33.10 --opt no_default_route=1 newnet

Create a Macvlan based network using the host interface eth0. Macvlan
networks can only be used as root.

    $ sudo podman network create -d macvlan -o parent=eth0 --subnet 192.5.0.0/16 newnet
    newnet

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**,
**[podman-network-inspect(1)](podman-network-inspect.html)**,
**[podman-network-ls(1)](podman-network-ls.html)**,
**[containers.conf(5)](https://github.com/containers/common/blob/main/docs/containers.conf.5.md)**

##  HISTORY

August 2021, Updated with the new network format by Paul Holzinger
<pholzing@redhat.com>

August 2019, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-network-disconnect'></a>

## podman-network-disconnect - Disconnect a container from a network

##  NAME

podman-network-disconnect - Disconnect a container from a network

##  SYNOPSIS

**podman network disconnect** \[*options*\] network container

##  DESCRIPTION

Disconnects a container from a network. A container can be disconnected
from a network by name or by ID. If all networks are disconnected from
the container, it behaves like a container created with `--network=none`
and it does not have network connectivity until a network is connected
again.

##  OPTIONS

#### **\--force**, **-f**

Force the container to disconnect from a network

##  EXAMPLE

Disconnect container from specified network:

    podman network disconnect test web

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**,
**[podman-network-connect(1)](podman-network-connect.html)**

##  HISTORY

November 2020, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-network-exists'></a>

## podman-network-exists - Check if the given network exists

##  NAME

podman-network-exists - Check if the given network exists

##  SYNOPSIS

**podman network exists** *network*

##  DESCRIPTION

**podman network exists** checks if a network exists. The **Name** or
**ID** of the network may be used as input. Podman returns an exit code
of `0` when the network is found. A `1` is returned otherwise. An exit
code of `125` indicates there was another issue.

##  OPTIONS

#### **\--help**, **-h**

Print usage statement

##  EXAMPLE

Check if specified network exists (the network does actually exist):

    $ podman network exists net1
    $ echo $?
    0

Check if nonexistent network exists:

    $ podman network exists webbackend
    $ echo $?
    1

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**

##  HISTORY

January 2021, Originally compiled by Paul Holzinger
`<paul.holzinger@web.de>`


---

<a id='podman-network-inspect'></a>

## podman-network-inspect - Display the network configuration for one or
more networks

##  NAME

podman-network-inspect - Display the network configuration for one or
more networks

##  SYNOPSIS

**podman network inspect** \[*options*\] *network* \[*network* \...\]

##  DESCRIPTION

Display the (JSON format) network configuration.

##  OPTIONS

#### **\--format**, **-f**=*format*

Pretty-print networks to JSON or using a Go template.

  **Placeholder**      **Description**
  -------------------- -------------------------------------------
  .Containers \...     Running containers on this network.
  .Created \...        Timestamp when the network was created
  .DNSEnabled          Network has dns enabled (boolean)
  .Driver              Network driver
  .ID                  Network ID
  .Internal            Network is internal (boolean)
  .IPAMOptions \...    Network ipam options
  .IPv6Enabled         Network has ipv6 subnet (boolean)
  .Labels \...         Network labels
  .Name                Network name
  .Network \...        Nested Network type
  .NetworkDNSServers   Array of DNS servers used in this network
  .NetworkInterface    Name of the network interface on the host
  .Options \...        Network options
  .Routes              List of static routes for this network
  .Subnets             List of subnets on this network

##  EXAMPLE

Inspect the default podman network.

    $ podman network inspect podman
    [
        {
            "name": "podman",
            "id": "2f259bab93aaaaa2542ba43ef33eb990d0999ee1b9924b557b7be53c0b7a1bb9",
            "driver": "bridge",
            "network_interface": "podman0",
            "created": "2021-06-03T12:04:33.088567413+02:00",
            "subnets": [
                {
                    "subnet": "10.88.0.0/16",
                    "gateway": "10.88.0.1"
                }
            ],
            "ipv6_enabled": false,
            "internal": false,
            "dns_enabled": false,
            "ipam_options": {
                "driver": "host-local"
            }
        }
    ]

Show the subnet and gateway for a network.

    $ podman network inspect podman --format "{{range .Subnets}}Subnet: {{.Subnet}} Gateway: {{.Gateway}}{{end}}"
    Subnet: 10.88.0.0/16 Gateway: 10.88.0.1

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**,
**[podman-network-ls(1)](podman-network-ls.html)**,
**[podman-network-create(1)](podman-network-create.html)**

##  HISTORY

August 2021, Updated with the new network format by Paul Holzinger
<pholzing@redhat.com>

August 2019, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-network-ls'></a>

## podman-network-ls - Display a summary of networks

##  NAME

podman-network-ls - Display a summary of networks

##  SYNOPSIS

**podman network ls** \[*options*\]

##  DESCRIPTION

Displays a list of existing podman networks.

##  OPTIONS

#### **\--filter**, **-f**=*filter=value*

Provide filter values.

The *filters* argument format is of `key=value`. If there is more than
one *filter*, then pass multiple OPTIONS: **\--filter** *foo=bar*
**\--filter** *bif=baz*.

Supported filters:

  ----------------------------------------------------------------------------
  **Filter**   **Description**
  ------------ ---------------------------------------------------------------
  driver       Filter by driver type.

  id           Filter by full or partial network ID.

  label        Filter by network with (or without, in the case of
               label!=\[\...\] is used) the specified labels.

  name         Filter by network name (accepts `regex`).

  until        Filter by networks created before given timestamp.

  dangling     Filter by networks with no containers attached.
  ----------------------------------------------------------------------------

The `driver` filter accepts values: `bridge`, `macvlan`, `ipvlan`.

The `label` *filter* accepts two formats. One is the `label`=*key* or
`label`=*key*=*value*, which shows images with the specified labels. The
other format is the `label!`=*key* or `label!`=*key*=*value*, which
shows images without the specified labels.

The `until` *filter* can be Unix timestamps, date formatted timestamps,
or Go duration strings (e.g. 10m, 1h30m) computed relative to the
machine's time.

The `dangling` *filter* accepts values `true` or `false`.

#### **\--format**=*format*

Change the default output format. This can be of a supported type like
\'json\' or a Go template. Valid placeholders for the Go template are
listed below:

  **Placeholder**      **Description**
  -------------------- -------------------------------------------
  .Created \...        Timestamp when the network was created
  .DNSEnabled          Network has dns enabled (boolean)
  .Driver              Network driver
  .ID                  Network ID
  .Internal            Network is internal (boolean)
  .IPAMOptions \...    Network ipam options
  .IPv6Enabled         Network has ipv6 subnet (boolean)
  .Labels              Network labels
  .Name                Network name
  .NetworkDNSServers   Array of DNS servers used in this network
  .NetworkInterface    Name of the network interface on the host
  .Options \...        Network options
  .Routes              List of static routes for this network
  .Subnets             List of subnets on this network

#### **\--no-trunc**

Do not truncate the network ID.

#### **\--noheading**, **-n**

Omit the table headings from the listing.

#### **\--quiet**, **-q**

The `quiet` option restricts the output to only the network names.

##  EXAMPLE

Display networks:

    $ podman network ls
    NETWORK ID    NAME         DRIVER
    88a7120ee19d  podman       bridge
    6dd508dbf8cd  podman6  bridge
    8e35c2cd3bf6  podman5  macvlan

Display only network names:

    $ podman network ls -q
    podman
    podman2
    outside
    podman9

Display name of network which support bridge plugin:

    $ podman network ls --filter driver=bridge --format {{.Name}}
    podman
    podman2
    podman9

List networks with their subnets:

    $ podman network ls --format "{{.Name}}: {{range .Subnets}}{{.Subnet}} {{end}}"
    podman: 10.88.0.0/16
    podman3: 10.89.30.0/24 fde4:f86f:4aab:e68f::/64
    macvlan:

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**,
**[podman-network-inspect(1)](podman-network-inspect.html)**,
**[podman-network-create(1)](podman-network-create.html)**

##  HISTORY

August 2021, Updated with the new network format by Paul Holzinger
<pholzing@redhat.com>

August 2019, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-network-prune'></a>

## podman-network-prune - Remove all unused networks

##  NAME

podman-network-prune - Remove all unused networks

##  SYNOPSIS

**podman network prune** \[*options*\]

##  DESCRIPTION

Remove all unused networks. An unused network is defined by a network
which has no containers connected or configured to connect to it. It
does not remove the so-called default network which goes by the name of
*podman*.

##  OPTIONS

#### **\--filter**

Provide filter values.

The *filters* argument format is of `key=value`. If there is more than
one *filter*, then pass multiple OPTIONS: **\--filter** *foo=bar*
**\--filter** *bif=baz*.

Supported filters:

  --------------------------------------------------------------------------
   Filter  Description
  -------- -----------------------------------------------------------------
   label   Only remove networks, with (or without, in the case of
           label!=\[\...\] is used) the specified labels.

   until   Only remove networks created before given timestamp.
  --------------------------------------------------------------------------

The `label` *filter* accepts two formats. One is the `label`=*key* or
`label`=*key*=*value*, which removes networks with the specified labels.
The other format is the `label!`=*key* or `label!`=*key*=*value*, which
removes networks without the specified labels.

The `until` *filter* can be Unix timestamps, date formatted timestamps,
or Go duration strings (e.g. 10m, 1h30m) computed relative to the
machine's time.

#### **\--force**, **-f**

Do not prompt for confirmation

##  EXAMPLE

Prune networks:

    podman network prune

Prune all networks created not created in the last two hours:

    podman network prune --filter until=2h

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**,
**[podman-network-rm(1)](podman-network-rm.html)**

##  HISTORY

February 2021, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-network-rm'></a>

## podman-network-rm - Remove one or more networks

##  NAME

podman-network-rm - Remove one or more networks

##  SYNOPSIS

**podman network rm** \[*options*\] \[*network\...*\]

##  DESCRIPTION

Delete one or more Podman networks.

##  OPTIONS

#### **\--force**, **-f**

The `force` option removes all containers that use the named network. If
the container is running, the container is stopped and removed.

#### **\--time**, **-t**=*seconds*

Seconds to wait before forcibly stopping the running containers that are
using the specified network. The \--force option must be specified to
use the \--time option. Use -1 for infinite wait.

##  EXAMPLE

Delete specified network:

    # podman network rm podman9
    Deleted: podman9

Delete specified network and all containers associated with the network:

    # podman network rm -f fred
    Deleted: fred

##  Exit Status

**0** All specified networks removed

**1** One of the specified networks did not exist, and no other failures

**2** The network is in use by a container or a Pod

**125** The command fails for any other reason

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**

##  HISTORY

August 2019, Originally compiled by Brent Baude <bbaude@redhat.com>


---

<a id='podman-network-update'></a>

## podman-network-update - Update an existing Podman network

##  NAME

podman-network-update - Update an existing Podman network

##  SYNOPSIS

**podman network update** \[*options*\] *network*

##  DESCRIPTION

Allow changes to existing container networks. At present, only changes
to the DNS servers in use by a network is supported.

NOTE: Only supported with the netavark network backend.

##  OPTIONS

#### **\--dns-add**

Accepts array of DNS resolvers and add it to the existing list of
resolvers configured for a network.

#### **\--dns-drop**

Accepts array of DNS resolvers and removes them from the existing list
of resolvers configured for a network.

##  EXAMPLE

Update a network:

    $ podman network update network1 --dns-add 8.8.8.8,1.1.1.1

Update a network and add/remove dns servers:

    $ podman network update network1 --dns-drop 8.8.8.8 --dns-add 3.3.3.3

##  SEE ALSO

**[podman(1)](podman.html)**,
**[podman-network(1)](podman-network.html)**,
**[podman-network-inspect(1)](podman-network-inspect.html)**,
**[podman-network-ls(1)](podman-network-ls.html)**


---

