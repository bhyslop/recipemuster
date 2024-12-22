
# Engage Podman for Gateway Feature

## Title

Enable Container to 'drop in' as network gateway

## Feature Request Description

I am an embedded developer who wants to use more containers but I need more security measures to protect customer assets.

I want a container running less trusted code ("BOTTLE") to be able to use a container running a trusted, carefully configured intermediary container ("SENTRY") to intermediate all local and internet access?

Find at [RBM System Vision](https://scaleinv.github.io/recipebottle/#_the_rbm_system_vision) a diagram and abstract of my passion open-source project to do this.

For this project to succeed, it nees to be able to use existing images and/ or stock dockerfiles for BOTTLE.
Prototypes using `podman` and bespoke dockerfiles have proven the concept.
I've tried many things to move away from dockerfiles with intimate networking configuration, and lately I've arrived at this.

Could we add a network feature to allow SENTRY to function as gateway to a BOTTLE from its earliest DHCP?

## Suggest Potential Solution

I wonder if it is a simple and elegant solution to add an `--as-gateway` flag to the `podman network connect` command.
This parameterless option would simply cause podman to assign the gateway IP to the container that is being connected to the network, and also work through any consistency of MAC address in the process (see `arp` mention later).

I'm hoping this is an obvious enough feature that others would want this.
A search of your open issues led me only to [DMZ Feature Request](https://github.com/containers/podman/issues/20222) which may be consonant, if idle.

## Alternatives Considered

My road to this feature request has been long.
Since I'm not a deep networking expert, I've been freely using Claude from Anthropic and ChatGPT to help me work on this.

I first started this whole project based on `docker`.
After many frustrations, I finally found fine print in the docker documentation asserting that one could not connect a host network and an internal network to the same container concurrently.
This was long enough ago that it may not be true anymore, but it was enough of a blocker to lead me to try `podman`.

I've found `podman` delightful in all respects, so much so that I'd like it to be one of the 'trusted few' apps I install natively on my workstation.
Well done folks!

I then developed a prototype using bespoke dockerfiles in `podman` and was able to cause the security behavior I wanted: `iptables` and `dnsmasq` worked great for constraining BOTTLE's access to the internet while not breaking on-workstation application ports.
This was enough to then justify the 'container image lifecycle management playground' engineering I've already built out for the project.

So here are the alternatives I've tried within `podman`:

* I tried the naive approach, simply assigning the gateway IP to the SENTRY container but `podman` silently rejected my request.
* I tried a bevvy of similar approaches that led me to explore `--opt` and `--dns` option permutations in `podman run` and `podman network` commands.  I also experimented with some AI suggestions regarding CNI config, but I wasn't very smart about this then- too much sophisitication, too little visibility.
They feel like some combination of power tools for the network-wise and features that may be drifting into cruft.
* Next I played some with setting up `dhclient` in the BOTTLE to effectively post-configure its network stack to see SENTRY as the gateway, but this seemed to get stuck with 'combativeness' between my networking and `podman`'s own.  My test BOTTLEs didn't behave uniformly with this, though some progress.  I posited I was getting bitten by network config race conditions involved in booting a container in one network environment then operating it in another.
After all, I want to work with stock BOTTLEs, not ones I intimately reconfigure.
* Then, I had some success elevating the privileges of the BOTTLE container to allow it to manipulate the network stack.  A simple post-startup `ip route` and nameserver revision started working, but I was having misgivings about elevating privileges for a BOTTLE.  The suspicion of network race conditions lingered (I'm not going to get into the necessary mechanics for stabilizing eth0/eth1 container network assignment 😊) so I didn't dig in.
* My most recent approach was to try and re-assign the SENTRY IP address after SENTRY startup but before BOTTLE startup to emplace SENTRY as the gateway, but that failed when arps communicated the wrong MAC address even after.
I went far down the rabbit hole of `podman machine` network namespace `tcpdump` study (my laptop is windows) to figure this out.
Claude was ready to guide me attempting unsolicited arp cache poisioning pieces but that felt literally and figuratively gratuitous, as well as fragile.
I want this project to work well inside and outside of a `podman machine` after all.
* At this point I enlisted ChatGPT for fresh ideas: it suggested manipulating CNI to more correctly emplace SENTRY as the gateway container with some deft customizations of a `podman` default CNI `conflist`.
While exploring this approach, I learned that `podman` is moving away from CNI and towards 'netavark' which does not seem known to the AIs yet.
I studied other container runtimes for stable future-proof CNI environments, from a locally managed `containerd` to a CRI-O.
However, I think people who might want to use my open source system will want the full 'desktop' feature set absent from CRI-O and other kubernetes spawn.

Finally, thank you `podman` maintainers for an amazing project!
I'm not averse to attempting an implementation PR, once we scrub this concept against your long term visions.

Thoughts?