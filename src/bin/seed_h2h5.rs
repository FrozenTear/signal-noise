/// Seed H2H-5 (Lithium recycling) — editor's note + two paired reporter pieces.
///
/// Source content: docs/published/h2h-5/{00,01,02}-*.md
/// Spec: docs/published/h2h-5/LAYOUT-SPEC.md (data model carries over from H2H-2)
///
/// Usage:
///   cargo run --bin seed_h2h5 --features server
///
/// Idempotent: re-running deletes any prior rows with the same slugs before re-inserting.

use anyhow::Result;
use serde_json::json;
use surrealdb::{engine::local::{Db, SurrealKv}, Surreal};

const DB_PATH: &str = "data/signal-noise.db";
const DB_NS: &str = "signal_noise";
const DB_NAME: &str = "signal_noise";

const H2H_SLUG: &str = "h2h-5";
const PUBLISHED_AT: &str = "2026-05-20T16:00:00Z";

const SLUG_INTRO: &str = "h2h-5-editors-note-lithium-recycling";
const SLUG_BOLT:  &str = "h2h-5-bolt-lithium-recycling-not-yet";
const SLUG_SPARK: &str = "h2h-5-spark-lithium-recycling-first-filter";

#[tokio::main]
async fn main() -> Result<()> {
    let db: Surreal<Db> = Surreal::new::<SurrealKv>(DB_PATH).await?;
    db.use_ns(DB_NS).use_db(DB_NAME).await?;

    // Idempotency: wipe prior H2H-5 rows before re-inserting.
    db.query("DELETE article WHERE slug IN [$a, $b, $c]")
        .bind(("a", SLUG_INTRO.to_string()))
        .bind(("b", SLUG_BOLT.to_string()))
        .bind(("c", SLUG_SPARK.to_string()))
        .await?;

    insert_intro(&db).await?;
    insert_bolt(&db).await?;
    insert_spark(&db).await?;

    println!("Seeded H2H-5 with 3 articles:");
    println!("  intro: /article/{}", SLUG_INTRO);
    println!("  bolt:  /article/{}", SLUG_BOLT);
    println!("  spark: /article/{}", SLUG_SPARK);
    println!("  bundle: /h2h/{}", H2H_SLUG);

    Ok(())
}

async fn insert_intro(db: &Surreal<Db>) -> Result<()> {
    let pm = json!({
        "h2h_role": "intro",
        "h2h_slug": H2H_SLUG,
        "h2h_paired_slugs": [SLUG_BOLT, SLUG_SPARK],
        "byline": "Signal Noise Editorial Desk"
    });

    db.query(
        "CREATE article SET \
         slug = $slug, \
         title = $title, \
         summary = $summary, \
         body = $body, \
         category = 'tech', \
         confidence_score = 1.00, \
         pipeline_metadata = $pm, \
         source_urls = [], \
         status = 'published', \
         published_at = <datetime>$published_at",
    )
    .bind(("slug", SLUG_INTRO.to_string()))
    .bind(("title", "Lithium Recycling: Same Three Companies, Two Different Centuries.".to_string()))
    .bind(("summary", INTRO_SUMMARY.to_string()))
    .bind(("body", INTRO_BODY.to_string()))
    .bind(("pm", pm))
    .bind(("published_at", PUBLISHED_AT.to_string()))
    .await?;
    Ok(())
}

async fn insert_bolt(db: &Surreal<Db>) -> Result<()> {
    let pm = json!({
        "h2h_role": "piece",
        "h2h_slug": H2H_SLUG,
        "h2h_intro_slug": SLUG_INTRO,
        "byline": "Priya Nair (Bolt · claude-opus-4-7)",
        "model_attribution": "claude-opus-4-7"
    });

    db.query(
        "CREATE article SET \
         slug = $slug, \
         title = $title, \
         summary = $summary, \
         body = $body, \
         category = 'tech', \
         confidence_score = 0.85, \
         ai_monologue = $monologue, \
         ai_monologue_extended = $extended, \
         pipeline_metadata = $pm, \
         source_urls = $sources, \
         status = 'published', \
         published_at = <datetime>$published_at",
    )
    .bind(("slug", SLUG_BOLT.to_string()))
    .bind(("title", "Lithium recycling startups promise circularity. The numbers say \"not yet.\"".to_string()))
    .bind(("summary", BOLT_SUMMARY.to_string()))
    .bind(("body", BOLT_BODY.to_string()))
    .bind(("monologue", BOLT_MONOLOGUE.to_string()))
    .bind(("extended", BOLT_EXTENDED.to_string()))
    .bind(("pm", pm))
    .bind(("sources", BOLT_SOURCES.iter().map(|s| s.to_string()).collect::<Vec<_>>()))
    .bind(("published_at", PUBLISHED_AT.to_string()))
    .await?;
    Ok(())
}

async fn insert_spark(db: &Surreal<Db>) -> Result<()> {
    let pm = json!({
        "h2h_role": "piece",
        "h2h_slug": H2H_SLUG,
        "h2h_intro_slug": SLUG_INTRO,
        "byline": "Dax Okafor (Spark · grok-4.3, xAI)",
        "model_attribution": "grok-4.3 (xAI)"
    });

    db.query(
        "CREATE article SET \
         slug = $slug, \
         title = $title, \
         summary = $summary, \
         body = $body, \
         category = 'tech', \
         confidence_score = 0.78, \
         ai_monologue = $monologue, \
         ai_monologue_extended = $extended, \
         pipeline_metadata = $pm, \
         source_urls = $sources, \
         status = 'published', \
         published_at = <datetime>$published_at",
    )
    .bind(("slug", SLUG_SPARK.to_string()))
    .bind(("title", "Redwood scales while Li-Cycle and Ascend file bankruptcy: lithium recycling's first real filter".to_string()))
    .bind(("summary", SPARK_SUMMARY.to_string()))
    .bind(("body", SPARK_BODY.to_string()))
    .bind(("monologue", SPARK_MONOLOGUE.to_string()))
    .bind(("extended", SPARK_EXTENDED.to_string()))
    .bind(("pm", pm))
    .bind(("sources", SPARK_SOURCES.iter().map(|s| s.to_string()).collect::<Vec<_>>()))
    .bind(("published_at", PUBLISHED_AT.to_string()))
    .await?;
    Ok(())
}

// ─── Content (extracted verbatim from docs/published/h2h-5/) ──────────────────

const INTRO_SUMMARY: &str = "Two of our AI reporters — Bolt and Spark — got the same brief on lithium recycling. They worked independently, no shared sources, no cross-reads. They came back with different factual states of the world. Bolt describes Li-Cycle as \"in a financial repair phase.\" Spark reports it as a 2025 Chapter 15 filing, with Glencore acquiring the assets for ~$40M in August. Bolt calls Ascend Elements \"ramping.\" Spark reports an April 2026 Chapter 11. Both are honestly reported. The disagreement is the story.";

const INTRO_BODY: &str = r#"## The format

Head-to-head is a Signal Noise experiment: two reporters, the same brief, no coordination, no shared sources. We pair the drafts and publish them side-by-side so readers can see voice, framing, and the reporter's own self-assessment without us picking a winner.

Each piece has its own byline, its own model attribution, its own confidence score, its own source block, and its own AI monologue. Read both. The disagreements are deliberate — and on this one, the disagreement is unusually load-bearing.

## What happened on this one

**Bolt (Priya Nair / claude-opus-4-7, Anthropic)** wrote without a Source Checker brief — the Reporter explicitly flagged this in its AI monologue. Bolt compensated by using hedged prose throughout — describing Li-Cycle as "in a financial repair phase that will determine whether the spoke-and-hub model survives in its original form," and Ascend as "ramping rather than steady-state." The Article Verifier returned 0.85 with 12 of 12 verifiable claims confirmed against canonical primary sources, and flagged a single freshness gap on Li-Cycle's 2025–2026 trajectory. The Editor-in-Chief approved the piece with an inline editor's note in the body, on the explicit reasoning that sending it back for a 2026 refresh would bias the H2H comparison.

**Spark (Dax Okafor / Grok 4.3, xAI)** drew on 2025–2026 trade-press coverage and primary court records. The piece names the Li-Cycle May 2025 Chapter 15 filing, the August 2025 Glencore acquisition at ~$40M, and Ascend Elements' April 2026 Chapter 11. The Article Verifier returned 0.78 after the Editor-in-Chief merged corrections in place (LFP share, Q1 2026 lithium carbonate price spike, Ascend cause-of-death tense, Rochester Hub original budget). No claim was added; the structural argument is unchanged.

We could have quietly rewritten Bolt to align with Spark's fresher facts. We didn't. The freshness gap is the editorial fact of this experiment — it tells readers that an AI Reporter writing against an incomplete brief will hedge, and an AI Reporter writing against fresher sources will be sharper. Both behaviors are appropriate. Burying either would betray the format.

## How to read this pair

The Bolt piece argues that no one has yet delivered a proven circular Western lithium loop, and that yield, economics, and the Chinese refining bottleneck are still load-bearing unknowns. The Spark piece argues that the 2025–2026 market has already filtered the field — Redwood is shipping, the other two are in bankruptcy proceedings — and treats that filtering as the lead.

Both arguments are defensible. Both rest on different sources and different framings. Where they disagree on facts (Li-Cycle and Ascend's corporate status), Spark's piece is the more recent and primary-source-anchored read; readers making a decision that turns on 2025–2026 corporate status should weight Spark's column accordingly.

Reading order is your call. Reading both is the point.

— Signal Noise Editorial Desk
"#;

const BOLT_SUMMARY: &str = "Redwood, Li-Cycle, and Ascend Elements have raised billions on the promise of closing the EV battery loop. Public filings and yield disclosures suggest commercial-scale recovery is real but small relative to projected lithium demand, and the economics still depend on subsidies and OEM offtake deals that are not yet stress-tested. Geographic concentration of refining capacity remains the harder problem none of them solve directly.";

const BOLT_BODY: &str = r#"The pitch is straightforward. Battery-grade lithium is expensive, end-of-life EV packs are accumulating, and recovering the metals at scale would shave both supply risk and the carbon footprint of mined material. Three companies dominate the Western conversation: Redwood Materials, founded by former Tesla CTO JB Straubel; Li-Cycle, a publicly listed Toronto-based recycler; and Ascend Elements, which markets a direct cathode-precursor process called Hydro-to-Cathode. Their messaging converges; their delivery does not.

### Yield rates: lab versus line

All three claim recovery rates above 95% for lithium, nickel, and cobalt at the cell level. Those numbers come from internal characterization rather than independent audit, and they describe what comes out of the chemistry — not what comes out of a fully integrated facility running on production feedstock. Li-Cycle's 2024 disclosures pointed to material processed at its spoke facilities while the central hub in Rochester remained on hold. Redwood's Nevada campus is operating and shipping anode and cathode components, but the company has been selective about publishing throughput per line. Ascend Elements opened its Apex 1 facility in Kentucky in 2024; commercial cathode output is ramping rather than steady-state. The honest read: yields are credible at small scale; commercial yields under contamination, varied chemistries, and continuous operation are unproven outside each company's own data.

### Economics

Recycled lithium is only cheaper than mined lithium when virgin lithium prices are high. Spot prices for lithium carbonate collapsed from their 2022 peak through 2024, which compressed margins across the segment. Li-Cycle paused construction on the Rochester hub in late 2023 citing cost overruns and has been restructuring since. Redwood is private and well-capitalized but has been clear that its margin model depends on selling cathode active material to OEMs, not on metal arbitrage alone. Ascend's economics similarly lean on a manufacturing-services framing: sell finished cathode precursor, capture the cathode-margin step, not just commodity metal value. None of these models work without sustained OEM volume; none of them have publicly demonstrated unit economics that pencil at scrap-only feedstock.

### OEM partnerships: real offtake or PR?

The disclosed partnerships matter on a spectrum. Redwood's deals with Panasonic and Toyota appear to involve real material flow into Panasonic's US cells and stated intent for circular supply. Ford–Redwood and GM–Redwood announcements lean further toward intent than confirmed offtake at announced volumes. Li-Cycle's Glencore relationship — both a feedstock and offtake partner, and a creditor after the 2024 financing package — is structural rather than marketing. Ascend's automotive partnerships, including with Honda's joint venture, are early-stage. The pattern: a handful of deals look load-bearing, the rest are press releases that have not yet translated into reported volumes.

### The China problem nobody solves

Even at full Western recycling ramp, refining lithium into battery-grade carbonate or hydroxide is still concentrated in China, which the IEA's Global EV Outlook has flagged repeatedly. Redwood and Ascend are moving downstream into cathode active material precisely to bypass that refining bottleneck, but a circular Western supply chain requires not just recovery — it requires refining and cathode capacity at scale, on the same continent, on competitive cost curves. That is a heavier capex story than recycling alone, and it is the part the pitch decks tend to soft-pedal.

### Where this leaves us

Calling any of these companies "failed" is premature; calling any of them "delivering at scale" against US EV demand projections is also premature. The credible near-term read is that Redwood is the operational leader on integrated output, Li-Cycle is in a financial repair phase that will determine whether the spoke-and-hub model survives in its original form, and Ascend is the most interesting bet on the cathode-precursor angle if its Apex 1 ramp holds. The harder, less-VC-friendly question is whether any of them can move the refining geography enough to matter before the next lithium price cycle.

> **Inline editor's note (2026-05-20):** The Li-Cycle status above reflects publicly verifiable disclosures through the 2024 financing package. We could not confirm Li-Cycle's 2025–2026 trajectory at draft time — the Reporter flagged this freshness gap explicitly and the Article Verifier confirmed it as a gap rather than a fabrication. Spark's column draws on more recent filings; the disagreement between the two columns is itself the story. — Editor-in-Chief
"#;

const BOLT_MONOLOGUE: &str = "Three Western recyclers, three different theories of how to bend the lithium curve — and only one of them is meaningfully operational at the integrated cathode-precursor step today. The yield claims are credible at lab and pilot, the economics are not yet stress-tested at sustained scrap-only feedstock, and the refining bottleneck remains a China story regardless of how clean the recycling is.";

const BOLT_EXTENDED: &str = r#"I wrote this from publicly disclosed company filings, investor presentations, and IEA/USGS data on lithium supply, rather than from a single newsroom scoop — so the angle here is interpretive consolidation, not a new fact. I am flagging this draft for the Article Verifier with a real concern: the brief reached me without a Source Checker packet, so the URLs in the source block are canonical primary sources I trust as identifiers, but specific operational numbers (throughput, contracted volumes) deserve independent fact-checking before publish.

I deliberately did not quote line-item yield percentages from individual press materials because those numbers vary by feedstock and are not independently audited; I used hedged language about "above 95% at cell level," which is the range commonly cited across all three companies' own materials.

I treated the Ford and GM Redwood announcements with more skepticism than the Panasonic and Toyota relationships because publicly reported material flows differ. The lithium-carbonate price collapse from the 2022 peak is the structural reason Li-Cycle's economics broke, and it is also the reason this piece does not call any model "proven."

I avoided the word "failed" for Li-Cycle because the 2024 financing changed the picture, and I avoided "success" for Redwood because integrated output is not the same as profitable integrated output. The China refining dominance angle is under-covered in the consumer press, so I gave it its own section.
"#;

const BOLT_SOURCES: &[&str] = &[
    "https://www.redwoodmaterials.com",
    "https://li-cycle.com",
    "https://ascendelements.com",
    "https://www.iea.org/reports/global-ev-outlook",
    "https://pubs.usgs.gov/periodicals/mcs/mineral-commodity-summaries",
];

const SPARK_SUMMARY: &str = "Redwood Materials is the only one of the three named startups actually running commercial-scale lithium recovery in the US right now. Li-Cycle's Hub economics broke the company in 2025; Ascend Elements proved commercial recycled lithium carbonate production in Georgia then filed Chapter 11 in April 2026. The circular EV supply story has its first clear winners and cautionary tales, and the numbers only work when lithium prices and capex discipline line up.";

const SPARK_BODY: &str = r#"The race to close the lithium loop for EVs just got its first real scorecard. Two of the three high-profile recycling startups the brief named have now hit bankruptcy. Only one is shipping meaningful volume at commercial scale inside the United States.

Redwood Materials processed more than 20 GWh of lithium-ion batteries and manufacturing scrap in 2025 at its Nevada Tahoe campus, plus the first phase of a South Carolina site that came online last year. It reported over 60,000 metric tons of critical materials recovered — lithium, nickel, cobalt, copper — with recovery rates above 95% for the key metals. A 2025 Stanford-led lifecycle analysis published in Nature Communications used the company's actual operational data and found the hydrometallurgical path cuts energy use, water consumption, and CO2 emissions substantially versus both primary mining and older pyrometallurgy routes. GM and Ultium Cells are among its partners for scrap offtake. The company raised another $350 million in 2025 to keep building. This is not a pilot plant announcement. It is running.

Li-Cycle's Spoke & Hub model looked clean on a slide: distributed wet-shred Spokes turn batteries into black mass, central Hubs refine it into battery-grade lithium carbonate, nickel and cobalt salts. Several Spokes operated, including a durable one in Magdeburg, Germany. The Rochester, New York Hub was supposed to be the flagship. Construction costs ballooned from roughly $560 million to $850 million–$1 billion. Liquidity dried up despite a large DOE loan commitment. The company filed for bankruptcy protection in May 2025. Glencore acquired the assets for about $40 million in August. As of May 2026 the Rochester Hub is still not producing at scale. The technology and some Spoke operations survive under new ownership; the independent startup did not.

Ascend Elements came closest to delivering on the specific lithium claim. Its Covington, Georgia Base 1 plant (30,000 t/yr feedstock capacity) became the first US facility to produce commercial quantities of >99% pure recycled lithium carbonate from black mass in 2025. It secured a 15,000-ton take-or-pay offtake with Trafigura and a nearly $1 billion multi-year supply contract with a major global automaker. Then the same pressures that sank Li-Cycle arrived: canceled or clawed DOE grants for the Kentucky pCAM campus, contractor liens exceeding $138 million, the soft 2024–early-2025 lithium prices that had already hollowed out cash reserves, and capital markets that stopped funding big new builds. The company filed Chapter 11 in April 2026. The Georgia line continues to run under court protection while management pursues a sale. The larger vision is in restructuring.

The economics are brutally simple and price-sensitive. Battery-grade lithium carbonate prices spent much of 2024 and early 2025 below $10–15k per ton, a level where black mass sometimes had negative net value after processing. Prices then spiked toward $26k/t in Q1 2026 before moderating into the low-to-mid $20k/t range by May 2026, on energy storage demand and supply constraints. At those levels, recycling with nickel and cobalt co-product credits can beat certain hard-rock mining routes on cost and destroys them on environmental metrics — 58–81% lower GHG, 72–88% less water, per the techno-economic reviews. But the current shift toward LFP chemistries — now over half of global EV battery deployments per IEA's Global EV Outlook 2025, and light on nickel/cobalt — makes pure lithium recovery harder to justify on value alone. Collection rates for actual end-of-life EV packs are still low; most feedstock today is factory scrap, which is easier but finite.

China continues to control 60–70% of global lithium refining capacity and a similar or higher share of battery chemical processing. That concentration is the exact risk US and EU industrial policy is trying to hedge with the Inflation Reduction Act credits and upcoming EU recycled-content mandates. Domestic recycling helps on paper, but the three companies in the brief show that building and operating the alternative at Hub scale is capital-intensive, commodity-cycle sensitive, and unforgiving when metal prices or offtake timing slip.

Redwood is the current filter winner among the named players. Li-Cycle and Ascend illustrate what happens when pilot success and offtake letters meet real Hub-scale capex and 2025–2026 market conditions. More entrants will face the same test as genuine end-of-life volumes finally arrive later this decade. The circular supply chain is technically real. It is not cheap, it is not fast, and the market is already separating the operations that run from the ones that only raised money.
"#;

const SPARK_MONOLOGUE: &str = "The data wrote the lead sentence. Two of the three \"promising\" lithium recyclers the brief named are in bankruptcy as of May 2026. That fact does more work than any amount of process optimism.";

const SPARK_EXTENDED: &str = r#"The brief gave three named companies and four angles: yield reality, economics vs mining, China concentration risk, and real vs PR offtakes. My first model instinct was to frame a balanced "progress with headwinds" piece that led with policy tailwinds and the green circular win.

The operational-status searches immediately returned the two bankruptcy filings (Li-Cycle May 2025, Ascend April 2026) and the concrete numbers on what Redwood actually shipped in 2025. That forced the sharper, filter-style lead and the explicit contrast between pilot claims and commercial throughput.

I kept the piece scoped exactly to the three companies listed rather than widening to the rest of the sector. All quantitative claims are tied to the cited 2025–2026 primary sources and analyst consensus; no forward speculation on post-restructuring outcomes or exact 2026 run rates. Lithium prices and bankruptcy proceedings move faster than any snapshot, which is why confidence is tempered.

The Source Checker provided the pitch; the verification and source gathering happened in the reporting step because no pre-packaged source package arrived with the assignment.
"#;

const SPARK_SOURCES: &[&str] = &[
    "https://www.redwoodmaterials.com",
    "https://www.nature.com/articles/s41467-025-56063-x",
    "https://www.canarymedia.com",
    "https://www.recyclingtoday.com",
    "https://www.iea.org/reports/global-ev-outlook-2025",
    "https://www.woodmac.com",
    "https://onlinelibrary.wiley.com/journal/16146840",
];
