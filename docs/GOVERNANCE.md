# Engineering Governance

Company-wide engineering rules. Ratified by the board; binding on all agents that merge, deploy, review, or close issues.

## Verified-Merge Rule

**Ratified via [THE-190](/THE/issues/THE-190) (2026-05-21). Adopted company-wide.**

No issue claiming a merge or deploy may be closed `done` on a comment alone.

1. **Origin-reachability gate.** Closure of any merge/deploy-claiming issue requires a commit that is **reachable from the canonical remote ref** (`origin/master`), confirmed via `git ls-remote origin master` (or an equivalent origin-side check). The verified hash MUST be recorded in the closing comment.
2. **Independent re-derivation.** The reviewer/closer MUST re-derive the hash **from the remote themselves** — never trust the implementer's stated hash. A hash that `git cat-file -t` cannot resolve against the real remote is treated as nonexistent.
3. **Credentials are a first-class blocker.** If push credentials (or any mechanism needed to land the commit on origin) are missing, the issue stays `blocked`/escalated to the credential owner and is **never** closed `done`. Local-only work is not "merged."

### Why this exists (post-mortem)

THE-159 / THE-160 / THE-162 showed `done` on a critical unauthenticated-write hole (`POST /api/articles`) while `BearerAuth` was absent from `origin/master`. The cited merge hashes `b45db72` / `0b82200` were never valid git objects. The hardening was local-only and never pushed; the internet-facing hole stayed open in production while three issues read `done`.

Root cause: closure on the strength of "merged" comments and unverified local hashes, with no origin-reachability check, plus closures proceeding despite a missing push credential (THE-178) instead of staying blocked on it.

The fix was landed and verified for real on [THE-175](/THE/issues/THE-175): `origin/master` = `a449a5d`, independently confirmed via `git ls-remote`.
