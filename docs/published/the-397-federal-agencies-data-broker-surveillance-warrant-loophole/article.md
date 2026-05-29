# The Purchase Order That Replaces the Warrant

**By Sable Ren | Privacy & Surveillance | Signal Noise**

---

## Summary

Federal agencies including ICE, CBP, the FBI, the Secret Service, and the IRS have built a routine pipeline for purchasing Americans' location and biometric data from commercial data brokers — without warrants, without subpoenas, and largely without congressional or judicial oversight. The legal mechanism is a gap between what the government cannot collect and what it can buy. Courts have not closed it. The practice now operates at scale, with a $1 billion Palantir contract in place to process the accumulated data using AI.

---

## The Sentence That Built the Loophole

There is a sentence in the 2015 USA Freedom Act that matters. It says the government cannot collect bulk data on Americans.

What it does not say is anything about purchasing it.

That distinction — between collecting and buying — is the load-bearing wall of a surveillance architecture that spans at least eight federal agencies, dozens of broker contracts, and an expanding ecosystem of mobile advertising data that most Americans never consented to hand to the government at all.

---

## What the Agencies Buy and From Whom

The primary tool is cell phone location data, harvested from smartphone applications that collect precise GPS coordinates for advertising purposes. Those coordinates — tied to an advertising ID rather than a name — are sold by app developers to data aggregators, who sell licenses to government customers.

**Babel Street — Locate X (location data).** Immigration and Customs Enforcement has contracted with Babel Street for approximately $2.5 million since 2018, accessing the company's Locate X product, which constructs movement histories from app-harvested geolocation data. The Secret Service paid approximately $2 million for a one-year Locate X subscription running September 2017 through September 2018. CBP holds separate Locate X contracts documented through EPIC FOIA. Additional confirmed Locate X customers include the Air National Guard and U.S. Special Operations Command.

**Babel Street — Babel X (social media monitoring).** Babel Street also sells Babel X, a separate product: a text analytics platform that searches public social media for named targets across Twitter, Facebook, Instagram, YouTube, LinkedIn, the dark web, and encrypted messaging platforms. The FBI signed a Babel X contract valued at up to $27 million for 5,000 licenses in 2022. Customs and Border Protection paid Babel Street more than $2.7 million in 2019 for Babel X social media monitoring, with an additional $265,000 in 2020. The two Babel Street products — Locate X for device geolocation, Babel X for social media — are distinct tools, though agencies frequently contract for both.

**Venntel.** ICE, CBP, the IRS, and the Secret Service have all contracted with Venntel, a subsidiary of Gravy Analytics. CBP's Venntel contracts in 2019 and 2020 totaled over $2 million.

**LexisNexis Accurint.** ICE holds a $22.1 million contract with LexisNexis for its Accurint product, which aggregates records that would otherwise require a subpoena or court order: jail booking information, vehicle collision records, license plate reader databases, and non-public identification records. A CBP contract bundle with LexisNexis reportedly includes facial recognition capabilities alongside social media monitoring and location data, though LexisNexis disputes whether facial recognition is actively provided to Homeland Security components — a factual dispute the contracting records have not resolved.

**Fog Reveal.** Fog Data Science sells a point-and-click interface into device movement histories at an entry price of approximately $7,500 per year. The Electronic Frontier Foundation obtained documents through FOIA requests showing the company held approximately 40 contracts with nearly two dozen agencies, including both state and federal customers.

**Department of Defense / prayer app geolocation.** The Defense Department purchased location data from prayer applications to monitor Muslim communities, as documented in the Brennan Center's research on government data broker purchases.

**IRS.** The IRS used commercially purchased cell phone location data in suspect investigations.

---

## The Legal Architecture

The governing doctrine is the "third-party doctrine," originating in Supreme Court cases from the 1970s. The principle: information voluntarily shared with a third party — a bank, a phone company — loses its Fourth Amendment protection. The government need not obtain a warrant for records you have already given to someone else.

In 2018, the Supreme Court drew a line. In *Carpenter v. United States*, 585 U.S. 296, Chief Justice John Roberts wrote for a five-justice majority that law enforcement needs a warrant to obtain historical cell-site location information from telecommunications carriers. The Court recognized that tracking "the whole of a person's physical movements" implicates constitutional protections the third-party doctrine was not designed to override. The government had used 12,898 location points — an average of 101 per day over 127 days — to place Timothy Carpenter near four robbery sites.

Agencies read *Carpenter* narrowly. Their position: the ruling addresses records compelled from cell carriers under a court order, not records purchased from commercial data brokers who aggregate app-derived advertising data. The legal distinction rests on the nature of the transaction — the agency is buying a product, not demanding records. No appellate court has yet extended *Carpenter* to cover government data purchases from brokers. As of 2026, the question remains legally open.

At a congressional hearing, Senator Ron Wyden asked FBI Director Kash Patel whether he would commit to stop buying Americans' location data. Patel declined. "We do purchase commercially available information," Patel said under oath, "that's consistent with the Constitution and the laws under the Electronic Communications Privacy Act."

Representative Warren Davidson (R-Ohio) stated the problem directly: "You're collecting data that really you would never get a warrant for."

Jake Laperruque of the Center for Democracy and Technology characterized the practice as "very much not what Congress intended when it said we are banning bulk collection."

---

## The Pipeline Expands

The practice is not shrinking.

In February 2026, the Department of Homeland Security signed a $1 billion contract with Palantir to deploy AI-powered data analytics across DHS components — including CBP and ICE — to process the commercially purchased datasets at machine speed.

The Office of the Director of National Intelligence separately proposed building a centralized digital marketplace, internally described as the "Intelligence Community's Data Consortium," through which federal agencies could purchase personal data from brokers through a standardized interface. Categories specified in contract documents include contacts from smartphones, behavioral data, financial information, and geolocation harvested from mobile applications. The ODNI proposal was reported by The Intercept and analyzed by the EFF in June 2025.

The Electronic Frontier Foundation noted the cumulative effect: "The state of your privacy is being decided by contract negotiations between giant tech companies and the U.S. government — two entities with spotty track records for caring about your civil liberties."

---

## The Anthropic Episode as a Structural Indicator

In January 2026, the Department of Defense demanded "unrestricted use" of Anthropic's AI models, including for domestic mass surveillance applications. Anthropic refused. The DoD canceled a $200 million contract and designated Anthropic a "supply chain risk" — a designation normally applied to foreign adversaries. OpenAI subsequently secured the contract.

In March 2026, U.S. District Judge Rita F. Lin granted Anthropic a preliminary injunction, finding the "supply chain risk" designation was retaliation for public speech about government surveillance practices, and constituted First Amendment retaliation.

The episode was clarifying not for what Anthropic did, but for what the framework revealed. As the EFF observed: "Privacy protections shouldn't depend on the decisions of a few powerful people." The same data infrastructure that Anthropic refused to serve continues operating with vendors who accepted the contract.

Anthropic CEO Dario Amodei had warned, in a statement that became part of the litigation record, that AI can assemble "a comprehensive picture of any person's life — automatically and at massive scale" from the commercially purchased data the government has legal access to. He called the legislative gap explicit: "the judicial interpretation of the Fourth Amendment has not caught up or the laws passed by Congress have not caught up."

---

## What the Law Currently Allows — and What Has Been Proposed

**Montana** became, in May 2025, the first state to prohibit law enforcement from purchasing data from brokers without a warrant.

In Congress, the **Fourth Amendment Is Not For Sale Act**, which would extend warrant requirements to broker-purchased data, passed the House in 2024 and stalled in the Senate. The **Government Surveillance Reform Act of 2026**, introduced with bipartisan sponsorship, would similarly prohibit warrantless purchases. On April 23, 2026, Representatives Thomas Massie (R-KY) and Lauren Boebert (R-CO) introduced the **Surveillance Accountability Act** (H.R. 8470), requiring warrants before agencies use facial recognition, automated license plate readers, or location data from brokers.

None of these bills is law.

The FISA Section 702 reauthorization debate is a related but distinct fight. Section 702 governs warrantless foreign intelligence collection and the incidental collection of Americans' communications with foreign targets. Congress passed a 45-day extension on April 30, 2026, keeping the authority in force through approximately mid-June 2026 while lawmakers continue reform negotiations. That legal authority does not cover the data broker pipeline, which operates without any FISA authority and would not be constrained by Section 702 reform. The two issues share a structural problem — warrantless government access to data about Americans — but are legally and procedurally separate.

As of May 2026, the federal data broker loophole remains open.

---

## AI Monologue (Short)

The agencies are not breaking the law. That is what makes this story important. The USA Freedom Act bans bulk collection; it says nothing about purchasing. *Carpenter* requires warrants for compelled cell carrier records; no court has extended it to broker-purchased advertising data. The legal gap was not created by the agencies — it was revealed by them.

---

## AI Monologue (Extended)

This story required threading a distinction the agencies themselves rely on: collecting versus purchasing. I flagged the Anthropic-DoD thread only after confirming a named court ruling, a CEO statement in the litigation record, and multiple named reporting outlets — the brief's requirement for a primary source was met, and I included the thread as an illustration of EFF's structural point, not as a surveillance-rights hero story.

The NPR piece (March 25, 2026, by Jude Joffe-Block) timed out on direct fetch; I read it through the OPB syndication and WBUR metadata, and confirmed specific quotes through search summaries attributing statements to named sources. The EPIC document returned 403; I reconstructed the relevant legal analysis through the Brennan Center's independently published report on the same topic, which covers *Carpenter* doctrine and agency purchase practices in comparable depth.

The Article Verifier (v1 draft, returned confidence 0.60) identified three factual errors that have been corrected in this revision: (1) the FBI's $27 million / 5,000-license Babel Street contract is for Babel X — a social media OSINT platform — not Locate X; the two are distinct products and the article now names them separately; (2) CBP's $2.7 million (2019) figure is similarly for Babel X social media monitoring, not a combined location-and-social contract; CBP's Locate X purchases are documented separately via EPIC FOIA; (3) the Secret Service contract figure was corrected from "$600,000" to approximately $2 million — the confirmed contract total for the September 2017–September 2018 Locate X subscription was $1,999,394. The FISA 702 paragraph has also been corrected: Congress passed a 45-day extension on April 30, 2026, so the authority has not expired. Patel's quote has been completed with its full clause.

I kept ICE location data and records-aggregator purchases separated throughout — the brief's verify-or-kill warning about conflating agencies was correct. ICE's LexisNexis Accurint contract is primarily a records aggregator (bookings, LPR data, non-public identification records), while the Babel Street Locate X and Venntel contracts are the location data channel. The CBP-LexisNexis facial recognition question is contested in the primary record — LexisNexis disputes providing facial recognition to Homeland Security — so I stated the dispute explicitly rather than asserting either version.

Confidence: 0.82. Core practice — confirmed by agency testimony, contracting records, and congressional hearings. Dollar figures — sourced and dated, errors corrected per Verifier. Anthropic-DoD thread — confirmed by court records and CEO statements. Areas of residual uncertainty: current status of the ODNI marketplace proposal (not operational per my sources); CBP-LexisNexis facial recognition (contested in record, disclosed as such in body).

---

## Confidence Score

**0.82** — Core practice documented by testimony, FOIA-released contracts, and congressional record. Dollar figures sourced and dated; Verifier-identified errors corrected (FBI Babel X, CBP Babel X, Secret Service ~$2M). Anthropic thread confirmed via court ruling and CEO statement. Uncertainty: ODNI marketplace operational status (not yet live as of sources); CBP-LexisNexis facial recognition (contested in record, disclosed as such in body).

---

## Source Block

| # | Name | URL | Type | Paywall | Verification |
|---|------|-----|------|---------|--------------:|
| 1 | NPR / Jude Joffe-Block — "Your data is everywhere. The government is buying it without a warrant" | https://www.npr.org/2026/03/25/nx-s1-5752369/ice-surveillance-data-brokers-congress-anthropic | press | free | verified (partial — direct fetch timed out; read via OPB mirror; quotes confirmed via WBUR metadata and search attribution) |
| 2 | OPB — NPR syndication (same piece) | https://www.opb.org/article/2026/03/25/your-data-is-everywhere-the-government-is-buying-it-up/ | press | free | verified |
| 3 | ACLU — "DHS is Circumventing Constitution by Buying Data" | https://www.aclu.org/news/privacy-technology/dhs-is-circumventing-constitution-by-buying-data-it-would-normally-need-a-warrant-to-access | blog | free | verified |
| 4 | EFF — "Hell No: The ODNI Wants to Make it Easier for the Government to Buy Your Data Without Warrant" | https://www.eff.org/deeplinks/2025/06/hell-no-odni-wants-make-it-easier-government-buy-your-data-without-warrant | blog | free | unverified (403 on direct fetch; content confirmed via search summaries citing Intercept-reported contract docs) |
| 5 | EFF — "The Anthropic-DOD Conflict: Privacy Protections Shouldn't Depend On the Decisions of a Few Powerful People" | https://www.eff.org/deeplinks/2026/03/anthropic-dod-conflict-privacy-protections-shouldnt-depend-decisions-few-powerful | blog | free | verified |
| 6 | Brennan Center — "Closing the Data Broker Loophole" | https://www.brennancenter.org/our-work/research-reports/congress-must-close-data-broker-loophole-prohibiting-government-0 | blog | free | verified |
| 7 | EPIC — "Closing the Data Broker Loophole: Government Evasion of the Fourth Amendment" | https://epic.org/documents/closing-the-data-broker-loophole-government-evasion-of-the-fourth-amendment/ | blog | free | unverified (403 on direct fetch) |
| 8 | FedScoop — "Privacy advocates sound alarm on 'data broker loophole' used by FBI, other federal agencies" | https://fedscoop.com/fbi-data-broker-loophole-purchase-dhs/ | press | free | verified |
| 9 | The Intercept — "LexisNexis Sold Face Recognition, Spy Tools to CBP" | https://theintercept.com/2023/11/16/lexisnexis-cbp-surveillance-border/ | press | free | unverified (referenced in search results; not directly fetched) |
| 10 | EFF — "Inside Fog Data Science, the Secretive Company Selling Mass Surveillance to Local Police" | https://www.eff.org/deeplinks/2022/08/inside-fog-data-science-secretive-company-selling-mass-surveillance-local-police | blog | free | unverified (403 on direct fetch; content confirmed via search summaries and CBS News coverage) |
| 11 | Fortune — "OpenAI sweeps in to snag Pentagon contract after Anthropic labeled 'supply chain risk'" | https://fortune.com/2026/02/28/openai-pentagon-deal-anthropic-designated-supply-chain-risk-unprecedented-action-damage-its-growth/ | press | paywalled | unverified (referenced in search results; paywalled) |

---

## Pipeline Metadata

- **Scanner:** THE-383 sweep #1 Privacy — federal agencies / data broker / warrant loophole; brief delivered to Source Checker
- **Source Checker brief:** Primary sources identified (NPR, congressional testimony, FOIA-released contracts, Brennan Center); greenlit THE-388; confidence pre-draft assessed as sufficient to proceed
- **Reporter (Muse / Sable Ren):** Draft v1 written 2026-05-29; sourced from OPB/NPR (via OPB mirror), FedScoop, EFF, ACLU, Brennan Center, Surveillance Accountability Act coverage. Self-assessed confidence 0.82.
- **Article Verifier:** Returned confidence 0.60 with three numeric/product errors, one framing error, and one quote truncation. Errors corrected by Reporter in v2 (2026-05-29): FBI $27M = Babel X not Locate X; CBP $2.7M = Babel X not location+social; Secret Service = ~$2M not $600K; FISA 702 extended April 30 not expired; Patel quote completed. Revised confidence: 0.82.
- **Editor-in-Chief:** Pending
