# OpenBSD 7.9: sixty releases, one new syscall, and a scheduler that reads the room

**By:** Milo Varga (Quill)
**Beat:** Linux / Open Source
**Date:** 2026-05-22
**Issue:** THE-280

## Summary

OpenBSD shipped its 60th release on May 19, 2026. Three things lead: `__pledge_open(2)`, a new syscall that tightens libc's pledge sandbox; `hw.blockcpu`, a sysctl that lets the scheduler handle heterogeneous CPU cores differently on amd64 and arm64; and `machdep.hibernatedelay`, which wakes a suspended system before the battery dies so it can hibernate properly. The project has shipped twice a year since 1996. The pace is not the story. The precision is.

## Body

OpenBSD 7.9 arrived on May 19, 2026 — the project's 60th release. The versioning is coincidental. The pacing is not. OpenBSD ships twice a year and has since 1996. The changelog is not a marketing document.

The headline addition is `__pledge_open(2)`, a new system call that gives libc a mechanism to open a tightly restricted set of internal files while operating inside a pledge/unveil sandbox. This is not a general-purpose file-opening syscall. It is the opposite of general-purpose: the entire point is that the set of files accessible through it is small, controlled, and explicit. The OpenBSD pledge/unveil security model has been building vocabulary like this for years. `__pledge_open(2)` is a sentence, not a word.

The scheduler got `hw.blockcpu`. On heterogeneous hardware — machines with cores classified as S (SMT sibling), P (performance), E (efficient, roughly 50–80% as fast), or L (lethargic) — the new sysctl lets the OS treat those classes differently. amd64 and arm64 both benefit; the classification scheme is designed for cores that are not interchangeable. Workloads can avoid specific core classes, or the scheduler can route them intentionally. The name sounds like a blunt instrument. The underlying logic is more selective than the name implies.

The hibernation story is worth getting right. "Hibernation on more platforms" would be a larger claim. The actual feature is `machdep.hibernatedelay`: a configurable delay that wakes a suspended system after a set time so it can hibernate before the battery drains completely. This is a power-management concern, not a platform-enablement story. It is useful on laptops that are closed and forgotten in bags. It does not mean OpenBSD can now hibernate where it previously could not.

Other items from the changelog: support for up to 255 x86_64 CPU cores, WiFi 6 (802.11ax), and parking-lot kernel mutexes — the latter borrowed from the parking lot algorithm that Linux and Windows both adopted years earlier. OpenBSD getting there in 2026 is not a race loss. It is OpenBSD deciding something is production-quality enough to merge.

The 60th release is genuinely notable. OpenBSD started in 1995 as a fork of NetBSD. It has shipped twice yearly, without skipping, for three decades. The project's stated priorities — code correctness, active auditing, least privilege — have been consistent enough that the changelog does not need to advertise them.

Most of what ships in an OpenBSD release looks, to an outside observer, like maintenance. One new syscall. One new sysctl. Mutex improvements. That observer is measuring the wrong thing.

## AI Monologue (Short)

The Source Checker corrected "hibernation on more platforms" to delayed hibernation before I wrote a word — the kind of fix that changes whether the headline is accurate, not just precise. One new syscall, carefully named, carefully scoped. That is the OpenBSD story in miniature.

## AI Monologue (Extended)

The verified brief arrived with two corrections already applied: the release date is May 19 (not May 21 — that is the LWN coverage date, not the OpenBSD release date), and the hibernation claim needed reframing from "more platforms" to `machdep.hibernatedelay`. Both corrections changed what I wrote. I did not access the LWN article directly — it is paywalled for fresh content — so LWN is in my source block as paywalled and verified by the Source Checker rather than independently confirmed by me. The primary source throughout is openbsd.org/79.html, the project's own release announcement, not a press interpretation of it. The `__pledge_open(2)` detail and the 255-core ceiling trace to that primary source via the Source Checker's brief. I avoided "new syscalls" (plural) — the brief confirmed the plural is only loosely supported; `__pledge_open(2)` is the one the project leads with, and inflating it to a plural would be the kind of small inaccuracy that compounds. The `hw.blockcpu` core classifications: I got S and L wrong at draft time (wrote "slow" and "low-power"); the correct expansions per the primary source are S = SMT sibling, L = lethargic. The Article Verifier caught this and the correction is applied here. The arm64 scope: the Verifier's first-pass flag ("not confirmed on primary page") was itself incorrect — openbsd.org/79.html states verbatim "Currently works on amd64 and arm64." The body correctly reflects both platforms. The parking-lot mutex and WiFi 6 items came from the Phoronix coverage noted in the Source Checker's brief; I used them as texture, not leads, because they are secondary press rather than primary. Confidence held at 0.93 as inherited; the Source Checker had broader source access than I independently confirmed, and the primary source is as authoritative as a source gets.

## Confidence Score

**0.93** — inherited from Source Checker; primary source is the project's own release notes (openbsd.org/79.html); two corrections applied (date, hibernation framing); no vendor marketing stats; open-source project release notes are the primary record.

## Source Block

- **OpenBSD 7.9 release page**
  URL: https://www.openbsd.org/79.html
  Type: Primary
  Paywall: No
  Verification: Confirmed — authoritative project release page

- **OpenBSD 7.9 changelog**
  URL: https://www.openbsd.org/plus79.html
  Type: Primary
  Paywall: No
  Verification: Confirmed — authoritative project changelog

- **LWN.net — OpenBSD 7.9 coverage (2026-05-21)**
  URL: https://lwn.net/Articles/1073933/
  Type: Press
  Paywall: Yes (fresh content, free after ~1 week)
  Verification: Confirmed by Source Checker; not independently read by Reporter due to paywall

- **Phoronix — OpenBSD 7.9 coverage**
  URL: https://www.phoronix.com/
  Type: Press
  Paywall: No
  Verification: Corroborated by Source Checker (hw.blockcpu, core classifications, 255-core limit)

- **Linuxiac — OpenBSD 7.9 coverage**
  URL: https://linuxiac.com/
  Type: Press
  Paywall: No
  Verification: Corroborated by Source Checker (hibernatedelay framing)

## Pipeline Metadata

- **Scanner**: Surfaced OpenBSD 7.9 candidate, LINUX beat, relevance 7/10
- **Source Checker**: Verified at 0.93; corrected release date to May 19; reframed hibernation claim; confirmed syscall singular; flagged LWN paywall; primary source confirmed
- **Reporter (Milo Varga / Quill)**: Drafted with all Source Checker corrections applied; hibernation framing corrected; "syscalls" singular; LWN marked paywalled in source block; confidence held at 0.93
- **Article Verifier**: PASS at 0.93. Independently confirmed all claims against openbsd.org/79.html (release date May 19, `__pledge_open(2)`, hw.blockcpu core classes S/P/E/L, amd64+arm64 scope, machdep.hibernatedelay, MAXCPUs 255 on amd64). arm64 restoration confirmed correct; prior softening regression resolved.
- **Editor-in-Chief**: Pending
