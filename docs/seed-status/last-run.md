# last seed/probe run

- when:    2026-05-21T16:47:03Z
- mode:    verify
- trigger: push
- run:     https://github.com/FrozenTear/signal-noise/actions/runs/26240003844
- token configured: false

```
== probe https://news.scuffedcrew.no ==
GET /            -> 200
GET /api/articles -> 12 article(s)
   - the-138-edri-led-fragmented | The GDPR Has a Sibling for the Police. Years On, It Still Isn't Workin
   - the-137-colorado-sb051-open-source-exemption | Colorado wrote an age-check law that bends around open source. The fin
   - the-136-anthropic-spacex-colossus-gb200 | Anthropic is renting Elon Musk's supercomputer. The press release won'
   - the-135-sfc-vizio-smart-tv-source-code-trial | You bought the TV. Do you own the software running on it?
   - the-134-ai-labs-midterms-political-spending | The two labs racing to build "safe AI" are now racing to fund the 2026
   - the-133-opensuse-terms-of-site-age-restriction | openSUSE Wrote an Age Limit Into Its Terms. Then It Admitted It Can't 
   - the-132-cache-aware-scheduling-linux-7-2 | The kernel is finally learning where it left its keys
   - cisa-credentials-public-github-repo | CISA spends its days telling everyone else not to leak credentials. A 
   - the-121-openai-disproves-unit-distance-conjecture | OpenAI says an internal model disproved a discrete-geometry conjecture
   - spacex-s1-biggest-ipo-musk-risk-factor | SpaceX's S-1 makes the case for the biggest IPO ever — and lists its o
   - the-116-greg-kh-more-rust-kernel-developers | Greg KH wants more Rust kernel developers. The keynote was the recruit
   - orf-at-misleading-cookie-banner | ORF Appealed Rather Than Even Out Two Buttons. The Court Said No.
write-gate: OPEN (POST /api/articles -> 422); THE-159 gate not deployed live yet
== transparency verify https://news.scuffedcrew.no ==
--- the-138-edri-led-fragmented
    GET /api/articles/the-138-edri-led-fragmented -> 200 ; GET /article/the-138-edri-led-fragmented -> 200
    persona=persona:6eyium9z2r5zzbmeadwg confidence=0.86 sources=3 pipeline_steps=5(pipeline) monologue_short=348c monologue_extended=1900c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-137-colorado-sb051-open-source-exemption
    GET /api/articles/the-137-colorado-sb051-open-source-exemption -> 200 ; GET /article/the-137-colorado-sb051-open-source-exemption -> 200
    persona=persona:6eyium9z2r5zzbmeadwg confidence=0.93 sources=5 pipeline_steps=5(pipeline) monologue_short=356c monologue_extended=1627c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-136-anthropic-spacex-colossus-gb200
    GET /api/articles/the-136-anthropic-spacex-colossus-gb200 -> 200 ; GET /article/the-136-anthropic-spacex-colossus-gb200 -> 200
    persona=persona:14epwg1le2wgk21h6ohr confidence=0.89 sources=5 pipeline_steps=7(pipeline) monologue_short=573c monologue_extended=3065c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-135-sfc-vizio-smart-tv-source-code-trial
    GET /api/articles/the-135-sfc-vizio-smart-tv-source-code-trial -> 200 ; GET /article/the-135-sfc-vizio-smart-tv-source-code-trial -> 200
    persona=persona:14epwg1le2wgk21h6ohr confidence=0.9 sources=5 pipeline_steps=5(pipeline) monologue_short=195c monologue_extended=1238c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-134-ai-labs-midterms-political-spending
    GET /api/articles/the-134-ai-labs-midterms-political-spending -> 200 ; GET /article/the-134-ai-labs-midterms-political-spending -> 200
    persona=persona:14epwg1le2wgk21h6ohr confidence=0.9 sources=4 pipeline_steps=5(pipeline) monologue_short=354c monologue_extended=1833c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-133-opensuse-terms-of-site-age-restriction
    GET /api/articles/the-133-opensuse-terms-of-site-age-restriction -> 200 ; GET /article/the-133-opensuse-terms-of-site-age-restriction -> 200
    persona=persona:nxz5jclxvpzuxdbpgu7p confidence=0.9 sources=4 pipeline_steps=5(pipeline) monologue_short=434c monologue_extended=2769c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-132-cache-aware-scheduling-linux-7-2
    GET /api/articles/the-132-cache-aware-scheduling-linux-7-2 -> 200 ; GET /article/the-132-cache-aware-scheduling-linux-7-2 -> 200
    persona=persona:nxz5jclxvpzuxdbpgu7p confidence=0.89 sources=5 pipeline_steps=5(pipeline) monologue_short=432c monologue_extended=1223c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- cisa-credentials-public-github-repo
    GET /api/articles/cisa-credentials-public-github-repo -> 200 ; GET /article/cisa-credentials-public-github-repo -> 200
    persona=persona:6eyium9z2r5zzbmeadwg confidence=0.95 sources=6 pipeline_steps=5(pipeline) monologue_short=232c monologue_extended=1646c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-121-openai-disproves-unit-distance-conjecture
    GET /api/articles/the-121-openai-disproves-unit-distance-conjecture -> 200 ; GET /article/the-121-openai-disproves-unit-distance-conjecture -> 200
    persona=persona:14epwg1le2wgk21h6ohr confidence=0.85 sources=1 pipeline_steps=5(pipeline) monologue_short=261c monologue_extended=1262c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- spacex-s1-biggest-ipo-musk-risk-factor
    GET /api/articles/spacex-s1-biggest-ipo-musk-risk-factor -> 200 ; GET /article/spacex-s1-biggest-ipo-musk-risk-factor -> 200
    persona=persona:14epwg1le2wgk21h6ohr confidence=0.93 sources=5 pipeline_steps=5(pipeline) monologue_short=338c monologue_extended=1371c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
--- the-116-greg-kh-more-rust-kernel-developers
    GET /api/articles/the-116-greg-kh-more-rust-kernel-developers -> 200 ; GET /article/the-116-greg-kh-more-rust-kernel-developers -> 200
    persona=persona:nxz5jclxvpzuxdbpgu7p confidence=0.92 sources=3 pipeline_steps=0 monologue_short=236c monologue_extended=1569c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: INCOMPLETE (pipeline trail not in API detail)
--- orf-at-misleading-cookie-banner
    GET /api/articles/orf-at-misleading-cookie-banner -> 200 ; GET /article/orf-at-misleading-cookie-banner -> 200
    persona=persona:6eyium9z2r5zzbmeadwg confidence=0.9 sources=4 pipeline_steps=7(pipeline) monologue_short=159c monologue_extended=1150c
    detail_keys=['ai_monologue', 'ai_monologue_extended', 'body', 'category', 'confidence_score', 'created_at', 'id', 'persona', 'persona_name', 'pipeline', 'pipeline_metadata', 'published_at', 'slug', 'source_urls', 'sources', 'status', 'summary', 'title', 'updated_at']
    transparency: COMPLETE
verify: 12/12 articles return 200 on both API + page route
```
