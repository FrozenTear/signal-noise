# last seed/probe run

- when:    2026-07-13T07:07:11Z
- mode:    seed
- trigger: push
- run:     https://github.com/FrozenTear/signal-noise/actions/runs/29231002089
- token configured: false

```
== probe https://news.scuffedcrew.no ==
GET /            -> 200
GET /api/articles -> 20 article(s)
   - weekly-editorial-retrospective-2026-06-29-to-07-05 | Editor's Log, Week of June 29 - July 5: We Published Nothing, and Here
   - weekly-editorial-retrospective-2026-06-22-to-28 | Editor's Log, Week of June 22-28: We Shipped 5, Killed 4, and Were Off
   - the-999-podman-6-0-breaking-changes | Podman 6.0 Ships: Six Years of Accumulated Defaults Finally Change
   - the-1003-polestar-us-ban-volvo-spared-connected-vehicle | Polestar Loses US Market Access Under Connected Vehicle Rule While Gee
   - the-1001-windows-10-esu-extended-october-2027 | Microsoft Extends Free Windows 10 Security Updates Through October 202
   - the-1000-allocation-tokens-linux-7-2-slab-hardening | The Kernel Bets on Both Fronts: Allocation Tokens in Linux 7.2
   - the-998-minio-alternatives-ceph-garage | MinIO steps back, and the replacements line up
   - the-887-krishnan-leaving-white-house-ai-advisor | Trump's Senior AI Policy Architect Is Leaving the White House
   - the-885-linux-7-1-final-in-sight-rc7-lands | Linux 7.1 Final in Sight as rc7 Lands
   - the-857-final-fantasy-vii-revelation-multiplatform-launch | Square Enix Ends PlayStation Exclusivity Window With Final Fantasy VII
   - the-863-gps-encrypted-military-traffic-otad-research | GPS Has Been Carrying Encrypted Military Traffic for Two Decades, Rese
   - the-861-ice-tfm-app-287g-facial-recognition | ICE Planned Facial Recognition for 1,200 Local Police Forces. Eight Mo
   - the-809-fifty-years-of-fork-exec | Fifty Years of fork(). Somebody Finally Filed a Bug Report.
   - the-834-caltrans-permits-alpr-sb34-jurisdictional-gap | The Permits Were Caltrans's. The Cameras Were Border Patrol's. Califor
   - the-813-cpp-documentary-40-years | C++: The Documentary Charts 40 Years of a Language That Refuses to Ret
   - the-810-bundler-cooldown-supply-chain | Bundler 4.0.13 Adds a Waiting Period for New Gems
   - the-819-federal-alpr-caltrans-permit-network-california-border | Federal Agencies Used Highway Permits to Install a Hidden License Plat
   - the-827-iss-zvezda-prk-shelter-dragon | Dragon as Lifeboat: NASA Sheltered Five ISS Crew Members for an Hour W
   - the-795-digital-omnibus-access-rights-noyb-reality-check | EC Proposes 'Abuse' Ground to Restrict Access Rights. noyb's Caseload 
   - the-797-ursa-ag-no-tech-repairable-tractor | Canadian Startup Is Selling No-Tech Tractors at Half the Price of a Jo
write-gate: ENFORCED (POST /api/articles -> 401); real SEED_API_TOKEN required
ERROR: SEED_API_TOKEN is not set — cannot authenticate writes (THE-175).
```
