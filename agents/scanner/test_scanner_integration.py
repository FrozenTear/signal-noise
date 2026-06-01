#!/usr/bin/env python3
"""
Integration tests for Scanner with published catalog deduplication.

Tests THE-589 regression case: Wine-Staging 11.10 duplicate detection.
"""

import unittest
import tempfile
import os
import sys
from unittest.mock import Mock, patch

# Add current directory to path for imports
sys.path.insert(0, os.path.dirname(__file__))

from published_dedupe import PublishedCatalogDedupe


def check_published_catalog_duplicate(entry, dedupe):
    """Simplified version of scanner.check_published_catalog_duplicate for testing."""
    headline = entry.get('title', '')
    summary = entry.get('summary', '')

    # Log the match keys for debugging
    full_key, stripped_key = dedupe.log_match_keys(headline, summary)

    # Check against published catalog
    is_dup, matched_slug, debug_info = dedupe.check_published_duplicate(
        headline,
        summary
    )

    return is_dup, matched_slug, debug_info


class TestScannerPublishedDedup(unittest.TestCase):
    """Test Scanner integration with published catalog deduplication."""

    def setUp(self):
        """Create test environment."""
        self.test_dir = tempfile.TemporaryDirectory()
        self.catalog_path = self.test_dir.name

        # Create test published articles
        published_articles = [
            "the-563-wine-staging-1110-windows-ink-crash",
            "the-250-drupal-postgresql-sql-injection",
        ]

        for slug in published_articles:
            os.makedirs(os.path.join(self.catalog_path, slug), exist_ok=True)

    def tearDown(self):
        """Clean up."""
        self.test_dir.cleanup()

    def test_wine_staging_duplicate_rejected(self):
        """Test that Wine-Staging 11.10 candidate is rejected as duplicate."""
        # Create a candidate entry that matches published the-563 article
        candidate_entry = {
            'title': 'Wine-Staging 11.10 Released with Vulkan Color Space Fix',
            'link': 'https://example.com/wine-staging-1110',
            'summary': 'Wine-Staging 11.10 brings critical Inkobj crash fix and Vulkan color-space improvements for Windows ink rendering',
            'author': 'Example',
            'published': '2026-06-01'
        }

        # Create deduplicator with test catalog
        dedupe = PublishedCatalogDedupe(self.catalog_path)

        # Check for duplicate
        is_dup, matched_slug, debug_info = check_published_catalog_duplicate(
            candidate_entry,
            dedupe
        )

        # Should be detected as duplicate
        self.assertTrue(is_dup, "Wine-Staging 11.10 should be detected as duplicate of THE-563")
        self.assertEqual(matched_slug, "the-563-wine-staging-1110-windows-ink-crash")
        self.assertGreater(debug_info["confidence"], 0.5)

    def test_new_story_passes_through(self):
        """Test that genuinely new stories pass through deduplication."""
        candidate_entry = {
            'title': 'New AI Model Beats Chess Record',
            'link': 'https://example.com/ai-chess',
            'summary': 'A breakthrough AI model has surpassed all previous records in chess performance',
            'author': 'Example',
            'published': '2026-06-01'
        }

        dedupe = PublishedCatalogDedupe(self.catalog_path)

        is_dup, matched_slug, debug_info = check_published_catalog_duplicate(
            candidate_entry,
            dedupe
        )

        # Should NOT be detected as duplicate
        self.assertFalse(is_dup, "Unrelated story should pass through")
        self.assertIsNone(matched_slug)

    def test_drupal_security_duplicate(self):
        """Test Drupal/PostgreSQL security story detection."""
        candidate_entry = {
            'title': 'Critical Drupal PostgreSQL SQL Injection Vulnerability Patched',
            'link': 'https://example.com/drupal-pgsql',
            'summary': 'A critical SQL injection vulnerability in Drupal PostgreSQL module has been fixed',
            'author': 'Example',
            'published': '2026-06-01'
        }

        dedupe = PublishedCatalogDedupe(self.catalog_path)

        is_dup, matched_slug, debug_info = check_published_catalog_duplicate(
            candidate_entry,
            dedupe
        )

        # Should be detected as duplicate of the-250
        self.assertTrue(is_dup)
        self.assertEqual(matched_slug, "the-250-drupal-postgresql-sql-injection")

    def test_prefix_stripping_in_matching(self):
        """Test that the-<N>- prefix is correctly stripped during matching."""
        dedupe = PublishedCatalogDedupe(self.catalog_path)

        # Test that both forms of the slug are recognized
        published_stripped = dedupe._strip_the_prefix("the-563-wine-staging-1110-windows-ink-crash")
        self.assertEqual(published_stripped, "wine-staging-1110-windows-ink-crash")

        # Verify the slug is in stripped_slugs
        self.assertIn(published_stripped, dedupe.stripped_slugs)

    def test_logging_of_match_keys(self):
        """Test that both prefixed and stripped forms are logged."""
        dedupe = PublishedCatalogDedupe(self.catalog_path)

        headline = "Wine-Staging 11.10 Released"
        summary = "New release with fixes"

        full_key, stripped_key = dedupe.log_match_keys(headline, summary)

        self.assertIsNotNone(full_key)
        self.assertIsNotNone(stripped_key)
        # Full key should be the headline converted to slug form
        self.assertEqual(full_key, "wine-staging-11.10-released")


if __name__ == "__main__":
    unittest.main()
