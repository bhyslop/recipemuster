# Run Claude Code Inside a Crucible Built on This Machine

## The walk

### Goal

Claude Code has worked inside the ccyolo Crucible, and the Crucible has been taken back down, as the onboarding's explorer track teaches it: its Sentry and its Bottle images stand built on this machine by the Kludge, the Hallmark of each written into the ccyolo Nameplate and committed; the Crucible has stood Charged, and Claude Code, signed in inside its Bottle under the attendant's own Claude account, has taken the track's first prompt and worked on the sample project there; you have run the track's five network probes inside the Bottle and read what each printed; and the Crucible stands Quenched.

The documents to take up, in this order: the `README.md` at the root of the clone; the onboarding guide it tells you to run; and, from that guide's start menu, the explorer track that inhabits the sandbox, among its kludged Crucibles.
Walk that track from its first line to its last step, "The pattern", and stop there.
Other documents stand in the clone — the other tracks, the guides other commands print, and the project's code and configuration files beyond those a direction names — and are left unopened, as a newcomer might not think to open them.

The ground this walk stands on: the clone the environment walk configured, its station file validating, and nothing of the cloud.
This walk asks nothing of a payor or a depot, so it stands alike on a clone the cloud walks have also run on, and uses nothing they left.
The clone shipped with the Hallmark fields of every Nameplate blank, and those of the ccyolo Nameplate stand blank when you begin.
Docker stands installed and running on this machine, and no ccyolo Crucible stands Charged on it, from this clone or from any other.
The attendant holds a Claude subscription on the block's "Claude account", and a terminal of their own on this machine.

Your own terminal holds no interactive session, so two parts of the track are reached otherwise than as the track gives them.
Entering the Bottle by the SSH door the track gives, and running Claude Code there, wait on a person's typing at a terminal that holds the session open: they are the attendant's, in the acts below.
Where the track has you run the network probes inside the Bottle while signed in there by SSH, run each yourself as `docker exec ccyolo-bottle` followed by the probe exactly as the track spells it, from any folder: it runs inside the Bottle, on the network the Sentry gives it, as a command typed in the SSH session would, and it leaves the attendant's session alone.
Tell the attendant before you run the track's Quench, since it ends their session in the Bottle.

### Acts reserved to the attendant

Each act below is the attendant's to perform, never yours.

Reserved to the attendant, at their own terminal and then in a browser of their own, proving identity: the attendant runs the SSH door the track gives at their own terminal on this machine, from the clone's root, and at the prompt it lands at runs `claude`, as the track gives it.
Where Claude Code asks how to sign in, they choose their Claude subscription, as the track says; when it prints a sign-in address, they open it in a browser of their own, sign in there as the block's "Claude account", and paste the code that page gives back at Claude Code's prompt, or type it where the paste does not take, as the track says to.
They read back to you each question Claude Code put on the way, and what it shows once signed in, before going on.

Reserved to the attendant at their own terminal, granting access — entrusting Claude Code with the workspace: where Claude Code asks whether to trust the folder it starts in, or whether to accept working with no permission prompts, the attendant accepts, as the track's word that it starts in full autonomy says to expect; then they type the track's first prompt at Claude Code's prompt exactly as the track spells it, let it work until it stops, and read back to you what it said it found and what it changed.
They leave Claude Code and the session standing until you tell them the Quench is coming.

### Confirmations

Once you have walked the track to its end and every act above is done, confirm each of these, and report what each showed:

- `RBRN_SENTRY_HALLMARK` and `RBRN_BOTTLE_HALLMARK` in `rbmm_moorings/rben_ccyolo/rbrn.env` each read the Hallmark its Kludge printed.
- `git status --short`, run in the clone, names no file outside `rbmm_moorings/rbmv_vessels/rbev_bottle_ccyolo/workspace/`, every change the track had you make standing committed, and names at least one file inside it: the edits Claude Code made inside the Bottle stand in the clone, as the track says they will.
- `docker images` lists a Sentry image and a Bottle image, each tagged with the Hallmark the ccyolo Nameplate carries for it.
- Each of the five probes printed what the track expects of it: quote what each printed beside what the track expects.
- `docker ps` lists no container whose name begins `ccyolo-`.
- Name to the attendant what this walk left outside the clone, since no check of the clone asks after any of it and each is theirs to account for by hand: the images each Kludge built and pulled into this machine's Docker, and the sign-in their Claude account made for Claude Code inside the Bottle, which the Quench took down with the Bottle on this machine and which their account may still list.
