<!--
Copyright 2026 Scale Invariant, Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
-->

# TEMPLATE — the licensing statement a repository states at its root

Copy the block under "The statement" into a file at your repository root, fill
every «...» substitution, and delete everything else in this file. Guillemets
mark the boundary between what this kit says and what only you can say; a
guillemet left standing is an unfinished file.

## Where it goes, and what to call it

Name the file something Apache-2.0 gives no legal role — `LICENSING.md` serves.
Do not name it `LICENSE` or `NOTICE`: both carry a defined meaning under
Apache-2.0, and a root file wearing either name says the repository as a whole
is offered on those terms, which is the opposite of what this statement says.

Ownership is parted by directory rather than by repository. Each installed kit
carries its own licence inside its own directory, and that licence governs that
directory. Content the repository carries under someone else's terms — a
third party's documents kept for reference, say — is not yours either, and the
statement excepts it the same way. What is left is yours, and this statement is
where you say so.

`«holders»` names everyone holding copyright in what is yours: one holder, or
several joined by "and" as your agreements name them. Drop the section for
others' content when the repository carries none.

Do not rewrite a header inside an installed kit. The install replaces those
directories whole, so a rewritten header does not survive the next parcel, and
the licence it would misstate is not yours to restate.

## The statement

```markdown
# Licensing

This repository is proprietary, except for what the sections below name.
Copyright «year» «holders». All rights reserved.

No part of it outside those exceptions may be copied, modified, distributed, or
used except under a written agreement with «holders».

## Except for the kits

The directories below are not covered by the statement above. Each was installed
from a parcel, each carries its own licence file inside it, and that licence
governs that directory:

«one line per installed kit directory, e.g.:»
- `Tools/buk/` — Apache License 2.0

## Except for content held by others

The paths below are not covered by the statement above either. Each is carried
here under its own holder's terms, and those terms govern it:

«one line per path carried under someone else's terms, e.g.:»
- `Refs/` — each document under its publisher's terms
```
