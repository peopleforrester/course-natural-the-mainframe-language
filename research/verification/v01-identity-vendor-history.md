<!-- ABOUTME: Adversarial fact-check of research/01-identity-vendor-history.md against primary sources. -->
<!-- ABOUTME: Verdict table, required corrections, and source list. All URLs accessed 2026-08-01. -->

# Verification: 01-identity-vendor-history.md

Verification date: 2026-08-01
Target file: `research/01-identity-vendor-history.md` (spike dated 2026-07-19)
Method: primary sources only for verdicts (vendor site, vendor documentation portal,
vendor press releases, IBM newsroom, Docker Hub registry API, Wayback Machine capture
of the original vendor press release). Wikipedia used only to cross-check historical
dates, never as the sole basis for a CONFIRMED verdict.

## Headline result

The ownership story is correct and has not changed since January 2025. Every current
GA version number in the file is correct and still current as of 2026-08-01. The file
is materially wrong on one thing: **supported platforms**. It lists z/VSE, Fujitsu
BS2000, and Unix as platforms Natural runs on. All three were formally retired by the
vendor, with published end-of-maintenance dates that have already passed. The file also
overstates the 2050 claim and asserts that no firm public lifecycle dates exist, which
is false.

## Verdict table

| Claim (quoted from the file) | Verdict | What is actually true | Primary source URL | Accessed |
|---|---|---|---|---|
| "Natural is a proprietary fourth-generation programming language (4GL) created by Software AG (Darmstadt, Germany)" | CONFIRMED | Vendor site confirms the product and the Software GmbH / Software AG trademark ownership. Footer: "© 2020-2026 Software GmbH" and "Software AG and all Software AG product names are either trademarks or registered trademarks of Software GmbH". | https://www.softwareag.com/en/adabas-natural/ | 2026-08-01 |
| "first released in 1979" | UNVERIFIED | No vendor or other primary source states 1979. The only support is the English Wikipedia "Software AG" article. German Wikipedia and the English Wikipedia biography of Peter Pagé both date the work to 1975: "Die ersten Versionen der ersten Variante für Großrechner wurden ab 1975 von Peter Pagé unter Mitwirkung von Margit Neumann entwickelt." 1975 is a development-start date, 1979 is an unsourced release date. Do not publish 1979 as fact without qualification. | https://de.wikipedia.org/wiki/Natural_(Programmiersprache) and https://en.wikipedia.org/wiki/Peter_Pag%C3%A9 | 2026-08-01 |
| "primarily developed by Peter Pagé" | CONFIRMED (secondary, two independent sources) | "From 1975, together with Margit Neumann, he developed the innovative software development environment Natural as the first fourth-generation programming language." Pagé joined Software AG in Darmstadt in 1971 as one of 6 employees and became VP in 1975. He was not a 1969 founder. | https://en.wikipedia.org/wiki/Peter_Pag%C3%A9 | 2026-08-01 |
| "runs on IBM Z mainframes (z/OS, z/VSE), Fujitsu BS2000, and open systems (Linux, Unix, Windows)" | **REFUTED** | All three of z/VSE, BS2000/OSD, and legacy Unix are retired platforms with published, already-elapsed EOM dates. Vendor letters: z/VSE "end-of-maintenance date ... is June 30, 2023", sustained support to 2024-06-30, "will discontinue support for IBM z/VSE"; BS2000/OSD "end-of-maintenance date ... is December 31, 2023", sustained support to 2024-12-31; legacy Unix (AIX, Solaris, HP-UX) EOM 2024-12-31, end of sustained support 2025-12-31. Vendor states: "Software AG will focus on Linux® and IBM® z/OS® as our strategic platforms for Adabas & Natural 2050+." | https://techcommunity.softwareag.com/t/software-ag-adabas-natural-product-roadmaps/235298 (attached PDFs: `AN-+IBM-+zVSE.pdf`, `AN-Fujitsu.pdf`, `Legacy-Unix-RetirementLetterEnglishEmpowerFinal.pdf`) | 2026-08-01 |
| Section 1 "Platforms" restatement: "The mainframe operating systems are IBM z/OS and z/VSE, plus Fujitsu (Siemens-lineage) BS2000. Open-systems Natural runs on Linux, Unix, and Windows." | **REFUTED** | Natural 9.3.3 for Linux and Cloud system requirements name only Red Hat Enterprise Linux 8 and 9 (x86-64), SUSE Linux Enterprise Server 15 SP3 or above (x86-64), and Red Hat Enterprise Linux 8 on z/Linux. AIX, Solaris, HP-UX, OpenVMS, BS2000, and z/VSE are absent. Windows remains supported as a separate documented product (Natural for Windows 9.3.3, doc set `natwin/9.3.3`). | https://documentation.softwareag.com/natux/9.3.3/en/webhelp/natux-webhelp/install/inst-sysreq.htm | 2026-08-01 |
| NaturalONE quote: developers on "mainframe, UNIX, Linux, OpenVMS or Windows platforms" | PARTIALLY CORRECT | The quote is accurate but is taken from the NaturalONE 9.1.4 doc set, which is four minor versions stale. Current GA is NaturalONE 9.3.3, whose docs are published at `/one/9.3.3/`. Quoting the 9.1.4 platform sentence propagates the retired OpenVMS and Unix platforms into the course. | https://documentation.softwareag.com/one/9.3.3/en/webhelp/one-webhelp/core/relnotes/rn-over.htm | 2026-08-01 |
| "Adabas & Natural ('A&N') is a standalone business held under Software GmbH, which is owned by private equity firm Silver Lake" | CONFIRMED | Press release: "Software AG is owned by Silver Lake, the global technology investment firm." Silver Lake appointed Martin Biegel, Martin Clemm, Robin Colman, and Toktam Khatibzadeh to lead Software GmbH, "which continues to be the holding company for ARIS, Adabas & Natural (A&N)". | https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html | 2026-08-01 |
| "Software AG survives as a brand of Software GmbH, not as an independent stock corporation" | CONFIRMED | Corporate site: "During 2024, Software AG transitioned from being publicly listed to a privately owned company with limited liability under German law. With that change, Software AG is now called Software GmbH." Vendor blog: "Software AG is a Software GmbH brand". | https://www.softwaregmbh.com/ and https://www.softwareag.com/en/blog/insights/adabas-natural-and-aris-launch-as-standalone/ | 2026-08-01 |
| "This is the outcome of Silver Lake's 2023 buyout" and "roughly 63% ownership by June 2023 (valued around 2.4 billion euros)" | CONFIRMED (cross-check) | "In June 2023, Silver Lake secured 63% of Software AG" in a deal valued at €2.4 billion, after an April 2023 agreement at €2.2 billion. | https://en.wikipedia.org/wiki/Software_AG | 2026-08-01 |
| "IBM did NOT acquire Adabas & Natural. IBM bought the webMethods and StreamSets integration businesses (announced Dec 2023, closed 2024)." | CONFIRMED | IBM newsroom release dated Jul 1, 2024: IBM completed its acquisition of StreamSets and webMethods from Software AG. Neither Adabas nor Natural is mentioned anywhere in the release. Announced December 2023; deal value €2.13 billion. | https://newsroom.ibm.com/2024-07-01-IBM-Completes-Acquisition-of-StreamSets-and-webMethods,-Bolstering-its-Automation,-Data-and-AI-Portfolios | 2026-08-01 |
| "about 2.13 billion euros / 2.33 billion USD" | CONFIRMED | €2.13 billion per the completion coverage; $2.33 billion per the original December 2023 announcement. Both figures describe the same deal in different currencies. | https://www.prnewswire.com/news-releases/ibm-to-acquire-streamsets-and-webmethods-platforms-from-software-ag-302017616.html | 2026-08-01 |
| "On January 7, 2025 (Darmstadt), Software GmbH announced that 'Adabas & Natural (A&N) and ARIS will launch as standalone businesses, each led by their own management teams.'" | CONFIRMED | Dateline is exactly "DARMSTADT, Germany, Jan. 7, 2025". The quoted wording is the release headline. | https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html | 2026-08-01 |
| Ownership has not changed again since January 2025 (implied by "Ownership as of mid-2026") | CONFIRMED | Searched specifically for a 2025 or 2026 acquisition, merger, rename, or divestiture involving Adabas, Natural, ARIS, or Software GmbH, including German-language sources and named plausible acquirers (Rocket Software, Broadcom). Nothing found. The corporate site as of copyright year 2026 still describes Software GmbH as holding both ARIS and Adabas & Natural. The Adabas & Natural product page still carries "© 2020-2026 Software GmbH". | https://www.softwaregmbh.com/ | 2026-08-01 |
| "TrendMiner was divested alongside webMethods/StreamSets in July 2024" | PARTIALLY CORRECT | Timing is right, buyer is different and unnamed in the file. TrendMiner went to Proemion GmbH under an agreement dated 2024-04-18 for €47 million, closing in July 2024. IBM did not acquire TrendMiner. As written, "alongside webMethods/StreamSets" invites the reader to assume IBM bought it too. | https://www.heise.de/en/news/Software-AG-Sell-off-continues-with-TrendMiner-for-47-million-euros-9692080.html | 2026-08-01 |
| "Alfabet and Cumulocity sales closed in January 2025 (to other, separately reported buyers)" | CONFIRMED, and the buyers are now identifiable | Alfabet went to Bizzdesign (backed by Main Capital Partners), January 2025. Cumulocity went to a management buyout backed by Schroders Capital, Verso Capital, and Avedon. The file could name both. | https://main.nl/press-release/bizzdesign-acquires-alfabet/ and https://www.rcrwireless.com/20250120/internet-of-things/cumulocity-software-ag | 2026-08-01 |
| "Natural for z/OS (mainframe) | 9.2.4" | CONFIRMED (three independent sources) | (1) Vendor release post lists "Natural for z/OS 9.2.4". (2) Documentation portal: `natmf/9.2.4` is the highest published doc set; `natmf/9.2.5` and all `natmf/9.3.x` return HTTP 404. German 9.2.4 PDFs are dated "März 2026". (3) Vendor 5-year release plan shows the z/OS Natural line as 9.2.4, then 9.2.5, then 9.3.1, on an annual October cadence, so 9.2.5 is not due until October 2026. | https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 and https://documentation.softwareag.com/natmf/9.2.4/en/webhelp/natmf-webhelp/rnotes_mf/rn-mf-over.htm | 2026-08-01 |
| "Natural for Linux and Cloud | 9.3.3" | CONFIRMED (four independent sources) | (1) Release post: "Natural for Linux and Cloud | NAT 9.3.3". (2) Docs portal: `natux/9.3.3` is the highest; `natux/9.3.4` returns 404. (3) Docker Hub `softwareag/natural-ce` newest tag is 9.3.3, last updated 2026-07-21. (4) Release plan shows 9.3.4 due October 2026. | https://documentation.softwareag.com/natux/9.3.3/en/webhelp/natux-webhelp/relnotes/rn-over.htm and https://hub.docker.com/v2/repositories/softwareag/natural-ce/tags/ | 2026-08-01 |
| "NaturalONE (Eclipse IDE) | 9.3.3" | CONFIRMED (three independent sources) | (1) Release post lists NaturalONE 9.3.3. (2) Docs portal: `/one/9.3.3/` release notes exist; `/one/9.3.4/` returns 404. (3) Release plan shows 9.3.4 due October 2026. | https://documentation.softwareag.com/one/9.3.3/en/webhelp/one-webhelp/core/relnotes/rn-over.htm | 2026-08-01 |
| "Adabas for z/OS | 8.6.1" with the cited source being the Oct 2025 release post | PARTIALLY CORRECT | The number is right, the sourcing is wrong. The October 2025 release post has **no row for plain Adabas on z/OS**. It lists the add-ons at 8.6.1 and says they "got aligned with the Adabas v8.6.1 version number". Independent confirmation that 8.6.1 is current: documentation portal `adamf/8.6.1` is the highest published Adabas mainframe doc set (docs dated March 2026), `adamf/8.6.2` returns 404, and the z/OS release plan shows 8.6.2 as the next release. | https://documentation.softwareag.com/adamf/8.6.1/en/webhelp/adamf-webhelp/install_os3/install-zos.htm | 2026-08-01 |
| "Adabas for Linux | 7.4" | CONFIRMED (three independent sources) | (1) Release post: "Adabas for Linux | ADA 7.4". (2) Docs portal: `adaos/7.4.0` is the highest; `adaos/7.5.0` returns 404; 7.4.0 PDFs dated April 2026. (3) Docker Hub `softwareag/adabas-ce` newest tag is 7.4.0, updated 2026-02-02. | https://documentation.softwareag.com/adaos/7.4.0/en/webhelp/adaos-webhelp/ and https://hub.docker.com/v2/repositories/softwareag/adabas-ce/tags/ | 2026-08-01 |
| Implicit claim that the October 2025 figures are still the current GA set | CONFIRMED | No newer release-information post exists. The vendor Tech Community News category's most recent release post is still "Adabas & Natural - Release information Oct. 2025" (2025-10-15). The next release wave is October 2026 per the 5-year release plan. Every version in the file's table is still current on 2026-08-01. | https://techcommunity.softwareag.com/c/news/ | 2026-08-01 |
| "A 'Natural for Visual Studio Code' and a 'Natural AI Code Assistant' are announced with a planned release around October 2026 and are not yet GA" | CONFIRMED, with two caveats | Vendor wording: "Natural for Visual Studio Code - experience Natural in a modern, developer-friendly IDE" and "Natural AI Code Assistant (working title) - accelerate development with AI-driven intelligence", delivered through a Co-Innovation Program: "Joining the program will allow customers to test and shape the product ahead of its planned release in October 2026." Caveat 1: the AI assistant name is explicitly a working title. Caveat 2: October 2026 is two months from this verification date, so this claim has a short shelf life and must be re-checked before publication. | https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 | 2026-08-01 |
| "In 2016, Software AG publicly committed to supporting Adabas and Natural 'through the year 2050 and beyond.'" | PARTIALLY CORRECT | The 2016 press release is real and dated 2016-08-31 (Reston, VA). The actual wording is different from the file's quotation marks: Software AG "released details of its agenda to support and further develop its Adabas and Natural product portfolio until beyond the year 2050", described as "the long-term objective of further developing Adabas & Natural products and services". Chief Customer Officer Eric Duffaut: "we ... want to support our Adabas & Natural customers in the long run ... beyond 2050". This is a strategic agenda, not a contractual support guarantee, and it names no supported version. The quoted string in the file is a paraphrase presented as a quotation. | http://web.archive.org/web/20170424054709/http://www.softwareag.com/us/Press/pressreleases/20160831_Adabas_Natural_2050_Agenda.asp | 2026-08-01 |
| "reflected today in the 'Adabas & Natural 2050+' branding on the vendor's current product page" | CONFIRMED | The current product page says "you need a forward-thinking partner with a vision … someone who can help you take your Adabas & Natural applications to 2050 and beyond". The 2050+ framing survives the ownership change: the January 2025 vendor blog refers to A&N's "2050+ strategy" and the October 2025 release post opens "In keeping with our Adabas & Natural 2050+ agenda". Note that the current pages express aspiration about customer applications, not a dated vendor support commitment. | https://www.softwareag.com/en/adabas-natural/ | 2026-08-01 |
| "The one firm, public lifecycle commitment is the 2016 pledge to support Adabas and Natural 'through 2050 and beyond'" | **REFUTED** | There are firm, public, dated lifecycle statements, and the file missed them. The three platform-retirement letters give explicit EOM dates (z/VSE 2023-06-30, BS2000/OSD 2023-12-31, legacy Unix 2024-12-31) plus explicit end-of-sustained-support dates and named final product releases. These are more concrete than the 2050 agenda and are directly relevant to a course. | https://techcommunity.softwareag.com/t/software-ag-adabas-natural-product-roadmaps/235298 | 2026-08-01 |
| "the actual EOM dates per version are served through the authenticated Empower portal ... These specific dates could not be extracted in this spike without login." | CONFIRMED | Both availability pages checked still say only: "You can view all available Software AG product versions and check the dates when their maintenance ends by visiting Software AG's Empower web site", with instructions to log in and read the EOM column. Additionally, endoflife.date does not track Adabas or Natural at all (462 products indexed, zero matches). | https://documentation.softwareag.com/natural/prd842/rnotes/availability.htm and https://endoflife.date/api/all.json | 2026-08-01 |
| Community Edition: free, components (NaturalONE CE, Natural CE, Adabas CE, Adabas Manager CE), Docker delivery, Windows 10/11 and Linux x86-64, "This Community Edition is for personal use only. Use for commercial production purposes is prohibited." | CONFIRMED | Every element verified verbatim on the vendor page, including the license sentence. Anonymous pull from Docker Hub was tested and works: `softwareag/natural-ce:9.3.3` (public, updated 2026-07-21), `softwareag/adabas-ce:7.4.0`, `softwareag/adabasmanager-ce:9.4.0`. One correction: the vendor registry `containers.softwareag.com` is **not** anonymously public. It redirects to a SAML login. Docker Hub is the anonymous path. | https://www.softwareag.com/en/developer/adabas-natural-community-edition/ | 2026-08-01 |
| "Software AG was founded in 1969 by six employees of the consulting firm AIV ... Peter Schnell ... Headquarters: Darmstadt, Germany." | UNVERIFIED | Wikipedia-only. No vendor history or timeline page was found stating 1969, the AIV origin, or the six founders. The current corporate site (softwaregmbh.com) gives no founding year and no headquarters. This is probably correct but it is not primary-sourced, and the file's own source line already concedes Wikipedia as the source. | https://www.softwaregmbh.com/ (absence of any founding statement) | 2026-08-01 |
| "ADABAS: launched in 1971 as a high-performance transactional DBMS" | UNVERIFIED | Wikipedia-only. Weak indirect corroboration: the 2016 vendor press release boilerplate says "With over 45 years of customer-centric innovation", consistent with an early-1970s origin but not confirming 1971 for ADABAS specifically. | https://en.wikipedia.org/wiki/Software_AG (cross-check only) | 2026-08-01 |
| "it can also access relational databases (DB2, and, as of Natural 9.3.1, MariaDB) and VSAM" | CONFIRMED | Natural 9.3.1 release notes: "Natural now supports the MariaDB database type. Entire Access 9.3.1 is a prerequisite for using the new database." Natural for Db2 and Natural for VSAM are both listed on the vendor Products A-Z page. | https://documentation.softwareag.com/natux/9.3.1/en/webhelp/natux-webhelp/relnotes/rn-931.htm and https://www.softwareag.com/en/products-a-z/ | 2026-08-01 |
| Natural 9.3.1 feature list (Availability Server, USR9201N, USR9205N SHA-256, Multi-Fetch 64KB to 64MB, WHICH command, ACBX by default) | CONFIRMED | All six verified verbatim in the 9.3.1 release notes. | https://documentation.softwareag.com/natux/9.3.1/en/webhelp/natux-webhelp/relnotes/rn-931.htm | 2026-08-01 |
| Open question 2: "General search surfaces NaturalONE doc sets up to 9.1.4 ... Worth a direct confirmation" | **REFUTED as an open question** | It is resolved. NaturalONE documentation is published for 9.3.1, 9.3.2, and 9.3.3 under a newer URL scheme, `documentation.softwareag.com/one/<version>/en/webhelp/one-webhelp/`. The `naturalONE/natONE9xx/` scheme the file searched was retired after 9.1.4. Delete the open question. | https://documentation.softwareag.com/one/9.3.3/en/webhelp/one-webhelp/core/relnotes/rn-over.htm | 2026-08-01 |
| Source list entry: "developer resources at https://developer.softwareag.com/en/Natural.html" | **REFUTED** | Dead. The host resolves in DNS (40.74.42.229) but every HTTPS connection fails (curl exit, HTTP code 000). It is not a 404, the host does not serve. Remove it or replace it with the Tech Community. | https://techcommunity.softwareag.com/ (working replacement, HTTP 200) | 2026-08-01 |
| "(the canonical en.wikipedia.org/wiki/Natural_(programming_language) URL returned HTTP 404 during this spike)" | CONFIRMED, and the fallback has since broken | The English Wikipedia article still 404s. The EverybodyWiki mirror the file relied on as its substitute now returns HTTP 403 and cannot be re-verified. Several section 1 and section 2 claims in the file rest on that mirror and are now unsupported by any retrievable source. | https://en.wikipedia.org/wiki/Natural_(programming_language) (404) | 2026-08-01 |
| Not in the file: vendor documentation portal has been restructured with new umbrella product names | NEW FINDING | The documentation portal now organizes Adabas & Natural under bundle names that do not appear anywhere in the file: **Natural Nexus** (for z/OS, and for Linux & Cloud), **Adabas Advantage** (for z/OS, and for Linux & Cloud), and **JOPAZ** (mainframe capacity optimization). These names are absent from the vendor Products A-Z page, so the naming is inconsistent across vendor properties. A course should use the product names the documentation portal uses, or note both. | https://documentation.softwareag.com/ and https://documentation.softwareag.com/adabas-advantage-zos | 2026-08-01 |
| Not in the file: additional announced products | NEW FINDING | The October 2025 post and the 5-year release plans also announce **Natural Messaging** (IBM MQ integration) and **Natural API Server** (working title, REST API creation and consumption), both alongside the VS Code and AI assistant items. The file mentions only two of the four. | https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 | 2026-08-01 |

## Corrections required

Numbered edits the source file needs, in file order.

1. **Executive summary, bullet 1.** Replace "first released in 1979" with a hedged
   formulation, because no primary source supports 1979. Suggested: "developed from
   1975 by Peter Pagé with Margit Neumann, and generally dated to a 1979 first release
   (a date attested only in secondary sources)."

2. **Executive summary, bullet 1.** Delete "z/VSE" and "Fujitsu BS2000" from the
   platform list and delete "Unix". Replace the platform clause with: "runs on IBM
   z/OS, on Linux (Red Hat Enterprise Linux 8 and 9, SUSE Linux Enterprise Server 15
   SP3 and above, including z/Linux), on Windows, and in containers and cloud. Support
   for IBM z/VSE, Fujitsu BS2000/OSD, and the legacy Unix platforms (AIX, Solaris,
   HP-UX) was formally retired between 2023 and 2025."

3. **Section 1, "Platforms" paragraph.** Same correction as item 2. Cite the Natural
   9.3.3 system requirements page rather than the marketing page for the platform list.

4. **Section 1, NaturalONE paragraph.** Keep the "Eclipse-based development
   environment" definition but stop citing the 9.1.4 doc set for platform coverage.
   Drop the "mainframe, UNIX, Linux, OpenVMS or Windows" sentence entirely, or mark it
   explicitly as a stale 9.1.4 statement that no longer reflects supported platforms.
   Re-cite NaturalONE to `documentation.softwareag.com/one/9.3.3/`.

5. **Section 2, "Natural first release" bullet.** Rewrite to lead with 1975 development
   (two independent sources) and treat 1979 as an unconfirmed release date. Add that
   Pagé joined Software AG in 1971 and was not among the founders, since the file
   currently sits the founding bullet and the Pagé bullet next to each other in a way
   that implies otherwise.

6. **Section 2, "Long-term support pledge" bullet.** Fix the fake quotation. The
   vendor never wrote "through the year 2050 and beyond". Replace with the actual
   wording and date: on 2016-08-31 Software AG announced the "Adabas & Natural 2050"
   agenda, "to support and further develop its Adabas and Natural product portfolio
   until beyond the year 2050". Add that this is a strategic agenda rather than a
   per-version support guarantee, and that the 2050+ framing has been carried forward
   by Software GmbH (January 2025 blog, October 2025 release post).

7. **Section 3, item 3.** Name TrendMiner's buyer and separate it from the IBM deal:
   "TrendMiner was sold separately to Proemion GmbH for €47 million (agreement
   2024-04-18, closing July 2024). IBM did not acquire TrendMiner."

8. **Section 3, item 3.** Name the Alfabet and Cumulocity buyers: Alfabet to Bizzdesign
   (Main Capital Partners), Cumulocity to a management buyout backed by Schroders
   Capital, Verso Capital, and Avedon.

9. **Section 4, versions table.** Keep all five version numbers, they are all correct.
   Fix the sourcing for the Adabas z/OS row: the October 2025 post does not contain an
   "Adabas for z/OS" row. Cite `documentation.softwareag.com/adamf/8.6.1/` instead, and
   note that the post only says the add-ons "got aligned with the Adabas v8.6.1 version
   number".

10. **Section 4, versions table.** Add a currency note: verified still current as of
    2026-08-01 against the documentation portal and Docker Hub; the next release wave
    is October 2026, when the vendor 5-year plan schedules Natural z/OS 9.2.5, Natural
    Linux and Cloud 9.3.4, NaturalONE 9.3.4, Adabas Linux 7.5, and Adabas z/OS 8.6.2.

11. **Section 4, roadmap paragraph.** Add the two omitted announced products (Natural
    Messaging, Natural API Server) and mark "Natural AI Code Assistant" as a working
    title. Add an expiry warning: the "not yet GA" statement is only valid until
    October 2026 and must be re-verified before the course ships.

12. **Section 4, end-of-life bullets.** Delete the sentence "The one firm, public
    lifecycle commitment is the 2016 pledge". Replace with the three published platform
    retirements and their dates (z/VSE EOM 2023-06-30, BS2000/OSD EOM 2023-12-31,
    legacy Unix EOM 2024-12-31 with end of sustained support 2025-12-31). Keep the
    accurate point that per-version EOM dates for supported platforms remain behind the
    Empower login, and add that endoflife.date does not track these products.

13. **Section 5, Community Edition.** Correct "available via Docker Hub and Software
    AG's public container registry". `containers.softwareag.com` requires a login and is
    not anonymously public. Docker Hub is the anonymous path, verified by pulling the
    manifest for `softwareag/natural-ce:9.3.3` without credentials.

14. **Open questions, item 2.** Delete it. NaturalONE 9.3.3 documentation is published
    at `documentation.softwareag.com/one/9.3.3/`; the `naturalONE/natONE9xx/` URL scheme
    was simply retired after 9.1.4.

15. **Sources list.** Remove `https://developer.softwareag.com/en/Natural.html`, the
    host does not serve. Remove or flag the EverybodyWiki entry, it now returns 403 and
    cannot be re-verified, which leaves several section 1 and 2 statements unsupported.

16. **New section or note.** Record that the documentation portal now uses the umbrella
    names Natural Nexus, Adabas Advantage, and JOPAZ, which conflict with the vendor
    Products A-Z naming. Decide which naming the course will use and say so.

17. **Section 3, ownership.** No change needed. The section is accurate and survives an
    adversarial search for post-January-2025 ownership changes. Add a re-verification
    date line, since this is the claim most likely to go stale.

## Sources

All accessed 2026-08-01.

Vendor primary sources:

- https://www.softwareag.com/en/adabas-natural/ : product page, 2050 and beyond wording, platform positioning (IBM Z, Linux, cloud), "© 2020-2026 Software GmbH" footer.
- https://www.softwaregmbh.com/ : corporate site, AG to GmbH transition during 2024, ARIS and Adabas & Natural as the two remaining businesses.
- https://www.softwareag.com/en/blog/insights/adabas-natural-and-aris-launch-as-standalone/ : 2025-01-07 blog, "Software AG is a Software GmbH brand", A&N 2050+ strategy.
- https://www.softwareag.com/en/products-a-z/ : full Adabas and Natural product list, including Natural for VSAM and Natural for Db2.
- https://www.softwareag.com/en/developer/adabas-natural-community-edition/ : Community Edition components, delivery, platforms, personal-use-only license sentence.
- https://www.softwareag.com/en/resources/adabas-natural/2050/ : current "Evolve with Adabas & Natural 2050+" page, aspirational wording with no dated support commitment.

Vendor press releases:

- https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html : dateline DARMSTADT, Germany, Jan. 7, 2025; Silver Lake ownership; four named executives; IBM July 2024; TrendMiner, Cumulocity, Alfabet divestments.
- http://web.archive.org/web/20170424054709/http://www.softwareag.com/us/Press/pressreleases/20160831_Adabas_Natural_2050_Agenda.asp : archived original 2016-08-31 "Adabas & Natural 2050" agenda release, verbatim "until beyond the year 2050", Eric Duffaut quote, CONNX acquisition closing 2016-07-31.

Vendor technical sources:

- https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 : posted 2025-10-15; full GA version table; Co-Innovation Program; Natural for Visual Studio Code; Natural AI Code Assistant (working title); Natural Messaging; Natural API Server (working title).
- https://techcommunity.softwareag.com/c/news/ : News category listing, confirms no newer release-information post exists.
- https://techcommunity.softwareag.com/t/software-ag-adabas-natural-product-roadmaps/235298 : 5-year release plans 2025 to 2029 for z/OS and for Linux and Cloud, plus the three platform-retirement letters for BS2000/OSD, z/VSE, and legacy Unix with their EOM and end-of-sustained-support dates.
- https://documentation.softwareag.com/ : documentation portal home, showing the Natural Nexus, Adabas Advantage, and JOPAZ umbrella names.
- https://documentation.softwareag.com/natmf/9.2.4/en/webhelp/natmf-webhelp/rnotes_mf/rn-mf-over.htm : Natural for z/OS 9.2.4, highest published mainframe doc set.
- https://documentation.softwareag.com/natux/9.3.3/en/webhelp/natux-webhelp/relnotes/rn-over.htm : Natural for Linux and Cloud 9.3.3.
- https://documentation.softwareag.com/natux/9.3.3/en/webhelp/natux-webhelp/install/inst-sysreq.htm : current supported operating systems (RHEL 8/9, SLES 15 SP3+, RHEL 8 on z/Linux).
- https://documentation.softwareag.com/natux/9.3.1/en/webhelp/natux-webhelp/relnotes/rn-931.htm : MariaDB, Natural Availability Server, USR9201N, USR9205N, Multi-Fetch 64MB, WHICH, ACBX.
- https://documentation.softwareag.com/one/9.3.3/en/webhelp/one-webhelp/core/relnotes/rn-over.htm : NaturalONE 9.3.3 release notes, current doc URL scheme.
- https://documentation.softwareag.com/adamf/8.6.1/en/webhelp/adamf-webhelp/install_os3/install-zos.htm : Adabas for z/OS 8.6.1, highest published doc set.
- https://documentation.softwareag.com/adaos/7.4.0/en/webhelp/adaos-webhelp/ : Adabas for Linux and Cloud 7.4.0, highest published doc set.
- https://documentation.softwareag.com/natural/prd842/rnotes/availability.htm : per-version EOM dates gated behind the Empower login.

Registry and independent checks:

- https://hub.docker.com/v2/repositories/softwareag/natural-ce/tags/ : tags 9.3.3 (2026-07-21), 9.3.2, 9.3.1, 9.2.1, 9.1.4.
- https://hub.docker.com/v2/repositories/softwareag/adabas-ce/tags/ : tags 7.4.0 (2026-02-02), 7.3.0, 7.2.0, 7.1.1, 7.0.1.
- https://hub.docker.com/v2/repositories/softwareag/adabasmanager-ce/tags/ : tags 9.4.0 (2025-10-14) and older.
- https://endoflife.date/api/all.json : 462 tracked products, zero matches for adabas, natural, or Software AG.

Third-party primary and secondary sources:

- https://newsroom.ibm.com/2024-07-01-IBM-Completes-Acquisition-of-StreamSets-and-webMethods,-Bolstering-its-Automation,-Data-and-AI-Portfolios : IBM completion release, Jul 1, 2024, no mention of Adabas or Natural.
- https://www.prnewswire.com/news-releases/ibm-to-acquire-streamsets-and-webmethods-platforms-from-software-ag-302017616.html : original December 2023 announcement.
- https://www.heise.de/en/news/Software-AG-Sell-off-continues-with-TrendMiner-for-47-million-euros-9692080.html : TrendMiner to Proemion, €47 million.
- https://main.nl/press-release/bizzdesign-acquires-alfabet/ : Alfabet to Bizzdesign, January 2025.
- https://www.rcrwireless.com/20250120/internet-of-things/cumulocity-software-ag : Cumulocity management buyout backed by Schroders Capital, Verso Capital, Avedon.

Cross-check only (never sole basis for a verdict):

- https://en.wikipedia.org/wiki/Software_AG : 1969 founding, six AIV employees, Peter Schnell, Darmstadt, ADABAS 1971, Natural 1979, Silver Lake 63% June 2023.
- https://en.wikipedia.org/wiki/Peter_Pag%C3%A9 : joined Software AG 1971, VP from 1975, developed Natural from 1975 with Margit Neumann.
- https://de.wikipedia.org/wiki/Natural_(Programmiersprache) : development from 1975, historical platform list.
- https://en.wikipedia.org/wiki/Natural_(programming_language) : still HTTP 404.
