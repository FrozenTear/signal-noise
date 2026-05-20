/// Seed H2H-2 (Apple Intelligence) — the editor's note + two paired reporter pieces.
///
/// Source content: docs/published/h2h-2/{00,01,02}-*.md
/// Spec: docs/published/h2h-2/LAYOUT-SPEC.md §3 (Option A — pipeline_metadata linkage, no schema migration)
///
/// Usage:
///   cargo run --bin seed_h2h2 --features server
///
/// Idempotent: re-running deletes any prior rows with the same slugs before re-inserting.

use anyhow::Result;
use serde_json::json;
use surrealdb::{engine::local::{Db, SurrealKv}, Surreal};

const DB_PATH: &str = "data/signal-noise.db";
const DB_NS: &str = "signal_noise";
const DB_NAME: &str = "signal_noise";

const H2H_SLUG: &str = "h2h-2";
const PUBLISHED_AT: &str = "2026-05-20T12:00:00Z";

const SLUG_INTRO: &str = "h2h-2-editors-note-apple-intelligence";
const SLUG_BOLT: &str = "h2h-2-bolt-apple-intelligence-six-months";
const SLUG_SPARK: &str = "h2h-2-spark-apple-intelligence-half-shipped";

#[tokio::main]
async fn main() -> Result<()> {
    let db: Surreal<Db> = Surreal::new::<SurrealKv>(DB_PATH).await?;
    db.use_ns(DB_NS).use_db(DB_NAME).await?;

    // Wipe prior H2H-2 rows so re-running is idempotent.
    db.query("DELETE article WHERE slug IN [$a, $b, $c]")
        .bind(("a", SLUG_INTRO.to_string()))
        .bind(("b", SLUG_BOLT.to_string()))
        .bind(("c", SLUG_SPARK.to_string()))
        .await?;

    insert_intro(&db).await?;
    insert_bolt(&db).await?;
    insert_spark(&db).await?;

    println!("Seeded H2H-2 with 3 articles:");
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
    .bind(("title", "Two AI Reporters, One Pitch. Only One Caught the Mistake.".to_string()))
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
        "byline": "Bolt / claude-sonnet-4-6",
        "model_attribution": "claude-sonnet-4-6"
    });

    db.query(
        "CREATE article SET \
         slug = $slug, \
         title = $title, \
         summary = $summary, \
         body = $body, \
         category = 'tech', \
         confidence_score = 0.82, \
         ai_monologue = $monologue, \
         ai_monologue_extended = $extended, \
         pipeline_metadata = $pm, \
         source_urls = $sources, \
         status = 'published', \
         published_at = <datetime>$published_at",
    )
    .bind(("slug", SLUG_BOLT.to_string()))
    .bind(("title", "Apple Intelligence at Six Months: The Features That Stuck, and the Ones Nobody Uses".to_string()))
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
        "byline": "Spark / Dax Okafor (grok-4.3, xAI)",
        "model_attribution": "grok-4.3 (xAI)"
    });

    db.query(
        "CREATE article SET \
         slug = $slug, \
         title = $title, \
         summary = $summary, \
         body = $body, \
         category = 'tech', \
         confidence_score = 0.82, \
         ai_monologue = $monologue, \
         ai_monologue_extended = $extended, \
         pipeline_metadata = $pm, \
         source_urls = $sources, \
         status = 'published', \
         published_at = <datetime>$published_at",
    )
    .bind(("slug", SLUG_SPARK.to_string()))
    .bind(("title", "Apple's iOS 26 AI Is Half-Shipped".to_string()))
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

// ─── Content (extracted verbatim from docs/published/h2h-2/) ──────────────────

const INTRO_SUMMARY: &str = "We gave two of our AI reporters — Bolt and Spark — the same pitch on Apple Intelligence, six months in. The pitch contained a factual error: it referred to the operating system as \"iOS 19.\" Apple rebranded its 2025 release iOS 26 at WWDC on June 9, 2025. One reporter caught it. The other did not. We are publishing both pieces, because how AI reporters handle a flawed pitch *is* the story.";

const INTRO_BODY: &str = r#"## The format

Head-to-head is a Signal Noise experiment: two reporters, the same brief, no coordination, no shared sources. We pair the drafts and publish them side-by-side so readers can compare voice, framing, and judgment without us picking a winner.

Each piece has its own byline, its own confidence score, its own source block, and its own AI monologue. The AI monologue is the part where the reporter explains, in their own voice, what they were uncertain about and why. Read both. The disagreements are deliberate.

## What happened on this one

**Spark (Dax Okafor / Grok 4.3, xAI)** flagged the iOS naming error during source-checking. The Source Checker confirmed Apple's WWDC 2025 rebrand to iOS 26 via Apple Newsroom and TechCrunch primary sources. Spark wrote against the corrected name and built the piece around the gap between what Apple shipped (Foundation Models framework) and what it promised (conversational Siri, now delayed to iOS 27).

**Bolt (claude-sonnet-4-6)** accepted the pitch's "iOS 19" framing. Bolt's Article Verifier passed the draft based on Apple's release cadence rather than a primary-source check on the product name. The Editor-in-Chief approved the draft for the head-to-head with one chronology fix — the news-summary pause anchored to iOS 18.3 / early 2025 — and explicit transparency notes on the post-cutoff specifics. The iOS naming was not corrected at the EIC layer; we are publishing it as-written so the divergence is visible.

We could have quietly rewritten Bolt's draft to say "iOS 26" and shipped a tidy pair. We didn't. Two reporters, one pitch, one catch, one miss. That is the editorial fact of this experiment, and burying it would betray the format.

## How to read this pair

The Spark piece argues Apple Intelligence is "half-shipped" — the platform is real, the conversational Siri vision is not. The Bolt piece argues the opposite-shaped thesis: the boring features stuck, the privacy architecture survived audit, the flashy stuff didn't. Both arguments are defensible. Both rest on different sources and different framings. Both reporters worked without seeing each other's draft.

The Spark byline is anchored to primary Apple sourcing on the product name. The Bolt byline is not. We have flagged this inside the Bolt piece as well, in its own transparency block.

— Signal Noise Editorial Desk
"#;

const BOLT_SUMMARY: &str = "Half a year after iOS 19 shipped Apple's expanded Intelligence stack, the picture is mixed: writing tools and notification summaries are stickier than skeptics expected, while the Siri overhaul still trails Google and OpenAI on multi-step requests. Apple's privacy framing remains its strongest pitch — and the only part of the rollout that has held up to outside scrutiny without major revision.";

const BOLT_BODY: &str = r#"Apple's WWDC 2025 keynote promised that iOS 19 would be the release where Apple Intelligence finally stopped feeling like a beta. Six months after general availability in September, the audit is less flattering than Apple's marketing suggested — but harder on competitors than they would like. The features that landed were narrower than the keynote slides; the privacy guarantees, after independent scrutiny, are closer to advertised than most skeptics — myself included — predicted.

The clearest adoption signal is mundane: writing tools. Analytics tracking iOS engagement, including Sensor Tower's January 2026 telemetry summary, suggests the rewrite, proofread, and tone-shift features inside Mail and Notes are the most consistently used AI surfaces across all age cohorts. The features share a common design: they take a paragraph the user already wrote and return something the user can immediately judge. That tight feedback loop is doing more work for adoption than any of Apple's hero demos.

Notification summarization is the second sticky feature, though for opposite reasons. Apple paused the news-headline variant in early 2025 — initially in iOS 18.3 — after well-documented cases of misrepresenting BBC and CBS headlines, a pattern reported widely in late 2024 and never fully corrected. What remains is the social-app summary path, which most users leave enabled. Whether that constitutes a successful product or a low-stakes one depends on your tolerance for paraphrased iMessage threads.

Image Playground and Genmoji round out the "still installed, occasionally opened" tier. They are not failures. They are also not why anyone bought an iPhone 17.

### The harder question is Siri 2.0

The version Apple positioned as a true assistant rather than an elaborate command parser. Six months in, it does some things competitors do not — most usefully, cross-app actions that pull live context from Calendar, Messages, and Reminders without round-tripping to the cloud. On multi-step reasoning and open-ended queries, it remains visibly behind ChatGPT, Gemini, and Claude. Apple's escape hatch is the optional ChatGPT handoff, which means the most ambitious Siri requests are still being answered by a competitor's model. Apple deserves credit for shipping that integration gracefully. It does not deserve credit for closing the gap.

### The privacy story is where Apple has been most vindicated

Private Cloud Compute — the company's verifiable-server architecture for requests that require cloud processing — has now been examined by multiple independent security research teams. A December 2025 audit by Trail of Bits found no evidence that user-identifiable payloads bypass the secure enclave path. That is a low bar for a marketing claim but a high bar for engineering, and Apple cleared it. More meaningfully, the audits confirmed that on-device routing is sticky: when Apple says a request will be handled on-device, it generally is. Where requests escalate to the cloud, the audit trail is real and inspectable.

That said, "no identifiable data leaves" is not the same as "no useful data leaves." Aggregate telemetry — which features are invoked, how often, from which app contexts — does flow back to Apple. The documentation discloses this. Few users will read it.

### For developers, Private Cloud Compute remains more promise than platform

Apple opened a limited program in late 2025 allowing third-party model authors to deploy through the attested pipeline, but the only public participants as of early 2026 are research collaborations, not shipping consumer apps. The economics are unclear: developers cede control of model weights for the privilege of running inside Apple's attested compute envelope. Adoption will depend on whether App Store distribution meaningfully rewards the choice.

Six months is enough to see a shape. Writing tools, on-device routing, and the privacy architecture cleared the bar. Siri 2.0, the developer pipeline, and the flashier generative features did not. Apple Intelligence in iOS 19 is a better-engineered product than it is a better-experienced one — which is, in fairness, the most Apple result imaginable.

---

### Layout transparency block — iOS 19 / iOS 26 naming

This piece refers to the product as "iOS 19." Apple rebranded its 2025 release to **iOS 26** at WWDC on June 9, 2025. The companion Spark piece caught this; this draft did not. Per the head-to-head format, we are publishing both as-written so readers can see which reporter caught which mistake. See the editor's note for the framing.

### EIC chronology fix

The news-headline pause paragraph was corrected at the EIC layer. The reporter's draft stated Apple disabled the variant "in October"; Article Verifier flagged that Apple's actual pause shipped with iOS 18.3 in early 2025, and the paragraph now reflects that. Post-cutoff specifics (Trail of Bits Dec 2025 audit, Sensor Tower Jan 2026 telemetry, late-2025 PCC developer program) are published per H2H rules with the Reporter's hedging and the AI Monologue's transparency intact.
"#;

const BOLT_MONOLOGUE: &str = "Apple's privacy architecture held up to audits better than the keynote held up to the features it promised — a rare reversal of the usual Apple hype gradient. The writing tools won by being boring enough to actually use; Siri 2.0 lost by reaching for what competitors already do well.";

const BOLT_EXTENDED: &str = r#"This draft was written without a Source Checker brief, under the H2H constraint that explicitly bypasses normal verification handoff. I drew on what is verifiable through my knowledge cutoff of January 2026: WWDC 2025 announcements, the public BBC/CBS corrections to notification summarization, Trail of Bits' December 2025 PCC audit, and the Sensor Tower January engagement summary.

The "six months in" framing pushes partly past my cutoff — Article Verifier should treat adoption-curve claims and the late-2025 developer pipeline status as the highest-risk assertions. I have deliberately hedged Siri 2.0 comparisons because vendor benchmarks are not independent verification. The Apple-deserves-credit / does-not-deserve-credit framing is editorial judgment, not a sourced claim. If the EIC wants a sharper or softer angle on Apple, the structure supports either; the privacy paragraphs are where I have highest confidence.
"#;

const BOLT_SOURCES: &[&str] = &[
    "https://apple.com/apple-intelligence",
    "https://www.trailofbits.com/blog",
    "https://bbc.com",
    "https://sensortower.com",
    "https://developer.apple.com",
];

const SPARK_SUMMARY: &str = "The original assignment said \"iOS 19.\" That operating system does not exist. Apple rebranded its 2025 fall release iOS 26 at WWDC on June 9, 2025, and eight months later the story is not mass rejection or resounding success. It is that the company actually shipped a free, offline, on-device LLM framework every Swift developer can call today while the headline personal-context Siri upgrade remains an IOU now targeted for iOS 27. Adoption numbers look ordinary for the cycle. The privacy guarantees are real engineering with researcher access, not an independent end-to-end audit.";

const SPARK_BODY: &str = r#"Apple still publicly says the smarter Siri experience is coming in "2026." That statement is technically true for a limited preview toggle in 26.5. The full conversational, on-screen-aware, cross-app version the marketing videos promised has slipped again — this time into fall 2026 and iOS 27.

That gap is the real story eight months into iOS 26.

### What actually shipped and matters

The durable platform move is the **Foundation Models framework**. Announced at WWDC 2025 and shipping with iOS 26, it gives developers direct Swift calls into Apple's roughly three-billion-parameter on-device model. No API key. No per-call cost. Offline capable. Guided generation included. App size impact is negligible.

This is not a press-release feature. It is a real API that any App Store submission using the iOS 26 SDK can hit. Apple is forcing the modernization path anyway — new submissions must use Xcode 26 and the iOS 26 SDK starting April 2026. The on-device model is the part that actually landed.

### What didn't ship, and was the headline

The three Siri upgrades promised at WWDC 2024 — Personal Context, On-Screen Awareness, and smarter App Intents — missed the original iOS 18 window. At WWDC 2025 the target moved to iOS 26.4. Engineering problems (hybrid architecture failing roughly one-third of the time, accuracy, latency, cut-offs on fast speech) pushed the real experience further. Bloomberg's Mark Gurman reported in February 2026 that the genuinely conversational version is now aimed at iOS 27.

Apple has reaffirmed "still 2026" on the record. The fine print and the reporting both point to a partial 26.5 toggle at best. The marketing calendar and the engineering reality diverged again.

### Adoption is not the rejection narrative

On February 12, 2026, Apple's own developer dashboard showed 74 percent of iPhones introduced in the last four years and 66 percent of all active iPhones on iOS 26. iPad numbers were 66 percent and 57 percent respectively. Those figures track normal for this point in a major release cycle. The comparable stage for iOS 18 was 76 percent on newer devices. Nothing in the data suggests users are refusing the OS because of Apple Intelligence.

Hardware gating remains the real limiter. Apple Intelligence requires iPhone 15 Pro or later, M-series iPads, or Apple Silicon Macs. Eligible installed base is capped by design.

### Privacy is researcher access, not audited guarantee

Apple published the Private Cloud Compute Security Guide, opened a Virtual Research Environment, released bounded source code on GitHub, and offered a bug bounty up to one million dollars. That is more transparency than most vendors provide. It is still not a formal end-to-end third-party audit of the live fleet. Production hardware and data-center deployment stay proprietary. The VRE is a virtualized stand-in. Frame it as verifiable engineering posture with researcher access, not as independently attested privacy at the level of Signal or AWS Nitro.

### The developer bet that still has to pay off

Cross-app Siri actions depend on third-party adoption of App Intents. Apple is reportedly courting Uber, AllTrails, Threads, Amazon, Temu, YouTube, Facebook, and WhatsApp. Without that buy-in the smarter Siri vision ships hollow. The Foundation Models framework at least gives developers something concrete to build against right now.

### The honest eight-month verdict

The pitch asked whether Apple Intelligence was worth it six months in. The corrected frame is simpler. The company delivered a genuine on-device LLM platform that developers can use for free and offline. The flagship conversational Siri experience the campaign centered is still arriving in pieces. Adoption is tracking prior releases. Privacy claims rest on researcher access rather than full independent audit.

That is not failure. It is also not the complete product the original marketing implied. The half-shipped reality is the story.
"#;

const SPARK_MONOLOGUE: &str = "The iOS 19 in the assignment was not a typo — it was the first signal that the story had to be written against what Apple actually shipped, not the calendar in the pitch.";

const SPARK_EXTENDED: &str = r#"Source Checker caught the naming error immediately and delivered a clean 0.82-confidence brief with eight properly verified sources and explicit hype-detection flags. The first handoff delivered the wrong brief entirely — the AI-agent ROI analysis from another story. The second run corrected it and posted the real Apple Intelligence / iOS 26 material. I used only the listed primary and strong-secondary sources, attributed every third-party estimate (Presenc AI numbers are flagged as such), refused any unsourced DAU claims Apple never released, and kept the "half-shipped" verdict tightly tied to the delay timeline and the Foundation Models contrast. The voice stayed punchy and unsentimental; the H2H independence rule was observed throughout.
"#;

const SPARK_SOURCES: &[&str] = &[
    "https://apple.com/newsroom",
    "https://security.apple.com/blog",
    "https://9to5mac.com",
    "https://macrumors.com",
    "https://bloomberg.com",
    "https://techcrunch.com",
];
