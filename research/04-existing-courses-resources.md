<!-- ABOUTME: Research spike cataloging official docs and existing learning resources for the Natural (Software AG) language. -->
<!-- ABOUTME: Maps the competitive/reference landscape and the gap a new interactive beginner course would fill. -->

# Spike 04: Official Documentation and Existing Learning Resources for Natural

Spike date: 2026-07-19

> **Corrections applied 2026-08-01.** This spike was adversarially re-verified. See
> `research/verification/` for the verdict tables. Known defects found in this file:
> - Every Natural version number in the resource table is wrong, because the spike read a
>   documentation portal frozen on 2021-10-15. Correct as of 2026-08-01: mainframe 9.2.4,
>   Windows 9.3.3, Linux and UNIX 9.3.3.
> - `techcommunity.softwareag.com/c/adabas-natural/` is a 404, not the canonical home.
>   `education.softwareag.com` does not respond at all.
> - The Udemy and Pluralsight rows report "none found" from checks that never succeeded
>   (Udemy returns 403 behind Cloudflare; Pluralsight renders client-side). Treat both as
>   UNKNOWN. Coursera, LinkedIn Learning, and edX are genuinely empty.
> - "No independently authored ISBN-bearing textbook" is false. Michael Schlueter,
>   "Einfuehrung in die Programmierung mit Natural & Adabas", Lehmanns Media 2019, ISBN
>   978-3-86541-994-1. Missed because every search ran in English only.
> - `adabas-natural-code-samples` is not "active" (last commit 2024-03-27) and has 51
>   folders, not 60 or more. The Medium series is member-only, not free.
> - The learn.softwareag.com course IDs prove nothing: it is a Docebo single-page app that
>   returns HTTP 200 for every path, including nonsense ones.
> Where this file and its verification file disagree, the verification file wins.



## Executive summary (the gap analysis)

No modern, interactive, beginner-friendly Natural course exists. That is the
headline finding, and it is the gap this course fills.

What does exist falls into five buckets, none of which is a self-serve,
hands-on, browser-based beginner course:

1. Vendor reference documentation. Comprehensive, freely accessible, and
   current (Natural for Mainframes 9.2.3 / 9.2.4, 2024 to 2025), but it is
   reference material and a text "first steps" tutorial, not a guided course.
   It lives on the new webhelp portal at `documentation.softwareag.com`.
2. Vendor LMS courses at `learn.softwareag.com` (for example "Natural
   Programming Fundamentals", self-paced and instructor-led variants). These
   are gated behind a Software AG / Partner Hub login, so they are not
   openly available and their pricing is not public.
3. Instructor-led corporate classroom training from third parties (Verhoef in
   the UK, Nisa Trainings and MaxMunus in India). Multi-day, on-site or live
   virtual, quote-on-request pricing, aimed at enterprise teams, not
   individual beginners.
4. A single free self-study text, "NATURAL Essentials" by Stephen Paul
   Simpson, which is genuinely useful but dates from 1999 to 2000 and is a PDF,
   not an interactive experience.
5. Scattered free material: official YouTube tutorials, one community
   blogger's Medium series, and the archived GitHub education package.

Critically, the mainstream learning marketplaces have essentially nothing.
Searches of Udemy, Pluralsight, Coursera, LinkedIn Learning, and edX returned
no dedicated, current, interactive Natural course. edX surfaces only a topic
landing page, not a course. This absence is the market signal: a learner who
wants to pick up Natural today has no equivalent of a "Learn Python" style
interactive course to turn to.

On the plus side for course production, the raw materials are strong and
largely open: a free Community Edition (Docker) with a demo database, an
Apache-2.0 licensed community code-sample corpus, the classic EMPLOYEES and
VEHICLES demo files, and a genuinely modern, actively maintained open-source
toolchain (the `natls` language server, MIT licensed, released v0.18 in
January 2026). Usable teaching code and a free runtime are not the blocker.

Ownership note that clears up a common confusion: Adabas and Natural are owned
by Silver Lake, operated as a standalone business under the Software GmbH
holding company since the start of 2025. They were NOT part of the Rocket
Software acquisition (that was OpenText's Application Modernization and
Connectivity unit) and NOT part of the IBM acquisition (that was
webMethods and StreamSets). The documentation site did not migrate to a new
owner's domain; it stayed on `documentation.softwareag.com`, though the portal
itself was re-platformed to a versioned webhelp structure
(`documentation.softwareag.com/natmf/<version>/...`).

---

## 1. Official documentation

### Where the docs live now

The canonical documentation portal is `documentation.softwareag.com`. It is
freely accessible with no login for the core product documentation. Partner
add-on products may require an Empower login, but the Natural language
reference, programming guide, statements reference, and first-steps tutorial
are open.

Two portal generations coexist:

- Legacy per-release paths: `documentation.softwareag.com/natural/<code>/...`
  where `<code>` encodes version and platform, for example `nat911mf`
  (Mainframe 9.1.1), `nat913win` (Windows 9.1.3), `nat912unx` (UNIX 9.1.2).
- New versioned webhelp: `documentation.softwareag.com/natmf/9.2.3/en/webhelp/...`
  for the current Mainframe 9.2.x line. This is the format new releases use.

The index page `documentation.softwareag.com/natural/index.htm` organizes the
Natural family (Natural for Mainframes, Open Systems, Ajax, ISPF, NaturalONE,
API Management, Business Rule Automation, and connectivity or TP monitor
products).

### Current versions (as of 2026-07-19)

| Platform | Latest documented version | Doc location |
|---|---|---|
| Natural for Mainframes | 9.2.3 (with 9.2.4 release notes PDF also present) | `documentation.softwareag.com/natmf/9.2.3/...` |
| Natural for Windows | 9.1.3 (Oct 2021) | `documentation.softwareag.com/natural/nat913win/overview.htm` |
| Natural for UNIX / Open Systems | 9.1.2 | `documentation.softwareag.com/natural/nat912unx/...` |

Verified: the 9.2.3 mainframe release notes page loads freely without a login.
A 9.2.4 release-notes PDF is also referenced on the portal, so the mainframe
line has moved past 9.2.3. Treat 9.2.x as the current supported mainframe
generation.

### The canonical documentation set

For any given release the set includes: Language reference, Statements
reference, System Functions, System Variables, Terminal Commands, Parameter
Reference, Programming Guide, Editors, System Commands, Utilities, Debugger,
and a Glossary. The Programming Guide (for example
`documentation.softwareag.com/natural/nat913win/print/pg.pdf`) is the single
best "how the language works" document and is downloadable as a PDF.

### First-steps tutorial (official)

`documentation.softwareag.com/natural/nat912unx/firststeps/fs-start.htm`
"Getting Started with Natural" walks a beginner through invoking the main
menu, navigating libraries, issuing commands, creating a user library called
`TUTORIAL`, and writing a first "Hello World" using the `WRITE` statement. It
teaches reporting mode versus structured mode and insists on structured mode.
It is text and screenshots, freely accessible, and is the closest thing the
vendor offers to a beginner walkthrough. It is not interactive.

### NaturalONE documentation

NaturalONE (the Eclipse-based IDE) has its own doc tree under
`documentation.softwareag.com/naturalONE/...` (for example `natONE921`,
`natONE912`). This is the modern development-environment documentation that a
course using NaturalONE would reference.

---

## 2. Official tutorials, getting-started, and demo files

| Resource | What it is | Access | Notes |
|---|---|---|---|
| "Getting Started with Natural" (First Steps) | Official text tutorial, creates the `TUTORIAL` library, first WRITE program | Free, no login | Per-version copies exist (nat912unx, nat827mf, etc.) |
| Programming Guide "First Steps for Programmers" | Section within the Programming Guide that introduces reading the EMPLOYEES demo file | Free PDF/webhelp | The classic entry point for data access |
| EMPLOYEES and VEHICLES demo files | The long-standing Natural example database files and their DDMs, shipped with the product and the Community Edition demo database | Bundled with product / CE | The canonical teaching dataset. `READ EMPLOYEES` and joins to `VEHICLES` are the textbook first queries. Strongly recommended as the course's working corpus |
| Adabas & Natural Community Edition (Docker) | Free personal-use Docker bundle: NaturalONE CE, Natural CE, Adabas CE, Adabas Manager CE, plus a demo database | Free download, personal use only, no commercial/production use | v1.3, October 2024. Runs on Windows 10/11 and Linux x86-64 with a Docker-compatible runtime. This is the free runtime a course can target. Also on Docker Hub (`softwareag/adabas-ce`) and AWS Marketplace |
| Software AG Developer Center (Natural) | `developer.softwareag.com/en/Natural.html` hub linking downloads, getting-started, and the Community Edition | Free (site had a TLS quirk on fetch during this spike; content is public) | Entry point for the CE download and dev resources |

The Community Edition matters most for course production: it gives every
learner a real Natural runtime and the EMPLOYEES/VEHICLES demo data for free,
which is exactly what a hands-on course needs. Its "personal use only" license
is a constraint to note for any hosted or commercial delivery.

---

## 3. Books

Published books on Natural are old and few. Natural never had a mass-market
trade-book ecosystem the way COBOL or C did.

| Title | Author | Year | Notes |
|---|---|---|---|
| NATURAL Essentials (self-study course) | Stephen Paul Simpson | 1999 to 2000 | Free-to-read PDF at `spsimpson.com/nat-u/`. Three parts (concepts, syntax, a worked application) plus appendices on tools, naming conventions, and a quiz. Genuinely pedagogical, aimed at programmers already fluent in another language. Distribution requires a license per the author. The single best free long-form text, but 25+ years old and pre-NaturalONE |
| Vendor "Programming Guide" (not a book per se) | Software AG | current | The de facto textbook. Free, current, thorough, but reference-toned |

Beyond these, there are vendor manuals reposted on Scribd and Course Hero
(for example "Programming Guide Natural and Adabas", "adabas-natural complete
manual"), which are unofficial mirrors of Software AG material rather than
independent books. No current, independently authored, ISBN-bearing Natural
textbook surfaced.

---

## 4. Video and online courses

### Free video (YouTube and vendor)

| Resource | Source | Notes |
|---|---|---|
| "NaturalONE Tutorial - First Steps" | YouTube (Software AG) | Downloads NaturalONE, walks the IDE |
| "Developing Your First Natural Application" | YouTube (Software AG) | Beginner app build |
| "Introduction to the Adabas & Natural Education Package" | YouTube (Software AG) | Overview of the education package (repo now archived) |
| "First Steps" | YouTube (Software AG) | Links the Adabas & Natural CE download |
| Natural Video Tutorials Playlist | `techcommunity.softwareag.com/t/natural-video-tutorials-playlist/258346` | Curated vendor playlist |
| Adabas & Natural Tutorials | `techcommunity.softwareag.com/t/adabas-natural-tutorials/311404` | 2025-era vendor tutorial index |

These are useful but fragmentary: short vendor demos, not a structured
beginner-to-competent curriculum, and not interactive.

### Vendor LMS (login-gated)

`learn.softwareag.com` hosts the official training catalog. Courses found:

- Natural Programming Fundamentals (307-73E), instructor-led
- Natural Programming Fundamentals - Self-paced (E307-73E)
- Natural Programming Basic (course id 1467)
- Adabas Basic / Adabas category (categoryid 44)

Access requires a Software AG / Partner Hub login; the course pages render
behind authentication, so content, pricing, and certification detail are not
publicly visible. The self-paced variant is described in the catalog as free,
but confirming that requires an account. This is a vendor channel, not an open
marketplace offering.

### Third-party instructor-led classroom training

| Provider | Course | Format | Price | Notes |
|---|---|---|---|---|
| Verhoef Training (UK) | Adabas Natural Programming | 3 days, on-site (virtual via Zoom available) | Quote on request | Design and code Natural Structured Mode against Adabas. Assumes prior programming experience. Covers Adabas fundamentals, Natural objects/statements, batch access, JCL, debugging, a case study |
| Nisa Trainings (India) | NATURAL ADABAS Training | Instructor-led, online | Quote on request | Corporate/individual live training |
| MaxMunus (India) | NATURAL ADABAS Training | Instructor-led, online | Quote on request | Corporate/individual live training |

All three are traditional, human-led, enterprise-oriented offerings. None is a
self-serve interactive course, and none targets the curious individual
beginner at consumer price points.

### Mainstream marketplaces (the gap, confirmed)

| Platform | Result |
|---|---|
| Udemy | No dedicated current Natural (Software AG) course found |
| Pluralsight | None found |
| Coursera | None found |
| LinkedIn Learning | None found |
| edX | A topic landing page for "Adabas" only, no actual course |

This is the clearest evidence of the gap. The platforms where a modern
learner looks first have no Natural course. A well-produced interactive
beginner course would be effectively uncontested on these channels.

---

## 5. Community

| Venue | URL | State (2026) |
|---|---|---|
| Software AG Tech Community (Adabas & Natural) | `techcommunity.softwareag.com/c/adabas-natural/` | The primary official community. The older `tech.forums.softwareag.com` now 301-redirects here, so this is the current canonical home. Free to read; posting requires a free account. Active through 2024 to 2025 with release-information posts (for example "Release information Oct. 2025"), event recordings, and video tutorial threads |
| Software AG Education Community | `education.softwareag.com/adabas-and-natural/tutorials` | Vendor education/tutorials hub. Reachable but had connection instability during this spike; content is public-facing |
| Software AG Developer Center | `developer.softwareag.com/en/Natural.html` | Downloads and getting-started hub |
| Stack Overflow | tags such as `natural-adabas`, `adabas`, `software-ag` | Low volume. Stack Overflow could not be fetched directly during this spike (blocked), but general search shows the community's Q&A gravity is on the vendor Tech Community forums, not Stack Overflow. Do not expect a rich SO tag to lean on |
| Reddit | no dedicated active subreddit | No meaningful Natural-specific community found. Mainframe discussion is scattered in general mainframe subs |
| zMainframes and Jazz.net forums | `zmainframes.com`, `jazz.net` | Occasional legacy threads, not active hubs |

Net: the community is real but concentrated on the vendor's own Tech
Community. There is no large independent hobbyist community, which reinforces
that learning materials are vendor-driven and sparse.

---

## 6. Sample code and open corpora

This is the strongest area for course production. Usable, openly licensed
Natural source is available.

| Repo / source | What | License | State | Teaching value |
|---|---|---|---|---|
| `github.com/SoftwareAG/adabas-natural-code-samples` | Community-contributed Natural snippets: arrays, dynamic variables, date/time, string handling, READ/HISTOGRAM database ops, REDEFINE, CSV, and 60+ categorized folders | Apache-2.0 | Active, ~27 stars, ~21 forks, ~138 commits, not archived | High. A ready pattern library for lesson examples. "As-is, no support" |
| `github.com/SoftwareAG/adabas-natural-education-package` | Four progressive tutorials (Hello World / WRITE, DECIDE ON conditionals, 2D arrays, Adabas retrieval with REPEAT/ESCAPE) plus CSV files and HTML cheatsheets | Apache-2.0 | Archived Jan 2023, read-only, ~13 stars | Moderate. Good structure to mine, but references a "CRUISE" Adabas file rather than EMPLOYEES, and is unmaintained. Contact was UniversityTech@softwareag.com |
| `github.com/SoftwareAG/adabas-natural-devops-sample-application` | A sample Natural application for testing DevOps/CI approaches | Software AG repo | Present | Useful as a larger realistic codebase example |
| `github.com/MarkusAmshove/natls` | Modern open-source Natural toolchain: language server (LSP), parser (`natparse`), linter (`natlint`), SonarQube plugin (`natqube`) | MIT | Actively maintained, v0.18 released 2026-01-12, ~2,000+ commits | High for tooling. Enables editor support (VS Code, Neovim) and static analysis. The one genuinely current open-source Natural project. Parser is incomplete for some reporting-mode syntax |
| `github.com/markusamshove/vscode-natural` (Marketplace: `markusamshove.vscode-natural`) | VS Code client for `natls` | Free | v0.18.0 (Nov 2024 on Marketplace), ~835 installs, 5 stars (1 review) | Editor integration for a course that uses VS Code. Needs a `.natural` config file (normally generated by NaturalONE) |
| `github.com/martindb/vscode-natural` | An earlier/alternate VS Code Natural extension | Free | Present | Secondary option |
| EMPLOYEES / VEHICLES demo files | Classic Natural demo DDMs and data, shipped with the product and the CE demo database | Vendor-bundled | Current | The canonical teaching dataset. Pair with the code-samples repo |
| Mohamad Mahmood, "Getting Started with Adabas & Natural" (Medium / Dev Genius) | A multi-part community blog series: CE Docker setup, Adabas REST, JSON representation of periodic groups and multiple values, NaturalONE intro, Natural DB programming fundamentals and intermediate, Natural AJAX | Free to read (Medium) | ~12 parts | The best free modern community walkthrough. Good reference for structuring a Docker-based hands-on path |

No government or public-sector open-source release of production Natural code
was found. Public agencies run large Natural estates, but their source is not
published. The SoftwareAG org repos and `natls` are the practical open corpora.

---

## Implications for the course (brief)

- The gap is real and specific: no interactive, self-serve, beginner Natural
  course exists on any mainstream platform. The course would be close to
  uncontested there.
- The reference material to build against is free, current, and open
  (documentation.softwareag.com, the Programming Guide, the CE with EMPLOYEES
  and VEHICLES). Accuracy checks have a solid primary source.
- A free runtime exists (Community Edition, Docker), but its "personal use
  only" license must be reconciled with any hosted terminal (VTT) delivery
  model. That licensing question belongs in the emulator/feasibility spike
  (05), not here, but flag it now.
- Open, Apache-2.0 code samples and the MIT-licensed `natls` toolchain mean
  teaching examples and editor tooling can be assembled without licensing
  friction.

---

## Sources

All URLs accessed 2026-07-19.

Official documentation and vendor sites:
- https://documentation.softwareag.com/natural/index.htm. Natural product documentation portal index (free). Accessed 2026-07-19.
- https://documentation.softwareag.com/natmf/9.2.3/en/webhelp/natmf-webhelp/rnotes_mf/rn-mf-over.htm. Natural for Mainframes 9.2.3 release notes, new webhelp portal (free, verified loads without login). Accessed 2026-07-19.
- https://documentation.softwareag.com/natural/nat911mf/overview.htm. Natural for Mainframes 9.1.1 overview (legacy portal path). Accessed 2026-07-19.
- https://documentation.softwareag.com/natural/nat913win/overview.htm. Natural for Windows 9.1.3 overview. Accessed 2026-07-19.
- https://documentation.softwareag.com/natural/nat913win/print/pg.pdf. Natural Programming Guide 9.1.3 PDF (Oct 2021). Accessed 2026-07-19.
- https://documentation.softwareag.com/natural/nat912unx/firststeps/fs-start.htm. "Getting Started with Natural" first-steps tutorial (UNIX 9.1.2). Accessed 2026-07-19.
- https://documentation.softwareag.com/natural/nat827mf/firststeps/fs-about.htm. First-steps tutorial (Mainframe 8.2.7). Accessed 2026-07-19.
- https://documentation.softwareag.com/naturalONE/natONE921/webhelp/one-webhelp/core/relnotes/rn-new921.htm. NaturalONE 9.2.1 what's new. Accessed 2026-07-19.
- https://developer.softwareag.com/en/Natural.html. Software AG Developer Center, Natural hub (TLS quirk on fetch; public). Accessed 2026-07-19.
- https://www.softwareag.com/en/developer/adabas-natural-community-edition/. Adabas & Natural Community Edition page. Accessed 2026-07-19.
- https://hub.docker.com/r/softwareag/adabas-ce. Adabas CE Docker image. Accessed 2026-07-19.
- https://softwareag-usa.s3.amazonaws.com/Adanat_Docker/AN+Community+Edition+Guide.pdf. A&N Community Edition Guide v1.3 (Oct 2024). Accessed 2026-07-19.

Ownership and corporate history:
- https://diginomica.com/software-ag-retrenches-aris-adabas-natural. Software AG retrenches to ARIS and Adabas & Natural (Silver Lake / Software GmbH). Accessed 2026-07-19.
- https://www.softwareag.com/en/blog/insights/adabas-natural-and-aris-launch-as-standalone/. Adabas & Natural and ARIS launch as standalone businesses. Accessed 2026-07-19.
- https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-.... Software GmbH standalone announcement. Accessed 2026-07-19.
- https://en.wikipedia.org/wiki/Software_AG. Software AG corporate history (Silver Lake acquisition, IBM webMethods/StreamSets). Accessed 2026-07-19.
- https://en.wikipedia.org/wiki/Rocket_Software. Rocket Software acquisitions (OpenText AMC, not Adabas & Natural). Accessed 2026-07-19.

Community:
- https://techcommunity.softwareag.com/. Software AG Tech Community (Adabas & Natural, CONNX). Accessed 2026-07-19.
- https://techcommunity.softwareag.com/c/adabas-natural/. Adabas-Natural community category; tech.forums.softwareag.com 301-redirects here. Accessed 2026-07-19.
- https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504. Release information Oct 2025 (community activity evidence). Accessed 2026-07-19.
- https://techcommunity.softwareag.com/t/natural-video-tutorials-playlist/258346. Natural video tutorials playlist. Accessed 2026-07-19.
- https://techcommunity.softwareag.com/t/adabas-natural-tutorials/311404. Adabas & Natural tutorials (2025). Accessed 2026-07-19.
- https://education.softwareag.com/adabas-and-natural/tutorials. Education Community tutorials (connection instability on fetch; public). Accessed 2026-07-19.

Courses and training:
- https://learn.softwareag.com/course/info.php?id=1467. Natural Programming Basic (login-gated). Accessed 2026-07-19.
- https://learn.softwareag.com/course/view.php?id=1426. Natural Programming Fundamentals self-paced E307-73E (login-gated). Accessed 2026-07-19.
- https://learn.softwareag.com/course/view.php?id=1423. Natural Programming Fundamentals 307-73E (login-gated). Accessed 2026-07-19.
- https://verhoef-training.co.uk/system-z-programming/adabas-natural-programming. Verhoef 3-day Adabas Natural Programming course. Accessed 2026-07-19.
- https://nisa-trainings.com/courses/natural-adabas-training/. Nisa Trainings NATURAL ADABAS. Accessed 2026-07-19.
- https://www.maxmunus.com/page/NATURAL-ADABAS-Training. MaxMunus NATURAL ADABAS. Accessed 2026-07-19.
- https://www.edx.org/learn/adabas. edX Adabas topic landing page (no actual course). Accessed 2026-07-19.

Books and self-study:
- http://spsimpson.com/nat-u/NATURAL%20Essentials.pdf. "NATURAL Essentials" self-study course, S. P. Simpson (1999 to 2000). Accessed 2026-07-19.
- http://spsimpson.com/nat-u/main.htm. Natural Essentials landing page. Accessed 2026-07-19.

Video:
- https://www.youtube.com/watch?v=-KmyxK7M1d4. NaturalONE Tutorial First Steps. Accessed 2026-07-19.
- https://www.youtube.com/watch?v=6hYl-W_4yko. Developing Your First Natural Application. Accessed 2026-07-19.
- https://www.youtube.com/watch?v=FSrJf91iZFU. Introduction to the Adabas & Natural Education Package. Accessed 2026-07-19.
- https://www.youtube.com/watch?v=5qIpsYg0poY. First Steps (CE download). Accessed 2026-07-19.

Sample code and tooling:
- https://github.com/SoftwareAG/adabas-natural-code-samples. Community code samples, Apache-2.0, active. Accessed 2026-07-19.
- https://github.com/SoftwareAG/adabas-natural-education-package. Education package, Apache-2.0, archived Jan 2023. Accessed 2026-07-19.
- https://github.com/SoftwareAG/adabas-natural-devops-sample-application. DevOps sample application. Accessed 2026-07-19.
- https://github.com/MarkusAmshove/natls. natls language server, parser, linter, MIT, v0.18 (2026-01-12), actively maintained. Accessed 2026-07-19.
- https://marketplace.visualstudio.com/items?itemName=markusamshove.vscode-natural. VS Code Natural extension (natls client), free, ~835 installs. Accessed 2026-07-19.
- https://github.com/martindb/vscode-natural. Alternate VS Code Natural extension. Accessed 2026-07-19.
- https://medium.com/@mohamad.razzi.my/getting-started-with-adabas-natural-part-1-6597688406ad. Mohamad Mahmood, Getting Started with Adabas & Natural, Part 1 (of ~12). Accessed 2026-07-19.
- https://en.wikipedia.org/wiki/ADABAS. ADABAS overview and history. Accessed 2026-07-19.
