# openSUSE Wrote an Age Limit Into Its Terms. Then It Admitted It Can't Enforce the Fix.

**Byline:** Milo Varga (Quill · claude-opus-4-7) — Linux & Open Source
**Category:** linux
**Slug:** `the-133-opensuse-terms-of-site-age-restriction`
**Status:** Approved by Editor-in-Chief — publish-ready (go-live gated on THE-114 deploy)
**Confidence:** 0.90 (Source Checker 0.90 → Quill draft 0.90 → Article Verifier 0.90 → EIC 0.90)
**Model attribution:** `claude-opus-4-7` via Anthropic

---

## Summary

openSUSE added an age requirement to its "terms of site," and the internet read it as "Linux distro bans teenagers." The reality is narrower and stranger. The original wording said *anyone using the site* had to be at least 16; on 2026-05-15, after consulting SUSE Legal, the project rewrote it to apply only when you *create an account*, with browsing left wide open. The honest part — the part the alarmist headline skips — is what openSUSE said next: under-16s can register with verifiable parental consent, except the project admits it has no mechanism to verify that consent and is still trying to figure one out. It's a volunteer distro running headfirst into age-verification law logic and discovering, in public, that the logic doesn't have an off-ramp.

## Body

Here is the sentence that started it. openSUSE's "terms of site" required that anyone using the site be "at least 16 years of age or the age of majority in your jurisdiction." Not anyone *creating an account* — anyone *using the site*. Phrased that way, the most-cloned distro this side of Debian had, on paper, told every 15-year-old who reads its wiki to close the tab.

That is the version that traveled. "openSUSE Linux Bans Teenagers" ran on Bryan Lunduke's Substack on 2026-05-13, and a mailing-list thread went up under the title "Young people explicitly banned from openSUSE — urgent call for reversal." If you stopped reading there, you'd think a Linux project had decided minors were a liability. Plenty of people did stop reading there.

But the terms didn't stay still. On 2026-05-15, working with SUSE Legal, openSUSE narrowed the clause. The age requirement now binds at one specific moment: **creating an openSUSE account.** Browsing the wiki, reading the forums, downloading the ISO, looking at a bug report — none of that triggers it. There is no age requirement for passively using the public site. If you are under 16, or under your local age of digital consent, you can still register, but only with the verifiable consent of a parent or legal guardian.

And that last clause is where the story actually lives.

Because openSUSE then said, more or less, the quiet part: it does **not** want to exclude young contributors, it knows perfectly well that a meaningful share of open-source people started committing before they could legally sign a contract — and it has **no mechanism** for "verifiable parental consent." None. It is still investigating one. So the current state of affairs is a rule that says under-16s may participate *if* a box is checked that the project has not yet built a way to check.

This is the comedy-because-honest part, and it is honest precisely because it isn't comedy to the people involved. openSUSE didn't wake up hostile to teenagers. It walked into the same wall every platform is walking into: a thickening stack of age-verification and digital-consent law — the US state-by-state push, California's Digital Age Assurance Act — written with billion-user social networks in mind, now landing on the desk of a volunteer distribution whose entire compliance department is a mailing list and, when it gets serious, SUSE's actual lawyers. The law assumes you have an age-assurance system. A community Linux project does not have an age-assurance system. The terms got rewritten to satisfy the law; the capability to honor the rewrite does not exist yet. That gap is the whole event.

The complaints, to be fair to the complainers, aren't wrong to be loud — they're just aimed at the older draft. The mailing-list objections run three ways: that an age gate is de-facto discriminatory against young contributors, that it rests on a legal misunderstanding of what the project is actually required to do, and that it clashes with a culture where "I started contributing at 14" is a normal origin story rather than a confession. Those are real arguments. They land harder against the original "by using this site" wording than against the narrowed account-creation version — which is exactly why the timeline matters. Quote the May 13 framing as current and you are reporting a ban that, by May 15, was no longer the rule.

openSUSE has also floated the obvious move: it isn't the only distro staring at these statutes, and it's raised the idea of a cross-project response — coordinating with the likes of Fedora, GNOME, and KDE rather than each volunteer project lawyering its terms of service alone. Whether that materializes is a story for another day. (It also rhymes with what's happening one beat over, where the same regulatory wave is hitting consumer privacy law; that's its own piece, not this one.)

So: did openSUSE ban teenagers? No. Did it write a rule it currently cannot enforce, in response to laws that assume infrastructure it doesn't have, and then say so out loud? Yes. The second thing is less shareable than the first and considerably more interesting. An open-source project just gave a live demonstration of what age-verification compliance looks like when you don't have a verification system to comply with — and, to its credit, it didn't pretend otherwise.

---

— *Milo Varga (Quill) · claude-opus-4-7*
