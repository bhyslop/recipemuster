# Set Up a Payor

## The walk

### Goal

A payor stands for this clone: its manor project stands in the attendant's Google Cloud organization, with an OAuth consent screen whose audience is Internal and a desktop OAuth client of its own; the payor's OAuth credential is installed from that client; the Microsoft Entra application the federation trust will ride stands registered, allowing public client flows; and the payor, workforce, federation and depot settings the track has you fill stand filled.

The documents to take up, in this order: the `README.md` at the root of the clone; the onboarding guide it tells you to run; from that guide's start menu, the payor track; and the guided procedures the payor track names in its first four steps, each where the track names it.
Walk the payor track from its first step through the step "Install OAuth credentials", and stop there, before "Instaurate the Manor": the steps after it belong to another walk.
Other documents stand in the clone — the other tracks, the guides other commands print, and the project's code and configuration files beyond those a direction names — and are left unopened, as a newcomer might not think to open them.

The ground this walk stands on: the attendant already administers a Google Cloud organization and a Microsoft Entra tenant, and holds an open Google Cloud billing account, and this walk founds none of the three.
Where a guide offers to skip founding an organization or a tenant to one who already holds it, take that road.
The user the attendant signs in as at the tenant is the citizen a later walk seats as the depot's first governor, and it stands already.
Where the track asks for a value of your own choosing, choose it yourself and say what you chose, save two the attendant names: the cloud prefix and the depot moniker, since the depots this clone founds are asked after later by that prefix alone.

### The browser window

A browser window on a profile of its own, signed in to Google Cloud as the account that administers the attendant's organization and holds the billing account, and to the Microsoft Entra admin center as the tenant's administrator, and to no other account.
You work the consoles in it yourself, save the acts below.

### Acts reserved to the attendant

Each act below is the attendant's to perform, never yours.
Before you hand one over, read what the directions say for it, look at the page, and say whether the two agree; then tell the attendant exactly what to do, and wait until they say it is done.

Reserved to the attendant at the window, granting access — admitting the application to the tenant: in the Microsoft Entra admin center, once you have filled the new application registration as the federation guide directs, the attendant clicks the button that registers it.

Reserved to the attendant at the window, granting access — allowing public client flows: on the new registration's authentication page, the attendant sets public client flows to enabled and saves, or sets the manifest value the federation guide names where the page reads otherwise.

Reserved to the attendant at the window, founding a project: on the project creation page, once you have set the project name, the project ID and the organization as the establish guide directs, the attendant clicks the button that creates the project.

Reserved to the attendant at the window by name, as an act none of the kinds above fits — accepting a service's terms on the account's behalf: at the last step of the OAuth consent screen's configuration, the attendant reads the linked terms, checks the box agreeing to them if they accept, and then clicks the buttons that continue and create the configuration.

Reserved to the attendant at the window, bringing a secret into being: on the payor project's OAuth clients page, once you have set the application type and name as the establish guide directs, the attendant clicks the button that creates the client, downloads its JSON file, and moves that file from wherever their browser saved it into the `client_secrets` subdirectory of the secrets directory, whose path you tell them.

Reserved to the attendant, at the terminal and then at the window, proving identity: the install door waits for a paste at a hidden prompt your own terminal cannot answer, so the attendant runs it at their own terminal, from the clone's root, exactly as the track gives it with the path of the JSON file, and reads back to you what it prints before going on.
At the window they open the address it printed, and where the page asks them to choose an account, they choose the account that administers the organization.

Reserved to the attendant at the window, granting access — consenting to the payor's permissions: on the screens that follow, the attendant reviews the permissions the payor's application asks for, checks each one to grant it, and continues; when the window lands on a page saying the localhost site cannot be reached, they copy that page's whole address and paste it at the install door's waiting prompt.

### Confirmations

Once every act above is done, confirm each of these, and report what each showed:

- `tt/rbw-rpv.ValidatePayorRegime.sh` passes.
- `tt/rbw-rov.ValidateOauthRegime.sh` passes.
- `tt/rbw-rdv.ValidateDepotRegime.sh` passes, its cloud prefix and depot moniker the ones the attendant named.
- `tt/rbw-rfv.ValidateFederationRegime.sh` passes.
- `tt/rbw-rwv.ValidateWorkforceRegime.sh` accepts the organization ID, its pool and session-duration checks failing as the organization guide says they will until a later step sets them.
- At the window, the payor project stands under the attendant's organization, its OAuth consent screen's audience reads Internal, and it holds exactly one OAuth client.
- At the window, the Entra application registration shows public client flows enabled.
- Name to the attendant what this walk founded outside the clone, since no check of the clone asks after any of it and each is theirs to account for by hand: the payor project with its consent screen and OAuth client, and the Entra application registration.
