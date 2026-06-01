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
        # Look for significant overlap in the full slug body (after stripping the prefix)
        vendor_tokens = {
            'wine', 'wine-staging', 'kernel', 'linux', 'gnu',
            'gnome', 'kde', 'firefox', 'chromium', 'safari',
            'vulkan', 'opengl', 'wayland', 'x11', 'drupal',
            'postgresql', 'mysql', 'mariadb'
        }

        for published_slug in self.published_slugs:
            published_stripped = self._strip_the_prefix(published_slug)
            published_tokens = self._tokenize_slug(published_stripped)

            # Calculate overlap: shared tokens between candidate and published
            overlap = candidate_tokens & published_tokens

            # Check for vendor/product token overlap
            vendor_overlap = overlap & vendor_tokens

            # If main vendor tokens match (e.g., 'wine', 'wine-staging'), it's very likely a duplicate
            # Require both vendor match AND significant core token overlap
            if len(vendor_overlap) > 0:
                # Calculate core overlap (significant tokens, not just stop words)
                core_overlap = overlap - {
                    'and', 'or', 'for', 'with', 'in', 'the', 'a', 'an',
                    'is', 'are', 'was', 'were', 'be', 'been'
                }

                # If we have vendor tokens plus other content tokens, it's a match
                if len(core_overlap) >= 3 or len(vendor_overlap) >= 2:
                    debug_info["pass_b_matches"].append({
                        "published_slug": published_slug,
                        "core_overlap_count": len(core_overlap),
                        "vendor_overlap": list(vendor_overlap)
                    })
                    debug_info["match_type"] = "vendor_token_overlap"
                    debug_info["confidence"] = 0.85
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
