<!-- ABOUTME: Research spike on the identity, vendor, ownership, and history of the Natural 4GL. -->
<!-- ABOUTME: Primary-source-cited; verifies current owner and current versions from live 2024-2026 sources. -->

# Natural (Software AG 4GL): Identity, Vendor, and History

Spike date: 2026-07-19

## Executive summary

- Natural is a proprietary fourth-generation programming language (4GL) created by Software AG (Darmstadt, Germany), first released in 1979, primarily developed by Peter Pagé. It is the application language most tightly paired with Software AG's ADABAS database and runs on IBM Z mainframes (z/OS, z/VSE), Fujitsu BS2000, and open systems (Linux, Unix, Windows), with cloud/container support.
- Ownership as of mid-2026: Adabas & Natural ("A&N") is a standalone business held under Software GmbH, which is owned by private equity firm Silver Lake. Software AG survives as a brand of Software GmbH, not as an independent stock corporation. This is the outcome of Silver Lake's 2023 buyout and the subsequent breakup of the old Software AG portfolio.
- IBM did NOT acquire Adabas & Natural. IBM bought the webMethods and StreamSets integration businesses (announced Dec 2023, closed 2024). A&N was retained and, per the Jan 7 2025 announcement, spun into its own standalone unit alongside ARIS. Alfabet and Cumulocity were sold to other buyers.
- Current GA versions as of October 2025 (per the vendor's release-information post): Natural for z/OS 9.2.4; Natural for Linux and Cloud 9.3.3; NaturalONE 9.3.3; Adabas for z/OS 8.6.1; Adabas for Linux 7.4. A "Natural for Visual Studio Code" and a "Natural AI Code Assistant" are announced with a planned release around October 2026 and are not yet GA.
- Licensing is proprietary/commercial, but a free "Adabas & Natural Community Edition" exists for personal, non-production use. It ships Natural, NaturalONE, Adabas, and Adabas Manager Community editions as Docker containers for Windows 10/11 and Linux x86-64. This is the practical, legal path for an individual to run Natural without an enterprise license.

---

## 1. What Natural is

Natural is a fourth-generation programming language (4GL): a high-level, English-like application-development language designed so that business logic and database access can be written with far less code than a 3GL such as COBOL or PL/I. Its syntax deliberately reads like structured natural language, which is the origin of the product name. As of 2026-07-19, per the EverybodyWiki mirror of the Wikipedia article and the Software AG developer materials, Natural programs can either be run interpretively or compiled to objects that execute faster and can call operating-system services more directly.

Paradigm and use: Natural is an imperative, structured, data-centric application language. It is used to build and maintain high-volume, transaction-processing business applications, the classic "system of record" workloads on mainframes (banking, insurance, government, utilities). It includes its own reporting, screen/map handling, and data-definition facilities, and integrates a data dictionary (Predict) for metadata.

Relationship to ADABAS: Natural is the native application language for ADABAS (Adaptable Database System), Software AG's high-performance non-relational (inverted-list) transactional DBMS first shipped in 1971. Many ADABAS applications are written in Natural, and the two are marketed and licensed together as "Adabas & Natural." Natural is not limited to ADABAS; it can also access relational databases (DB2, and, as of Natural 9.3.1, MariaDB) and VSAM, but the ADABAS pairing is the defining one. Source: EverybodyWiki/Wikipedia mirror and the Natural 9.3.1 release notes, accessed 2026-07-19.

Platforms: Per the NaturalONE documentation (natONE914, "What is NaturalONE?") and the vendor product page, Natural targets developers working on "mainframe, UNIX, Linux, OpenVMS or Windows platforms." The mainframe operating systems are IBM z/OS and z/VSE, plus Fujitsu (Siemens-lineage) BS2000. Open-systems Natural runs on Linux, Unix, and Windows. The current product page states: "Use Adabas & Natural to build and deploy high-performance applications on IBM Z, Linux or cloud while connecting seamlessly to the latest innovations." Accessed 2026-07-19.

NaturalONE (the IDE): NaturalONE is "an Eclipse-based development environment for developing and maintaining Natural applications with web-based user interfaces and Natural services" (NaturalONE documentation, natONE914, accessed 2026-07-19). It bundles editors, testing/debugging, automated Predict documentation, versioning, and deployment into one Eclipse framework, and integrates with open-source DevOps tooling such as Ant and Jenkins. It lets developers who traditionally worked natively on the mainframe use a modern desktop IDE against Natural code. NaturalONE and the Natural Development Server are aligned with Natural version 9.

---

## 2. History and origins

- Vendor founding: Software AG was founded in 1969 by six employees of the consulting firm AIV (Institut fuer Angewandte Informationsverarbeitung). Co-founder Peter Schnell served as chairman for many years. Headquarters: Darmstadt, Germany. Source: Wikipedia "Software AG," accessed 2026-07-19.
- ADABAS: launched in 1971 as a high-performance transactional DBMS. This is the database Natural is built to serve. Source: Wikipedia "Software AG," accessed 2026-07-19.
- Natural first release: 1979, primarily developed by Peter Pagé. Some sources credit initial mid-1970s design work to Peter Pagé (also rendered "Peter Page") and Margit Neumann. Source: Wikipedia "Software AG" and EverybodyWiki "Natural (programming language)," accessed 2026-07-19.
- Long-term support pledge: In 2016, Software AG publicly committed to supporting Adabas and Natural "through the year 2050 and beyond." This is reflected today in the "Adabas & Natural 2050+" branding on the vendor's current product page. Sources: EverybodyWiki mirror and softwareag.com/en/adabas-natural, accessed 2026-07-19.
- Modernization: Natural was integrated with Eclipse to create NaturalONE, moving development off green-screen tooling and into a DevOps-capable IDE.

Version-milestone note: The primary sources located in this spike give reliable data for the current 9.x line (see section 4) and the qualitative milestones above (1979 first release, 2016 through-2050 pledge, the NaturalONE/Eclipse era). A complete numbered version genealogy (for example the 2.x through 8.x history and exact release years) was not obtainable from primary vendor pages without the authenticated Empower portal, and is flagged as an open question below rather than asserted from memory.

---

## 3. Current ownership and vendor status (verified live)

This is the section that most needed live verification, because ownership changed materially in 2023-2025.

What happened, in order:

1. Silver Lake buyout: In 2023, private equity firm Silver Lake acquired a controlling stake in Software AG, reaching roughly 63% ownership by June 2023 (valued around 2.4 billion euros). Software AG was subsequently converted from a stock corporation (AG) into a limited liability company, and the holding entity is now "Software GmbH." Source: Wikipedia "Software AG," accessed 2026-07-19.
2. IBM acquisition (integration products only): Software AG agreed to sell its webMethods and StreamSets platforms to IBM for about 2.13 billion euros / 2.33 billion USD (announced December 2023, closed in 2024). IBM did NOT buy Adabas & Natural. Sources: Wikipedia "Software AG" and multiple search results (diginomica, CIO.com), accessed 2026-07-19.
3. Further divestitures: TrendMiner was divested alongside webMethods/StreamSets in July 2024. Alfabet and Cumulocity sales closed in January 2025 (to other, separately reported buyers). Source: PRNewswire press release dated January 7, 2025, and softwareag.com blog, accessed 2026-07-19.
4. A&N becomes standalone: On January 7, 2025 (Darmstadt), Software GmbH announced that "Adabas & Natural (A&N) and ARIS will launch as standalone businesses, each led by their own management teams." Silver Lake appointed four senior executives to lead the group: Martin Biegel, Martin Clemm, Robin Colman, and Toktam Khatibzadeh. Group CEO Sanjay Brahmawar departed. Christian Lucas, Managing Partner of Silver Lake and Chairman of the Supervisory Board of Software GmbH, said Silver Lake is "excited to invest in both ARIS and Adabas & Natural and their world class products, and support their multi-year growth acceleration plans as independent companies." Sources: PRNewswire release (2025-01-07) and softwareag.com blog "Adabas & Natural and ARIS launch as standalone," accessed 2026-07-19.

Bottom line on ownership as of mid-2026:

- Owner: Silver Lake (private equity), through the holding company Software GmbH.
- Operating entity/brand: The product is maintained by the standalone Adabas & Natural business unit within Software GmbH. "Software AG" continues to be used as a brand: the blog states "Software AG is a Software GmbH brand." So the product still appears under the Software AG name and the softwareag.com domain, even though the legal independent "Software AG" (the AG stock corporation) effectively no longer exists as the parent.
- Official product/support presence: Product page at https://www.softwareag.com/en/adabas-natural/ (marketed as "Adabas & Natural 2050+"); documentation at https://documentation.softwareag.com/; developer resources at https://developer.softwareag.com/en/Natural.html; technical community at https://techcommunity.softwareag.com/; and the authenticated customer support/lifecycle portal Empower at https://empower.softwareag.com/.

Naming caution: Do not describe Adabas & Natural as "an IBM product" or "acquired by IBM." That is a common and incorrect conflation. IBM took the integration middleware (webMethods, StreamSets); A&N stayed with Silver Lake's Software GmbH.

---

## 4. Current versions and lifecycle

Latest GA versions, as of October 2025, per the vendor's "Adabas & Natural Release information Oct. 2025" post on the Software AG Tech Community (techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504), accessed 2026-07-19:

| Product | Latest GA (Oct 2025) | Notes |
|---|---|---|
| Natural for z/OS (mainframe) | 9.2.4 | Mainframe line |
| Natural for Linux and Cloud | 9.3.3 | Open-systems / container line |
| NaturalONE (Eclipse IDE) | 9.3.3 | |
| Adabas for z/OS | 8.6.1 | Utilities (Fastpath, Vista, System Coordinator) aligned to this version |
| Adabas for Linux | 7.4 | |

Roadmap items announced but not yet GA (as of the Oct 2025 post, accessed 2026-07-19): "Natural for Visual Studio Code" and a "Natural AI Code Assistant," with a planned release around October 2026. Treat these as forthcoming, not shipping.

Corroborating version data points (accessed 2026-07-19):
- Natural 9.3.1 for Linux/Unix/Windows release notes exist at documentation.softwareag.com/natux/9.3.1/... and list new features: a Natural Availability Server for high availability, new SYSEXT APIs (USR9201N time conversion, USR9205N SHA-256 hashing), MariaDB support, Multi-Fetch buffer raised from 64KB to 64MB, the new WHICH system command, and ACBX calls generated by default. The page does not state an explicit calendar release date.
- NaturalONE documentation is published per minor version (natONE911, natONE913, natONE914 = 9.1.4 are all live doc sets). The Oct 2025 release post lists NaturalONE 9.3.3 as current GA, which is newer than the 9.1.4 doc set that surfaced in general search. The 9.3.3 figure is the authoritative "latest GA" number.

End-of-life / end-of-maintenance:
- Software AG maintains a "Product Availability and End of Maintenance" facility, but the actual EOM dates per version are served through the authenticated Empower portal (empower.softwareag.com to Products to Product Version Availability, with the EOM column). These specific dates could not be extracted in this spike without login. Source: documentation.softwareag.com/natural/prd842/rnotes/availability.htm and empower.softwareag.com, accessed 2026-07-19.
- The one firm, public lifecycle commitment is the 2016 pledge to support Adabas and Natural "through 2050 and beyond," reinforced by current "2050+" branding. Per-version EOM dates are an open item (see below).

Verification note per the recency rule: every version number above is stated "as of 2025-10 (vendor release post), accessed 2026-07-19." No numerical regression is recommended anywhere in this document; the newest published GA figures are the ones cited.

---

## 5. Licensing and how an individual can run Natural

Natural is proprietary, commercial software. Production use requires a paid license from the Adabas & Natural business (Software GmbH).

Free option for individuals: the Adabas & Natural Community Edition, at https://www.softwareag.com/en/developer/adabas-natural-community-edition/ (accessed 2026-07-19). Verified facts:

- Cost: free to download and use.
- Components: NaturalONE Community Edition, Natural Community Edition, Adabas Community Edition, and Adabas Manager Community Edition, plus a getting-started guide.
- Delivery: Docker containers, available via Docker Hub and Software AG's public container registry.
- Platforms: Windows 10/11 and Linux (x86, 64-bit), requiring Docker.
- License limit (quoted): "This Community Edition is for personal use only. Use for commercial production purposes is prohibited."
- Access: registration through a download form ("Register here to gain access").

Practical guidance for a course/portfolio context: the Community Edition is the legitimate, no-cost way for an individual to install and run Natural and NaturalONE locally (via Docker) for learning and demonstration. It is explicitly non-production and personal-use only, which fits training and portfolio work. There is no indication of a perpetual free production tier; anything beyond personal learning requires a commercial license.

---

## Open questions and unresolved conflicts

1. Per-version EOM dates: The concrete end-of-maintenance dates for Natural 9.1 / 9.2 / 9.3 and the mainframe 9.2.4 line are gated behind the authenticated Empower portal and were not retrieved. Resolve by logging into empower.softwareag.com (Products to Product Version Availability) or by asking a licensed contact. The public anchor remains the "through 2050+" pledge.
2. NaturalONE version surface: General search surfaces NaturalONE doc sets up to 9.1.4, while the Oct 2025 release post lists NaturalONE 9.3.3 GA. Treated here as: 9.3.3 is current GA; the 9.1.x doc sets are simply older published documentation still online. Worth a direct confirmation on the developer center if precision matters for the course.
3. Full numbered version history: A complete 1979-to-present version genealogy with dated releases (2.x through 8.x) was not obtainable from primary vendor pages in this spike. If the course needs a version timeline, the Empower "General Support Information" pages or Software AG historical release notes would be the primary source to mine.
4. Legal-entity precision: "Software AG" now denotes a brand under Software GmbH rather than the former AG stock corporation. If the course makes legal-entity claims, phrase them as "the Adabas & Natural business unit of Software GmbH (a Silver Lake company), marketed under the Software AG brand."

---

## Sources

All URLs accessed 2026-07-19.

- https://www.softwareag.com/en/adabas-natural/: current vendor product page; "Adabas & Natural 2050+" branding; platform statement (IBM Z, Linux, cloud); support/documentation links. Substantiates section 1 and 3 (branding, platforms, official site).
- https://www.softwareag.com/en/developer/adabas-natural-community-edition/: Community Edition: free, personal-use-only, components, Docker/Windows/Linux delivery, registration. Substantiates section 5 (licensing).
- https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504: "Adabas & Natural Release information Oct. 2025"; current GA versions (Natural z/OS 9.2.4, Natural Linux/Cloud 9.3.3, NaturalONE 9.3.3, Adabas z/OS 8.6.1, Adabas Linux 7.4) and the VS Code / AI Code Assistant roadmap. Substantiates section 4 (versions).
- https://documentation.softwareag.com/natux/9.3.1/en/webhelp/natux-webhelp/relnotes/rn-931.htm: Natural 9.3.1 (Linux/Unix/Windows) release notes; MariaDB support, Availability Server, new APIs. Substantiates section 1 (MariaDB) and 4 (feature corroboration).
- https://documentation.softwareag.com/naturalONE/natONE914/core/introduction/intro-whatis.htm: "What is NaturalONE?" Eclipse-based IDE definition; documentation for NaturalONE 9.1.4. Substantiates section 1 (NaturalONE) and open question 2.
- https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html: press release dated 2025-01-07; A&N and ARIS standalone; Silver Lake ownership; leadership (Biegel, Clemm, Colman, Khatibzadeh); Alfabet/Cumulocity closings; Brahmawar departure. Substantiates section 3 (ownership).
- https://www.softwareag.com/en/blog/insights/adabas-natural-and-aris-launch-as-standalone/: vendor blog (2025-01-07); "Software AG is a Software GmbH brand"; A&N standalone strategy; A&N described as decades-long leader in non-relational transactional database software. Substantiates section 3 (brand vs entity).
- https://en.wikipedia.org/wiki/Software_AG: Software AG founding (1969, six AIV employees, Peter Schnell), Darmstadt HQ, ADABAS 1971, Natural 1979 (Peter Pagé), Silver Lake 63% stake (2023), IBM webMethods/StreamSets sale (~2.33B USD). Cross-check for section 2 and 3.
- https://en.everybodywiki.com/Natural_(programming_language): Natural as proprietary 4GL, mid-1970s design (Peter Pagé, Margit Neumann), interpreted vs compiled, 2016 support-through-2050 pledge, NaturalONE/Eclipse integration. Cross-check for section 1 and 2 (the canonical en.wikipedia.org/wiki/Natural_(programming_language) URL returned HTTP 404 during this spike).
- https://documentation.softwareag.com/natural/prd842/rnotes/availability.htm and https://empower.softwareag.com/: "Product Availability and End of Maintenance" pointer; per-version EOM dates live in the authenticated Empower portal. Substantiates section 4 (lifecycle) and open question 1.
- Supporting search-result context (not individually fetched): diginomica.com and cio.com articles on Software AG's breakup and the IBM integration-business sale; corroborates section 3 narrative.
