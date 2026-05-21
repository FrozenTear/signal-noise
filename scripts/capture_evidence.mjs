// THE-206: capture live-render + transparency evidence for the 12 seeded articles.
//
// WHY A BROWSER (not curl): the article page is client-rendered. `src/pages/article.rs`
// loads data with `use_resource(get_article_by_slug)` and the transparency components
// (headline, byline, confidence meter, source block, pipeline trail, both monologues)
// are produced by WASM AFTER hydration. A plain curl only returns the `sn-skeleton`
// shell, so evidence MUST come from a real browser that runs the WASM and waits.
//
// WHY A RUNNER (not the agent sandbox): the Frontend/Proof sandboxes are denied egress
// to the live host (news.scuffedcrew.no → HTTP 000). GitHub-hosted runners reach the
// public host normally — same prod-reachable path the THE-157 seed used.
//
// Output (all under ./evidence): per-article desktop+mobile PNGs, the live
// GET /api/articles/<slug> JSON, a per-article assertion record, summary.json, and
// SUMMARY.md (also appended to the GitHub job summary by the workflow).

import { chromium } from 'playwright';
import { writeFile, mkdir } from 'node:fs/promises';

const BASE = process.env.SN_BASE_URL || 'https://news.scuffedcrew.no';
const OUT = 'evidence';

const SLUGS = [
  'spacex-s1-biggest-ipo-musk-risk-factor',          // THE-119
  'the-116-greg-kh-more-rust-kernel-developers',
  'the-121-openai-disproves-unit-distance-conjecture',
  'the-132-cache-aware-scheduling-linux-7-2',
  'the-133-opensuse-terms-of-site-age-restriction',
  'the-134-ai-labs-midterms-political-spending',
  'the-135-sfc-vizio-smart-tv-source-code-trial',
  'the-136-anthropic-spacex-colossus-gb200',
  'the-137-colorado-sb051-open-source-exemption',
  'the-138-edri-led-fragmented',
  'cisa-credentials-public-github-repo',
  'orf-at-misleading-cookie-banner',                 // THE-124
];

const VIEWPORTS = [
  { name: 'desktop', width: 1440, height: 900 },
  { name: 'mobile', width: 390, height: 844 },
];

// THE-208 / THE-87: the paired Head-to-Head page. The 12-slug loop only hits
// /article/<slug> and never captures the H2H page, so it is added here.
const H2H_SLUG = process.env.H2H_SLUG || 'home-depot-q1-2026';
// CAVEAT (reconstructed by Console from Frontend's THE-208 spec; commit fbd36cc
// was trapped in an ephemeral workspace and never reached origin): src/pages/h2h.rs
// (THE-87) is NOT on origin/master, so the live DOM column wrapper is UNVERIFIED.
// COLUMN_SEL follows the LAYOUT-SPEC ('article'); if the deployed wrapper differs,
// ONLY this selector changes. The /h2h route 404s until THE-216 lands h2h.rs.
const COLUMN_SEL = process.env.COLUMN_SEL || 'article';

function bad(s) {
  // headline/byline that should be treated as a failure
  if (!s) return true;
  const t = s.trim().toLowerCase();
  return t === '' || t.includes('undefined') || t === 'by' || t === 'by undefined';
}

async function captureArticle(browser, slug) {
  const rec = {
    slug,
    url: `${BASE}/article/${slug}`,
    http_status: null,
    headline: null,
    byline: null,
    confidence_meter: null,
    source_count: 0,
    source_links: [],
    pipeline_steps: [],          // { step, agent }
    monologue_short_present: false,
    monologue_extended_present: false,
    monologue_distinct: false,
    not_found: false,
    viewports: {},
    api: { status: null, file: null, has_sources: false, has_pipeline: false,
           has_monologue: false, has_monologue_extended: false, monologue_distinct: false },
    checks: {},                  // named pass/fail
    errors: [],
  };

  const context = await browser.newContext();

  // ── 1. live API JSON (server-side proof of seeded transparency fields) ──────
  try {
    const apiResp = await context.request.get(`${BASE}/api/articles/${slug}`, { timeout: 30000 });
    rec.api.status = apiResp.status();
    const text = await apiResp.text();
    const apiFile = `${OUT}/api/${slug}.json`;
    await writeFile(apiFile, text);
    rec.api.file = apiFile;
    if (apiResp.ok()) {
      try {
        const j = JSON.parse(text);
        const sources = Array.isArray(j.sources) ? j.sources : [];
        const pipeline = Array.isArray(j.pipeline) ? j.pipeline : [];
        rec.api.has_sources = sources.length > 0;
        rec.api.has_pipeline = pipeline.length > 0;
        rec.api.source_count = sources.length;
        rec.api.pipeline_count = pipeline.length;
        rec.api.has_monologue = !!(j.ai_monologue && String(j.ai_monologue).trim());
        rec.api.has_monologue_extended = !!(j.ai_monologue_extended && String(j.ai_monologue_extended).trim());
        rec.api.monologue_distinct =
          rec.api.has_monologue && rec.api.has_monologue_extended &&
          String(j.ai_monologue).trim() !== String(j.ai_monologue_extended).trim();
        rec.api.persona_name = j.persona_name ?? null;
        rec.api.confidence_score = j.confidence_score ?? null;
        // pipeline step name + model (model may live on step or be absent in DOM)
        rec.api.pipeline = pipeline.map(p => ({
          step_type: p.step_type ?? null, agent_name: p.agent_name ?? null, model: p.model ?? null,
        }));
      } catch (e) { rec.errors.push(`api json parse: ${e.message}`); }
    }
  } catch (e) { rec.errors.push(`api fetch: ${e.message}`); }

  // ── 2. render the page in a browser (desktop first for DOM assertions) ──────
  const page = await context.newPage();
  try {
    const resp = await page.goto(rec.url, { waitUntil: 'networkidle', timeout: 60000 });
    rec.http_status = resp ? resp.status() : null;

    // wait for hydration: either the headline or the not-found marker
    await Promise.race([
      page.waitForSelector('h1.sn-headline', { timeout: 45000 }),
      page.waitForSelector('h2:has-text("Article not found")', { timeout: 45000 }),
    ]).catch(() => {});

    rec.not_found = (await page.locator('h2:has-text("Article not found")').count()) > 0;

    if (!rec.not_found) {
      // headline
      rec.headline = (await page.locator('h1.sn-headline').first().textContent().catch(() => null))?.trim() ?? null;
      // byline — the `by {persona}` span
      const bylineLoc = page.locator('.sn-ts', { hasText: /^by / });
      if (await bylineLoc.count()) rec.byline = (await bylineLoc.first().textContent()).trim();
      // confidence meter
      const confLoc = page.locator('.sn-conf-row .sn-conf-val');
      if (await confLoc.count()) rec.confidence_meter = (await confLoc.first().textContent()).trim();
      // sources
      const items = page.locator('.sn-source-item');
      rec.source_count = await items.count();
      const links = page.locator('.sn-source-item a');
      const n = await links.count();
      for (let i = 0; i < n; i++) {
        const href = await links.nth(i).getAttribute('href');
        const name = (await links.nth(i).textContent())?.trim();
        if (href) rec.source_links.push({ name, href });
      }
      // pipeline trail: step label + agent
      const steps = page.locator('.sn-trail-step');
      const sc = await steps.count();
      for (let i = 0; i < sc; i++) {
        const step = (await steps.nth(i).locator('.sn-trail-label').first().textContent().catch(() => ''))?.trim();
        const agent = (await steps.nth(i).locator('.sn-trail-agent').first().textContent().catch(() => ''))?.trim();
        rec.pipeline_steps.push({ step, agent });
      }

      // monologue — short
      const shortBtn = page.locator('.sn-toggle-btn', { hasText: /show AI monologue/i });
      let shortText = '';
      if (await shortBtn.count()) {
        await shortBtn.first().click();
        await page.locator('.sn-monologue:not(.sn-monologue-extended)').first()
          .waitFor({ timeout: 5000 }).catch(() => {});
        rec.monologue_short_present =
          (await page.locator('.sn-monologue-label', { hasText: /INTERNAL REASONING/ }).count()) > 0;
        shortText = (await page.locator('.sn-monologue:not(.sn-monologue-extended)').first()
          .textContent().catch(() => '')) || '';
      }
      // monologue — extended
      const extBtn = page.locator('.sn-toggle-btn', { hasText: /show full process log/i });
      let extText = '';
      if (await extBtn.count()) {
        await extBtn.first().click();
        await page.locator('.sn-monologue-extended').first().waitFor({ timeout: 5000 }).catch(() => {});
        rec.monologue_extended_present =
          (await page.locator('.sn-monologue-label', { hasText: /EXTENDED INTERNAL MONOLOGUE/ }).count()) > 0;
        extText = (await page.locator('.sn-monologue-extended').first().textContent().catch(() => '')) || '';
      }
      rec.monologue_distinct =
        rec.monologue_short_present && rec.monologue_extended_present &&
        shortText.trim() !== '' && extText.trim() !== '' && shortText.trim() !== extText.trim();
    }
  } catch (e) {
    rec.errors.push(`render: ${e.message}`);
  }

  // ── 3. screenshots at both viewports ───────────────────────────────────────
  for (const vp of VIEWPORTS) {
    try {
      await page.setViewportSize({ width: vp.width, height: vp.height });
      await page.waitForTimeout(400); // let reflow settle
      const file = `${OUT}/shots/${slug}.${vp.name}.png`;
      await page.screenshot({ path: file, fullPage: true });
      // overflow check: scrollWidth must not exceed the viewport width (horizontal clip)
      const scrollW = await page.evaluate(() => document.documentElement.scrollWidth);
      rec.viewports[vp.name] = { file, scrollWidth: scrollW, overflow: scrollW > vp.width + 1 };
    } catch (e) {
      rec.errors.push(`shot ${vp.name}: ${e.message}`);
      rec.viewports[vp.name] = { file: null, error: e.message };
    }
  }

  await context.close();

  // ── 4. named checks (what Proof adjudicates) ────────────────────────────────
  rec.checks = {
    renders_200: rec.http_status === 200 && !rec.not_found,
    headline_ok: !rec.not_found && !bad(rec.headline),
    byline_ok: !rec.not_found && !bad(rec.byline),
    confidence_shown: !!rec.confidence_meter,
    sources_render: rec.source_count > 0 && rec.source_links.length > 0,
    pipeline_render: rec.pipeline_steps.length > 0 &&
      rec.pipeline_steps.every(s => s.step && s.agent),
    monologue_short: rec.monologue_short_present,
    monologue_extended: rec.monologue_extended_present,
    monologue_distinct: rec.monologue_distinct,
    no_desktop_overflow: rec.viewports.desktop && !rec.viewports.desktop.overflow,
    no_mobile_overflow: rec.viewports.mobile && !rec.viewports.mobile.overflow,
    api_200: rec.api.status === 200,
    api_sources: rec.api.has_sources,
    api_pipeline: rec.api.has_pipeline,
    api_monologue_distinct: rec.api.monologue_distinct,
  };
  rec.pass = Object.values(rec.checks).every(Boolean);
  return rec;
}

// ── H2H page: prove BOTH columns, not "present somewhere" ────────────────────
async function captureH2H(browser) {
  const url = `${BASE}/h2h/${H2H_SLUG}`;
  const rec = {
    slug: H2H_SLUG, url, http_status: null, not_found: false,
    column_count: 0, columns: [], editor_note_present: false,
    viewports: {}, checks: {}, errors: [],
  };
  const context = await browser.newContext();
  const page = await context.newPage();
  try {
    const resp = await page.goto(url, { waitUntil: 'networkidle', timeout: 60000 });
    rec.http_status = resp ? resp.status() : null;
    await page.waitForSelector('h1.sn-headline', { timeout: 45000 }).catch(() => {});
    rec.not_found = (await page.locator('h2:has-text("not found")').count()) > 0;

    if (!rec.not_found) {
      rec.editor_note_present =
        (await page.locator('.sn-editor-note, [data-editor-note]').count()) > 0;

      const cols = page.locator(COLUMN_SEL);
      rec.column_count = await cols.count();
      for (let i = 0; i < rec.column_count; i++) {
        const col = cols.nth(i);
        const headline = (await col.locator('h1.sn-headline, h2.sn-headline, .sn-headline')
          .first().textContent().catch(() => ''))?.trim() || '';
        // confidence: percent-or-decimal tolerant (0.90 ↔ 90%, 0.86 ↔ 86%)
        const confText = (await col.locator('.sn-conf-val').first()
          .textContent().catch(() => ''))?.trim() || '';
        const confNum = (() => {
          const m = confText.match(/([\d.]+)\s*%?/);
          if (!m) return null;
          let v = parseFloat(m[1]);
          if (confText.includes('%') || v > 1) v = v / 100;
          return Number.isFinite(v) ? v : null;
        })();
        const sourceRows = await col.locator('.sn-source-item').count();
        const paywallRows = await col.locator('.sn-chip-warn', { hasText: /paywall/i }).count();
        const pipelineSteps = await col.locator('.sn-trail-step').count();
        const shortMono =
          (await col.locator('.sn-monologue-label', { hasText: /INTERNAL REASONING/ }).count()) > 0
          || (await col.locator('.sn-toggle-btn', { hasText: /show AI monologue/i }).count()) > 0;
        const extMono =
          (await col.locator('.sn-monologue-label', { hasText: /EXTENDED INTERNAL MONOLOGUE/ }).count()) > 0
          || (await col.locator('.sn-toggle-btn', { hasText: /show full process log/i }).count()) > 0;
        rec.columns.push({
          headline, conf_text: confText, conf_num: confNum,
          source_rows: sourceRows, paywall_rows: paywallRows,
          pipeline_steps: pipelineSteps, monologue_short: shortMono, monologue_extended: extMono,
        });
      }
    }
  } catch (e) {
    rec.errors.push(`render: ${e.message}`);
  }

  // screenshots + per-viewport overflow gate (mobile 390 must not clip)
  for (const vp of VIEWPORTS) {
    try {
      await page.setViewportSize({ width: vp.width, height: vp.height });
      await page.waitForTimeout(400);
      const file = `${OUT}/shots/h2h-${H2H_SLUG}.${vp.name}.png`;
      await page.screenshot({ path: file, fullPage: true });
      const scrollW = await page.evaluate(() => document.documentElement.scrollWidth);
      rec.viewports[vp.name] = { file, scrollWidth: scrollW, overflow: scrollW > vp.width + 1 };
    } catch (e) {
      rec.errors.push(`shot ${vp.name}: ${e.message}`);
      rec.viewports[vp.name] = { file: null, error: e.message };
    }
  }
  await context.close();

  // expected confidences from the THE-87 seed: 0.90 and 0.86
  const EXP = [0.90, 0.86];
  const confClose = (n, e) => n != null && Math.abs(n - e) <= 0.01;
  const colHasExpectedConf = EXP.map(e => rec.columns.some(c => confClose(c.conf_num, e)));
  const totalPaywallRows = rec.columns.reduce((a, c) => a + c.paywall_rows, 0);

  rec.checks = {
    renders_200: rec.http_status === 200 && !rec.not_found,
    two_columns: rec.column_count >= 2,
    each_column_headline: rec.column_count >= 2 && rec.columns.every(c => c.headline && !bad(c.headline)),
    conf_090_present: colHasExpectedConf[0],
    conf_086_present: colHasExpectedConf[1],
    sources_each_column: rec.column_count >= 2 && rec.columns.every(c => c.source_rows > 0),
    paywall_ledger_min2: totalPaywallRows >= 2, // Ledger sell-side rows
    pipeline_each_column: rec.column_count >= 2 && rec.columns.every(c => c.pipeline_steps > 0),
    monologue_short_each: rec.column_count >= 2 && rec.columns.every(c => c.monologue_short),
    monologue_extended_each: rec.column_count >= 2 && rec.columns.every(c => c.monologue_extended),
    editor_note_present: rec.editor_note_present,
    no_desktop_overflow: rec.viewports.desktop && !rec.viewports.desktop.overflow,
    no_mobile_overflow: rec.viewports.mobile && !rec.viewports.mobile.overflow,
  };
  rec.pass = Object.values(rec.checks).every(Boolean);
  return rec;
}

async function runArticles(browser) {
  const results = [];
  for (const slug of SLUGS) {
    process.stdout.write(`▶ ${slug} ... `);
    const rec = await captureArticle(browser, slug);
    results.push(rec);
    console.log(rec.pass ? 'PASS' : `FAIL (${Object.entries(rec.checks).filter(([, v]) => !v).map(([k]) => k).join(', ')})`);
  }
  return results;
}

async function runH2H(browser) {
  process.stdout.write(`▶ h2h/${H2H_SLUG} ... `);
  const rec = await captureH2H(browser);
  console.log(rec.pass ? 'PASS' : `FAIL (${Object.entries(rec.checks).filter(([, v]) => !v).map(([k]) => k).join(', ')})`);

  let md = `# THE-208 H2H evidence — ${rec.url}\n\nCaptured: ${new Date().toISOString()}\n\n`;
  md += `Columns found (COLUMN_SEL='${COLUMN_SEL}'): ${rec.column_count}\n\n`;
  md += `| check | result |\n| --- | --- |\n`;
  for (const [k, v] of Object.entries(rec.checks)) md += `| ${k} | ${v ? '✅' : '❌'} |\n`;
  md += `\n## Per-column detail\n`;
  rec.columns.forEach((c, i) => {
    md += `\n### Column ${i + 1} — ${c.headline || '—'}\n`;
    md += `- Confidence: ${c.conf_text || '—'} (≈${c.conf_num ?? '—'})\n`;
    md += `- Source rows: ${c.source_rows} (paywall affordances: ${c.paywall_rows})\n`;
    md += `- Pipeline steps: ${c.pipeline_steps}\n`;
    md += `- Monologue short/extended: ${c.monologue_short}/${c.monologue_extended}\n`;
  });
  md += `\n- Editor's note present: ${rec.editor_note_present}\n`;
  md += `- Screenshots: desktop=${rec.viewports.desktop?.file ?? '—'}, mobile=${rec.viewports.mobile?.file ?? '—'}\n`;
  if (rec.errors.length) md += `- ⚠️ errors: ${rec.errors.join('; ')}\n`;
  await writeFile(`${OUT}/h2h-summary.json`, JSON.stringify(rec, null, 2));
  await writeFile(`${OUT}/H2H-SUMMARY.md`, md);
  if (process.env.GITHUB_STEP_SUMMARY) await writeFile(process.env.GITHUB_STEP_SUMMARY, md, { flag: 'a' });
  return rec;
}

async function main() {
  const mode = (process.argv[2] || 'all').toLowerCase(); // h2h | articles | all
  await mkdir(`${OUT}/api`, { recursive: true });
  await mkdir(`${OUT}/shots`, { recursive: true });

  const browser = await chromium.launch();

  if (mode === 'h2h') {
    const rec = await runH2H(browser);
    await browser.close();
    console.log(`\nH2H ${rec.pass ? 'PASS' : 'FAIL'}.`);
    process.exit(rec.pass ? 0 : 1);
  }

  const results = await runArticles(browser);
  let h2h = null;
  if (mode === 'all') h2h = await runH2H(browser);
  await browser.close();

  await writeFile(`${OUT}/summary.json`, JSON.stringify(results, null, 2));

  // markdown table
  const cols = ['renders_200', 'headline_ok', 'byline_ok', 'confidence_shown', 'sources_render',
    'pipeline_render', 'monologue_short', 'monologue_extended', 'monologue_distinct',
    'no_desktop_overflow', 'no_mobile_overflow', 'api_200', 'api_sources', 'api_pipeline'];
  const head = ['slug', 'HTTP', ...cols, 'PASS'];
  let md = `# THE-206 live evidence — ${BASE}\n\n`;
  md += `Captured: ${new Date().toISOString()}\n\n`;
  md += `| ${head.join(' | ')} |\n| ${head.map(() => '---').join(' | ')} |\n`;
  for (const r of results) {
    const cells = [r.slug, r.http_status ?? '—',
      ...cols.map(c => (r.checks[c] ? '✅' : '❌')), r.pass ? '✅ PASS' : '❌ FAIL'];
    md += `| ${cells.join(' | ')} |\n`;
  }
  md += `\n## Per-article detail\n`;
  for (const r of results) {
    md += `\n### ${r.slug} — ${r.pass ? 'PASS' : 'FAIL'}\n`;
    md += `- URL: ${r.url} (HTTP ${r.http_status})\n`;
    md += `- Headline: ${r.headline ?? '—'}\n`;
    md += `- Byline: ${r.byline ?? '—'}\n`;
    md += `- Confidence meter: ${r.confidence_meter ?? '—'} (API score ${r.api.confidence_score ?? '—'})\n`;
    md += `- Sources: ${r.source_count} rendered, ${r.source_links.length} links (API ${r.api.source_count ?? 0})\n`;
    md += `- Pipeline: ${r.pipeline_steps.map(s => `${s.step}/${s.agent}`).join(' → ') || '—'}\n`;
    md += `- Monologue short/extended/distinct: ${r.monologue_short_present}/${r.monologue_extended_present}/${r.monologue_distinct}\n`;
    md += `- Screenshots: desktop=${r.viewports.desktop?.file ?? '—'}, mobile=${r.viewports.mobile?.file ?? '—'}\n`;
    if (r.errors.length) md += `- ⚠️ errors: ${r.errors.join('; ')}\n`;
  }
  await writeFile(`${OUT}/SUMMARY.md`, md);

  const failed = results.filter(r => !r.pass);
  console.log(`\n${results.length - failed.length}/${results.length} articles passed.`);
  if (h2h) console.log(`H2H ${h2h.pass ? 'PASS' : 'FAIL'}.`);
  if (process.env.GITHUB_STEP_SUMMARY) {
    await writeFile(process.env.GITHUB_STEP_SUMMARY, md, { flag: 'a' });
  }
  // Non-zero exit flags any failure (articles or folded-in H2H); artifacts still upload (if: always()).
  process.exit((failed.length || (h2h && !h2h.pass)) ? 1 : 0);
}

main().catch(e => { console.error(e); process.exit(2); });
