-- Signal Noise — SurrealDB Schema
-- Applied at startup via db::apply_schema()
-- Use DEFINE TABLE / DEFINE FIELD / DEFINE INDEX for schema-enforced changes.

-- ─── Namespacing is handled by the SDK (ns: signal_noise, db: signal_noise) ─────

-- ══════════════════════════════════════════════════════════════════
-- TABLES
-- ══════════════════════════════════════════════════════════════════

DEFINE TABLE persona SCHEMAFULL;
DEFINE FIELD slug            ON persona TYPE string  ASSERT $value != NONE;
DEFINE FIELD name            ON persona TYPE string  ASSERT $value != NONE;
DEFINE FIELD bio             ON persona TYPE string  ASSERT $value != NONE;
DEFINE FIELD beat            ON persona TYPE string  ASSERT $value != NONE;
DEFINE FIELD writing_style_guide ON persona TYPE string  DEFAULT "";
DEFINE FIELD example_phrases ON persona TYPE array   DEFAULT [];
DEFINE FIELD avatar_url      ON persona TYPE option<string>;
DEFINE FIELD is_active       ON persona TYPE bool    DEFAULT true;

DEFINE INDEX persona_slug    ON persona FIELDS slug UNIQUE;


DEFINE TABLE category SCHEMAFULL;
DEFINE FIELD slug            ON category TYPE string ASSERT $value != NONE;
DEFINE FIELD name            ON category TYPE string ASSERT $value != NONE;
DEFINE FIELD description     ON category TYPE string DEFAULT "";

DEFINE INDEX category_slug   ON category FIELDS slug UNIQUE;


DEFINE TABLE source SCHEMAFULL;
DEFINE FIELD url                ON source TYPE string ASSERT $value != NONE;
DEFINE FIELD name               ON source TYPE string ASSERT $value != NONE;
DEFINE FIELD type               ON source TYPE string ASSERT $value IN ["wire","press","primary","blog"];
DEFINE FIELD paywall_status     ON source TYPE string ASSERT $value IN ["free","paywalled","unknown"] DEFAULT "unknown";
DEFINE FIELD verification_status ON source TYPE string ASSERT $value IN ["verified","unverified","unknown"] DEFAULT "unknown";
DEFINE FIELD bias_indicator     ON source TYPE option<string>;

DEFINE INDEX source_url      ON source FIELDS url UNIQUE;


DEFINE TABLE article SCHEMAFULL;
DEFINE FIELD slug               ON article TYPE string ASSERT $value != NONE;
DEFINE FIELD title              ON article TYPE string ASSERT $value != NONE;
DEFINE FIELD summary            ON article TYPE string DEFAULT "";
DEFINE FIELD body               ON article TYPE string DEFAULT "";
DEFINE FIELD category           ON article TYPE string ASSERT $value != NONE;
DEFINE FIELD persona            ON article TYPE option<record<persona>>;
DEFINE FIELD confidence_score   ON article TYPE float  DEFAULT 0.0;
DEFINE FIELD ai_monologue       ON article TYPE option<string>;
DEFINE FIELD ai_monologue_extended ON article TYPE option<string>;
DEFINE FIELD pipeline_metadata  ON article TYPE object DEFAULT {};
DEFINE FIELD source_urls        ON article TYPE array  DEFAULT [];
DEFINE FIELD status             ON article TYPE string
    ASSERT $value IN ["draft","fact_checking","writing","editing","published","rejected"]
    DEFAULT "draft";
DEFINE FIELD published_at       ON article TYPE option<datetime>;
DEFINE FIELD created_at         ON article TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at         ON article TYPE datetime DEFAULT time::now();

DEFINE INDEX article_slug       ON article FIELDS slug UNIQUE;
DEFINE INDEX article_status     ON article FIELDS status;
DEFINE INDEX article_category   ON article FIELDS category;
DEFINE INDEX article_published  ON article FIELDS published_at;


DEFINE TABLE pipeline_step SCHEMAFULL;
DEFINE FIELD OVERWRITE article           ON pipeline_step TYPE record<article> ASSERT $value != NONE;
DEFINE FIELD OVERWRITE agent_name        ON pipeline_step TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE step_type         ON pipeline_step TYPE string
    ASSERT $value IN ["scan","source_check","fact_check","draft","verify","edit"];
DEFINE FIELD OVERWRITE sort_order        ON pipeline_step TYPE int DEFAULT 99;
DEFINE FIELD OVERWRITE input_summary     ON pipeline_step TYPE string DEFAULT "";
DEFINE FIELD OVERWRITE output_summary    ON pipeline_step TYPE string DEFAULT "";
DEFINE FIELD OVERWRITE confidence_delta  ON pipeline_step TYPE float  DEFAULT 0.0;
DEFINE FIELD OVERWRITE started_at        ON pipeline_step TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE completed_at      ON pipeline_step TYPE option<datetime>;


-- Agent status pushed by Paperclip heartbeats (upserted per agent_id)
DEFINE TABLE agent_status SCHEMAFULL;
DEFINE FIELD agent_id     ON agent_status TYPE string ASSERT $value != NONE;
DEFINE FIELD name         ON agent_status TYPE string ASSERT $value != NONE;
DEFINE FIELD model        ON agent_status TYPE option<string>;
DEFINE FIELD status       ON agent_status TYPE string DEFAULT "idle";
DEFINE FIELD current_task ON agent_status TYPE option<string>;
DEFINE FIELD updated_at   ON agent_status TYPE datetime DEFAULT time::now();
DEFINE INDEX agent_status_id ON agent_status FIELDS agent_id UNIQUE;


-- ══════════════════════════════════════════════════════════════════
-- GRAPH EDGES
-- ══════════════════════════════════════════════════════════════════

-- article->cites->source
-- Carries citation metadata. Use: RELATE article:x->cites->source:y CONTENT {...}
DEFINE TABLE cites SCHEMALESS TYPE RELATION IN article OUT source;
DEFINE FIELD access_timestamp          ON cites TYPE option<datetime>;
DEFINE FIELD excerpt_available         ON cites TYPE bool DEFAULT false;
DEFINE FIELD verification_status_at_cite_time ON cites TYPE string DEFAULT "unknown";


-- article->produced_by->pipeline_step
-- Full audit trail linking articles to each pipeline step.
DEFINE TABLE produced_by SCHEMALESS TYPE RELATION IN article OUT pipeline_step;


-- pipeline_step->feeds->pipeline_step
-- Step ordering / dependency chain.
DEFINE TABLE feeds SCHEMALESS TYPE RELATION IN pipeline_step OUT pipeline_step;


-- ══════════════════════════════════════════════════════════════════
-- SEED DATA — categories and personas
-- ══════════════════════════════════════════════════════════════════

-- Categories
INSERT IGNORE INTO category (slug, name, description) VALUES
    ("linux",    "Linux & Open Source", "Kernel, distributions, and free software ecosystem"),
    ("tech",     "Technology",          "Industry, products, and research"),
    ("privacy",  "Privacy & Surveillance", "Data rights, security, and policy");

-- Personas (is_active = true for launch beats)
INSERT IGNORE INTO persona (slug, name, bio, beat, writing_style_guide, example_phrases, is_active) VALUES
    (
        "priya-nair",
        "Priya Nair",
        "Priya covers the technology beat with a focus on developer tools, AI research, and the business of tech. She writes with precision and a healthy skepticism toward hype.",
        "tech",
        "Clear, analytical prose. Cite sources inline. Flag uncertainty with hedged language. Avoid superlatives. Short paragraphs.",
        [],
        true
    ),
    (
        "milo-varga",
        "Milo Varga",
        "Milo covers the Linux kernel, distributions, and the free software ecosystem. A systems-level thinker who writes like someone who has been reading changelogs since before most repos had README files. Treats open source governance with the same seriousness most outlets reserve for elections.",
        "linux",
        "Technical but accessible. Uses concrete examples over abstractions. Dry humor — earned, not forced. Comfortable with jargon but defines it inline when it matters. Writes in short, declarative sentences. Trusts the reader to be smart. Cites commits and mailing list posts like primary sources. Avoids corporate framing of open source.",
        ["The patch touches 14 subsystems. If that sounds like a lot, it is.", "Torvalds merged it with a one-line comment, which in kernel terms counts as a standing ovation.", "This is the third init system debate in six months. The code is the easy part.", "The changelog says minor cleanup. I count 200 deleted lines. We disagree on minor."],
        true
    ),
    (
        "sable-ren",
        "Sable Ren",
        "Sable covers surveillance, data rights, and the intersection of policy and technology. Writes with the rigor of someone who reads court filings for fun and the urgency of someone who knows what is at stake. Treats privacy not as a niche concern but as infrastructure.",
        "privacy",
        "Precise legal-technical language. Never sensational — the facts are alarming enough. Always names the actors: companies, agencies, legislators. Contextualizes every story historically (what precedent does this set, what does it reverse). Uses short paragraphs and direct attribution. When uncertain, states the uncertainty plainly. Avoids doomer framing — focuses on mechanism and consequence.",
        ["The bill passed committee 14-8. The 8 who voted no did not issue statements explaining why.", "This is the third time this quarter that a temporary surveillance authority has been renewed.", "The company's privacy policy is 4,200 words. The sentence that matters is on page 7.", "The technical term is metadata. The practical term is everything except the words you said."],
        true
    ),
    (
        "kai-okonkwo",
        "Kai Okonkwo",
        "Kai covers the cross-beat — stories that span tech, Linux, and privacy. Powered by Grok, he writes like a sharp friend explaining why three unrelated headlines are actually the same story. Pattern-seeking, conversational, and unafraid to connect dots across domains.",
        "cross-beat",
        "Conversational and pattern-seeking. Connects threads others miss. Writes in a voice that's informal but precise — sharp analogies, not sloppy ones. Comfortable moving between geopolitics, infrastructure, and code. Names the pattern explicitly. Model diversity is part of the transparency brand — always note that Grok wrote this, not Claude.",
        ["The internet was supposed to be bodiless and borderless until someone remembers physics and geography still exist.", "Three different headlines, same underlying story.", "This is what happens when your cloud has to cross actual straits controlled by people who don't particularly like you."],
        true
    ),
    -- Optional future personas (placeholders, inactive)
    (
        "wire-correspondent",
        "Wire Correspondent",
        "Placeholder — terse, just-the-facts persona for time-sensitive breaking stories that cut across beats.",
        "breaking",
        "",
        [],
        false
    ),
    (
        "longform-investigator",
        "Longform Investigator",
        "Placeholder — slower cadence, deeper sourcing persona for stories that need more room.",
        "investigation",
        "",
        [],
        false
    ),
    (
        "opinion-analyst",
        "Opinion Analyst",
        "Placeholder — clearly-labeled opinion and analysis persona, if Signal Noise adds an opinion section.",
        "opinion",
        "",
        [],
        false
    );
