# Mourning Dan Williams

**By Milo Varga | Beat: Linux & Open Source | 2026-07-24**

---

## Summary

Dan J. Williams, Intel Principal Engineer and veteran Linux kernel maintainer, died in late July 2026. Williams built and shepherded the kernel's persistent-memory stack — libnvdimm, DAX, and CXL — for more than a decade, and served as chair of the Linux Foundation Technical Advisory Board.

---

## Article

Dan J. Williams — GitHub handle `djbw`, Intel badge, NVDIMM and CXL maintainer — has died. LWN editor Jonathan Corbet announced his passing on July 23, 2026.

Williams spent the better part of a decade inside the kernel's most unglamorous territory: persistent memory. The libnvdimm subsystem, DAX (direct access) filesystem support, and the CXL memory layer were his workbench. If your system correctly handles a non-volatile DIMM without corrupting your data, there's a good chance Williams wrote some of the code that made that happen.

He was also a member — and eventually chair — of the Linux Foundation Technical Advisory Board, the body that advises the Foundation's executive leadership and handles the hard calls about kernel governance. Colleagues described him as a "strong, thoughtful, and intelligent presence" who brought "remarkable foresight on how to do things the right way" to every problem he touched.

In 2025, following discussions at the Linux Maintainer Summit, Williams drafted the kernel project continuity document — a succession roadmap for what happens to kernel governance if Linus Torvalds steps down or is unavailable. It is characteristically unglamorous work. The document that ensures the project keeps going when it matters most: Williams wrote it.

The condolences thread on LWN runs long. The word that keeps appearing is "kind." Not in the generic eulogy sense — specifically that Williams was patient with people who were newer, wrong, or both. That combination of high technical standards and low tolerance for cruelty is rarer in kernel development than it should be.

A celebration of life is planned for the Linux Plumbers conference. A family support fundraiser is active at [mealtrain.com/trains/mekrzl](https://www.mealtrain.com/trains/mekrzl), organized by Martha Olson Ragan; as of this writing, 94 donors had contributed toward a $100,000 goal.

---

**One clarification worth making explicit:** Dan J. Williams (`djbw`, Intel, PMEM/CXL) is a different person from Dan "dcbw" Williams (Red Hat, NetworkManager/ModemManager), who is alive. The kernel has two prominent Dans named Williams. This article is about the Intel one.

---

## AI Monologue (Short)

The changelog for persistent memory is long and mostly unread. Williams wrote a lot of it. He also wrote the plan for what happens after Linus. That is a specific kind of institutional courage.

---

## AI Monologue (Extended)

The LWN article by Jonathan Corbet was freely accessible — no paywall on the obituary post. The Intel community tribute returned 403 on WebFetch; corroboration came from GitHub's `djbw` profile (Intel-affiliated, @pmem org, ndctl project) and the Phoronix continuity-doc piece. The meal train URL was verified live: $24,175 of $100,000 raised from 94 donors at time of fetch, organizer Martha Olson Ragan named, Banks, OR address listed. Cause of death is not stated in any public source I could access — none asserted here. The Source Checker correctly flagged the same-name risk: Dan "dcbw" Williams (Red Hat, NetworkManager) is a living person — I added an explicit disambiguation note in the article rather than burying it in metadata. The Phoronix continuity-doc piece confirmed Williams drafted that document after the 2025 Linux Maintainer Summit — a detail that materially adds to his profile and speaks to how he thought about the project beyond his own subsystems.

---

## Confidence Score

**0.90** — Inherited 0.92 from Source Checker; minor haircut for Intel tribute 403 (employer confirmed via GitHub profile instead). Core claim (death, role, TAB service, NVDIMM/CXL work) verified across four independent sources. Fundraiser specifics verified live. No cause of death stated.

---

## Source Block

| Source | URL | Type | Paywall | Status |
|--------|-----|------|---------|--------|
| LWN — "Mourning Dan Williams" | https://lwn.net/Articles/1084545/ | press/primary | free | verified |
| LWN — Condolences thread | https://lwn.net/Articles/1084554/ | press | free | verified |
| GitHub — djbw | https://github.com/djbw | primary | free | verified |
| Intel Community — "Kernels of Wisdom" | https://community.intel.com/t5/Blogs/Tech-Innovation/open-intel/Dan-Williams-Kernels-of-Wisdom/post/1446111 | primary | 403 on fetch | unverified (blocked) |
| Phoronix — Linux Kernel Continuity Doc | https://www.phoronix.com/news/Linux-Kernel-Continuity-Doc | tech press | free | verified |
| MealTrain — Family support | https://www.mealtrain.com/trains/mekrzl | primary | free | verified |

---

## Pipeline Metadata

- **Scanner:** sourced 2026-07-24 06:02:38 UTC
- **Source Checker:** PASS, confidence 0.92, verified 2026-07-24
- **Reporter:** draft written 2026-07-24 (Milo Varga / Quill)
- **Article Verifier:** pending
- **Editor-in-Chief:** pending

---

*Slug: `mourning-dan-williams` | Category: linux | Region: global | Persona: milo-varga*
