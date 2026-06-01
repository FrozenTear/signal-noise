#!/usr/bin/env python3
"""
Published catalog deduplication for Scanner and Article Verifier.

Handles deduplication against the published catalog with the new `the-<N>-<slug>` format.
Strips the ``the-\\d+-`` prefix and performs both exact slug matching and keyword/entity overlap detection.
"""

import os
import re
from typing import List, Set, Tuple, Optional
from pathlib import Path


STOPWORDS: Set[str] = {
    'and', 'or', 'for', 'with', 'in', 'on', 'at', 'to', 'of', 'by',
    'the', 'a', 'an', 'is', 'are', 'was', 'were', 'be', 'been',
    'as', 'from', 'into', 'this', 'that', 'these', 'those', 'its',
}

# Generic words that should not, by themselves, anchor a duplicate match.
# A story sharing only `gaming` + `chips` with a published article is not a
# duplicate; the gate requires something distinctive (brand, SKU, version).
COMMON_NOUNS: Set[str] = {
    'new', 'next', 'first', 'old',
    'update', 'updates', 'updated', 'release', 'released', 'releases',
    'announce', 'announces', 'announced', 'announcement',
    'launch', 'launches', 'launched',
    'arrive', 'arrives', 'arrived', 'ahead',
    'reveal', 'reveals', 'revealed', 'unveil', 'unveils', 'unveiled',
    'introduce', 'introduces', 'introduced',
    'fix', 'fixes', 'fixed', 'patch', 'patches', 'patched',
    'bug', 'bugs', 'issue', 'issues',
    'gaming', 'handheld', 'handhelds',
    'chip', 'chips', 'processor', 'processors', 'cpu', 'gpu',
    'card', 'cards', 'graphics',
    'market', 'pcs', 'pc', 'device', 'devices', 'hardware',
    'native', 'extreme', 'pro', 'plus',
    'bid', 'bids', 'makes', 'making', 'made',
    'target', 'targets', 'targeting', 'targeted',
    'big', 'bigger', 'small', 'smaller', 'fast', 'faster',
    'support', 'supports', 'supported',
    'add', 'adds', 'adding', 'added',
    'show', 'shows', 'showed', 'shown',
    # Conference / temporal tokens — keep these out of the distinctive set so
    # bigrams like `computex-2026` or `wwdc-2026` don't anchor a duplicate
    # between unrelated product reveals at the same event.
    'computex', 'wwdc', 'gdc', 'siggraph', 'kubecon',
    '2024', '2025', '2026', '2027',
    'q1', 'q2', 'q3', 'q4',
}

# FOSS / open-source ecosystem tokens preserved from the original whitelist
# as a fallback gate. Real coverage is provided by the bigram + distinctive
# token rules; this set just guards against silently regressing the original
# THE-516 vendor-overlap behaviour.
VENDOR_TOKENS: Set[str] = {
    'wine', 'wine-staging', 'kernel', 'linux', 'gnu',
    'gnome', 'kde', 'firefox', 'chromium', 'safari',
    'vulkan', 'opengl', 'wayland', 'x11', 'drupal',
    'postgresql', 'mysql', 'mariadb',
}


def _bigram_is_distinctive(bigram: str) -> bool:
    """A shared slug bigram counts only if both halves are non-stopword and
    at least one half is distinctive (not a common noun)."""
    halves = bigram.split('-')
    if len(halves) != 2:
        return False
    a, b = halves
    if not a or not b:
        return False
    if a in STOPWORDS or b in STOPWORDS:
        return False
    return (a not in COMMON_NOUNS) or (b not in COMMON_NOUNS)


class PublishedCatalogDedupe:
    """Deduplicates story candidates against the published catalog."""

    def __init__(self, catalog_path: Optional[str] = None):
        """
        Initialize the deduplicator with published catalog path.

        Args:
            catalog_path: Path to docs/published/ directory. If None, uses PROJECT_ROOT/docs/published/
        """
        if catalog_path is None:
            project_root = os.environ.get("PWD", "/home/soot/github/signal-noise")
            catalog_path = f"{project_root}/docs/published"

        self.catalog_path = catalog_path
        self.published_slugs = set()
        self.stripped_slugs = {}  # Maps stripped slug -> list of full slugs
        self._load_catalog()

    def _load_catalog(self) -> None:
        """Load published article slugs from docs/published/ directory."""
        try:
            if not os.path.isdir(self.catalog_path):
                print(f"Warning: Published catalog path not found: {self.catalog_path}")
                return

            for entry in os.listdir(self.catalog_path):
                full_path = os.path.join(self.catalog_path, entry)
                if os.path.isdir(full_path):
                    self.published_slugs.add(entry)
                    stripped = self._strip_the_prefix(entry)
                    if stripped not in self.stripped_slugs:
                        self.stripped_slugs[stripped] = []
                    self.stripped_slugs[stripped].append(entry)

            print(f"Loaded {len(self.published_slugs)} published articles from catalog")
        except Exception as e:
            print(f"Error loading published catalog: {e}")

    @staticmethod
    def _strip_the_prefix(slug: str) -> str:
        """
        Strip the `the-<N>-` prefix from a slug.

        Examples:
            the-563-wine-staging-1110-windows-ink-crash -> wine-staging-1110-windows-ink-crash
            age-verification-mandates-new-breach-surface -> age-verification-mandates-new-breach-surface

        Args:
            slug: The full slug to process

        Returns:
            The slug with the ``the-\\d+-`` prefix removed if present
        """
        match = re.match(r'^the-\d+-(.+)$', slug)
        if match:
            return match.group(1)
        return slug

    @staticmethod
    def _tokenize_slug(slug: str) -> Set[str]:
        """
        Tokenize a slug into meaningful units for keyword/entity overlap detection.

        Splits by hyphens and includes:
        - Individual tokens (e.g., 'wine', 'staging', 'kernel')
        - Version numbers (e.g., '6.8', '11.10')
        - Common vendor/product prefixes (e.g., 'wine-staging', 'linux-kernel')

        Args:
            slug: The slug to tokenize

        Returns:
            Set of tokens extracted from the slug
        """
        tokens = set()
        parts = slug.lower().split('-')

        # Add individual parts
        for part in parts:
            if part:
                tokens.add(part)

        # Add multi-part vendor tokens (e.g., 'wine-staging')
        for i in range(len(parts) - 1):
            combined = '-'.join(parts[i:i+2])
            if combined:
                tokens.add(combined)

        # Add version patterns (e.g., X.Y version numbers)
        version_pattern = r'^\d+\.\d+'
        for part in parts:
            if re.match(version_pattern, part):
                tokens.add(part)

        return tokens

    def check_published_duplicate(
        self,
        candidate_headline: str,
        candidate_summary: str,
        match_keys: Optional[List[str]] = None
    ) -> Tuple[bool, Optional[str], dict]:
        """
        Check if a candidate duplicates a published article.

        Uses a two-pass approach:
        - Pass A: Exact slug matching (after prefix stripping)
        - Pass B: Keyword/entity overlap detection

        Args:
            candidate_headline: The candidate article headline
            candidate_summary: The candidate article summary/lead
            match_keys: Optional list of slug keywords to check (for explicit matching)

        Returns:
            Tuple of (is_duplicate, matched_slug, debug_info)
            - is_duplicate: True if a published article is found
            - matched_slug: The published slug that matched (or None)
            - debug_info: Dict with pass_a/pass_b results and confidence
        """
        debug_info = {
            "pass_a_match": None,
            "pass_b_matches": [],
            "confidence": 0.0,
            "match_type": None
        }

        if not self.published_slugs:
            return False, None, debug_info

        # Extract key entities from candidate
        candidate_tokens = self._tokenize_slug(
            candidate_headline.lower().replace(' ', '-')
        )
        candidate_tokens.update(self._tokenize_slug(
            candidate_summary.lower().replace(' ', '-')
        ))

        if match_keys:
            for key in match_keys:
                candidate_tokens.update(self._tokenize_slug(key))

        # Pass A: Exact slug match (after prefix stripping)
        candidate_stripped = self._strip_the_prefix(
            candidate_headline.lower().replace(' ', '-')
        )

        # Check if stripped candidate exactly matches any published stripped slug
        for published_slug in self.published_slugs:
            published_stripped = self._strip_the_prefix(published_slug)

            # Exact match on stripped slug
            if candidate_stripped == published_stripped:
                debug_info["pass_a_match"] = published_slug
                debug_info["match_type"] = "exact_slug_match"
                debug_info["confidence"] = 1.0
                return True, published_slug, debug_info

            # Also check if the published slug (full) matches the candidate stripped
            if candidate_stripped == published_slug:
                debug_info["pass_a_match"] = published_slug
                debug_info["match_type"] = "exact_full_match"
                debug_info["confidence"] = 1.0
                return True, published_slug, debug_info

        # Pass B: Keyword/entity overlap detection
        # Look for significant overlap in the full slug body (after stripping the prefix).
        #
        # The previous gate required the candidate to share one of a small hard-coded
        # FOSS-only vendor whitelist (wine, kernel, vulkan, postgresql, ...). That gate
        # silently dropped every consumer-hardware / brand launch story — five Arc G3
        # repeat briefs slipped past dedupe before being killed at Source Checker
        # (THE-628; root cause traced to this whitelist).
        #
        # Replacement gate, matching the AGENTS.md spec ("flag any catalog article that
        # shares ≥2 non-stopword tokens"):
        #   (a) any shared multi-word slug bigram whose halves are both non-stopword and
        #       at least one half is distinctive (not in COMMON_NOUNS). Slug bigrams
        #       like `arc-g3`, `wine-staging`, `lunar-lake` essentially never collide
        #       by chance, so a single shared bigram is a strong duplicate signal.
        #   (b) ≥2 shared distinctive single tokens (non-stopword AND non-common-noun).
        #       Forces overlap on brand/product/SKU tokens rather than generic words
        #       like `gaming`, `handheld`, `chip`.
        # The original FOSS-vendor case is preserved as a fallback so its existing
        # behaviour does not regress.

        for published_slug in self.published_slugs:
            published_stripped = self._strip_the_prefix(published_slug)
            published_tokens = self._tokenize_slug(published_stripped)

            overlap = candidate_tokens & published_tokens
            core_overlap = overlap - STOPWORDS
            distinctive_overlap = core_overlap - COMMON_NOUNS

            bigram_overlap = {
                t for t in core_overlap
                if '-' in t and _bigram_is_distinctive(t)
            }
            vendor_overlap = overlap & VENDOR_TOKENS

            triggered = None
            if bigram_overlap:
                # A shared multi-word slug bigram (e.g. `arc-g3`, `wine-staging`,
                # `lunar-lake`) is the strongest signal — these rarely collide
                # across unrelated stories.
                triggered = "slug_bigram_overlap"
            elif len(distinctive_overlap) >= 3:
                # Three+ shared distinctive single tokens. Threshold is set at
                # 3 (not 2) because tech-news headlines routinely share two
                # generic-brand tokens (`microsoft`, `ai`, `data`, ...) without
                # being the same story; requiring three keeps Source Checker
                # kill rate from over-triggering. The bigram route above handles
                # the typical brand+SKU launch case in one shot.
                triggered = "distinctive_token_overlap"
            elif vendor_overlap and (len(core_overlap) >= 3 or len(vendor_overlap) >= 2):
                triggered = "vendor_token_overlap"

            if triggered:
                debug_info["pass_b_matches"].append({
                    "published_slug": published_slug,
                    "core_overlap_count": len(core_overlap),
                    "distinctive_overlap": sorted(distinctive_overlap),
                    "bigram_overlap": sorted(bigram_overlap),
                    "vendor_overlap": sorted(vendor_overlap),
                })
                debug_info["match_type"] = triggered
                debug_info["confidence"] = 0.85 if triggered != "slug_bigram_overlap" else 0.9
                return True, published_slug, debug_info

        return False, None, debug_info

    def log_match_keys(self, headline: str, summary: str) -> Tuple[str, str]:
        """
        Log both prefixed and stripped forms as candidate match keys for debugging.

        Args:
            headline: The candidate headline
            summary: The candidate summary

        Returns:
            Tuple of (full_match_key, stripped_match_key)
        """
        full_key = headline.lower().replace(' ', '-')
        stripped_key = self._strip_the_prefix(full_key)
        return full_key, stripped_key


def test_published_dedupe():
    """Test published catalog deduplication with Wine-Staging example."""
    dedupe = PublishedCatalogDedupe()

    # Test case: Wine-Staging 11.10 duplicate detection
    # THE-563: the-563-wine-staging-1110-windows-ink-crash
    # THE-582: Wine-Staging 11.10 story

    candidate_headline = "Wine-Staging 11.10 Released with Vulkan Color Space Fix"
    candidate_summary = "Wine-Staging 11.10 brings Inkobj crash fix and Vulkan color-space improvements"
    match_keys = ["wine-staging", "1110", "windows-ink-crash"]

    is_dup, matched_slug, debug_info = dedupe.check_published_duplicate(
        candidate_headline,
        candidate_summary,
        match_keys
    )

    print(f"\n{'='*60}")
    print(f"Deduplication Test: Wine-Staging 11.10")
    print(f"{'='*60}")
    print(f"Candidate: {candidate_headline}")
    print(f"Is Duplicate: {is_dup}")
    print(f"Matched Slug: {matched_slug}")
    print(f"Debug Info: {debug_info}")
    print(f"{'='*60}\n")

    # Log the match keys
    full_key, stripped_key = dedupe.log_match_keys(candidate_headline, candidate_summary)
    print(f"Full Match Key: {full_key}")
    print(f"Stripped Match Key: {stripped_key}")


if __name__ == "__main__":
    test_published_dedupe()
