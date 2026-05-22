# The FBI Wants to Watch Every Road in America

**By Sable Ren | Privacy & Surveillance**

Automated license-plate readers have spread across American roads faster than most people realize — mounted on telephone poles, at parking-lot entrances, on patrol cars, and at highway on-ramps, quietly logging every passing vehicle to cloud databases accessible to law enforcement. Flock Safety cameras scan the quiet suburbs. Motorola Solutions' Vigilant network covers highways and commercial corridors. Together, these private and public systems have compiled what is almost certainly the largest vehicle-movement database in American history — billions of records in aggregate, indexed by plate, time, and location.

The FBI would like to query all of it. In near real time.

A request for proposals posted to SAM.gov on May 14, 2026 by the FBI's Directorate of Intelligence shows the bureau seeking a nationwide license-plate-reader access contract worth up to $36 million — structured as six coverage areas at $6 million each. The procurement, first reported by 404 Media's Joseph Cox on May 18 and confirmed by Ars Technica the following day, calls for a SaaS platform that would let agents search approximately 30 billion records spanning five years and receive location alerts within two minutes. The requirement is blunt: data accessible "in near real time."

The FBI "requires professional service firms that can provide License Plate Readers (LPRs) for tracking subjects on roads and highways over the US and its territories," the RFP states, as quoted by Ars Technica.

## How the current patchwork works

ALPR networks already operate at scale, but access is fragmented. When a Flock Safety camera reads a plate, it uploads — within seconds — the plate number, vehicle make, model, color, timestamp, and GPS coordinates to Flock's cloud. Local law-enforcement agencies that subscribe can query their own cameras and, through Flock's nationwide agency-sharing network, data from participating agencies elsewhere in the country. Motorola Solutions' Vigilant subsidiary runs the National Vehicle Location Service (NVLS), a separate but overlapping national platform with similar cross-agency query capability.

What doesn't currently exist — at least not in a formalized, federal form — is a single query interface giving a bureau like the FBI real-time access to the aggregate of those networks across all 50 states and territories. That's what this RFP is designed to build. A proposed 75% national coverage target, paired with two-minute alert capability, would make it the most comprehensive federal vehicle-location tool ever assembled.

## What "near real time" changes

The phrase matters. ALPR data has long been available to federal investigators through subpoenas to local agencies, tip lines, or existing data-sharing compacts — but those are retrospective channels. You request records after you know who you're looking for. Near-real-time access inverts that relationship: query the system while a person is still driving, receive an alert the moment their plate is scanned, respond before they reach their destination.

That's not a marginal upgrade in investigative capability. That's a different kind of tool entirely.

Civil-liberties advocates have spent years making this distinction. The Electronic Frontier Foundation's January 2024 review found that California agencies were illegally sharing ALPR data out of state, in violation of California's ALPR data-sharing law (Civil Code §1798.90.5 et seq., SB 34). The ACLU has documented cases in which law enforcement acted on ALPR misreads, leading to wrongful vehicle stops and at least one wrongful arrest. In San Jose, a lawsuit over warrantless ALPR use resulted in a settlement and reform of department policy.

Those cases involved local and regional networks. A federated national system with two-minute alert capability is orders of magnitude larger in scope.

## What the FBI formally asked for — and what advocates say it means

The RFP is a procurement document, not a legal brief. It describes what the FBI wants to buy; it does not explain what legal authority it would invoke to use it. That gap is where the civil-liberties argument lives.

**What the FBI formally requested:** nationwide ALPR query access "in near real time," covering approximately 30 billion records, with two-minute alerts and up to $36 million over the life of the contract.

**What Fourth Amendment scholars and civil-liberties groups infer from that:** a warrantless mass-tracking capability that, depending on how it's deployed, would allow investigators to monitor the movements of ordinary Americans with no individualized suspicion — no warrant, no court order, no probable cause. Under *Carpenter v. United States* (2018), the Supreme Court held that obtaining historical cell-site location data requires a warrant. Whether that logic extends to federally aggregated ALPR data has not been tested at the Supreme Court level.

The distinction between "requested" and "inferred consequence" is not hair-splitting. It's the difference between an administrative procurement and a constitutional question — and civil-liberties advocates argue the bureau is treating the former as a way to quietly answer the latter in its own favor.

## The vendor side

Flock Safety and Motorola Solutions are the likely frontrunners for the contract, given their existing national footprint. Flock has publicly stated that federal data-sharing through its platform is "disabled by default," requiring opt-in consent from the local agencies that own the cameras, and that no federal agency receives a standing backdoor.

That statement should be understood for what it is: a vendor claim, not independently verified, and not binding on the terms of a future federal contract. The RFP describes a managed service that would provide the query capability; how any existing "disabled by default" settings would interact with a federal contract is not addressed in current reporting.

## The federalism wrinkle

California and Virginia have both passed laws restricting how ALPR data can be shared with federal agencies, Ars Technica reported. Those state-level protections create a legal friction point: even if the FBI contracts for 75% national coverage, data from states with restrictive laws may be effectively off-limits — or require separate litigation to access.

That friction is, from a civil-liberties perspective, the system working as designed. From an FBI procurement perspective, it may explain why the RFP is structured by coverage area rather than a single national contract.
