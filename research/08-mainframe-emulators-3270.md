# Mainframe Emulators, 3270 Terminal Technology, and What a Browser Natural Course Can Borrow

Spike date: 2026-07-31

## Executive summary

No open-source project gives us a meaningful head start on the *runtime*. Hercules is a
genuinely excellent, actively maintained, open-source S/370 through z/Architecture emulator,
and it is irrelevant to this course: it can legally run only 1970s-era IBM operating systems
(MVS 3.8j and friends), Natural has never been legally available for those systems, and
running a modern z/OS on Hercules is not licensable. Two independent legal walls stand
between us and "real Natural on an emulated mainframe," and neither has a door. The
Rust/WASM Natural-subset interpreter decision from spike 05 stands unchanged.

What open source *does* give us is a large head start on the **look and feel**, and this is
worth taking seriously because it is cheap.

Three assets are immediately usable and license-clean:

1. **The rbanffy 3270 webfont** (BSD-3-Clause and OFL-1.1-RFN, 1,981 stars, last activity
   2026-02-13). It descends from the x3270 font, which descends from Georgia Tech's
   3270tool, which was hand-copied from a physical 3270. It ships as woff2. Dropping this
   into xterm.js buys more authenticity per hour of work than anything else on this list.
2. **The 3270 screen model as documentation**, above all the x3270 wiki's data-stream and
   Operator Information Area pages (CC BY-SA 4.0) and the IBM 3270 Data Stream Programmer's
   Reference (GA23-0059-07) on bitsavers. These are precise enough to reimplement from.
3. **Two MIT-licensed "declare a screen as a list of fields" server libraries**,
   `racingmars/go3270` (Go) and `FuzzyMainframes/TN3270Sharp` (C#), as API design references
   for our Tier 2 map renderer.

A browser 3270 emulator does exist, in several forms, but none of them helps us. Every one
either needs a WebSocket-to-TCP bridge to reach a real host (browsers cannot open raw TCP
sockets) or is GPL-3.0, or both. `mflorence99/go-3270` is the only WebAssembly 3270 emulator
found, it is a one-star hobby archaeology project with no declared license, and it renders to
canvas rather than to a terminal. We have no host to talk to, so the protocol half of all of
these projects is dead weight for us. What we want is the *screen model*, not the *wire
protocol*, and that we reimplement.

**Recommendation, short form.** Yes, make the terminal look like a 3270 green screen: 3270
webfont, fixed 24x80 Model 2 grid, green-on-dark with an amber alternative, a subtle
opt-in CRT overlay written in our own CSS, and an Operator Information Area strip below the
grid. That package is roughly three to four days of work and it is the single highest
value-to-effort item in this spike. And yes, build a real map/screen renderer in Tier 2,
but implement the 3270 **field model** (fields, attribute bytes, protected/numeric/intensified/
hidden, the modified data tag, AID keys, Read Modified semantics) and explicitly **not** the
3270 data stream (SBA/SF/SFE orders, 12-bit buffer address encoding, EBCDIC, TN3270E
negotiation). The field model is what a Natural programmer using `INPUT USING MAP` actually
experiences; the data stream is invisible to them and would cost weeks to teach nothing.

Staying line-oriented through Tier 2 would gut the maps module. Maps are where Natural
developers spend their working lives, and the modified data tag, autoskip, and PF-key
handling are not learnable from prose.

---

## Project inventory

Licenses, versions, and activity verified against the GitHub API, the npm registry,
crates.io, and project sites on 2026-07-31.

### Emulators and operating environments

| Project | What it is | License | Current version | Last activity | Useful to us |
|---|---|---|---|---|---|
| [SDL-Hercules-390/hyperion](https://github.com/SDL-Hercules-390/hyperion) | The maintained Hercules 4.x: S/370, ESA/390, z/Architecture emulator in C | Q Public License 1.0 | Release_4.9.1 (2025-12-07) | Pushed 2026-07-11; 343 stars; ~11,600 commits | Reference only. Cannot run Natural. Backend-heavy. |
| [Hercules-Aethra/aethra](https://github.com/Hercules-Aethra/aethra) | Jay Maynard's experimental fork of Hyperion | Q Public License 1.0 | no tagged release | Pushed 2025-09-11; 33 stars | No. Lower activity than Hyperion. |
| hercules-390.org (Hercules 3.x) | The original Roger Bowler line | Q Public License 1.0 | 3.13 legacy | Effectively frozen; no GitHub repo at `hercules-390/hercules` | No. Superseded by Hyperion. |
| [MVS Turnkey 5 (TK5)](https://www.prince-webdesign.nl/tk5) | Ready-to-run MVS 3.8j + Hercules bundle by Rob Prins | No explicit license statement; free download | Update 5, dated 2026-02-18 | Actively updated | Reference only, for our own screenshots and understanding. |
| TK4- (Juergen Winkelmann) | Earlier MVS 3.8j turnkey, service level 8505 | Free redistribution, no formal OSI license | Update 08 plus patches | Host `wotho.ethz.ch` did not resolve during this spike; mirror at `wotho.pebble-beach.ch` | No. Superseded in practice by TK5. |
| TK4+ (TK4ROB) | Rob Prins' predecessor to TK5 | n/a | Withdrawn | No longer downloadable | No. |
| [MVS-sysgen/sysgen (MVS/CE)](https://github.com/MVS-sysgen/sysgen) | MVS Community Edition; automated Jay Moseley sysgen, rebuilt regularly | No LICENSE file declared | v2.1.5 (2026-07-13) | Pushed 2026-07-13; 85 stars | Reference only. Most current MVS 3.8j build. |
| [MVS-sysgen/docker-mvsce](https://github.com/MVS-sysgen/docker-mvsce) | Docker packaging of MVS/CE | No LICENSE file declared | rolling | Pushed 2026-03-03; 18 stars | Reference only. |
| [Jay Moseley's installMVS](https://www.jaymoseley.com/hercules/installMVS/iMVSintroV8.htm) | Tutorial for a from-tapes MVS 3.8j sysgen using the IBM MVS 3.7 starter system | Personal site, freely readable | v8 of the writeup | Long-standing, maintained | Reference only. Best explanation of what a sysgen is. |
| [joergschultzelutter/tk5-hercules](https://github.com/joergschultzelutter/tk5-hercules) | TK5 in an Alpine Docker image | Unlicense | rolling | Pushed 2025-06-15; 13 stars | Reference only. |

### 3270 emulators, clients, and libraries

| Project | What it is | License | Current version | Last activity | Useful to us |
|---|---|---|---|---|---|
| [pmattes/x3270](https://github.com/pmattes/x3270) | The x3270 family: x3270, c3270, s3270, b3270, wc3270, ws3270, wb3270, tcl3270, pr3287 | BSD-3-Clause | tag 4.5ga6, tagged 2026-07-27 | Pushed 2026-07-30; 68 stars; very active | **Medium.** BSD lets us vendor code. `b3270`'s JSON screen representation is a good design reference. We do not need the protocol. |
| [pmattes/Wx3270-New](https://github.com/pmattes/Wx3270-New) | Full Windows GUI 3270 front end driving b3270 | not declared | rolling | Pushed 2026-07-30; 17 stars | Low. Demonstrates the backend/frontend split. |
| [rbanffy/3270font](https://github.com/rbanffy/3270font) | Vector monospaced font derived from the x3270 font | BSD-3-Clause AND OFL-1.1-RFN | ships woff/woff2 | Pushed 2026-02-13; 1,981 stars | **Highest.** Adopt directly. |
| [zowe/tn3270-ng2](https://github.com/zowe/tn3270-ng2) | Zowe Desktop's browser 3270, Angular/TypeScript | EPL-2.0 | v2.18.5-RC1 (2026-06-22) | Pushed 2026-07-27; 19 stars | Low. Requires the Zowe App Server as a WebSocket-to-TCP bridge. Read, do not vendor. |
| [bencz/IronTerm](https://github.com/bencz/IronTerm) | Browser-side TN3270E and TN5250 in plain JS modules, static files, no server code | GPL-3.0 | rolling | Pushed 2026-05-21; 7 stars | Low. Closest thing to a client-side 3270 renderer, but still needs a websockify bridge, and GPL-3.0 rules out vendoring into a commercial course. |
| [bencz/web3270](https://github.com/bencz/web3270) | .NET 10 + SignalR + HTML5 canvas 3270 | GPL-3.0 | rolling | Pushed 2026-05-07; 3 stars | Low. Its README is an unusually good checklist of what a full data-stream implementation entails. |
| [mflorence99/go-3270](https://github.com/mflorence99/go-3270) | Go compiled to WebAssembly (`wasm_exec.js` present), renders 3270 into an HTML canvas via a requestAnimationFrame blit; Lit/Material front end | **No license file (all rights reserved)** | pre-release | Pushed 2026-02-11; 1 star | Low. The only WASM 3270 found. Architecturally interesting, legally unusable, explicitly a hobby project. |
| [moshix/web3270](https://github.com/moshix/web3270) | Single Go binary serving a browser 3270 client, points at a real TN3270 host | NOASSERTION | rolling | Pushed 2026-07-01; 3 stars | Low. |
| [3270io/3270Web](https://github.com/3270io/3270Web) | Go web UI that embeds `s3270`, plus session recording and REST API | No LICENSE file | rolling | Pushed 2026-07-22; 3 stars | Low. |
| [zpgu/WebTerminal](https://github.com/zpgu/WebTerminal) | Spring Boot + xterm.js serving SSH/Telnet/TN3270; drives `c3270` over a pty | GPL-3.0 | 0.9-SNAPSHOT | Pushed 2026-07-30; 31 stars | **Medium as proof of concept.** Demonstrates that a 3270 session renders acceptably inside xterm.js via c3270. |
| [h3270](https://h3270.sourceforge.net/) | Server-side Java rendering 3270 screens as pure HTML | LGPL | 1.3.3 | Last commit roughly 2013; inactive | Low. Historical interest in its HTML layout engine. |
| [AlanKrueger/freehost3270](https://github.com/AlanKrueger/freehost3270) | Java-applet 3270-to-web | LGPL-2.1 | dormant | Pushed 2011-07-18; 8 stars | No. Dead, applet-based. |
| [racingmars/go3270](https://github.com/racingmars/go3270) | Go library for *writing* 3270 server applications from field lists | MIT | pre-1.0 | Pushed 2026-01-02; 48 stars | **High as a design reference.** The cleanest field/screen API found. |
| [FuzzyMainframes/TN3270Sharp](https://github.com/FuzzyMainframes/TN3270Sharp) | .NET equivalent of go3270 | MIT | pre-release, self-described unstable | Pushed 2026-04-24; 9 stars | Medium as a second reference API. |
| [lowobservable/pytn3270](https://github.com/lowobservable/pytn3270) | Python TN3270 library | ISC | rolling | Pushed 2025-09-06; 34 stars | Medium. Small, readable emulation core. |
| [lowobservable/oec](https://github.com/lowobservable/oec) | IBM 3174 controller replacement driving real 3278/3279 hardware | ISC | rolling | Pushed 2026-02-08; 64 stars | Low, but the most authoritative "how the hardware actually behaves" code found. |
| [IBM/tnz](https://github.com/IBM/tnz) | IBM's Python 3270 automation and terminal library | Apache-2.0 | rolling | Pushed 2026-07-23; 90 stars | Medium. Vendor-blessed, permissive, readable screen model. |
| [PerryWerneck/lib3270](https://github.com/PerryWerneck/lib3270) / [pw3270](https://github.com/PerryWerneck/pw3270) | GTK 3270 emulator and its extracted core library | LGPL-3.0 | rolling | Pushed 2025-11-28 / 2026-05-18; 13 / 81 stars | Low. LGPL is workable for dynamic linking but pointless for us. |
| [dmolony/dm3270](https://github.com/dmolony/dm3270) | Java tn3270 emulator | Apache-2.0 | dormant | Pushed 2020-02-13; 59 stars | Low. Frequently cited as a readable data-stream implementation. |
| [Open3270/Open3270](https://github.com/open3270/open3270) | C# 3270 library | MIT | dormant | Pushed 2021-04-30; 57 stars | Low. |
| [downarowiczd/rust3270](https://github.com/downarowiczd/rust3270) | 3270 terminal server protocol implementation in Rust | MIT | 0.1.1 (2025-06-19), 873 downloads | Pushed 2026-02-03; 1 star | Low, but it is the only Rust 3270 code in existence. Read it; do not depend on it. |
| [thorhs/mfform](https://github.com/thorhs/mfform) | Rust library for "input forms simulating mainframe input forms, think 3270" | MIT | 0.4.0 (2024-07-12) | Pushed 2024-07-12; 0 stars | Low. Tiny, but conceptually the closest existing Rust code to our Tier 2 need. |

### Terminal, aesthetics, and teaching platforms

| Project | What it is | License | Current version | Last activity | Useful to us |
|---|---|---|---|---|---|
| [@xterm/xterm](https://github.com/xtermjs/xterm.js) | The web terminal component | MIT | 6.0.0 (published 2025-12-22) | Registry entry modified 2026-07-27; very active | Already the chosen stack. Supports a fixed cols/rows grid. |
| [remojansen/cool-retro-term-webgl](https://github.com/remojansen/cool-retro-term-webgl) (npm `cool-retro-term-renderer`) | WebGL CRT renderer for xterm.js: curvature, phosphor glow, scanlines | **GPL-3.0** | 1.0.1 (2025-12-28) | Published 2025-12-28 | **Do not link.** GPL-3.0 is incompatible with a proprietary course bundle. Look at it for the effect vocabulary, then write our own CSS. |
| [openmainframeproject/cobol-programming-course](https://github.com/openmainframeproject/cobol-programming-course) | Open Mainframe Project's "Getting Started" COBOL course | CC-BY-4.0 | rolling | Pushed 2026-06-05; 3,606 stars | **High as a positioning benchmark.** Courseware, not a runtime; still needs a real z/OS. |
| [IBM Z Xplore](https://www.ibm.com/products/z/resources/mainframe-skills) | IBM's free, always-on, badge-based challenge platform against live z/OS | Proprietary, free to use | ongoing | Successor to Master the Mainframe | Reference for pedagogy and for what "authentic" sets as a bar. Not reusable. |
| GnuCOBOL to WASM (via Emscripten) | COBOL compiled to C, then to WebAssembly; used by Cloudflare Workers and documented by Fermyon | GnuCOBOL is GPL/LGPL | ongoing | ongoing | **Medium as precedent.** Proves the "mainframe-adjacent language in the browser" shape works. Gives us nothing for Natural. |
| [FuzzyMainframes/Awesome-Mainframes](https://github.com/FuzzyMainframes/Awesome-Mainframes) | Curated index of mainframe projects | No license declared | rolling | Pushed 2026-04-24; 114 stars | Useful discovery index. |

---

## 1. Hercules

### What it is and who maintains it

Hercules is an open-source software implementation of the IBM System/370 and ESA/390
architectures plus 64-bit z/Architecture, written almost entirely in C. Roger Bowler started
it in 1999. It runs on Linux, Windows, Solaris, FreeBSD, and macOS.

There have been three lines:

- **Hercules 3.x**, the original `hercules-390.org` line. Effectively frozen. There is no
  live GitHub repository at `hercules-390/hercules` (the API returns 404).
- **SDL Hercules 4.x "Hyperion"**, maintained by David "Fish" Trout at SoftDevLabs. This is
  the current line and the only one under real development. Latest tagged release is
  `Release_4.9.1`, published 2025-12-07, preceded by 4.9 on 2025-10-13 and 4.8 on 2025-03-27.
  The repository was last pushed 2026-07-11 and carries about 11,600 commits and 343 stars.
- **Hercules Aethra**, Jay Maynard's fork of Hyperion, carrying Fish's fixes plus
  experimental work. 33 stars, last pushed 2025-09-11, no tagged release. Lower activity
  than Hyperion.

Practical read: if you want Hercules in 2026, you want SDL Hyperion.

### License

The `COPYRIGHT` file in the Hyperion repository states plainly that all materials are
copyrighted by Roger Bowler and others and that Hercules may be distributed under the terms of
the **Q Public License Version 1.0**, with Roger Bowler, Jan Jaeger, and Jay Maynard named as
the initial developers. QPL 1.0 is OSI-approved. It is also a copyleft-ish license that is
GPL-incompatible, which is why Hercules does not appear in some distributions' main
repositories. GitHub reports the license as `NOASSERTION` because it does not auto-detect QPL.

The repository also bundles separate licenses for vendored components under `LICENSES/`
(`crypto`, `decNumber` under an ICU license, `softfloat`, `telnet`).

### The critical distinction: emulator versus operating system

This distinction is the whole legal story, and it is regularly muddled in blog posts.

**The emulator is open source.** Anyone can download, build, modify, and redistribute
Hercules under the QPL. There is no legal difficulty here at all.

**The operating systems are separately licensed, and mostly not free.**

- **MVS 3.8j (OS/VS2 Release 3.8) is the one that is freely usable.** IBM distributed it
  without a license fee, it was orderable from IBM as a no-charge product, and it is
  conventionally described as public domain in the United States. The Hercules FAQ and the
  community are careful about the nuance: outside the United States, "public domain" is not
  always a legal category, so the more defensible description is "copyrighted software
  provided at no charge, which IBM has never sought to restrict." The same bucket holds
  **VM/370 Release 6** and **DOS/VS Release 34**.
- **Everything modern is licensed to a machine.** OS/390, z/OS, z/VM, z/VSE, and any other
  ESA/390 or z/Architecture operating system are licensed to a specific machine. The Hercules
  FAQ states directly that in practice you cannot run any classic ESA or z/Architecture
  operating system on a PC unless you obtain a license from IBM allowing you to do so, and
  IBM does not issue such licenses for Hercules.
- **IBM's own sanctioned emulated z/OS exists and is paid.** IBM Z Development and Test
  Environment (ZD&T) and Wazi as a Service run z/OS on x86 or in IBM Cloud under commercial
  license. That is the only legitimate route to an emulated modern z/OS, and it is not free.

So Hercules gives us a legal, fully open, high-fidelity 1970s mainframe. It gives us no route
whatsoever to a 2026 one.

---

## 2. Turnkey distributions

A "turnkey" is a pre-built MVS 3.8j system packaged with Hercules configuration files, DASD
volumes, and a set of community tools, so that a newcomer can go from download to a TSO logon
in minutes instead of performing a multi-day system generation.

### MVS Turnkey 5 (TK5)

Rob Prins' TK5 is the current recommended starting point. As of this spike, **Update 5 is
current and is dated 2026-02-18**, and it is cumulative (it supersedes updates 1 through 4).

The complete package ships:

- Hyperion Hercules SDL 4.9.1 for 64-bit Windows and Linux (other platforms get 4.3.99999)
- MVS 3.8J itself
- 15 DASD volumes, mostly 3390 type, down from 28 in TK4+
- ISPF 2.2, BREXX370, RPF, editors, and a broad utility set
- HTTPD 3.3.0 web server and an FTP daemon
- Documentation, including a migration path from TK3 and TK4 systems

The download is free. The site carries no explicit open-source license statement, which is
normal for this ecosystem: the components have heterogeneous provenance (IBM no-charge
software, CBT Tape contributions, individual authors' utilities) and the practical convention
is free redistribution without a single unifying license. That ambiguity is one more reason
not to ship any of it inside a commercial course.

### TK4- and TK4+

Two different things with confusingly similar names.

- **TK4-** is Juergen Winkelmann's system, based on MVS 3.8j service level 8505, historically
  hosted at `wotho.ethz.ch/tk4-/` with a mirror at `wotho.pebble-beach.ch/tk4-/`. The primary
  host did not resolve from this machine during the spike, and the community has long noted
  that the site goes offline for extended stretches. It is still widely mirrored and
  Dockerized.
- **TK4+ (also called TK4ROB)** was Rob Prins' evolution of TK4-. It has been superseded by
  TK5 and is no longer downloadable from the author's site.

### MVS/CE (MVS Community Edition)

`MVS-sysgen/sysgen` on GitHub is the most current MVS 3.8j build available. It automates Jay
Moseley's sysgen procedure and is rebuilt regularly, so it tracks community fixes more
closely than a hand-assembled turnkey. Latest release `v2.1.5` was published 2026-07-13, the
repo was last pushed the same day, 85 stars. There is **no LICENSE file** in the repository,
so its formal status is the same informal free-redistribution convention as TK5. A Docker
image (`mainframed767/mvsce`) is the recommended way to run it, exposing ports for FTP,
telnet, 3270, card reader, punch, and a web interface.

### Jay Moseley's build

Jay Moseley's `installMVS` writeup is the from-scratch path: it walks through performing a
real system generation of MVS 3.8j starting from the IBM-provided MVS 3.7 starter system and
the distribution tapes. It is the reference that MVS/CE automates, and it remains the best
explanation anywhere of what a sysgen is and why anyone would do one.

### Usability for teaching

Honest assessment: TK5 or MVS/CE in Docker is a genuinely good teaching environment **for
JCL, TSO, ISPF, JES2, assembler, COBOL, and PL/I on a 1970s system**. It boots in seconds, it
is free, it is well documented, and there is an active community. If this were a JCL course,
this spike would end with "ship TK5 in a container."

It is the wrong tool for a Natural course for four separate reasons: it cannot run Natural
(section 3), it requires a per-student backend container (rejected in spike 05 on cost and
operational grounds), its licensing is informal enough that bundling it into a paid product is
uncomfortable, and MVS 3.8j's TSO is so far from a 2026 Natural developer's environment that
it would teach the wrong mental model.

---

## 3. Could Hercules or any emulator run Natural?

**No. There is no legal path, and the technical path is also blocked.** Two independent walls,
either of which alone is sufficient.

### Wall 1: Natural requires an operating system that cannot be legally emulated

Natural for Mainframes is a current, commercially licensed product. The vendor's own
documentation portal shows Natural for Mainframes at version 9.2.x, with the z/OS installation
guide for 9.2.2 dated June 2026 and 9.2.4 operations documentation dated March 2026. It
installs on **z/OS**, with sibling editions for **z/VSE** and **BS2000**, and it runs under
the TP monitors CICS, IMS TM, TSO, Com-plete, TIAM, and openUTM, plus batch.

Not one of those operating systems can be legally run on Hercules. z/OS, z/VSE, and z/VM are
licensed to specific machines, and IBM does not license them for Hercules. The only sanctioned
emulated z/OS is IBM's paid ZD&T or Wazi. So even before Software AG enters the picture, the
host platform is unavailable.

### Wall 2: Natural itself requires a license, and no free-for-commercial edition exists

Established in spike 05 and unchanged: the free Adabas and Natural Community Edition is
Docker-based, current (image 9.3.3 as of July 2026), and licensed **for personal use only**,
with commercial production use explicitly prohibited. Hosting it as the backend of a paid
course is outside that license. The paid "Natural for Open Systems" is legally clean but
carries per-seat cost and a single-vendor dependency on a Silver Lake owned standalone.

### What about an old Natural that ran on MVS 3.8?

The obvious loophole is worth closing explicitly. Natural 1.0 dates from 1979 and early
Natural releases did run on MVS-era systems contemporary with 3.8. That does not create a
path:

- Software AG has never released any Natural version into the public domain or under any free
  license. Every version remains under copyright and under the same commercial license model.
- No Natural distribution tapes circulate the way IBM's no-charge MVS materials do. Natural
  was never a no-charge IBM product; it was always a priced third-party product.
- Obtaining and running a 1980s Natural tape without a license would be straightforward
  copyright infringement, and it would additionally require an Adabas license for anything
  touching data.
- Even if all of that were solved, teaching Natural 1.x semantics in 2026 would be
  professionally useless.

**Verdict, stated plainly for the course spec: there is no legal way to put real Software AG
Natural on an emulated mainframe for this course. The custom Rust/WASM interpreter is not a
compromise chosen for convenience; it is the only lawful option that does not require paid
per-seat vendor licensing.** This is a point worth making in the course marketing, not hiding.

---

## 4. 3270 terminal emulation

### The x3270 family

The reference implementation, and by a wide margin the most active. Paul Mattes has maintained
it since 1993.

- **Repository:** `pmattes/x3270` on GitHub, 68 stars, last pushed 2026-07-30. Releases are
  cut as git tags rather than GitHub Releases.
- **Current version:** tag `4.5ga6`, tagged 2026-07-27. The prior GA was `4.5ga5` on
  2025-12-23 and `4.4ga6` on 2025-04-25. This is a genuinely active project, not a
  maintenance-mode one.
- **License:** BSD 3-Clause. The `LICENSE.md` file carries the standard three-clause text with
  copyrights from Paul Mattes (1993 to 2026), Don Russell, Dick Altenbern, Jeff Sparkes, and
  Georgia Tech Research Corporation. **This is a permissive license. We could legally vendor
  x3270 code into a proprietary product with attribution.**

Components in the repository: `x3270` (X11 GUI), `c3270` (curses), `s3270` (scripting, no
display), `b3270` (protocol back end), `wc3270` / `ws3270` / `wb3270` (Windows equivalents),
`tcl3270`, `pr3287` (printer), plus `playback`, `mitm`, and `st-relay` test tooling. The
Windows GUI `wx3270` lives in a separate repo, `pmattes/Wx3270-New` (C#).

**`b3270` is the architecturally interesting piece.** It is a generic back end that implements
the 3270 protocol and host I/O and communicates with any front end over a simple XML- or
JSON-based protocol. `wx3270` is built on it. If anyone ever wanted a browser 3270, compiling
`b3270` to WebAssembly and driving it from JavaScript would be the obvious design. Nobody has
done it: searching for x3270-to-WASM, b3270-to-Emscripten, and 3270-in-WebAssembly produced
no such project.

### The TN3270 and TN3270E protocol

TN3270 is 3270 over Telnet. The relevant RFCs:

- **RFC 1041**, Telnet 3270 Regime Option, an early standardization attempt that TN3270E is
  explicitly unrelated to.
- **RFC 1576**, TN3270 Current Practices, which documents the de facto standard: negotiate
  three Telnet options (Terminal-Type, Binary Transmission, End of Record) and then exchange
  3270 data streams as Telnet records.
- **RFC 2355**, TN3270 Enhancements (TN3270E). Adds printer emulation, client-requested
  device/LU names, and support for the ATTN and SYSREQ keys plus SNA response handling. It
  obsoleted RFC 1647.

Related: RFC 1646, RFC 2561, RFC 2562, RFC 3049.

**None of this matters to us.** We have no host. Our interpreter and our screen live in the
same WASM module in the same tab. Implementing TN3270E would be pure ceremony.

### Browser-based and WebAssembly 3270 emulators

This was the highest-priority search, and the answer is nuanced.

**The structural constraint.** Browsers cannot open raw TCP sockets. TN3270 is a raw TCP
protocol. Therefore **every** browser 3270 that talks to a real host requires a server-side
WebSocket-to-TCP bridge (websockify, or a purpose-built app server). Zowe's own documentation
states this directly: the terminal plug-in does not connect to the TN3270 server, the Zowe
Application Server acts as a bridge over WebSockets. This is not a gap anyone can close; it is
a browser security boundary.

**What exists:**

| Project | Where the 3270 logic runs | Rendering | Needs a bridge | License |
|---|---|---|---|---|
| `zowe/tn3270-ng2` | Browser (Angular/TypeScript) | DOM | Yes, Zowe App Server | EPL-2.0 |
| `bencz/IronTerm` | Browser (plain JS modules) | DOM/canvas | Yes, websockify | GPL-3.0 |
| `bencz/web3270` | Server (.NET 10), pushes screen snapshots | HTML5 canvas via SignalR | Server is the bridge | GPL-3.0 |
| `mflorence99/go-3270` | **Browser, Go compiled to WebAssembly** | HTML canvas, Go `gg` drawing into a device context, blitted via requestAnimationFrame | Yes, TS handles telnet negotiation and WebSocket | **None declared** |
| `moshix/web3270` | Server (Go single binary) | Browser client | Server is the bridge | NOASSERTION |
| `3270io/3270Web` | Server (Go), embeds `s3270` | Browser client | Server is the bridge | None declared |
| `zpgu/WebTerminal` | Server (Java), drives `c3270` over a pty | **xterm.js** | Server is the bridge | GPL-3.0 |
| `h3270` | Server (Java) | Pure HTML, configurable layout engine | Server is the bridge | LGPL, inactive since ~2013 |
| `FreeHost3270` | Java applet | Applet | Yes | LGPL-2.1, dead since 2011 |

**`mflorence99/go-3270` is the only WebAssembly 3270 emulator in existence that this search
found.** Confirmed WASM by the presence of `src/client/assets/wasm_exec.js` (the Go WASM
runtime shim) alongside `src/emulator/main.go`. Its architecture is instructive: a screen
buffer of cells and fields mirroring the hardware, a glyph cache for fast rendering, drawing
into an off-screen device context, blitted into a `<canvas>`. The author is explicit that it
exists "for fun, as a voyage through computer archaeology" and that he does not expect anyone
to use it. It has one star, no license file (which means all rights reserved), and it is
incomplete against GA23-0059-07.

**`zpgu/WebTerminal` is the most interesting existence proof for our purposes.** We would
never ship it, but it demonstrates that a real 3270 session driven through `c3270` renders
acceptably inside xterm.js. That is evidence for the "keep xterm.js and drive it with absolute
positioning" option in section 8.

### Server-side libraries worth reading

For our Tier 2 renderer, the most valuable references are libraries that let you **write** a
3270 application by declaring fields, because that is the abstraction level a Natural map
sits at:

- **`racingmars/go3270`** (MIT, 48 stars, pushed 2026-01-02). Builds 3270 data streams from
  field lists and processes the client response into attention keys plus field values. Its
  `RunTransactions()` model for passing control screen to screen maps closely onto how a
  Natural program flows between maps. Moshix's Minesweeper is a worked example built on it.
- **`FuzzyMainframes/TN3270Sharp`** (MIT, 9 stars, pushed 2026-04-24). Explicitly modeled on
  go3270 and self-described as unstable, but a useful second opinion on API shape.
- **`IBM/tnz`** (Apache-2.0, 90 stars, pushed 2026-07-23). Vendor-authored, permissive, and
  actively maintained, with a clean Python screen model.
- **`lowobservable/pytn3270`** (ISC, 34 stars) and **`lowobservable/oec`** (ISC, 64 stars).
  `oec` is a 3174 controller replacement that drives physical 3278/3279 terminals over coax.
  When a behavioral question has no clear answer in the manuals, this is the code that has
  been tested against real hardware.

### Rust

Almost nothing exists.

- **`rust3270`** (MIT, v0.1.1 published 2025-06-19, 873 downloads, 1 star,
  `downarowiczd/rust3270` last pushed 2026-02-03) is a 3270 terminal *server* protocol
  implementation. It is the only Rust 3270 code found. Read it for how one person chose to
  encode fields and attributes in Rust; do not take a dependency on a 0.1.1 crate with one
  star.
- **`mfform` / `mfform-lib`** (MIT, v0.4.0, 2024-07-12, `thorhs/mfform`) is "a simple input
  form simulating mainframe input forms, think 3270." Conceptually it is the closest existing
  Rust code to our Tier 2 requirement, and it is 400 lines of hobby project.
- **`rs3270`** (v0.1.2, 2023) merely shells out to the `x3270` client.

Conclusion: our Tier 2 field model is greenfield Rust, same as the interpreter. That is fine.
The model is small; the documentation is excellent.

---

## 5. What a 3270 screen actually is

This is the material we would reimplement. It is documented well enough to build from without
touching anyone's code.

### Models and dimensions

Two display types and four sizes.

| Type | Description |
|---|---|
| 3278 | Monochrome, the green screen |
| 3279 | Color |

| Model | Rows | Columns | Buffer positions |
|---|---|---|---|
| 2 | 24 | 80 | 1,920 |
| 3 | 32 | 80 | 2,560 |
| 4 | 43 | 80 | 3,440 |
| 5 | 27 | 132 | 3,564 |

A `-E` suffix indicates support for the 3270 Extended Data Stream. A full model string looks
like `3279-4-E` (color, 43 by 80, extended) or `3278-2` (monochrome, 24 by 80, no extension).

Models 3, 4, and 5 also implicitly emulate a Model 2, which is their **default screen size**.
An Erase/Write command sets the terminal to the default size; an Erase/Write Alternate command
sets it to the alternate (the larger size in the table). Some hosts allow oversize screens
beyond the alternate size, negotiated through structured fields.

**This matters directly to Natural.** Natural's own `TMODEL` parameter takes exactly these
model numbers and documents exactly these dimensions, with `0` meaning "let the
environment-dependent driver decide," and a `(lines,cols)` form for NWO server terminals
supporting 24 to 250 lines and 80 to 250 columns. When we say "the course terminal is a Model
2," we are using the vendor's own vocabulary.

### The screen buffer

The display is a visual representation of a **screen buffer**: an array of characters with
attributes, one buffer position per screen position, in EBCDIC. The first 80 positions are row
1, the next 80 are row 2, and so on for a Model 2.

Buffer addressing has a historical quirk worth knowing but not reproducing. Because the
original BSC protocol required transparency, addresses could not use binary values below
x'40'. So **12-bit addressing** splits an address into two 6-bit halves and encodes each into
an EBCDIC-displayable byte, costing two bytes per address. **14-bit addressing** extends this
for buffers larger than 4,096 positions.

A buffer containing at least one field is **formatted**. A buffer with no fields is
**unformatted**.

### Fields and the attribute byte

The host programs the buffer with **fields** using a Start Field (SF, x'1D') or Start Field
Extended (SFE, x'29') order.

Three facts that surprise people and that a course must teach explicitly:

1. **The attribute byte occupies a screen position.** It consumes one of the 1,920 cells and
   displays as a blank. This is why 3270 screens have a leading space before every entry field
   and why field layout arithmetic always seems off by one.
2. **A field runs from its Start Field order to the next Start Field order**, scanning right,
   wrapping from end of row to start of next row, and wrapping from the bottom-right corner
   back to the top-left. A buffer with a single SF has one field covering the whole screen.
3. **The attribute byte itself cannot be modified by the operator.**

The standard field attribute byte:

| Bit | Meaning |
|---|---|
| 0 to 1 | Not independently set; derived from the other bits so the byte lands on a valid graphic character |
| 2 | Protected. 1 = protected (output only), 0 = unprotected (operator may type) |
| 3 | Numeric. 1 = numeric only |
| 4 to 5 | Display mode: `00` normal and not light-pen detectable, `01` normal and detectable, `10` intensified and detectable, `11` non-display (hidden) |
| 6 | Reserved, set to 0 |
| 7 | Modified Data Tag (MDT) |

Two derived behaviors matter pedagogically:

- **Protected plus numeric equals autoskip.** The cursor jumps past the field without
  stopping. This is how 3270 screens implement labels and separators that the operator tabs
  straight through.
- **Non-display (`11`) is how password fields work.** The data is in the buffer and is
  transmitted; it simply is not drawn.

**The Modified Data Tag is the heart of the model.** The MDT is set automatically when the
operator changes a field, and it can be pre-set by the host. When the operator presses a key
that reads the screen, only fields with MDT set are transmitted. This is a 1970s bandwidth
optimization that became a programming idiom: it is why a Natural map returns only what
changed, and why pre-setting MDT is the standard trick for forcing a field to come back
unchanged.

### Extended attributes

SFE, Modify Field (MF), and Set Attribute (SA) carry extended attributes on capable
terminals:

- **Color**, codes x'F0' to x'FF', with 1 through 7 mapping to blue, red, pink, green,
  turquoise, yellow, white. Most terminals support no more than these seven.
- **Highlighting**: blink, reverse video, underscore, plus intensify.
- **Character set / code page**, including the Graphic Escape (GE) mechanism for switching to
  a secondary code page for line-drawing and APL characters.

SA applies attributes at the character level, overriding the field default. That is
the mechanism behind highlighting a single word inside a field.

### Orders and commands

**Orders** (embedded in the data stream, each occupying a buffer position and generally
displayed as a blank):

| Order | Meaning |
|---|---|
| SF (x'1D') | Start Field |
| SFE (x'29') | Start Field Extended |
| SBA | Set Buffer Address |
| SA | Set Attribute |
| MF | Modify Field |
| IC | Insert Cursor |
| PT | Program Tab |
| RA | Repeat to Address |
| EUA | Erase Unprotected to Address |
| GE | Graphic Escape |
| NUL (x'00'), FF (x'0C'), SO (x'0E'), SI (x'0F'), NL (x'15'), EM (x'19') | Format control, mostly printing or DBCS |

**Host commands:** Write (W), Erase/Write (EW), Erase/Write Alternate (EWA), Erase All
Unprotected (EAU), Read Buffer (RB), Read Modified (RM), Read Modified All (RMA), Write
Structured Field (WSF).

**The Write Control Character (WCC)** rides with write commands and carries four operational
bits: start printer, sound alarm, keyboard restore (unlock the keyboard and reset the AID),
and reset all MDT bits.

### The AID model: PF, PA, CLEAR, ATTN

3270 terminals are **block mode**. The operator edits the screen locally with no host
interaction at all. Data goes to the host only when the operator presses a key that generates
an **Attention Identifier (AID)**.

| Key | AID | What is transmitted |
|---|---|---|
| ENTER | x'7D' | AID, cursor address, and all modified fields |
| PF1 to PF9 | x'F1' to x'F9' | Same as ENTER |
| PF10, PF11, PF12 | x'7A', x'7B', x'7C' | Same |
| PF13 to PF21 | x'C1' to x'C9' | Same |
| PF22, PF23, PF24 | x'4A', x'4B', x'4C' | Same |
| PA1 | x'6C' | **AID only.** No field data (a "short read") |
| PA2 | x'6E' | AID only |
| PA3 | x'6B' | AID only |
| CLEAR | x'6D' | AID only, and the terminal erases its own buffer and reverts to the default screen size |
| Structured field reply | x'88' | Structured field data |

(The x3270 documentation references PA1 through PA4; the commonly published AID table defines
PA1 through PA3, which is what real keyboards carried.)

Two behavioral rules to teach:

- **The keyboard locks the moment an AID key is pressed**, and stays locked until the host
  sends a command with the keyboard-restore bit set. It also locks on operator errors, such as
  typing into a protected field or overflowing a field. Only the Reset key (or the host)
  clears it. This is why "X SYSTEM" on the status line is the most familiar sight on a
  mainframe and why mainframe users develop the reflex of hitting Reset.
- **ATTN is not an AID.** It is an out-of-band interrupt. In TN3270 it maps to a Telnet-level
  signal (x3270's `Attn()` action sends Telnet BREAK), and TN3270E explicitly adds ATTN and
  SYSREQ support per RFC 2355. Functionally it is "interrupt the running host program," which
  under TSO is the attention interrupt. CLEAR wipes the screen; ATTN interrupts the program.
  Conflating them is a classic beginner error and worth a callout box.

### Insert mode

3270 terminals have an explicit insert mode. With it set, typed data is inserted into the
field and existing data shifts right (raising X Overflow if the field is full). With it unset,
typing overwrites. Insert mode is normally cleared automatically on reset, and its state is
shown in the Operator Information Area.

### The Operator Information Area

The OIA is a status line at the bottom of the display, **outside the 1,920-character buffer**.
On a Model 2 the buffer is rows 1 to 24 and the OIA sits below as a 25th line. Nothing the
host writes lands there; it is entirely the terminal's own report on its own state. That
distinction is worth teaching because it explains why the OIA never scrolls and why a program
cannot write to it.

The c3270 OIA, which is a good and precisely documented model to copy:

**Left, mode indicator:** `4 A ▪` for TN3270 3270 mode, `4 A N` for NVT, `4 B ▪` for TN3270E
3270 mode, `4 B ?` for TN3270E unbound, `4 B S` for TN3270E SSCP-LU.

**Message area, where `X` means a locked keyboard:**

| Text | Meaning |
|---|---|
| `X SYSTEM` | Command acknowledged, waiting for the host to unlock the keyboard |
| `X Wait` | Waiting for the host to unlock the keyboard |
| `X Protected` | Operator error: tried to modify a protected field |
| `X NUM` | Operator error: non-numeric character into a numeric field |
| `X Overflow` | Operator error: tried to insert into a full field |
| `X [TCP]`, `X [TELNET]`, `X [TN3270E]`, `X [DNS]`, `X [Proxy]` | Waiting on a connection or negotiation stage |
| `X [Field]` | Waiting for the host to format the screen |
| `X Scrolled n` | Display is scrolled back |
| `X -f` | Invalid AID in the current mode |

**Miscellaneous indicators (blank unless active):** `I` insert mode, `T` typeahead buffered,
`R` reverse-input mode, `P` printer session active, `S` secure session (green if the host is
verified, yellow if not), `C x` compose in progress, `s` script active, a digit or `+` for
screen tracing.

**Right:** the LU name, an optional timing figure for how long the emulator waited for the host
to respond to the last AID, and the **cursor position as row/column with `001/001` at the upper
left**.

For our purposes the reusable subset is small and high-impact: a mode indicator, the
`X`-plus-message area, the insert-mode `I`, and the cursor row/column readout.

---

## 6. Other teaching emulators to model after

### IBM Z Xplore

IBM's free, always-on, challenge-based platform, and the successor to Master the Mainframe.
Available globally at no cost, year-round, self-paced, with badges earned across COBOL, RACF,
Linux on Z, CICS, Db2, and system utilities. Its distinguishing feature is that learners get
accounts on **real** IBM Z environments rather than simulations.

Not open source and not reusable. It is the relevant benchmark in two ways: it sets what
"authentic" means to a learner who has seen it, and it demonstrates that a free, high-quality,
real-iron alternative exists for adjacent skills. Our differentiator cannot be "you get a
mainframe"; it has to be "you learn Natural, which nobody else teaches interactively, with
zero setup."

### Open Mainframe Project

More than 20 projects and working groups, including Zowe, COBOL Programming Course, COBOL
Check, Polycephaly, Mainframe Open Education, GenevaERS, Feilong, Tessia, Zorow, CBT Tape,
ConsoleZ, Software Discovery Tool, TerseDecompress, ATOM, Ambitus, and ADE.

The directly relevant one is **`openmainframeproject/cobol-programming-course`** (CC-BY-4.0,
3,606 stars, last pushed 2026-06-05), a collaboration between American River College, IBM, and
IBM clients. It is courseware, not a runtime: it teaches COBOL using VS Code with the Zowe and
Z Open Editor extensions, running against a real z/OS. It is the closest structural analogue to
what we are building, and its limitation is exactly the gap we fill. A learner without a
mainframe account cannot complete it.

Two things worth borrowing from it: the CC-BY-4.0 courseware-as-a-repo model (their star count
is itself a distribution channel), and the chapter structure, which sequences language
fundamentals before environment mechanics.

### COBOL in the browser and GnuCOBOL to WASM

GnuCOBOL (GPL/LGPL, formerly OpenCOBOL, adopted into the GNU Project in 2013) compiles COBOL
to C. Emscripten then compiles that C to WebAssembly. Cloudflare shipped exactly this pipeline
to add COBOL support to Workers, and Fermyon documents COBOL-on-Wasm for Spin.

This is the strongest precedent for the shape of what we are doing: a legacy business language
executing in a browser sandbox with no mainframe involved. It validates the architecture and
gives us nothing directly reusable, because Natural has no open-source compiler to route
through. GnuCOBOL's existence is precisely what Natural lacks, and it is why our interpreter
has to be written rather than adapted.

### Interactive TSO/ISPF simulators

Searched for; effectively none exist as open-source browser projects. The community's answer
to "I want to practice ISPF" is uniformly "run TK5 or MVS/CE in Docker and connect c3270 to
it," with the paid alternatives being IBM Wazi, ZD&T, and Udemy courses that assume you have
access somewhere. There is no browser ISPF.

That is a genuine market observation, not just a negative search result: nobody has built a
zero-install, browser-native mainframe learning environment for any mainframe language. If our
Tier 1 lands well, the same engine shape generalizes.

### Moshix's Minesweeper

`moshix/minesweeper` (Go, no license declared, 3 stars) is a Minesweeper implementation for
3270 terminals built on `racingmars/go3270`, deliberately written as a community learning
resource for how to structure a 3270 application. Worth reading for how a modern developer
organizes screen flow, not for its code.

---

## 7. Green-screen aesthetics in xterm.js

### xterm.js can do a fixed 24x80 grid

Yes, directly. `new Terminal({ cols: 80, rows: 24 })` fixes the grid. The important part is
what you then do **not** do:

- Do not attach `@xterm/addon-fit` and do not call `fit()`. FitAddon exists to make the
  terminal track its container; we want the opposite.
- Set `scrollback: 0` so the display is a genuine fixed screen rather than a scrolling log.
  This is not cosmetic. A 3270 does not scroll, and letting the course terminal scroll teaches
  the wrong mental model from lesson one. It also makes Tier 2's full-screen maps behave.
- Size the container with CSS so the fixed grid is never clipped, and scale with a CSS
  transform or a font-size step rather than by changing `cols`/`rows`.
- Disable user resize handling entirely.

Current version: `@xterm/xterm` 6.0.0, MIT, published 2025-12-22, registry entry modified
2026-07-27. Reminder from `docs/gotchas-rust-wasm.md`: the package is scoped; the unscoped
`xterm` is deprecated.

### The font is the single biggest lever

**`rbanffy/3270font`.** BSD-3-Clause AND OFL-1.1-RFN. 1,981 stars, last pushed 2026-02-13.
Ships as a webfont in woff and woff2, plus TTF/OTF. It is a vector font derived from the x3270
bitmap font, which was translated from Georgia Tech's 3270tool, which was hand-copied from a
physical 3270 terminal. Provenance does not get better than that for this purpose.

Licensing note: OFL-1.1-**RFN** means Reserved Font Name. We can embed and redistribute the
font freely, including commercially. If we *modify* it, we must rename the modified font. We
are not modifying it, so this costs us nothing beyond keeping the license file in the bundle.

Practical: `@font-face` with the woff2, then set xterm's `fontFamily`. Verify the font loads
before constructing the Terminal (use `document.fonts.ready`), because xterm measures character
cell dimensions at construction and will lock in the fallback font's metrics otherwise. This is
a common and confusing bug.

### Color schemes

xterm.js accepts a full `ITheme`: `background`, `foreground`, `cursor`, `cursorAccent`,
`selectionBackground`, and the sixteen ANSI colors.

Three palettes worth shipping, with a toggle:

- **Green phosphor (P1).** The 3278 default. Foreground around `#33FF33` on a background around
  `#0A140A` or `#001100`. Pure `#00FF00` on pure `#000000` is what people expect but is harsh;
  pulling both slightly off the extremes reads better and fatigues less.
- **Amber (P3).** Foreground around `#FFB000` on a near-black background. Amber was the premium
  alternative on later terminals and many operators preferred it. Cheap to add, and it gives
  the course a second visual identity for free.
- **3279 color.** The seven-color IBM palette (blue, red, pink, green, turquoise, yellow,
  white) maps cleanly onto ANSI slots. This is the honest palette for Tier 2 maps, since real
  Natural maps on a 3279 use exactly these.

Set the cursor to a block, and consider a slow blink. The 3270 cursor was a solid underscore or
block, not a thin bar.

### CRT effects

Two routes.

**`remojansen/cool-retro-term-webgl`** (npm `cool-retro-term-renderer` 1.0.1, published
2025-12-28) is a WebGL CRT renderer for xterm.js with curvature, phosphor glow, and scanlines,
and it explicitly supports an 80x24 configuration. It is **GPL-3.0**. Linking it into a
proprietary course bundle would put the bundle under GPL-3.0. **Do not use it.** Its demo is
worth studying for the effect vocabulary.

**Write our own CSS.** The effect is a handful of lines and carries no license entanglement:

- **Scanlines:** an absolutely positioned overlay with
  `repeating-linear-gradient(to bottom, rgba(0,0,0,0.15) 0 1px, transparent 1px 3px)` and
  `pointer-events: none`.
- **Phosphor glow:** `text-shadow: 0 0 2px currentColor` on the terminal text, kept subtle.
  Heavy glow destroys legibility at small sizes.
- **Vignette:** a radial-gradient overlay darkening the corners.
- **Curvature:** either skip it or use a light `border-radius` plus inset shadow. Real barrel
  distortion needs WebGL or an SVG filter and is not worth it.
- **Flicker:** a slow low-amplitude opacity keyframe. Optional and off by default.

`@xterm/addon-webgl` is the performance renderer and does nothing aesthetic; it is orthogonal
to all of the above and worth enabling regardless.

**Accessibility constraint that must not be negotiated away.** Scanlines, glow, and flicker all
reduce contrast and legibility, and flicker can trigger motion sensitivity. Ship the CRT
overlay as an off-by-default or subtle-by-default toggle, respect
`prefers-reduced-motion: reduce` by disabling flicker entirely, and make sure the plain
green-on-dark scheme without effects passes contrast checks on its own. Learners will spend
hours reading this pane.

---

## 8. Recommendation

### Ranked by value against effort

**Tier A: do these now. Days of work, disproportionate return.**

| # | Item | Effort | Value |
|---|---|---|---|
| A1 | Adopt the `rbanffy/3270font` webfont as the terminal font | Hours | Highest. One change carries most of the authenticity. |
| A2 | Lock the terminal to a fixed 24x80 grid, `scrollback: 0`, no FitAddon; call it a Model 2 in the course text | Hours | High. Correct mental model from lesson one, and it uses Natural's own `TMODEL` vocabulary. |
| A3 | Green-on-dark theme with an amber alternative and a subtle, toggleable, self-authored CRT overlay | ~1 day | High. Avoids the GPL-3.0 trap entirely. |
| A4 | An Operator Information Area strip rendered as HTML **below** the 24x80 grid | 1 to 2 days | High. After the font, the OIA is the most recognizable 3270 signal, and it is a live teaching surface. |

On A4, ship this minimum set: a mode indicator on the left; a message area that shows `X` plus
text when input is inhibited; an `I` when insert mode is on; and the cursor position as
`row/col` with `001/001` at the top left. Reuse the c3270 vocabulary verbatim so that a learner
who later opens a real emulator recognizes it. Then use it: when the interpreter is running a
program, show `X SYSTEM`. When the learner types into a protected field in Tier 2, show
`X Protected`. The OIA turns invisible state into a teachable one, which is exactly what the
project's "errors are teaching surfaces" requirement asks for.

**Tier B: Tier 2 work. Weeks, and it is the difference between teaching maps and mentioning
them.**

| # | Item | Effort | Value |
|---|---|---|---|
| B1 | Implement the 3270 **field model** in the Rust interpreter: screen buffer of cells, fields with attribute bytes, protected/numeric/intensity/hidden, MDT, autoskip | 1 to 2 weeks | High. This is `INPUT USING MAP`. |
| B2 | Implement the AID key model: ENTER, PF1 to PF24, PA1 to PA3, CLEAR, plus keyboard lock and Reset semantics | 3 to 5 days | High. PF-key handling is core Natural. |
| B3 | Implement Read Modified semantics so only MDT-set fields return | Folded into B1 | High. Teaches the single most Natural-specific behavior of maps. |
| B4 | A full-screen map renderer (canvas or a DOM grid), sharing the theme and font with the xterm.js pane | 1 to 2 weeks | High. |
| B5 | A map definition format for lessons, modeled on `racingmars/go3270`'s field-list API | 2 to 3 days | Medium. Lets lesson authors write maps without touching Rust. |

**Explicitly out of scope for B, and this is the important boundary:** do not implement the
3270 **data stream**. No SBA/SF/SFE/RA/EUA byte orders, no 12-bit or 14-bit buffer address
encoding, no EBCDIC translation, no WCC bit packing, no structured fields, no Telnet, no
TN3270E negotiation. That work is several weeks, it is invisible to a Natural programmer, and
it teaches nothing that appears in any Natural source file. Implement the semantics the
programmer sees; skip the wire format they never see.

**Tier C: reference only. Read, do not integrate.**

| # | Item | Why |
|---|---|---|
| C1 | Run TK5 or MVS/CE in Docker locally, with c3270 | For our own understanding, and for authentic reference screenshots we can describe in prose. Never ships. |
| C2 | Read `b3270`'s JSON screen protocol | The best existing answer to "how do you represent a 3270 screen as structured data." BSD-3-Clause, so we could even vendor from it. |
| C3 | Read `racingmars/go3270` and `FuzzyMainframes/TN3270Sharp` (both MIT) | API design for B5. |
| C4 | Read `downarowiczd/rust3270` (MIT) and `thorhs/mfform` (MIT) | The only prior Rust art. Small enough to read in an afternoon. |
| C5 | Read the x3270 wiki data-stream and OIA pages, and GA23-0059-07 on bitsavers | Primary sources for section 5. Cite them in the course's own reference appendix. |

### Answering the two direct questions

**Should the course terminal look like a 3270 green screen? Yes, with an honesty requirement.**

Adopt the font, the fixed Model 2 grid, the green and amber palettes, and the OIA. This is
three to four days of work for the largest perceived-quality jump available anywhere in the
project.

The honesty requirement: do not dress a line-oriented terminal as a 3270 and let learners infer
they are on a mainframe. State plainly in the course, as the project's existing content-accuracy
rule already requires, that this is a teaching interpreter over sample data rendered on a
3270-style display. This is comfortable to say because it happens to be accurate about real
Natural too. A Natural program that only uses `WRITE` and `DISPLAY` produces line-oriented
report output, and on a real mainframe that output is displayed on a real 3270. Tier 1's
line-oriented model is not a simplification of how Natural works; it is how Natural works when
you have not written a map yet. Tier 2 then introduces maps as the thing that turns the screen
from a report into a form, which is the correct pedagogical arc regardless of implementation.

**Should Tier 2 implement a real map/screen renderer, or stay line-oriented? Implement it, at
the field-model level.**

Staying line-oriented in Tier 2 would hollow out the module. Maps are what Natural developers
work in daily. The specific things that make maps worth teaching (protected versus unprotected
fields, autoskip, the modified data tag, non-display fields, PF-key dispatch, the keyboard
lock) are behavioral. A learner either presses PF3 and watches the screen respond, or reads a
paragraph claiming that PF3 does something. Only one of those produces a person who can work.

On renderer architecture, there are two viable designs and one that should be rejected:

- **Recommended: a second renderer for map mode.** A canvas or CSS-grid component that draws
  the 24x80 cell buffer directly, sharing the font, palette, and OIA with the xterm.js pane. The
  interpreter exposes a screen buffer; the renderer draws it and routes keystrokes back through
  field-navigation logic (tab between unprotected fields, autoskip, insert mode, field
  overflow). Two renderers over one theme is less work than it sounds and far less fragile than
  the alternative, because field navigation is a poor fit for a stream terminal's input model.
- **Viable fallback: drive xterm.js with absolute cursor positioning** and intercept keystrokes
  before they reach the terminal, which is close to what `c3270` does. `zpgu/WebTerminal`
  demonstrates that a real 3270 session renders acceptably this way. Choose this if the map
  renderer turns out to be a schedule risk; the cost is that every field-navigation behavior has
  to be layered on top of a component that does not know fields exist.
- **Rejected: adopting any existing browser 3270.** All of them exist to talk to a real host
  over a bridge, which is the one thing we do not need, and the two most complete client-side
  implementations (`IronTerm`, `bencz/web3270`) are GPL-3.0. `mflorence99/go-3270` is WASM and
  is the right architecture, and it has no license at all.

### Sequencing

1. Tier A items A1 through A3 can land alongside the current interpreter milestone. They touch
   only the front end and carry no interpreter risk.
2. A4 (the OIA) should land before Tier 1 ships, because the runaway-loop cap in Module 7 and
   the per-lesson reset in Module 9 both want a place to report state, and `X SYSTEM` plus a
   friendly message is a better home for that than a line of terminal output.
3. B1 through B3 (the field model) belong in the interpreter and should be designed now even
   though they ship in Tier 2, because the screen buffer has to be a first-class concept in the
   interpreter's state, and the non-negotiable architecture constraint (an explicit statement
   loop with an explicit frame stack, never a recursive evaluator) applies to `INPUT USING MAP`
   exactly as it applies to `INPUT`. A map read is a yield point. Retrofitting that later would
   be as painful as retrofitting the resumable state machine.

---

## What we must not do

Collected in one place so the constraint list is checkable.

- **Do not ship Hercules, TK5, TK4-, MVS/CE, or any MVS image with the course.** No pirated OS
  images are involved in any of these, but their licensing is informal (TK5 and MVS/CE carry no
  formal license at all), they need a backend, and none of them can run Natural.
- **Do not link or vendor GPL-3.0 code into the course bundle.** This rules out
  `cool-retro-term-renderer`, `bencz/IronTerm`, `bencz/web3270`, and `zpgu/WebTerminal`. Read
  them; write our own.
- **Do not vendor `mflorence99/go-3270`.** No license file means all rights reserved.
- **Do not vendor EPL-2.0 code (`zowe/tn3270-ng2`) without legal review.** EPL-2.0 is
  file-level copyleft. Reading it is fine.
- **Do not claim the course connects to a mainframe, or that it runs Software AG Natural.** It
  runs our teaching interpreter over sample data, rendered on a 3270-style display.
- **Do not attempt to obtain an old Natural distribution for an emulated MVS.** There is no
  license under which that would be lawful.
- **Safe to use commercially, confirmed:** `rbanffy/3270font` (BSD-3-Clause and OFL-1.1-RFN,
  keep the license file, do not rename-and-modify), `pmattes/x3270` (BSD-3-Clause, attribution
  required if vendored), `racingmars/go3270`, `FuzzyMainframes/TN3270Sharp`,
  `downarowiczd/rust3270`, `thorhs/mfform` (all MIT), `IBM/tnz` (Apache-2.0),
  `lowobservable/*` (ISC), `@xterm/xterm` (MIT). The x3270 wiki content is CC BY-SA 4.0, so
  quote and attribute rather than copying wholesale into course text.

---

## Sources

All accessed 2026-07-31 unless noted.

### Hercules and operating systems

- [SDL-Hercules-390/hyperion on GitHub](https://github.com/SDL-Hercules-390/hyperion) - the maintained Hercules 4.x line; 343 stars, ~11,600 commits, last pushed 2026-07-11. Repository metadata read via the GitHub API.
- [SDL-Hercules-390/hyperion releases](https://github.com/SDL-Hercules-390/hyperion/releases) - `Release_4.9.1` published 2025-12-07, `Release_4.9` 2025-10-13, `Release_4.8` 2025-03-27, `Release_4.7` 2024-03-10.
- [hyperion `COPYRIGHT` file](https://github.com/SDL-Hercules-390/hyperion/blob/master/COPYRIGHT) - states distribution under the Q Public License Version 1.0 and names Roger Bowler, Jan Jaeger, and Jay Maynard as initial developers. Read via the GitHub raw content API.
- [Hercules documentation home](https://sdl-hercules-390.github.io/html/) - substantiates that Hercules implements System/370, ESA/390, and 64-bit z/Architecture and runs on Linux, Windows, Solaris, FreeBSD, and macOS, and that Hyperion is the current line.
- [Hercules Version 4 FAQ (Aethra mirror)](https://hercules-aethra.github.io/html/hercfaq.html) - substantiates that OS/390, z/OS, and other ESA or z/Architecture operating systems are licensed to a particular machine and cannot be run without an IBM license.
- [Hercules Version 3 FAQ](http://www.hercules-390.org/hercfaq.html) - the legacy 3.x line's FAQ; QPL licensing and the OS licensing distinction.
- [Hercules-Aethra/aethra on GitHub](https://github.com/Hercules-Aethra/aethra) - Jay Maynard's fork; 33 stars, last pushed 2025-09-11, no tagged release.
- [Hercules (emulator) on Wikipedia](https://en.wikipedia.org/wiki/Hercules_(emulator)) - project history, Roger Bowler as originator in 1999, maintainer lineage.
- [Available distributions of MVS 3.8J](https://mdickinson.dyndns.org/hercules/obtaining_an_os/obtaining_mvs38j.php) - community survey of TK4-, TK4+, TK5, and MVS/CE and their relationships.
- [What makes MVS 3.8j legal? (turnkey-mvs archive)](https://turnkey-mvs.yahoogroups.narkive.com/ROEtpvDf/what-makes-mvs-3-8j-legal) - the nuance that MVS 3.8, VM/370 R6, and DOS/VS R34 were no-charge IBM products, that "public domain" is a US-specific framing, and that the safer description outside the US is copyrighted software provided at no charge.
- [IBM Public Domain Software Collection (Jay Maynard)](https://www.ibiblio.org/jmaynard/) - the canonical archive of the no-charge IBM materials.

### Turnkey distributions

- [MVS Turnkey 5 Update 5](https://www.prince-webdesign.nl/tk5) - substantiates that TK5 is an MVS 3.8J implementation, that Update 5 is dated 2026-02-18 and is cumulative, that it ships Hyperion Hercules SDL 4.9.1 (64-bit Windows/Linux) with 4.3.99999 elsewhere, 15 mostly-3390 DASD volumes, ISPF 2.2, BREXX370, RPF, HTTPD 3.3.0, and an FTP daemon, and that it is a free download with no explicit license statement.
- [An update on MVS Turnkey 4](https://www.prince-webdesign.nl/index.php/software/update-on-mvs-turnkey-4) - substantiates that TK4+ is superseded by TK5 and is no longer downloadable.
- [MVS-sysgen/sysgen on GitHub](https://github.com/MVS-sysgen/sysgen) - MVS/CE; automated Jay Moseley sysgen, release `v2.1.5` published 2026-07-13, 85 stars, no LICENSE file. Metadata read via the GitHub API; README read via the raw content API.
- [MVS-sysgen/docker-mvsce](https://github.com/MVS-sysgen/docker-mvsce) - Docker packaging; last pushed 2026-03-03, no LICENSE file.
- [Installing MVS 3.8j (Jay Moseley)](https://www.jaymoseley.com/hercules/installMVS/iMVSintroV8.htm) and [Performing a System Generation](https://www.jaymoseley.com/hercules/installMVS/iSYSGENv8.htm) - the from-tapes sysgen procedure using the IBM MVS 3.7 starter system.
- [joergschultzelutter/tk5-hercules](https://github.com/joergschultzelutter/tk5-hercules) - TK5 on Alpine in Docker; Unlicense, last pushed 2025-06-15.
- [The MVS 3.8j Tur(n)key System (TK4-)](https://wotho.ethz.ch/tk4-/) and [mirror](https://wotho.pebble-beach.ch/tk4-/) - Juergen Winkelmann's TK4-, service level 8505. The primary host did not resolve from this machine on 2026-07-31.

### Natural licensing and platform requirements

- [Natural Installation for z/OS Version 9.2.2, June 2026 (PDF)](https://documentation.softwareag.com/natmf/9.2.2/en/webhelp/natmf-webhelp/pdf/inst_zos.pdf) - substantiates that current Natural for Mainframes is 9.2.x and installs on z/OS.
- [Natural for z/OS Operations Version 9.2.4, March 2026 (PDF)](https://documentation.softwareag.com/natmf/9.2.4/de/webhelp/natmf-webhelp/pdf/ops_mf.pdf) - the most recent mainframe documentation version observed.
- [Natural Version 9.2.3 Release Notes for Mainframes](https://documentation.softwareag.com/natmf/9.2.3/en/webhelp/natmf-webhelp/rnotes_mf/rn-mf-over.htm) - current mainframe release line.
- [Introducing Natural RPC](https://documentation.softwareag.com/natural/nat827mf/rpc/intro_rpc.htm) - substantiates the supported mainframe TP monitors: Com-plete, CICS, IMS TM, TSO, TIAM, openUTM, and batch.
- [TMODEL parameter](https://documentation.softwareag.com/natural/nat911mf/parms/tmodel.htm) - Natural's own IBM 3270 terminal model parameter, listing model 2 as 24x80, model 3 as 32x80, model 4 as 43x80, model 5 as 27x132, with `0` meaning driver-determined and a `(lines,cols)` form for NWO server terminals from 24 to 250 lines and 80 to 250 columns.
- [Invoking a Map with INPUT USING MAP](https://documentation.softwareag.com/natural/nat828mf/edis/mapt_mf_INPUT_USING.htm) and [Map Editor](https://documentation.softwareag.com/natural/nat914unx/edis/edis_ux_map.htm) - substantiates that a Natural map is a screen layout referenced by `INPUT USING MAP` or `WRITE USING MAP`, containing text fields and data fields, and that `INPUT` for a predefined map can be processed on a buffered 3270-type terminal.
- [Adabas & Natural Community Edition](https://www.softwareag.com/en/developer/adabas-natural-community-edition/) - personal-use-only licensing; carried forward from spike 05, re-confirmed as the standing constraint.

### 3270 emulators and libraries

- [pmattes/x3270 on GitHub](https://github.com/pmattes/x3270) - 68 stars, last pushed 2026-07-30, tag `4.5ga6` dated 2026-07-27, prior GAs `4.5ga5` 2025-12-23 and `4.4ga6` 2025-04-25. Repository tree confirms the `b3270`, `c3270`, `s3270`, `tcl3270`, `pr3287`, `wb3270`, `wc3270`, `wpr3287`, `playback`, and `mitm` components. Metadata and tag dates read via the GitHub API.
- [x3270 `LICENSE.md`](https://github.com/pmattes/x3270/blob/main/LICENSE.md) - verbatim BSD 3-Clause text, copyrights Paul Mattes 1993 to 2026, Don Russell 2004 to 2005, Dick Altenbern 2004, Jeff Sparkes 1990, Georgia Tech Research Corporation 1989.
- [x3270 project page](https://x3270.bgp.nu/) - component overview and GitHub link.
- [x3270 Wiki: 3270 data stream protocol](https://x3270.miraheze.org/wiki/3270_data_stream_protocol) (CC BY-SA 4.0, page last edited 2025-08-30) - block-mode behavior, screen buffer of characters with attributes in EBCDIC, 1,920 positions for a model 2, orders including NUL/FF/SO/SI/NL/EM/SF/SFE with hex values, field definition and the wrap-around scanning rule, formatted versus unformatted buffers, Write / Erase-Write / Erase-Write-Alternate / Read Buffer commands, keyboard lock and Reset semantics, the AID list including Clear, Enter, PA keys and 24 PF keys, that an AID transmits all modified fields with EBCDIC NULs removed, insert mode behavior, and Graphic Escape.
- [x3270 Wiki: 3270 models](https://x3270.miraheze.org/wiki/3270_models) (CC BY-SA 4.0, page last edited 2022-01-17) - 3278 monochrome versus 3279 color, the model 2/3/4/5 row and column table, the `-E` extended data stream suffix, and default versus alternate screen size behavior on Erase/Write and Erase/Write Alternate.
- [x3270 Wiki: c3270 Operator Information Area](https://x3270.miraheze.org/wiki/C3270/Operator_Information_Area) (CC BY-SA 4.0, page last edited 2025-09-05) - the full OIA field list: mode indicators, the `X` message table including `X SYSTEM`, `X Wait`, `X Protected`, `X NUM`, `X Overflow`, `X [TCP]`, `X [TELNET]`, `X [TN3270E]`, `X [Field]`, `X Scrolled n`, `X -f`; the miscellaneous indicators `C x`, `T`, `R`, `I`, `P`, `S`, screen tracing digit, `s`; the LU name; the timing field; and cursor position with `001/001` at the upper left.
- [3270 Data Stream Programming: Fields (Tommy Sprinkle)](http://www.tommysprinkle.com/mvs/P3270/fields.htm) - the attribute byte bit layout (bits 0-1 derived, bit 2 protected, bit 3 numeric, bits 4-5 display mode with `11` as hidden, bit 6 reserved, bit 7 MDT), that the attribute character consumes a buffer position and displays as a blank, and that protected plus numeric yields autoskip.
- [3270 Programming Overview (Prycroft Six)](https://www.prycroft6.com.au/misc/3270.html) - 12-bit and 14-bit buffer addressing and the BSC transparency reason for it, the SBA/SF/SFE/SA/MF/IC/PT/RA/EUA order set, WCC bits (start printer, sound alarm, keyboard restore and AID reset, reset MDT), extended attributes with the seven-color x'F0' to x'FF' mapping and blink/reverse/underscore highlighting, and AID examples x'7D' ENTER, x'F1' PF1, x'6D' CLEAR, x'88' structured field.
- [3270 field attributes (IBM CICS TS documentation)](https://www.ibm.com/docs/en/cics-ts/6.x?topic=terminals-3270-field-attributes) - IBM's own statement of the protection, modification, and display-intensity attributes.
- [Attention identifier constants list (DFHAID), IBM Documentation](https://www.ibm.com/docs/en/txseries/9.1.0?topic=constants-attention-identifier-list-dfhaid) and [CICS AID Keys](https://www.mainframestechhelp.com/tutorials/cics/aid-keys.htm) - the AID hex table: PF1-PF9 x'F1'-x'F9', PF10-PF12 x'7A'-x'7C', PF13-PF21 x'C1'-x'C9', PF22-PF24 x'4A'-x'4C', PA1 x'6C', PA2 x'6E', PA3 x'6B', CLEAR x'6D', ENTER x'7D', structured field x'88'.
- [IBM GA23-0059-07, 3270 Information Display System Data Stream Programmer's Reference, June 1992 (PDF, bitsavers)](https://bitsavers.org/pdf/ibm/3270/GA23-0059-07_3270_Data_Stream_Programmers_Reference_199206.pdf) - the primary specification. Cite this in the course reference appendix.
- [Operator Information Area (IBM 3270 host connection docs)](http://ps-2.kev009.com/tl/techlib/manuals/adoclib/3270hcon/hconugd/hobo5.htm) and [Understanding the OIA (HCL)](https://help.hcl-software.com/zie/zieweb/3.0/doc/troubleshoot/oia.html) - the OIA as an extra line below the display, with column-assigned fields for control unit status, connection protocol, system available, security, session shortname, program messages in columns 9 to 17, communications messages, message waiting, APL mode, insert mode in column 52, and graphics cursor mode.
- [RFC 2355, TN3270 Enhancements](https://www.rfc-editor.org/rfc/rfc2355.html) - TN3270E: printer and terminal emulation over Telnet, client-requested device/LU name, ATTN and SYSREQ key support, SNA response handling.
- [RFC 1576, TN3270 Current Practices](https://www.rfc-editor.org/rfc/rfc1576.html) - the de facto TN3270 standard of negotiating Terminal-Type, Binary Transmission, and End of Record.
- [TN3270 documentation index](http://tn3270.wikidot.com/) - substantiates the RFC set 1041, 1576, 1646, 1647, 2355, 2561, 2562, 3049, and that TN3270E is unrelated to the RFC 1041 3270 Regime option.
- [Using the 3270 Terminal, Zowe Docs](https://docs.zowe.org/stable/user-guide/mvd-3270/) - substantiates that browsers cannot supply the TCP networking terminals require, so the Zowe Application Server acts as a WebSocket-to-TCP bridge.
- [zowe/tn3270-ng2](https://github.com/zowe/tn3270-ng2) - EPL-2.0, 19 stars, last pushed 2026-07-27, latest release v2.18.5-RC1 published 2026-06-22.
- [bencz/IronTerm](https://github.com/bencz/IronTerm) - GPL-3.0, 7 stars, last pushed 2026-05-21; README substantiates browser-side TN3270E and TN5250 datastream implementation in plain JS with a websockify-style bridge required.
- [bencz/web3270](https://github.com/bencz/web3270) - GPL-3.0, 3 stars, last pushed 2026-05-07; README enumerates the full command, order, WCC, and attribute-byte implementation surface.
- [mflorence99/go-3270](https://github.com/mflorence99/go-3270) - no license file, 1 star, last pushed 2026-02-11; README describes Go-to-WASM with canvas rendering via a requestAnimationFrame blit and a buffer of cells and fields; `src/client/assets/wasm_exec.js` confirms the Go WebAssembly runtime.
- [moshix/web3270](https://github.com/moshix/web3270) - NOASSERTION license, 3 stars, last pushed 2026-07-01; single Go binary serving a browser 3270 pointed at a TN3270 host.
- [3270io/3270Web](https://github.com/3270io/3270Web) - no license file, 3 stars, last pushed 2026-07-22; Go web UI embedding `s3270`.
- [zpgu/WebTerminal](https://github.com/zpgu/WebTerminal) - GPL-3.0, 31 stars, last pushed 2026-07-30; Spring Boot plus xterm.js serving SSH, Telnet, and TN3270 by driving `c3270` over a pty.
- [h3270](https://h3270.sourceforge.net/) and [h3270 on Open Hub](https://openhub.net/p/h3270) - server-side Java rendering 3270 screens as pure HTML with a configurable layout engine; first commit December 2003, last commit roughly 2013.
- [AlanKrueger/freehost3270](https://github.com/AlanKrueger/freehost3270) - LGPL-2.1, 8 stars, last pushed 2011-07-18; Java-applet 3270-to-web, dead.
- [racingmars/go3270](https://github.com/racingmars/go3270) - MIT, 48 stars, last pushed 2026-01-02; builds 3270 data streams from field lists and processes attention keys plus field values; `RunTransactions()` screen-flow model.
- [FuzzyMainframes/TN3270Sharp](https://github.com/FuzzyMainframes/TN3270Sharp) - MIT, 9 stars, last pushed 2026-04-24; explicitly modeled on go3270, self-described as unstable.
- [IBM/tnz](https://github.com/IBM/tnz) - Apache-2.0, 90 stars, last pushed 2026-07-23.
- [lowobservable/pytn3270](https://github.com/lowobservable/pytn3270) (ISC, 34 stars, last pushed 2025-09-06) and [lowobservable/oec](https://github.com/lowobservable/oec) (ISC, 64 stars, last pushed 2026-02-08) - Python TN3270 library and an IBM 3174 controller replacement driving real 3278/3279 hardware.
- [PerryWerneck/pw3270](https://github.com/PerryWerneck/pw3270) (LGPL-3.0, 81 stars, last pushed 2026-05-18) and [lib3270](https://github.com/PerryWerneck/lib3270) (LGPL-3.0, 13 stars, last pushed 2025-11-28).
- [dmolony/dm3270](https://github.com/dmolony/dm3270) - Apache-2.0, 59 stars, last pushed 2020-02-13.
- [Open3270/Open3270](https://github.com/open3270/open3270) - MIT, 57 stars, last pushed 2021-04-30.
- [downarowiczd/rust3270](https://github.com/downarowiczd/rust3270) and [rust3270 on crates.io](https://crates.io/crates/rust3270) - MIT, v0.1.1 published 2025-06-19, 873 downloads, 1 star, last pushed 2026-02-03.
- [thorhs/mfform](https://github.com/thorhs/mfform) and [mfform-lib on crates.io](https://crates.io/crates/mfform-lib) - MIT, v0.4.0 published 2024-07-12.
- [moshix/minesweeper](https://github.com/moshix/minesweeper) - Go, no license declared, 3 stars, last pushed 2025-11-11; a 3270 application written on go3270 as a community teaching example.

### Terminal, fonts, and aesthetics

- [rbanffy/3270font](https://github.com/rbanffy/3270font) - 1,981 stars, last pushed 2026-02-13; licensed BSD-3-Clause AND OFL-1.1-RFN per its metainfo; a vector font derived from the x3270 font, which was translated from Georgia Tech's 3270tool, itself hand-copied from a 3270 terminal; distributed as a webfont under SIL OFL 1.1.
- [@xterm/xterm on the npm registry](https://registry.npmjs.org/@xterm/xterm) - version 6.0.0, MIT, published 2025-12-22, registry entry modified 2026-07-27. Queried directly.
- [xterm.js](https://xtermjs.org/) and [xtermjs/xterm.js](https://github.com/xtermjs/xterm.js/) - the terminal component and its addon set including `addon-webgl`.
- [remojansen/cool-retro-term-webgl](https://github.com/remojansen/cool-retro-term-webgl) and [cool-retro-term-renderer on npm](https://registry.npmjs.org/cool-retro-term-renderer) - version 1.0.1, **GPL-3.0**, published 2025-12-28; a WebGL CRT renderer for xterm.js with curvature, phosphor glow, and scanlines, configurable at 80x24. Version and license queried directly from the registry.
- [Using CSS to create a CRT (Alec Lownes)](https://aleclownes.com/2017/02/01/crt-display.html) and [Retro CRT terminal screen in CSS + JS (DEV)](https://dev.to/ekeijl/retro-crt-terminal-screen-in-css-js-4afh) - the self-authored CSS approach: repeating-gradient scanlines at 1px, animated scanline drift, flicker keyframes, and color separation.

### Teaching platforms and precedent

- [IBM Z Xplore / Mainframe Training and Skills](https://www.ibm.com/products/z/resources/mainframe-skills) and [From Master the Mainframe to Continuous Learning (Planet Mainframe)](https://planetmainframe.com/2025/08/from-master-the-mainframe-to-continuous-learning/) - free, globally available, year-round, self-paced, badge-based challenges across COBOL, RACF, Linux on Z, CICS, Db2, and utilities, with access to live IBM Z environments; successor to Master the Mainframe.
- [openmainframeproject/cobol-programming-course](https://github.com/openmainframeproject/cobol-programming-course) - CC-BY-4.0, 3,606 stars, last pushed 2026-06-05; a collaboration between American River College, IBM, and IBM clients, taught with VS Code plus Zowe and Z Open Editor against a real z/OS.
- [COBOL Programming Course, Open Mainframe Project](https://openmainframeproject.org/projects/cobol-programming-course/) - project page.
- [Open Mainframe Project growth announcement (PR Newswire)](https://www.prnewswire.com/news-releases/open-mainframe-project-announces-continued-growth-in-community-contributions-and--adoption-as-mainframes-accelerate-innovation-in-enterprise-hybrid-technology-301382458.html) - substantiates the 20-plus project roster including Zowe, COBOL Check, Polycephaly, Mainframe Open Education, GenevaERS, Feilong, Tessia, Zorow, CBT Tape, ConsoleZ, and others.
- [COBOL in WebAssembly (Fermyon)](https://developer.fermyon.com/wasm-languages/cobol) and [Cloudflare Workers Now Support COBOL](https://blog.cloudflare.com/cloudflare-workers-now-support-cobol/) - substantiates the GnuCOBOL-to-C-to-Emscripten-to-WASM pipeline and that Cloudflare chose GnuCOBOL because it is free software.
- [GnuCOBOL on Wikipedia](https://en.wikipedia.org/wiki/GnuCOBOL) - formerly OpenCOBOL, adopted into the GNU Project in 2013.
- [FuzzyMainframes/Awesome-Mainframes](https://github.com/FuzzyMainframes/Awesome-Mainframes) - 114 stars, last pushed 2026-04-24; discovery index used to cross-check this spike's project coverage.

### Prior spikes in this repo that this one builds on

- `research/05-emulator-and-wasm-feasibility.md` (spike date 2026-07-19) - establishes the Natural Community Edition personal-use-only constraint, the absence of any open-source Natural interpreter, and the Rust-to-WASM plus xterm.js architecture decision.
- `research/06-rust-wasm-toolchain.md` and `docs/gotchas-rust-wasm.md` - establish the `@xterm/xterm` scoped package requirement, the `wasm-pack --target web` build, and the resumable-state-machine approach to `INPUT` that Tier 2's map reads must also follow.
