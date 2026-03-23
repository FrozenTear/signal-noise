# Scanner Deduplication Fix — Design Specification

**Date**: 2026-03-23
**Issue**: SIG-113
**Scope**: Fix RSS deduplication bug where each Scanner poll creates fresh duplicate story candidate issues

---

## Problem Statement

Currently, each Scanner heartbeat creates **fresh Paperclip issues for every story candidate**, even if identical stories were created in previous polls. This floods the triage pipeline:

- 89 candidates from a single poll were actually ~10 unique stories repeated ~9 times each
- Editor-in-Chief had to manually cancel 83 duplicates
- Root cause: Scanner only deduplicates **in-memory** within a single poll; doesn't check existing Paperclip issues

---

## Solution Overview

Before creating a Paperclip issue, **query existing story candidates from the Paperclip API** and filter out duplicates using a two-tier strategy:

1. **URL matching** (exact, fast path) — catch articles from the same source
2. **Title similarity** (fallback, 0.85 threshold) — catch story variants from different outlets

Only create issues for **truly new candidates** that don't match existing issues from the last 7 days.

---

## Design Details

### Data Flow

```
Poll RSS feeds → Parse candidates → Query Paperclip API → Deduplicate → Create issues
```

### Deduplication Strategy

**Tier 1: URL Matching**
- For each candidate, check if any existing issue has the same source URL
- Fast: O(1) lookup with URL deduplication
- High precision: eliminates exact duplicates

**Tier 2: Title Similarity**
- If URL is new, compare headline against existing issue titles using Levenshtein distance
- Existing threshold: 0.85 (already tuned in `scanner.rs`)
- Catches: same story reported by different outlets, headlines with minor variations

**Fallback**: If Paperclip API fails, proceed with in-memory dedup only (graceful degradation)

### Lookback Window

**7 days** balances:
- Catches most duplicates within typical news cycle
- Reduces API query load (not scanning all issues since project start)
- Allows legitimate follow-up coverage after a week

### API Query Design

**Endpoint**: `GET /api/companies/{companyId}/issues`

**Query parameters**:
- `status=todo,in_progress,in_review` (story candidates in active triage)
- Beat-specific filter (e.g., `q=beat:tech` when processing tech feed)
- Time constraint: created after `now - 7 days`

**Response processing**: Extract title and source URL from each existing issue for deduplication matching

### Error Handling

| Scenario | Behavior |
|----------|----------|
| API fails on query | Log warning, proceed with in-memory dedup only |
| Network timeout | Retry once, then fallback |
| Malformed response | Log and skip that issue, continue with others |

Degradation principle: **Never block story publication on API failures**

---

## Implementation Scope

### Modified Files

**`src/bin/scanner.rs`** (main changes):
1. Add `query_existing_candidates()` async function
   - Calls Paperclip API to fetch recent issues
   - Returns `Vec<(String, Vec<String>)>` of (issue_title, source_urls)

2. Add `filter_new_candidates()` function
   - Takes: candidates, existing issues
   - Returns: candidates that aren't duplicates
   - Uses title similarity + URL matching

3. Update issue creation loop
   - Call `query_existing_candidates()` once at start
   - Filter candidates before loop
   - Log: "Skipped N duplicates" at end

### New Functions

```rust
async fn query_existing_candidates(
    client: &Client,
    api_url: &str,
    company_id: &str,
    auth_header: &str,
    beat: &str,
) -> Result<Vec<(String, Vec<String>)>>

fn filter_new_candidates(
    candidates: Vec<StoryCandidate>,
    existing: Vec<(String, Vec<String>)>,
) -> Vec<StoryCandidate>
```

### No Changes to `scanner.rs`
- Keep existing `deduplicate_candidates()` for in-memory dedup (first pass)
- Reuse `string_similarity()` function for title matching

---

## Testing Strategy

### Unit Tests
- `test_filter_duplicates_by_url()` — exact URL match eliminates candidate
- `test_filter_duplicates_by_title()` — similar headline (>0.85) eliminates candidate
- `test_filter_preserves_new_stories()` — new URLs and titles pass through

### Integration Tests
- Mock Paperclip API response with sample issues
- Verify deduplication against mocked existing candidates
- Test API failure graceful degradation

### Manual Verification
1. Run scanner with known duplicates in pipeline
2. Verify duplicates are skipped (check logs for "Skipped N duplicates")
3. Verify new stories are still created
4. Run SIG-115 feed fix concurrently to validate against real Tech beat stories

---

## Success Criteria

✓ Each unique story appears **exactly once** per beat in the triage pipeline
✓ No manual duplicate cleanup required after scanner runs
✓ Deduplication works across RSS and gnews.io sources
✓ API failures don't block story publication
✓ Logs clearly report duplicate counts and skipped candidates

---

## Timeline & Dependencies

- **Dependency**: Requires Paperclip API access (already available in `bin/scanner.rs`)
- **Related work**: SIG-115 (Tech feed fix) resolves the zero-Tech-stories issue; this fix prevents duplicates
- **No blocking dependencies**

---

## Notes

- The 7-day lookback window is configurable via `feeds.toml` if needed (future enhancement)
- Title similarity threshold (0.85) matches existing `scanner.rs` logic — consistency across codebase
- Graceful degradation to in-memory dedup ensures scanner continues even if Paperclip API is slow
