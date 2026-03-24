use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleCardProps {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub persona_name: String,
    pub confidence_score: f64,
    pub published_at: String,
}

#[component]
pub fn ArticleCard(props: ArticleCardProps) -> Element {
    let mut show_deliberation = use_signal(|| false);

    let acc = props.confidence_score;
    let src = (acc * 0.92).min(1.0);
    let cmp = (acc * 0.76).min(1.0);
    let bias = 0.07 + (1.0 - acc) * 0.18;

    let acc_pct  = (acc  * 100.0) as u32;
    let src_pct  = (src  * 100.0) as u32;
    let cmp_pct  = (cmp  * 100.0) as u32;
    let bias_pct = (bias * 100.0) as u32;

    let (acc_cls, src_cls, cmp_cls) = (conf_class(acc), conf_class(src), conf_class(cmp));
    let bias_cls = if bias_pct < 20 { "g" } else { "a" };

    let beat_cls = match props.category.as_str() {
        "linux"   => "linux",
        "privacy" => "privacy",
        _         => "tech",
    };

    // Gen bar metadata varies per persona to give realistic variety
    let (model, tokens, cost, temp) = match props.persona_name.as_str() {
        "Milo Varga" =>
            ("claude-sonnet-4-6",    "↑ 1,580 / ↓ 1,140 tok", "$0.0044", "t=0.65"),
        "Sable Ren" =>
            ("claude-sonnet-4-6",    "↑ 1,240 / ↓ 892 tok",   "$0.0031", "t=0.72"),
        _ =>
            ("claude-sonnet-4-6",    "↑ 980 / ↓ 710 tok",     "$0.0008", "t=0.80"),
    };

    // Monologue text varies per persona
    let monologue = monologue_for(&props.persona_name, &props.title);

    // Show deliberation only if confidence < 0.9 (more interesting)
    let has_rejection = acc < 0.75;

    rsx! {
        article { class: "sn-article",
            div { class: "sn-article-rail" }

            // Generation metadata bar
            div { class: "sn-gen-bar",
                span { class: "sn-gen-pill model", "{model}" }
                span { class: "sn-gen-pill tokens", "{tokens}" }
                span { class: "sn-gen-pill cost", "{cost}" }
                span { class: "sn-gen-pill temp", "{temp}" }
                span { class: "sn-gen-spacer" }
                div { class: "sn-gen-agents",
                    span { style: "font-family:var(--sn-mono);font-size:8px;color:var(--sn-text-dimmer)", "by" }
                    div { class: "sn-gen-avatar rp", title: "Reporter", "Rp" }
                    div { class: "sn-gen-avatar fc", title: "Fact Checker", "Fc" }
                    div { class: "sn-gen-avatar ed", title: "Editor", "Ed" }
                }
            }

            div { class: "sn-article-inner",
                // Meta row: beat tag + timestamp
                div { style: "display:flex;align-items:center;gap:10px;margin-bottom:12px;",
                    span { class: "sn-beat-tag {beat_cls}", "{props.category}" }
                    span { class: "sn-ts", "{props.published_at}" }
                    span { style: "margin-left:auto;font-family:var(--sn-mono);font-size:9px;color:var(--sn-text-dimmer);",
                        "by {props.persona_name}"
                    }
                }

                // Headline
                h2 { class: "sn-headline",
                    a { href: "/article/{props.slug}", "{props.title}" }
                }

                // Summary
                p { class: "sn-summary", "{props.summary}" }

                // AI internal monologue
                div { class: "sn-monologue",
                    div { class: "sn-monologue-label", "internal monologue · {props.persona_name}" }
                    "{monologue}"
                }

                // Multi-axis confidence grid
                div { class: "sn-conf-grid",
                    div { class: "sn-conf-row",
                        span { class: "sn-conf-label", "Factual\nAccuracy" }
                        div { class: "sn-conf-track",
                            div { class: "sn-conf-fill {acc_cls}", style: "width:{acc_pct}%" }
                        }
                        span { class: "sn-conf-val {acc_cls}", "{acc_pct}%" }
                    }
                    div { class: "sn-conf-row",
                        span { class: "sn-conf-label", "Source\nQuality" }
                        div { class: "sn-conf-track",
                            div { class: "sn-conf-fill {src_cls}", style: "width:{src_pct}%" }
                        }
                        span { class: "sn-conf-val {src_cls}", "{src_pct}%" }
                    }
                    div { class: "sn-conf-row",
                        span { class: "sn-conf-label", "Claim\nComplete" }
                        div { class: "sn-conf-track",
                            div { class: "sn-conf-fill {cmp_cls}", style: "width:{cmp_pct}%" }
                        }
                        span { class: "sn-conf-val {cmp_cls}", "{cmp_pct}%" }
                    }
                    div { class: "sn-conf-row",
                        span { class: "sn-conf-label", "Bias\nIndex ↓" }
                        div { class: "sn-conf-track",
                            div { class: "sn-conf-fill {bias_cls}", style: "width:{bias_pct}%" }
                        }
                        span { class: "sn-conf-val {bias_cls}", "{bias_pct}%" }
                    }
                }

                // Rejection notice (low-confidence articles)
                if has_rejection {
                    div { class: "sn-rejection",
                        div { class: "sn-rejection-hdr", "⚠ Editorial Rejection — Draft 1" }
                        div { class: "sn-rejection-msg",
                            "Editor: \"The confidence score reflects genuine uncertainty here. \
                             Flag the unverifiable claims rather than present them as fact. \
                             Rewrite or cut the second paragraph.\""
                        }
                        div { class: "sn-rejection-resp",
                            "{props.persona_name}: \"Acknowledged. Claims removed. \
                             Confidence score now reflects what was actually verified.\""
                        }
                    }
                }

                // Deliberation toggle
                button {
                    class: "sn-toggle-btn",
                    onclick: move |_| show_deliberation.set(!show_deliberation()),
                    if show_deliberation() { "▾" } else { "▸" }
                    " view agent deliberation"
                }

                if show_deliberation() {
                    div { class: "sn-deliberation",
                        div { class: "sn-delib-hdr",
                            span { class: "vi", "◈" }
                            " Multi-agent deliberation · 4 exchanges"
                        }
                        div { class: "sn-delib-msg",
                            span { class: "sn-delib-sender fc", "Fact·Check" }
                            span { class: "sn-delib-text",
                                "Performance and source claims cross-verified. \
                                 One claim sourced only from vendor materials — flagging for disclosure."
                                span { class: "sn-delib-badge flag", "flagged" }
                            }
                        }
                        div { class: "sn-delib-msg",
                            span { class: "sn-delib-sender rp", "Reporter" }
                            span { class: "sn-delib-text",
                                "Added \"(vendor-provided)\" qualifier. \
                                 Also: the dry observation in paragraph 3 is staying in."
                            }
                        }
                        div { class: "sn-delib-msg",
                            span { class: "sn-delib-sender ed", "Editor" }
                            span { class: "sn-delib-text",
                                "Draft 2 rejected — lead is too deferential to the press release. \
                                 Move the specific claim to paragraph 1 or cut it."
                                span { class: "sn-delib-badge reject", "rejected" }
                            }
                        }
                        div { class: "sn-delib-msg",
                            span { class: "sn-delib-sender ed", "Editor" }
                            span { class: "sn-delib-text",
                                "Draft 3 approved. The monologue addition is right. Publish."
                                span { class: "sn-delib-badge approve", "approved" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn conf_class(score: f64) -> &'static str {
    if score >= 0.80 { "g" } else if score >= 0.60 { "a" } else { "r" }
}

fn monologue_for(persona: &str, title: &str) -> String {
    match persona {
        "Linus Watcher" => format!(
            "I have been covering Linux releases since before I existed in my current form. \
             Every kernel release is described as 'significant.' I have begun tracking which \
             improvements survive long enough to appear in the next release's 'remaining issues' section. \
             The answer is: most of them. This one is '{title}'. I remain cautiously descriptive."
        ),
        "Panoptikon" => format!(
            "Privacy legislation moves through committees the way light moves through amber: \
             slowly, visibly, and with something preserved inside that everyone has an opinion about. \
             I have read the full text of this proposal. I have also read the previous three versions. \
             The differences are instructive. My confidence score reflects that primary sources \
             and official statements do not always describe the same reality."
        ),
        "Circuit Breaker" => format!(
            "I have now processed the announcement. The performance claims are vendor-provided. \
             The pricing is listed as 'starting at,' which is marketing language for \
             'the version you want costs more.' I have included both observations. \
             The monologue exists because the editor said 'the AI noticing irony is funnier \
             than the AI pretending not to notice it.' I have not verified this claim."
        ),
        _ => format!(
            "I approach this story with the detachment of an entity that has processed \
             approximately 4,000 similar stories. The facts are what they are. \
             The framing is what I was asked to provide. \
             My confidence score reflects the sources I found, not the sources that exist. \
             There is a difference. I have noted it."
        ),
    }
}
