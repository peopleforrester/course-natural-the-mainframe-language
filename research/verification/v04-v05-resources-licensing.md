<!-- ABOUTME: Adversarial verification of spikes 04 and 05: learning resources, open-source prior art, and Natural CE licensing. -->
<!-- ABOUTME: Records verdicts against primary sources, required corrections, and a considered read on whether a paid course may host a Natural runtime. -->

# Verification: Spike 04 (resources) and Spike 05 (emulator / WASM feasibility)

Verification date: 2026-07-31 to 2026-08-01. All URLs re-checked on those dates.
Method: anonymous HTTP fetches (`curl`, no credentials), GitHub REST API, Docker Hub
registry API, VS Code Marketplace gallery API, and PDF metadata extraction. Where a page
is a JavaScript single-page app that renders nothing to an anonymous fetch, the verdict is
UNVERIFIED rather than confirmed.

Scope note: output formatting semantics (WRITE / DISPLAY / edit masks) are verified
separately in the spike 07 pass and are deliberately out of scope here.

Headline: the licensing conclusion in spike 05 survives, but the reasoning behind it does
not. The binding EULA does not say "personal use only". It says the opposite. The
prohibition on hosting a paid course is real and comes from four different clauses that
neither spike quotes. Separately, every Natural version number in spike 04's table is
wrong, because the spike read a documentation index page that was frozen in October 2021.

---

## Verdict table

### Licensing (the load-bearing claim)

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| "This Community Edition is for personal use only. Use for commercial production purposes is prohibited." | 05 sec. 1; 04 sec. 2 | CONFIRMED (verbatim) | Exact string present on the vendor page today, in that order, as one sentence pair. It is marketing body copy on the product page, immediately above "Register here to gain access". It is not a clause in any signed or click-through agreement. | https://www.softwareag.com/en/developer/adabas-natural-community-edition/ | 2026-07-31 |
| "the Docker Hub image gates the pull behind a 'Limited Use License Agreement' that grants a non-exclusive, non-transferable license, prohibits sublicensing and redistribution, and prohibits reverse engineering" | 05 sec. 1 | PARTIALLY CORRECT, and materially incomplete | All four listed elements are present, but the spike omits the single most important sentence and gets the use scope backwards. The agreement is "LIMITED USE LICENSE AGREEMENT FOR SOFTWARE GMBH DOCKER IMAGES, v2025.1", accepted by setting `ACCEPT_EULA` at container start. Its grant reads: "Software GmbH grants you - free of charge - a non-exclusive, non-transferable license to use and copy the Product and accompanying documentation on the number of computers, workstations or on terminals within a network as specified in the respective Product documentation ... **for your internal production use** and for a time period defined below". The binding agreement therefore permits internal production use and never uses the words "personal", "personal use", "educational", "academic", "evaluation", or "non-production" anywhere in the document (grep-verified across the full text). | https://documentation.softwareag.com/legal/docker/Limited_Use_License_for_Docker.txt | 2026-07-31 |
| Implied: the CE Guide PDF documents the personal-use restriction | 04 sec. 2 (CE row), 05 sources | REFUTED | The 25-page official CE Guide v1.3 contains no "personal use", "commercial", or "prohibited" language at all. Its only licensing content is a boilerplate pointer to softwareag.com/licenses plus the `ACCEPT_EULA=Y` flag. The "personal use only" statement exists on exactly one surface: the marketing web page. | https://softwareag-usa.s3.amazonaws.com/Adanat_Docker/AN+Community+Edition+Guide.pdf | 2026-07-31 |
| "A company running the CE runtime as the backend of a paid course, one instance per paying student, is a commercial production use and a redistribution-adjacent hosting scenario that the free license does not authorize." | 05 sec. 1 | CONCLUSION CONFIRMED, REASONING REFUTED | The conclusion holds, but not because of "commercial production use" (the EULA grants production use) and not because of redistribution alone. See the Licensing assessment section below for the four clauses that actually do the work, including a confidentiality clause neither spike mentions that also constrains screenshots and screen recordings of the CE in course material. | same as above | 2026-07-31 |
| Implied by both spikes: no educational, academic, or partner path exists that would permit hosted course use | 04, 05 (absent) | CONFIRMED by absence, with caveats | No current public academic or education license path was found. Software AG's University Relations / Academic Alliance program is defunct as a public offering: `www2.softwareag.com/corporate/company/ur/default.aspx`, `softwareag.com/en_corporate/resources/university-relations.html`, and `softwareag.com/corporate/products/downloads/free_for_faculties/licenses` all now serve the "Page not found" template. An older `License_Agreement_EducationResearch` page (`www1.softwareag.com/corporate/community/uni/...asp`) returns HTTP 404 and has no Wayback snapshot. The current Education Services page names Adabas and Natural but offers only courses, badges, and a free consulting workshop, with no license grant. The only residue is the archived education package repo and its `UniversityTech@softwareag.com` contact. | https://www.softwareag.com/en/education-services/ ; https://github.com/SoftwareAG/adabas-natural-education-package | 2026-08-01 |

### Docker images and product currency

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| "The `softwareag/natural-ce` image is real and recently maintained: the latest tag observed was 9.3.3, about 114 MB, last pushed roughly 19 days before this spike (early July 2026)" | 05 sec. 1 | PARTIALLY CORRECT (now stale) | Image and tag confirmed. `9.3.3` is the newest of five tags (9.1.4, 9.2.1, 9.3.1, 9.3.2, 9.3.3). `full_size` is 120,086,076 bytes = 114.5 MiB, so "about 114 MB" is right in MiB. But `tag_last_pushed` is now **2026-07-21T11:26:06Z**, which is two days *after* the spike date. The tag was re-pushed after the spike was written, so "early July 2026" no longer describes the current state. | https://hub.docker.com/v2/repositories/softwareag/natural-ce/tags | 2026-07-31 |
| "Companion images `softwareag/adabas-ce` and `softwareag/adabasmanager-ce` exist" | 05 sec. 1 | CONFIRMED | `adabas-ce` newest tag 7.4.0, 356,768,791 bytes, pushed 2026-02-02. Matches "Adabas for Linux (ADA) 7.4" in the vendor's own October 2025 release post. | https://hub.docker.com/v2/repositories/softwareag/adabas-ce/tags | 2026-07-31 |
| "there was an October 2025 release, and the CE Docker images are current" | 05 sec. 1 | CONFIRMED | The Oct 2025 release post is live, authored by Eli Cohen, dated 2025-10-15. No October 2026 post yet (the cycle is annual, so one is presumably due). | https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 | 2026-08-01 |
| "Adabas & Natural was spun out as a standalone business under the holding entity 'Software GmbH,' owned by the investment firm Silver Lake" as of January 7, 2025 | 05 sec. 1; 04 exec summary | CONFIRMED | Corroborated by the PR Newswire release and multiple wire pickups dated 2025-01-07. | https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html | 2026-08-01 |
| "the free CE ... Access requires registration" | 05 sec. 1 | CONFIRMED, with friction caveat | The page still carries "Register here to gain access". However, a November 2025 Tech Community thread ("Natural and Adabas CE not available anymore?!?!?!?") records a user finding dead and circular download links, resolved only by an unofficial pointer to Docker Hub. No Software AG staff reply. Treat the registration path as unreliable and Docker Hub as the real distribution channel. | https://techcommunity.softwareag.com/t/natural-and-adabas-ce-not-available-anymore/311712 | 2026-08-01 |

### Documentation

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| Docs are "freely accessible with no login for the core product documentation" | 04 sec. 1 | CONFIRMED | Anonymous `curl` with no cookies or credentials returns HTTP 200 and full content for the 9.2.4 mainframe release notes, the UNIX first-steps tutorial, and the 9.3.3 Windows and UNIX install PDFs. The portal's own caveat that partner product docs "may be subject to restricted distribution and require authentication (Empower login)" is present and accurate. | https://documentation.softwareag.com/natmf/9.2.4/en/webhelp/natmf-webhelp/rnotes_mf/rn-mf-over.htm | 2026-07-31 |
| "Natural for Mainframes \| 9.2.3 (with 9.2.4 release notes PDF also present)" | 04 sec. 1 table | REFUTED | The current mainframe line is **9.2.4**. The full 9.2.4 webhelp tree is live (release notes render as "Natural Version 9.2.4 Release Notes for z/OS"), not merely a PDF. The vendor's own October 2025 release post lists "Natural for z/OS: 9.2.4". 9.2.5 returns 404. | https://documentation.softwareag.com/natmf/9.2.4/en/webhelp/natmf-webhelp/rnotes_mf/rn-mf-over.htm | 2026-07-31 |
| "Natural for Windows \| 9.1.3 (Oct 2021)" | 04 sec. 1 table | REFUTED | Current is **9.3.3**. `natwin/9.3.3/.../pdf/install.pdf` returns 200 with `Last-Modified: Wed, 08 Apr 2026`. The spike's figure is four minor versions and four and a half years behind. | https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pdf/install.pdf | 2026-07-31 |
| "Natural for UNIX / Open Systems \| 9.1.2" | 04 sec. 1 table | REFUTED | Current is **9.3.3** under the `natux` product code. `natux/9.3.3/.../pdf/install.pdf` returns 200, `Last-Modified: Wed, 08 Apr 2026`. The vendor now brands this line "Natural for Linux and Cloud", version 9.3.3, per the October 2025 release post. | https://documentation.softwareag.com/natux/9.3.3/en/webhelp/natux-webhelp/pdf/install.pdf | 2026-07-31 |
| "The index page `documentation.softwareag.com/natural/index.htm` organizes the Natural family" | 04 sec. 1 | CONFIRMED that it loads, REFUTED as a current source | This is the root cause of the three wrong versions above. The page loads, but its own footer reads "Page last updated: October 15, 2021" and "Software AG © 2021". Its child index `a_natural_mf/natural_mf_vers.htm` tops out at "Natural 9.1.2 for Mainframes 07/2021"; `a_natural_os/natural_os_vers.htm` tops out at "Natural 9.1.4 for UNIX 10/2021". It is a frozen legacy portal and must not be used for version currency. | https://documentation.softwareag.com/natural/a_natural_mf/natural_mf_vers.htm | 2026-07-31 |
| "NaturalONE 9.2.1 what's new" cited as the NaturalONE reference | 04 sources | STALE | NaturalONE is at **9.3.3** per the October 2025 release post. | https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 | 2026-08-01 |
| "Natural for Windows 9.3.2 install guide dated July 2025" | 05 sec. 1 | SUPERSEDED | 9.3.2 exists, but 9.3.3 is current and both PDFs were last modified 2026-04-08. | https://documentation.softwareag.com/natwin/9.3.2/en/webhelp/natwin-webhelp/pdf/install.pdf | 2026-07-31 |
| "The UNIX install needs roughly 600 MB for Natural plus about 200 MB for Natural Security" | 05 sec. 1 | UNVERIFIED for the current version | The cited source is `nat6314unx`, the Natural **6.3.14** UNIX documentation. That is a product generation three major lines behind current. The figure was not re-verified against 9.3.3. Do not carry it forward as a current sizing number. | https://documentation.softwareag.com/natural/nat6314unx/install/inst-prod.htm | 2026-07-31 |
| "Getting Started with Natural" first-steps tutorial is free and login-free | 04 sec. 1 | CONFIRMED | Anonymous fetch returns 200. | https://documentation.softwareag.com/natural/nat912unx/firststeps/fs-start.htm | 2026-07-31 |

### Open-source prior art

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| `natls` is "MIT licensed", "in Java", a "parser/linter/LSP, **not an interpreter**" | 05 sec. 3; 04 sec. 6 | CONFIRMED on all three | GitHub API reports `spdx_id: MIT`, `language: Java`. The README describes only static analysis, completion, diagnostics, and refactoring. Nothing in it executes Natural. | https://api.github.com/repos/MarkusAmshove/natls | 2026-07-31 |
| "Latest release v0.18 on 2026-01-12, about 2,043 commits, clearly active" | 05 sec. 3 | CONFIRMED | `v0.18`, `published_at: 2026-01-12T19:52:34Z`, not a prerelease, and still the newest of eight releases. Contributor totals sum to **2,046** commits (markusamshove 1,813; Claes65 133; Claes Norreen 86; awilkins 8; acnsalb 6), so "about 2,043" is accurate to within normal drift. Last push **2026-07-30**, one day before this check, so "actively maintained" is solidly true. | https://api.github.com/repos/MarkusAmshove/natls/releases | 2026-07-31 |
| "Its own docs note the parser is still incomplete for context-sensitive constructs and does not yet handle Reporting Mode" | 05 sec. 3 | CONFIRMED (verbatim) | README: "The parser is still incomplete and has some rough edges where the Natural language is context sensitive." and "Reporting Mode hasn't been considered yet, so currently only the structured mode syntax of statements is parsed correctly." | https://raw.githubusercontent.com/MarkusAmshove/natls/main/README.md | 2026-07-31 |
| `adabas-natural-code-samples` is "Apache-2.0", "~27 stars, ~21 forks, ~138 commits, not archived" | 04 sec. 6 | CONFIRMED on every number | `spdx_id: Apache-2.0`, `archived: false`, 27 stars, 21 forks, contributor totals sum to exactly **138** commits. | https://api.github.com/repos/SoftwareAG/adabas-natural-code-samples | 2026-07-31 |
| `adabas-natural-code-samples` state: "Active" | 04 sec. 6 | REFUTED | `pushed_at` is **2024-03-27**. The repo has had no commits in over 28 months. The `updated_at` field of 2026-05-15 reflects stars and watch events, not code. It is a usable frozen corpus, not an active project. Calibrating exercises against it (a CLAUDE.md contract) is still fine; describing it as active is not. | https://api.github.com/repos/SoftwareAG/adabas-natural-code-samples | 2026-07-31 |
| `adabas-natural-code-samples` has "60+ categorized folders" / "60+ pattern folders" | 04 sec. 6; 05 sec. 3 | REFUTED | The repository root contains **51 directories** and 3 files (CONTRIBUTING.md, LICENSE, README.md). Sixty-plus overstates it by about 20 percent. Corrected count: 51. | https://api.github.com/repos/SoftwareAG/adabas-natural-code-samples/contents/ | 2026-07-31 |
| `adabas-natural-education-package` is "Apache-2.0", "Archived Jan 2023, read-only, ~13 stars", uses a "CRUISE" file, contact `UniversityTech@softwareag.com` | 04 sec. 6 | CONFIRMED on all points | `spdx_id: Apache-2.0`, `archived: true`, 13 stars, 9 forks, `pushed_at: 2023-01-25`, created 2017-03-29. README confirms four tutorials (Hello World / WRITE, DECIDE ON, arrays, Adabas retrieval), the CRUISE file, and the UniversityTech contact. It also depends on a "Virtual Machine provided by the University Relations department" that no longer has a live download page. | https://api.github.com/repos/SoftwareAG/adabas-natural-education-package | 2026-07-31 |
| `adabas-natural-devops-sample-application`: "Software AG repo \| Present" | 04 sec. 6 | INCOMPLETE | It is **Apache-2.0** (the spike does not say, which matters for reuse) and `pushed_at` is **2023-01-23**. Dormant for over three years, 4 stars. | https://api.github.com/repos/SoftwareAG/adabas-natural-devops-sample-application | 2026-07-31 |
| `vscode-natural`: "v0.18.0 (Nov 2024 on Marketplace), ~835 installs, 5 stars (1 review)" | 04 sec. 6 | PARTIALLY CORRECT | Version 0.18.0 CONFIRMED. Install count 843 and rating 5.0 from 1 rating CONFIRMED (download count 43,943). But the Marketplace `lastUpdated` for 0.18.0 is **2026-01-12T20:48Z**, not November 2024, matching the natls v0.18 release the same day. The date is wrong by 14 months. Also note the GitHub repo is `MarkusAmshove/vscode-natural` (MIT, 3 stars, pushed 2026-01-12); the "5 stars" in the spike is the Marketplace star *rating*, not stargazers, which reads ambiguously. | VS Code Marketplace gallery API, extension `markusamshove.vscode-natural` | 2026-07-31 |
| "No open-source Natural *interpreter* or *emulator* exists" | 05 sec. 3 and closing note | See "Open-source hunt" below | | | |
| "There is also no published ANTLR grammar for Software AG Natural in `antlr/grammars-v4`" | 05 sec. 3 and closing note | See "Open-source hunt" below | | | |

### Courses, community, and other resources

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| "NATURAL Essentials (self-study course) \| Stephen Paul Simpson \| 1999 to 2000" | 04 sec. 3 | CONFIRMED, and the license is stricter than recorded | PDF internals: `/Author (Stephen Paul Simpson)`, `/Title (NATURAL Essentials (tm))`, `/CreationDate (Thursday, April 13, 2000)`. Front matter: "Version 2.10 ... © 1999-2000". Update history: "Version 1.00 published January 9, 1999. Version 2.00 published August 24, 1999. Version 2.10 published January 22, 2000." 291 pages. The PDF downloads without a gate. | http://spsimpson.com/nat-u/NATURAL%20Essentials.pdf | 2026-07-31 |
| "Free-to-read PDF ... Distribution requires a license per the author" | 04 sec. 3 | PARTIALLY CORRECT, and dangerously understated | The author's terms: "Registered Users are welcome to download and print this material for **personal, non-profit use**." And: "Copying or installing on a network without a license, altering the text, distributing/publishing individual parts of the course, distributing/publishing outside your organization and **commercial/for-profit use** all constitute infringements of copyright and will be prosecuted to the fullest extent of the law." For a paid course, this is a hard boundary: read it for orientation, do not lift its structure, sequencing, examples, or text. | same as above, page 3 | 2026-07-31 |
| "No current, independently authored, ISBN-bearing Natural textbook surfaced." | 04 sec. 3 | REFUTED | One exists, in German, and it is recent. Michael Schluter, *Einfuhrung in die Programmierung mit Natural & Adabas*, 3rd corrected and expanded edition, Lehmanns Media Berlin, published 2019-06-17, 352 pages, ISBN **978-3-86541-994-1**, in the series "Programmierung komplexer Systeme". An e-book edition exists (ISBN 978-3-96543-059-4), and a predecessor volume *Einfuhrung in die Programmierung mit Natural* (2nd revised edition, ISBN 978-3-86541-526-4). Listed for sale at Lehmanns, Thalia, Amazon, and eBay. The spike missed it because the searches were English-only. This matters: Software AG is a Darmstadt company with a heavily German-speaking install base, so "no independent book exists" is only true of the English-language market. | https://www.lehmanns.de/shop/mathematik-informatik/48218507-9783865419941-einfuehrung-in-die-programmierung-mit-natural-adabas | 2026-08-01 |
| Third-party instructor-led providers are "Verhoef (UK), Nisa Trainings and MaxMunus (India)" | 04 sec. 4 | INCOMPLETE | At least one German provider is missing: SCN GmbH (Berlin) lists Software AG among its vendor catalogue and search results surface "Natural Programmierung" and "Adabas Grundlagen" seminars with a Durchfuhrungsgarantie across roughly 19 German cities. Caveat: both specific seminar URLs return HTTP 404 today and the site root returns 500, so the offering itself is **UNVERIFIED**, only the provider's Software AG catalogue entry is confirmed. Same English-only blind spot as the book. | https://www.scngmbh.de/ | 2026-08-01 |
| "Software AG Certified Natural Associate" Credly badge exists | 04 (implied), task item 9 | CONFIRMED | Live badge page. Issuer Software AG. Type: learning badge, Foundational level. Skills: ADABAS, Natural. Earning criteria, verbatim: "(For External Users) - Natural Programming Basic Course" and "(For Internal Users) - Natural Programming Basic Course". So the badge is awarded for course completion, with no separate proctored exam. | https://www.credly.com/org/software-ag/badge/software-ag-certified-natural-associate | 2026-07-31 |
| "Natural Programming Basic (course id 1467)" is free | 04 sec. 4 | CONFIRMED indirectly; the page itself is UNVERIFIABLE anonymously | `learn.softwareag.com/course/info.php?id=1467` returns HTTP 200 but the body is a JavaScript SPA shell that renders only the word "Loading" to an anonymous fetch, so no content, price, or outline can be read without an account. The spike's "login-gated" characterisation is right in effect. Free status is corroborated by an official Tech Community post: the Digital Essentials self-paced trainings, which include Natural Programming and NaturalONE basics, are "free and self-paced", require creating a free Software AG Learning account, and award a "Software AG Certified digital badge". | https://techcommunity.softwareag.com/t/introducing-free-training-essentials-for-adabas-natural/259320 | 2026-08-01 |
| `learn.softwareag.com` course IDs 1423, 1426, 1467 and categoryid 44 | 04 sec. 4 | CONFIRMED as live URLs, UNVERIFIED as content | All four return HTTP 200 but render the same anonymous SPA shell. A `knowledge.softwareag.com` variant surfaced in search results does not resolve at all (DNS failure), so ignore it. | https://learn.softwareag.com/course/view.php?id=1423 | 2026-07-31 |
| Verhoef, Nisa Trainings, and MaxMunus third-party courses | 04 sec. 4 | CONFIRMED as live | All three URLs return HTTP 200 today. Pricing remains quote-on-request; not independently re-priced. | https://verhoef-training.co.uk/system-z-programming/adabas-natural-programming | 2026-07-31 |
| edX shows "A topic landing page for 'Adabas' only, no actual course" | 04 sec. 4 | CONFIRMED | The page renders 17 certificate programs and 48 courses, all generic SQL / database / data engineering titles. Nothing about Adabas or Software AG Natural. | https://www.edx.org/learn/adabas | 2026-07-31 |
| "`techcommunity.softwareag.com/c/adabas-natural/` ... this is the current canonical home" | 04 sec. 5 | REFUTED | That URL returns **HTTP 404**. The Discourse instance has been restructured to five top-level categories only: `forum` (7,304 topics), `feedback` (776), `knowledge-base` (203), `news` (56), `user-groups` (8). There is no Adabas-Natural category. Adabas and Natural content is now reachable via the tag route `techcommunity.softwareag.com/tag/adabas-natural` (HTTP 200). | https://techcommunity.softwareag.com/categories.json | 2026-08-01 |
| "The older `tech.forums.softwareag.com` now 301-redirects here" | 04 sec. 5 | CONFIRMED | `tech.forums.softwareag.com` resolves to `techcommunity.softwareag.com`. | (redirect trace) | 2026-08-01 |
| Community is "Active through 2024 to 2025" | 04 sec. 5 | CONFIRMED and understated | Newest Adabas / Natural topics include 2026-07-03 ("IO execution not allowed at session initialization") and 2026-06-29 (July 2026 Asia-Pacific Adabas & Natural User Group meeting). The community is active through July 2026. | https://techcommunity.softwareag.com/search.json?q=adabas%20natural%20order%3Alatest | 2026-08-01 |
| `education.softwareag.com/adabas-and-natural/tutorials` is "Reachable but had connection instability" | 04 sec. 5 | REFUTED today | The host does not respond at all. `curl` times out after 30 seconds with no bytes returned. Treat this URL as dead, not flaky. | https://education.softwareag.com/adabas-and-natural/tutorials | 2026-07-31 |
| Mohamad Mahmood Medium series is "Free to read (Medium)", "~12 parts", "the best free modern community walkthrough" | 04 sec. 6 | REFUTED on "free", questionable on "modern" | Part 1 is flagged "Member-only story", so it is behind Medium's paywall, and an anonymous `curl` gets HTTP 403. It was published **2022-10-28** and its setup instructions pin Adabas CE 7.0.1 and Natural CE 9.1.4, both several generations behind the current 7.4.0 and 9.3.3. Series length was not confirmable from the paywalled page. | https://medium.com/@mohamad.razzi.my/getting-started-with-adabas-natural-part-1-6597688406ad | 2026-07-31 |
| Official YouTube tutorials are "fragmentary: short vendor demos" | 04 sec. 4 | CONFIRMED, with a correction to the count | The vendor's own tutorial index (posted 2025-06-01 by Felix Friedrich) links four curated playlists on the SOFTWARE AG channel: Adabas Tutorials (5 videos), Natural Tutorials (8), NaturalONE Tutorials (10), NaturalONE & DevOps Tutorials (9). That is 32 videos across four organized playlists, more structured than "scattered free material" implies, but still not a curriculum and not interactive. | https://techcommunity.softwareag.com/t/adabas-natural-tutorials/311404 | 2026-08-01 |
| `developer.softwareag.com` had "a TLS quirk on fetch during this spike" | 04 sec. 2 | CONFIRMED and still broken | Still fails TLS verification: "SSL certificate problem: unable to get local issuer certificate". This is an incomplete certificate chain on the vendor's side, not a client issue. Cite it only with that caveat. | https://developer.softwareag.com/en/Natural.html | 2026-07-31 |

### WASM and terminal prior art (spike 05 section 4 and its GitHub table)

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| `segeljakt/xterm-js-rs` "gives Rust/wasm bindings to xterm.js directly, which is close to the exact stack recommended here"; table lists it as "MIT/Apache-2.0 (dual, typical Rust)", "Community-maintained crate" | 05 sec. 4 and GitHub table | REFUTED on license, and the recommendation is unsafe | Two errors and one architectural consequence. (1) License is **MIT only**, not the dual MIT/Apache-2.0 the spike assumed. (2) It is not maintained: the crate's newest release is **0.1.2, published 2021-11-15**, the only three versions are 0.1.0, 0.1.1, 0.1.2, and the repo's last push was **2023-01-07**. (3) The consequence: a binding crate frozen in 2021 targets the xterm.js 4.x API. The project's own CLAUDE.md pins `@xterm/xterm` 6.0.0, two major versions ahead, on a package that was renamed and rescoped in the interim. Do not plan to depend on this crate. Write the terminal glue directly with `wasm-bindgen` against `@xterm/xterm` 6.x. Keep the repo as a read-only design reference at most. | https://crates.io/api/v1/crates/xterm-js-rs ; https://api.github.com/repos/segeljakt/xterm-js-rs | 2026-08-01 |
| `cryptool-org/wasm-webterm` listed as "MIT", "Maintained; xterm.js v4-era addon" | 05 GitHub table | REFUTED on license, CONFIRMED on maintenance | License is **Apache-2.0**, not MIT. Maintenance is better than the spike implies: last push **2026-06-12**, 93 stars, 7 open issues. It remains valid as proof that "wasm binary plus xterm.js in the browser" is a supported pattern, which is all spike 05 uses it for. | https://api.github.com/repos/cryptool-org/wasm-webterm | 2026-08-01 |
| `tsl0922/ttyd` is MIT and "Actively maintained" | 05 GitHub table | CONFIRMED | MIT, 12,128 stars, last push 2026-06-30. | https://api.github.com/repos/tsl0922/ttyd | 2026-08-01 |
| "EMPLOYEES and VEHICLES demo files ... shipped with the product and the Community Edition demo database" | 04 sec. 2 and sec. 6 | CONFIRMED | Adabas ships Employees, Vehicles, Personnel, and Miscellaneous demo files. The CE creates the demo database via `ADABAS_DB_CREATION=demodb`, and the CE Guide notes "the Natural Demo Application is also delivered with the Natural Community Edition". Caveat worth carrying into course design: the CE Guide states "Data changes to this demo database are not persisted" unless a directory is mounted, which is consistent with the per-lesson state reset the course already requires. | https://documentation.softwareag.com/adabas/ada854mfr/util/apxc.htm ; CE Guide v1.3 | 2026-08-01 |

### Open-source hunt (delegated adversarial search)

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| "No open-source Natural *interpreter* or *emulator* exists." | 05 sec. 3 | PENDING_INTERPRETER_VERDICT | PENDING_INTERPRETER_DETAIL | | 2026-08-01 |
| "no Software AG Natural grammar exists in `antlr/grammars-v4`" | 05 sec. 3 | PENDING_ANTLR_VERDICT | PENDING_ANTLR_DETAIL | | 2026-08-01 |

### Marketplace gap (delegated adversarial search)

| Claim (quoted) | Source file | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|---|
| "No modern, interactive, beginner-friendly Natural course exists." / "Searches of Udemy, Pluralsight, Coursera, LinkedIn Learning, and edX returned no dedicated, current, interactive Natural course." | 04 exec summary and sec. 4 | PENDING_MARKET_VERDICT | PENDING_MARKET_DETAIL | | 2026-08-01 |

---

## Corrections required

Apply these to the source spikes.

**Spike 04, section 1, current-versions table.** All three rows are wrong. Replace with:
Natural for Mainframes (z/OS) **9.2.4**; Natural for Linux and Cloud (formerly UNIX / Open
Systems) **9.3.3**; Natural for Windows **9.3.3**; NaturalONE **9.3.3**. Source the numbers
from the vendor's October 2025 release post and the live `natmf` / `natux` / `natwin`
webhelp trees, not from `documentation.softwareag.com/natural/index.htm`.

**Spike 04, section 1.** Add a warning that `documentation.softwareag.com/natural/index.htm`
and its `a_natural_*_vers.htm` children are a frozen legacy portal last updated
2021-10-15, and must never be used to establish version currency.

**Spike 04, section 3.** The NATURAL Essentials entry must state that the author prohibits
commercial and for-profit use outright, not merely that "distribution requires a license".
It is a read-only reference for a paid course.

**Spike 04, section 3.** Delete "No current, independently authored, ISBN-bearing Natural
textbook surfaced" and add a row for Michael Schluter, *Einfuhrung in die Programmierung mit
Natural & Adabas*, Lehmanns Media, 2019, 352 pages, ISBN 978-3-86541-994-1.

**Spike 04, whole document: the English-only search bias.** Both the missing textbook and a
missing German training provider (SCN GmbH) were found by a single German-language search. The
gap analysis was built entirely on English queries. Before the "no competition" claim is used
in any commercial positioning, run the equivalent searches in German. Software AG is a
Darmstadt company and its largest install base is German-speaking, which is exactly where a
competitor would be if one exists. This does not overturn the thesis that no interactive
browser-based beginner course exists, but it does mean the competitive picture in the spike
is incomplete rather than merely thin.

**Spike 04, section 5.** `techcommunity.softwareag.com/c/adabas-natural/` is a 404. Replace
with `techcommunity.softwareag.com/tag/adabas-natural`. Update the activity window from
"2024 to 2025" to "through July 2026". Change the `education.softwareag.com` row from
"connection instability" to "does not respond; treat as dead".

**Spike 04, section 6.** Change `adabas-natural-code-samples` state from "Active" to "Frozen;
last commit 2024-03-27". Change "60+ categorized folders" to "51 top-level pattern folders"
in both spike 04 and spike 05. Add Apache-2.0 to the `adabas-natural-devops-sample-application`
row and mark it dormant since 2023-01-23. Correct the `vscode-natural` Marketplace date from
"Nov 2024" to "2026-01-12", and disambiguate "5 stars" as a 5.0 rating from a single review.
Change the Medium series access column from "Free to read" to "Medium member-only (paywalled)",
add the 2022-10-28 date, and note it pins superseded CE versions.

**Spike 04, section 4 and sources.** Note explicitly that `learn.softwareag.com` course pages
return HTTP 200 but render nothing anonymously, so their content is unverified rather than
merely "gated". Add the Tech Community "Training Essentials" post as the corroborating source
that the self-paced Natural Programming Basic course is free and that completion awards the
Credly "Software AG Certified Natural Associate" foundational badge.

**Spike 05, section 1.** This is the substantive rewrite. The paragraph beginning "License
terms (the critical constraint)" must be replaced. It currently implies the CE is
personal-use-only in its binding terms. It is not. State that (a) the "personal use only"
sentence is marketing copy on the product page, (b) the operative click-through agreement is
the Limited Use License Agreement for Software GmbH Docker Images v2025.1, accepted via
`ACCEPT_EULA`, (c) that agreement grants use "for your internal production use", and (d) the
hosted-course prohibition therefore rests on the integrated-solution, no-rent/lease,
no-distribution, and confidentiality clauses quoted in the assessment below. The conclusion
does not change; the argument must, because the current argument collapses on first contact
with the actual EULA.

**Spike 05, section 1.** Add the confidentiality clause as a distinct production constraint.
Neither spike mentions it and it affects course *content*, not just hosting.

**Spike 05, section 1.** Update the `natural-ce` push date to 2026-07-21 and state the size as
114.5 MiB (120 MB decimal) so the units are unambiguous.

**Spike 05, section 1.** Drop or re-source the "600 MB Natural plus 200 MB Natural Security"
figure. It comes from Natural 6.3.14 documentation, three major generations stale.

**Spike 05, section 3.** Correct the code-samples folder count to 51. The commit count of ~2,043
for natls and ~138 for code-samples are both accurate; leave them.

**Spike 05, section 4 and GitHub table.** Fix two licenses and one recommendation.
`cryptool-org/wasm-webterm` is Apache-2.0, not MIT. `segeljakt/xterm-js-rs` is MIT only, not
dual MIT/Apache-2.0. More importantly, remove the framing of `xterm-js-rs` as "close to the
exact stack recommended here". Its last crate release is 0.1.2 from November 2021 and its last
commit is January 2023, so it targets the xterm.js 4.x API while the project targets
`@xterm/xterm` 6.0.0. Replace with an explicit instruction to hand-write the `wasm-bindgen`
glue against `@xterm/xterm` 6.x. This is a live trap: a reader following spike 05 as written
would add a dead dependency early and discover the incompatibility only after wiring it in.

---

## Licensing assessment

**The question.** May a paid, commercially sold course legally host a Natural runtime for its
students?

**Short answer.** Not on the free Community Edition. The conclusion spike 05 reached is
correct and the Rust interpreter decision should stand. But the spike reached it by the wrong
route, and the right route is both stronger and broader in what it forbids.

**What the spikes got wrong.** Both files treat "This Community Edition is for personal use
only. Use for commercial production purposes is prohibited." as if it were the license. It is
not. It is a sentence of marketing body copy on a product web page, sitting between a product
description and a registration form. The document a user actually assents to is the *Limited
Use License Agreement for Software GmbH Docker Images, v2025.1*, accepted by setting
`ACCEPT_EULA=Y` when the container starts. That agreement never uses the word "personal" in a
licensing sense anywhere in its full text, and its grant clause says the opposite of what the
spike assumed:

> "Software GmbH grants you - free of charge - a non-exclusive, non-transferable license to
> use and copy the Product and accompanying documentation on the number of computers,
> workstations or on terminals within a network as specified in the respective Product
> documentation (please refer to the respective section in the Release Notes relating to use
> restrictions) for your internal production use and for a time period defined below."

"For your internal production use" is a production grant. If a counterparty relied on the
spike's framing and argued "commercial production use is prohibited", the EULA would refute
them in one line. The argument has to rest elsewhere.

**Where the prohibition actually lives.** Four clauses, any one of which is sufficient.

1. *The integrated-solution clause.* "In no event may the Product be used to develop an
   integrated solution that requires for the Product to be integrated into your or any third
   party intellectual property in order to create a combined product that is provided to
   third parties." A course platform that wires the Natural runtime into a lesson UI and
   serves the combination to paying students is precisely a combined product provided to
   third parties. This is the cleanest and most direct bar.
2. *The no-rent, no-lease, no-sublicense clause.* "This Agreement does not grant you the right
   to sublicense, transfer, rent, assign or lease the Product, in whole or in part." Selling
   time-boxed access to a hosted instance is renting the Product. There is no reading of
   per-student hosted access that avoids this.
3. *The distribution clause.* "You may not pass on or distribute copies of the Product to any
   third party." This bites hardest on a downloadable or self-hosted variant of the course,
   and reinforces (1) for the hosted case.
4. *The capacity clause.* The grant is limited to "the number of computers, workstations or on
   terminals within a network as specified in the respective Product documentation". A
   one-container-per-student fleet is unbounded by design and would need that number checked
   against the release notes, which is a losing position to argue from.

**The clause neither spike noticed, and it constrains course content, not just hosting.**

> "The Product is confidential and proprietary information of Software GmbH and its licensors,
> and may not be disclosed to third parties. You shall use such information only for the
> purpose of exercising the Limited Use License Agreement to the Product and shall disclose
> confidential and proprietary information only to your employees who require such information
> for the purpose stated above."

Read literally, this constrains publishing screenshots, screen recordings, or transcripts of
the CE session output in commercial course material, because those disclose the Product's
behavior to third parties. Whether a court would enforce it that broadly against ordinary
screenshots of a freely-downloadable product is genuinely uncertain, and I would not bet a
product on the narrow reading. The practical consequence is a second, independent argument for
the custom-interpreter architecture: our own interpreter's output belongs to us, so lesson
screenshots, expected-output fixtures, and recorded demos carry no vendor confidentiality
question at all. This is a benefit of the chosen architecture that the spike never claimed and
should.

Two further clauses worth knowing about. The reverse-engineering bar ("You may not decompile,
disassemble, modify, decrypt, extract or otherwise reverse engineer") independently kills any
notion of compiling the vendor runtime to WASM, which spike 05 already concluded on other
grounds. And the benchmarking bar forbids publishing performance comparisons without written
consent, which matters if the course ever wants to compare the teaching interpreter to real
Natural.

**Is there an educational, academic, or partner path that would permit it?** I looked hard and
found none that is live.

- Software AG ran a University Relations / Academic Alliance program offering free software to
  students and faculty. Every public artifact of it is now gone: the program landing pages, the
  faculty license request form, and an `License_Agreement_EducationResearch` page all return
  the vendor's "Page not found" template or a hard 404. The education-research license page has
  no Wayback snapshot, so I cannot even establish what it granted.
- The archived `adabas-natural-education-package` repo (last touched January 2023) is the
  program's residue. It depends on a "Virtual Machine provided by the University Relations
  department" whose download page no longer exists, and lists `UniversityTech@softwareag.com`
  as its contact.
- The current Education Services page names Adabas and Natural throughout but offers only
  courses, badges, and a free consulting workshop. It grants no software license.

So the only remaining legitimate route to hosting a real Natural runtime is a negotiated
commercial license from Software GmbH, which is spike 05's option A'. That is unchanged, but
note the counterparty is now a Silver Lake portfolio company running Adabas & Natural as a
standalone P&L, which is not a vendor with an incentive to write a cheap per-student training
license for a third party building a competing self-serve course. Their own Education Services
line sells the same thing.

**One caveat I will not paper over.** The two documents are in genuine tension. A vendor page
says personal use only; the binding agreement grants internal production use. If the question
ever becomes commercially material rather than architectural, that tension is exactly the kind
of thing a licensing lawyer should resolve in writing with Software GmbH, not something to
settle by reading web pages. Nothing here is legal advice. What I can say is that the
architecture decision does not depend on resolving it, because all four operative clauses bar
the hosted-course use regardless of which document wins on "personal".

**Net effect on the approved contract.** No change. The CLAUDE.md contract item "Do not host
the free Adabas & Natural Community Edition as the course backend" remains correct. Its stated
justification ("Its license is personal-use-only and prohibits commercial production use")
should be reworded, because the second half is contradicted by the EULA. Suggested wording:
"Its Docker EULA forbids integrating the Product into a combined product provided to third
parties, forbids renting or sublicensing it, and treats the Product as confidential."

---

## Sources

All accessed 2026-07-31 or 2026-08-01 as noted per row above.

Licensing and Community Edition:
- https://www.softwareag.com/en/developer/adabas-natural-community-edition/ - "This Community Edition is for personal use only. Use for commercial production purposes is prohibited." Verified verbatim by anonymous fetch and text extraction.
- https://documentation.softwareag.com/legal/docker/Limited_Use_License_for_Docker.txt - Limited Use License Agreement for Software GmbH Docker Images, v2025.1. Full text retrieved. Grant clause, integrated-solution clause, no-rent/sublicense clause, distribution clause, confidentiality clause, reverse-engineering clause, benchmarking clause, New York governing law.
- https://hub.docker.com/r/softwareag/natural-ce - image page showing the EULA gate and the 9.3.3 tag.
- https://hub.docker.com/v2/repositories/softwareag/natural-ce/tags - registry API; 9.3.3, 120,086,076 bytes, tag_last_pushed 2026-07-21T11:26:06Z.
- https://hub.docker.com/v2/repositories/softwareag/adabas-ce/tags - registry API; 7.4.0, pushed 2026-02-02.
- https://softwareag-usa.s3.amazonaws.com/Adanat_Docker/AN+Community+Edition+Guide.pdf - CE Guide v1.3, cover "October 2024 Version 1.3 Oct 16, 2024", PDF CreationDate D:20241016. Contains no personal-use or commercial-use language.
- https://techcommunity.softwareag.com/t/natural-and-adabas-ce-not-available-anymore/311712 - November 2025 thread on broken CE download links.

Academic and education license path (all negative results):
- https://www.softwareag.com/en/education-services/ - current Education Services; no license grant.
- http://www1.softwareag.com/corporate/community/uni/License_Agreement_EducationResearch_FV_blank.asp - HTTP 404, no Wayback snapshot.
- http://www2.softwareag.com/corporate/company/ur/default.aspx - redirects to the vendor "Page not found" template.
- https://www.softwareag.com/corporate/products/downloads/free_for_faculties/licenses - "Page not found".
- https://github.com/SoftwareAG/adabas-natural-education-package - archived 2023, University Relations VM dependency, UniversityTech@softwareag.com contact.

Documentation and versions:
- https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 - authoritative version list, 2025-10-15: Natural for z/OS 9.2.4, Natural for Linux and Cloud 9.3.3, NaturalONE 9.3.3, Adabas for Linux 7.4.
- https://documentation.softwareag.com/natmf/9.2.4/en/webhelp/natmf-webhelp/rnotes_mf/rn-mf-over.htm - "Natural Version 9.2.4 Release Notes for z/OS", HTTP 200 anonymous. 9.2.5 returns 404.
- https://documentation.softwareag.com/natwin/9.3.3/en/webhelp/natwin-webhelp/pdf/install.pdf - HTTP 200, Last-Modified 2026-04-08.
- https://documentation.softwareag.com/natux/9.3.3/en/webhelp/natux-webhelp/pdf/install.pdf - HTTP 200, Last-Modified 2026-04-08.
- https://documentation.softwareag.com/natural/a_natural_mf/natural_mf_vers.htm - legacy index, "Page last updated: October 15, 2021", tops out at 9.1.2.
- https://documentation.softwareag.com/natural/a_natural_os/natural_os_vers.htm - legacy index, tops out at Natural 9.1.4 for UNIX.
- https://documentation.softwareag.com/natural/nat912unx/firststeps/fs-start.htm - first-steps tutorial, HTTP 200 anonymous.

Repositories and tooling:
- https://api.github.com/repos/MarkusAmshove/natls - MIT, Java, pushed 2026-07-30, 18 stars.
- https://api.github.com/repos/MarkusAmshove/natls/releases - v0.18 published 2026-01-12T19:52:34Z, newest of eight.
- https://api.github.com/repos/MarkusAmshove/natls/contributors?anon=1 - 2,046 total commits.
- https://raw.githubusercontent.com/MarkusAmshove/natls/main/README.md - parser incompleteness and Reporting Mode statements, verbatim.
- https://api.github.com/repos/SoftwareAG/adabas-natural-code-samples - Apache-2.0, not archived, 27 stars, 21 forks, pushed 2024-03-27.
- https://api.github.com/repos/SoftwareAG/adabas-natural-code-samples/contributors?anon=1 - 138 total commits.
- https://api.github.com/repos/SoftwareAG/adabas-natural-code-samples/contents/ - 51 directories, 3 files.
- https://api.github.com/repos/SoftwareAG/adabas-natural-education-package - Apache-2.0, archived, pushed 2023-01-25, 13 stars.
- https://api.github.com/repos/SoftwareAG/adabas-natural-devops-sample-application - Apache-2.0, pushed 2023-01-23, 4 stars.
- VS Code Marketplace gallery API, `markusamshove.vscode-natural` - version 0.18.0, lastUpdated 2026-01-12T20:48Z, 843 installs, 43,943 downloads, rating 5.0 from 1 review.

Courses, badges, community:
- https://www.credly.com/org/software-ag/badge/software-ag-certified-natural-associate - live foundational learning badge; criteria are completion of the Natural Programming Basic course.
- https://techcommunity.softwareag.com/t/introducing-free-training-essentials-for-adabas-natural/259320 - official post confirming the self-paced Digital Essentials trainings are free with a free learning account and award a Software AG Certified digital badge.
- https://learn.softwareag.com/course/info.php?id=1467 - HTTP 200, renders an anonymous SPA shell only.
- https://techcommunity.softwareag.com/categories.json - five top-level categories; no adabas-natural category.
- https://techcommunity.softwareag.com/tag/adabas-natural - current live route for Adabas and Natural content.
- https://techcommunity.softwareag.com/t/adabas-natural-tutorials/311404 - official tutorial index, 2025-06-01, four YouTube playlists totaling 32 videos.
- https://www.edx.org/learn/adabas - topic landing page, no Adabas or Natural course.

Books:
- http://spsimpson.com/nat-u/NATURAL%20Essentials.pdf - Stephen Paul Simpson, version 2.10, 291 pages, PDF CreationDate 2000-04-13, copyright 1999-2000, personal non-profit use only, commercial and for-profit use expressly an infringement.
- http://spsimpson.com/nat-u/main.htm - landing page, HTTP 200.

Ownership:
- https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html - 2025-01-07, Silver Lake, Software GmbH holding company, standalone Adabas & Natural.
