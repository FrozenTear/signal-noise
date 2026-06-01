#!/usr/bin/env python3
"""
Backtest fixture for the Intel Arc G3 dedupe regression (THE-628).

After publishing THE-455 (`the-455-intel-arc-g3-handheld-chips`) on 2026-05-29,
the Scanner produced five additional Arc G3 briefs that the published-catalog
dedupe pass failed to flag. All five were killed downstream at Source Checker,
but each cost adapter spend.

Root cause: the Pass B gate in `published_dedupe.PublishedCatalogDedupe`
required overlap on a small hard-coded FOSS-only vendor whitelist (wine,
kernel, vulkan, postgresql, ...). Consumer hardware brands (Intel, Arc, G3)
were not in the whitelist, so Pass B exited with no match for every Arc G3
candidate.

This fixture pins each of the five repeat-brief headlines (taken verbatim
from the issue titles / candidate briefs on THE-467, THE-507, THE-530,
THE-565, THE-620) and asserts that each is now flagged as a duplicate of
`the-455-intel-arc-g3-handheld-chips`.
"""

import os
import tempfile
import unittest

from published_dedupe import PublishedCatalogDedupe


PUBLISHED_REFERENCE_SLUG = "the-455-intel-arc-g3-handheld-chips"


# (issue_id, headline, summary) tuples captured from the live briefs.
ARC_G3_REPEATS = [
    (
        "THE-467",
        "Intel Arc G3 processors target handheld gaming market",
        "Intel's new Arc G3 line aims at handheld gaming devices.",
    ),
    (
        "THE-507",
        "Intel makes a bid for handheld gaming PCs with new Arc G3 processors",
        "Intel releases Arc G3 chips for handheld gaming PCs.",
    ),
    (
        "THE-530",
        "Intel makes a bid for handheld gaming PCs with new Arc G3 processors",
        "Intel announces Arc G3 handheld chips, repeat coverage of the launch.",
    ),
    (
        "THE-565",
        "Intel Arc G3 processors target handheld gaming PC market",
        "Intel positions Arc G3 against AMD Ryzen Z-series for handhelds.",
    ),
    (
        "THE-620",
        "Intel Arc G3 Handheld Gaming Processors Announced",
        "Intel releases new Arc G3 graphics processors targeting handheld "
        "gaming devices, challenging AMD's dominance with Ryzen Z-series chips.",
    ),
]


class TestArcG3DedupeBacktest(unittest.TestCase):
    """All five Arc G3 repeats must dedupe against THE-455."""

    @classmethod
    def setUpClass(cls):
        cls._tmpdir = tempfile.TemporaryDirectory()
        cls.catalog_path = cls._tmpdir.name

        # Minimal catalog: THE-455 plus a handful of unrelated published slugs
        # so the precision assertions in test_no_false_positives_against_catalog
        # have something to chew on.
        seed = [
            PUBLISHED_REFERENCE_SLUG,
            "the-470-steamos-3-8-6-native-hdmi-vrr",
            "the-505-amdgpu-drm-next-hdmi-frl-slip",
            "the-506-amd-zen-6-kernel-32-model-ids",
            "the-580-dell-xps-13-wildcat-lake-18a",
            "the-585-steam-deck-back-in-stock-price-hike",
            "the-563-wine-staging-1110-windows-ink-crash",
        ]
        for slug in seed:
            os.makedirs(os.path.join(cls.catalog_path, slug), exist_ok=True)

    @classmethod
    def tearDownClass(cls):
        cls._tmpdir.cleanup()

    def setUp(self):
        self.dedupe = PublishedCatalogDedupe(self.catalog_path)

    def test_all_five_arc_g3_repeats_flagged(self):
        misses = []
        for issue_id, headline, summary in ARC_G3_REPEATS:
            is_dup, matched_slug, debug = self.dedupe.check_published_duplicate(
                headline, summary
            )
            if not is_dup or matched_slug != PUBLISHED_REFERENCE_SLUG:
                misses.append((issue_id, matched_slug, debug))
        self.assertEqual(
            misses,
            [],
            f"Arc G3 repeat briefs not flagged as duplicates of "
            f"{PUBLISHED_REFERENCE_SLUG}: {misses}",
        )

    def test_match_type_is_a_real_signal(self):
        """Each match must trip on a meaningful Pass B rule (bigram or
        distinctive token overlap), not via a slug equality coincidence."""
        for issue_id, headline, summary in ARC_G3_REPEATS:
            _, _, debug = self.dedupe.check_published_duplicate(headline, summary)
            self.assertIn(
                debug["match_type"],
                {
                    "slug_bigram_overlap",
                    "distinctive_token_overlap",
                    "vendor_token_overlap",
                    "exact_slug_match",
                    "exact_full_match",
                },
                f"{issue_id} matched with unexpected type {debug['match_type']}",
            )

    def test_no_false_positives_against_catalog(self):
        """Unrelated brand/product stories must not collide with the Arc G3
        article. Guards the new bigram/distinctive-token rule against the
        most likely regression mode (over-triggering on shared common nouns)."""
        unrelated = [
            (
                "Sony PS5 Pro Firmware Adds VRR Toggle",
                "Sony ships PS5 Pro firmware update with new VRR controls.",
            ),
            (
                "AMD Ryzen Z2 Extreme Benchmarks Leak",
                "Early Ryzen Z2 Extreme handheld benchmarks surface ahead of launch.",
            ),
            (
                "Nintendo Switch 2 Dev Kit Photos Surface",
                "Leaked Nintendo Switch 2 dev kit images circulate online.",
            ),
            (
                "Samsung 990 Pro SSD Firmware Recall",
                "Samsung pulls 990 Pro SSD firmware over wear-leveling bug.",
            ),
        ]
        for headline, summary in unrelated:
            is_dup, matched_slug, debug = self.dedupe.check_published_duplicate(
                headline, summary
            )
            self.assertFalse(
                is_dup and matched_slug == PUBLISHED_REFERENCE_SLUG,
                f"False positive: '{headline}' flagged as duplicate of "
                f"{PUBLISHED_REFERENCE_SLUG} via {debug.get('match_type')}",
            )


if __name__ == "__main__":
    unittest.main()
