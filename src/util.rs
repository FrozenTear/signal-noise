/// Lightweight markdown-ish text → HTML converter.
/// Splits on blank lines into `<p>` paragraphs and handles `#`/`##`/`###` headings.
///
/// All text content passes through `html_escape` before insertion, so article bodies
/// and monologues rendered via `dangerous_inner_html` in Dioxus are XSS-safe even
/// when the stored text contains `<script>` or other HTML.
pub fn simple_md_to_html(md: &str) -> String {
    let mut html = String::new();
    let mut in_paragraph = false;

    for line in md.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if in_paragraph {
                html.push_str("</p>");
                in_paragraph = false;
            }
            continue;
        }

        if let Some(heading) = trimmed.strip_prefix("### ") {
            if in_paragraph { html.push_str("</p>"); in_paragraph = false; }
            html.push_str(&format!("<h3>{}</h3>", html_escape(heading)));
        } else if let Some(heading) = trimmed.strip_prefix("## ") {
            if in_paragraph { html.push_str("</p>"); in_paragraph = false; }
            html.push_str(&format!("<h2>{}</h2>", html_escape(heading)));
        } else if let Some(heading) = trimmed.strip_prefix("# ") {
            if in_paragraph { html.push_str("</p>"); in_paragraph = false; }
            html.push_str(&format!("<h1>{}</h1>", html_escape(heading)));
        } else {
            if !in_paragraph {
                html.push_str("<p>");
                in_paragraph = true;
            } else {
                html.push(' ');
            }
            html.push_str(&html_escape(trimmed));
        }
    }

    if in_paragraph {
        html.push_str("</p>");
    }

    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
