# <a id="rbsoe_scenario"></a>Configure Your Repository's Environment

## <a id="rbsve_walk"></a>The walk

### Goal

Your clone's environment stands configured as the onboarding's first track teaches it: the station file beside the clone validates, the secrets directory the repository settings name stands with its `client_secrets` subdirectory inside it, and you have run every render and validate door the track names and read what each one said.

The documents to take up, in this order: the `README.md` at the root of the clone; the onboarding guide it tells you to run; and, from that guide's start menu, the track that configures your repository's environment, which the menu tells a newcomer to run first.
Walk that track to its last step, "Next steps", and stop there: the menu's other tracks belong to other walks.
Other documents stand in the clone — the other tracks, the guides other commands print, and the project's code and configuration files beyond those a direction names — and are left unopened, as a newcomer might not think to open them.

The clone was prepared before you arrived, and its station file already stands beside it, written for this walk.
Where the track offers to create a station file on a fresh clone, judge that direction against what you find, but do not run it: it would write over the station file this walk stands on.

This walk needs no browser window, and no act in it is reserved to the attendant: every act the track directs is yours.

### Confirmations

Once you have walked the track to its end, confirm each of these, and report what each showed:

- `tt/buw-rsv.ValidateStationRegime.sh` passes.
- The secrets directory the repository settings name, `RBRR_SECRETS_DIR` in `rbmm_moorings/rbrr.env` read against the clone's root, stands and holds an empty `client_secrets` subdirectory.
- `tt/rbw-rrv.ValidateRepoRegime.sh` passes.
- `tt/rbw-rdv.ValidateDepotRegime.sh` fails, naming the blank depot fields, as the track says it will before a depot is levied.
