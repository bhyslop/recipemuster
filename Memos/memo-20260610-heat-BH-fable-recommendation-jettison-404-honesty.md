# Fable recommendation — rbfl_jettison success message claims a tolerance the code refuses

Date: 2026-06-10
Status: recommendation from the post-wrap ₣BH review (second Fable pass); not yet acted on.

## The behavior

`rbfl_jettison` (rbfld_Delete.sh) ends with `buc_success "Jettisoned or nonexistent: <ref>"`,
but the code accepts only HTTP 202/204 and `buc_die`s on everything else — including the 404 a
nonexistent ref returns from the Docker v2 manifests DELETE. The message describes an idempotent
delete; the code implements a strict one. One of them is wrong.

## Recommended repair

Tolerate 404 as success — i.e., make the code match the message, not the reverse.
Idempotent deletes are the house philosophy (`rbuh_poll_until_gone`, the rbgjl06 convergence loop's
404-is-already-gone), and a cleanup-of-last-resort verb that dies on already-gone is the wrong
shape for its job. If strictness is preferred instead, change the message to "Jettisoned" and let
404 keep dying loud.

## Adjacent note for the record (no action)

The ₢BHAAa docket cinched "honest LRO handling (`rbge_lro_ok` where GAR returns an LRO)"; the
landed verb instead rides the Docker v2 manifests DELETE, where no LRO exists, trusting 202/204.
Live verification confirmed the 202 corresponds to actual deletion, and a missed delete is visible
via `rbw-il`, so this is acceptable — but it is a trust-202 cousin of the gap this heat closed for
package-grain deletes, and worth remembering if jettison ever misbehaves.
