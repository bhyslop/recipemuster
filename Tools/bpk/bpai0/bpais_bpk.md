## Bash Parcel Kit (BPK)

BPK carries kit code between repositories by parcel: a parcel is formed at its
forge's main checkout, on the forge's trunk, and installed into the repository
that receives it through the engine the parcel brought.

**A received kit directory is never edited in place.** A file under a kit this
repository receives by parcel changes only by a new parcel from that kit's forge.

**Excursus**: `Tools/bpk/bpai0/bpaie_portage.md` carries the receiving pace's
docket shape and the movement it performs. Read it before you close a pace whose
diff touches a file under a kit directory another repository receives by parcel,
before you slate any crossing, and when you mount a receiving pace.
