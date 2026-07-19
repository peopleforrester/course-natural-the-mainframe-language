<!-- ABOUTME: Research spike on the academic footprint and job market for the Natural (Software AG 4GL / ADABAS) language.
ABOUTME: Informs the go/no-go decision on publishing a "learn Natural" course. -->

# Natural (Software AG / ADABAS): Academic Footprint and Job Market

Spike date: 2026-07-19

## Executive summary

Natural is a Software AG fourth-generation language that runs against the ADABAS inverted-list database, mostly on IBM z/OS mainframes. It is real, it is still in production at large government and financial institutions, and the people who know it are retiring. That combination is the whole investment thesis for a course, and it is also the reason to be cautious.

Blunt verdict: this is a genuine niche with a real skills-gap tailwind, but it is a small niche with a thin, mostly contract-driven, government-and-insurance-concentrated job market. Total live US openings that name Natural/ADABAS number in the low tens at any given moment (roughly 8 to 60 depending on the board and how loosely you match), not the thousands you would see for COBOL, let alone a mainstream language. The realistic paying audience is not "new grads learning to code." It is existing mainframe and COBOL developers cross-training onto a specific stack, staff at a shrinking set of employers who still run it (state governments, insurers, a few banks), and offshore/nearshore support teams. If the course is cheap to produce, evergreen, and priced for corporate/contractor upskilling, it can work as a long-tail asset. If it needs a large enrollment to break even, the numbers do not support it. Treat this as a "own a small, defensible corner" play, not a "capture a growing market" play.

On the specific University of Texas claim: the strong, verifiable fact is that UT Austin runs its Interactive Degree Audit (IDA) system on Software AG Natural and ADABAS and has since the mid-1980s, and third-party tech-usage data ranks UT Austin as the single most Natural/ADABAS-heavy organization it tracks. What I could not verify is that UT teaches Natural as a for-credit university course. The evidence points to UT being a major operational user that trains its own staff, not a school with a Natural curriculum. That distinction matters for a course pitch, and I flag it explicitly below.

---

## 1. The academic / University of Texas connection

### What is verified (with sources)

- **UT Austin runs Natural/ADABAS in production.** The University of Texas at Austin's Interactive Degree Audit (IDA) system is written in Software AG's Natural language against an ADABAS database. The automated degree-audit system dates to 1985; a TN3270 mainframe version shipped in 1996 and was later exposed over the web. Historically it ran on an Amdahl mainframe under OS/390. UT Austin's own registrar and OneStop pages still document IDA as a live student-facing service. (Sources: EDUCAUSE degree-audit case study; UT Austin Registrar / OneStop IDA pages.)
- **UT Austin is the top-ranked Natural/ADABAS user in third-party usage data.** Sumble's technology-usage profile for Natural/ADABAS lists the University of Texas at Austin first, ahead of Erste Group, Cognizant, and Morgan Stanley, and characterizes it as a 12-team deployment. This is consistent with UT being a large, long-standing operational user. (Source: Sumble Natural/ADABAS page.)
- **A separate UT campus appears in an older ADABAS site census.** A March-2000 list of US ADABAS sites names "University of Texas (San Antonio)" running a Natural/ADABAS environment, alongside Brown University and the University of Cincinnati. This is dated and should be treated as historical, not current. (Source: georgenet.net ADABAS sites list, dated ~2000.)
- **Software AG runs a University Relations / Academic Alliance program and ships a free Adabas & Natural Education Package.** The vendor provides a free VM image, e-learning, video tutorials, and public "Natural Programming Basic" and "Adabas Basic" courses through learn.softwareag.com and education.softwareag.com, and offers its software free for academic and research use. Press material from the University Relations program historically named partners such as Yale, MIT, and UC Berkeley, though that program was oriented to Software AG's broader modern portfolio (big data, cloud, integration), not specifically to teaching legacy Natural/ADABAS. (Sources: Software AG education portal; University Relations pages; GitHub adabas-natural-education-package.)

### What could NOT be verified (honest gaps)

- **That "Natural is taught at the University of Texas" as a university course.** I found no course catalog entry, CS/MIS syllabus, or continuing-education listing showing UT (Austin or any UT System campus) teaches Natural for credit. The verifiable UT relationship is operational: UT Austin *uses* Natural/ADABAS heavily and therefore trains its own IT staff to maintain IDA and related administrative systems. It is reasonable inference (not established fact) that any "Natural training at UT" is internal staff enablement, not a classroom offering.
- **A named Texas state government / university workforce pipeline specifically for Natural.** Texas state agencies clearly ran Adabas/Natural (see Section 2), and the state has a mainframe workforce, but I did not find a documented Texas-specific Natural training pipeline analogous to the Open Mainframe Project's COBOL push.

### The broader academic reality

The honest picture across academia is bleak for Natural specifically. Software AG's own skills blog states plainly that "most universities no longer offer related instruction, since no one would dream of using these technologies for greenfield development projects." The active academic mainframe pipelines that do exist are aimed at COBOL and IBM Z, not Natural:

- The **IBM Z Academic Initiative** partners with 120-plus US schools to put enterprise-computing content into curricula, and **Master the Mainframe** reached 4,286 students across 600-plus schools. These are COBOL/z/OS programs, not Natural programs. (Source: IBM / Open Mainframe Project.)
- The **Open Mainframe Project COBOL Programming Course** (with American River College and IBM) is free and well-supported. There is no equivalent community-scale Natural course. (Sources: Open Mainframe Project; IBM newsroom, 2020.)

**Inference for the course thesis:** the absence of a Natural teaching pipeline is simultaneously the risk (small audience, no feeder of curious students) and the opportunity (almost no competing structured instruction outside Software AG's own paywalled/vendor materials). A well-made independent Natural course would have very little direct competition.

---

## 2. Where Natural/ADABAS is still running in production

### Verified named users and sectors

- **US state governments (public sector is the heaviest concentration).**
  - **California**: the California Department of Technology (CDT) offers ADABAS as a managed z Systems service and states it "delivers extremely high transaction levels, over 1 million commands per second," billing roughly $613/hour of processing and $3.10/GB-month storage. CDT running ADABAS as a catalog service implies multiple California agencies still depend on it. (Source: CDT ADABAS service page.)
  - **CalSTRS** (California State Teachers' Retirement System) is a named Adabas & Natural customer. (Source: AppsRunTheWorld customer database.)
  - **Alaska** issued a Request for Information for IBM mainframe z/OS Adabas and Natural licensing and maintenance, confirming an active state deployment. (Source: Alaska Online Public Notices RFI.)
  - Historical (circa 2000) US site census names **Colorado** (Dept. of Labor and Employment), **Idaho** (Health and Welfare), the **FBI**, **EPA**, **NJ Transit**, and multiple **Texas** agencies (Dept. of Highways and Public Transportation, Comptroller's Office, Attorney General's Office, Texas Lottery Commission, Texas Guaranteed Student Loan Corp). Treat specific agency names as dated; treat the pattern (state governments running Adabas/Natural) as still true given the current CDT, Alaska, and unemployment-insurance evidence. (Source: georgenet.net, ~2000.)
  - **State unemployment-insurance (UI) systems** are a documented Adabas/Natural stronghold. Government Technology and a Hawaii UI RFP glossary both tie Adabas/Natural to state UI systems, and federal Department of Labor funding plus "COBOL-era programmers will soon be leaving" is cited as the driver behind multi-state UI modernization consortiums. (Sources: govtech.com; Hawaii UI RFP glossary.)
- **Insurance and financial services.**
  - **Erste Group** and **Morgan Stanley** appear as major Natural/ADABAS users in Sumble's data. (Source: Sumble.)
  - Software AG case material cites an insurance client where Adabas & Natural applications were "directly responsible for over $2 billion in annual revenue." (Source: Software AG insurance case study.)
- **Federal and defense-adjacent.** American Armed Forces Mutual Aid Association is a named customer. (Source: AppsRunTheWorld.)
- **Cross-industry.** AppsRunTheWorld describes adoption "spanning over 21 industries," most concentrated in the US and UK. Sumble counts 757 organizations that mention Natural/ADABAS. Enlyft (a competing tech-usage tracker) has historically pegged Software AG Adabas at roughly 0.18% database-management-system market share, i.e., a small single-digit-thousands universe of detectable sites. (Sources: AppsRunTheWorld; Sumble; Enlyft.)

### Installed base and longevity

- Software AG committed in 2016 to support Adabas and Natural **through 2050 and beyond** (the "Adabas & Natural 2050+" program), and continues to invest (NaturalONE IDE, Natural for VS Code, a "Natural AI Code Assist" tool announced as coming). This is the single most important durability signal: the vendor is actively telling customers to stay, not migrate, which keeps maintenance demand alive for decades. (Sources: Software AG 2050+ resources and skills blog.)
- No public figure gives a precise global customer count or total lines of Natural code. Vendor and analyst language ("millions of transactions daily," "billions of transactions weekly" at CDT) confirms scale without a countable installed base.

**Inference:** the production footprint is large in transaction volume and business-criticality but concentrated in a modest number of big, slow-moving institutions (governments, insurers, a few banks). Those institutions rarely disappear and rarely migrate quickly, which is exactly why maintenance skills stay in demand even as the platform is "legacy."

---

## 3. The mainframe skills gap as it applies to Natural

The generic "COBOL programmers are retiring" narrative applies to Natural in a sharper form, because Natural has an even thinner and older talent pool than COBOL and far less academic replenishment.

Verified data points:

- **Forrester (2018), commissioned study:** enterprises lost an average of 23% of specialized mainframe staff over the prior five years, and 63% of those vacated roles remained unfilled. (Cited via Modern Systems / OneAdvanced modernization eBook.)
- **Futurum Group, 2024 Global Mainframe Skills Report:** 79% of organizations are struggling to fill mid-career legacy roles as the aging workforce retires. (Cited via Software AG skills material.)
- **Software AG's own framing:** decades of institutional Adabas/Natural knowledge are concentrated in a few near-retirement individuals, and universities no longer teach the stack, so there is no organic replacement pipeline. Vendor mitigations named: rehiring retirees part-time to mentor, structured onboarding, role-based learning paths. (Source: Software AG "building the next generation of Adabas & Natural experts" and "your experts will retire" blogs.)

**Inference:** the skills gap is real and, for Natural specifically, more acute than for COBOL because (a) fewer people ever learned it, (b) it is single-vendor rather than an industry standard, and (c) there is effectively no free academic feeder. This is the strongest single argument for a course. The catch is that a skills gap creates demand for *trained people*, which is a demand for *training*, but only to the extent that employers or individuals will pay for that training rather than migrate off the platform or outsource maintenance.

---

## 4. Job market (current, 2026)

Data is thin and noisy; job-board totals for a niche 4GL swing widely by how loosely the board matches "Natural." I am reporting what specific boards showed on the access date and marking soft numbers as such.

Observed on 2026-07-19:

- **ZipRecruiter** ("Adabas Natural"): approximately **59** listings described as currently hiring; stated pay bands in the ~$57 to ~$86/hour range; a note that average remote "Adabas Natural" pay in Atlanta, GA was ~$64.25/hour (most $55.72 to $80.67). Many are contract. (Source: ZipRecruiter Adabas-Natural.)
- **Glassdoor**: ~**28** "Natural Adabas" jobs nationally; a narrower "Natural Adabas developer, remote" cut showed ~**8**. (Source: Glassdoor.)
- **SimplyHired**: ~**11 to 21** "natural adabas developer" listings depending on query. (Source: SimplyHired.)
- **Indeed**: an active "remote Natural Adabas" jobs page existed (count not cleanly parseable from the search snippet). (Source: Indeed.)
- **Salary/rate signal (aggregate, soft):** contractor rates cluster around **$55 to $86/hour**; some permanent/senior roles quoted **$103K to $120K/year**; one "senior part-time" aggregate skewed lower (~$80K average, wide spread). Treat these as job-board aggregates, not surveyed compensation. Requirements consistently list Natural, ADABAS, JCL, NaturalONE, and a mainframe (z/OS) background. (Sources: ZipRecruiter, SimplyHired, Glassdoor.)

Characterization:

- **Volume:** low tens of openings at any moment in the US when you require the Natural/ADABAS combination. This is one to two orders of magnitude smaller than COBOL.
- **Geography:** US-heavy, with clusters wherever the big users sit (state capitals and government hubs such as Austin/Texas, Sacramento/California, and financial centers). Remote and contract postings are common because employers cannot find local talent.
- **Contract vs permanent:** skewed toward contract and staff-augmentation, which is typical of legacy-maintenance work. Government roles (e.g., Texas HHSC, TxDOT postings) exist but are often titled generically ("Programmer III," "Senior Developer") rather than naming Natural, so the true count of Natural-touching government roles is undercounted by keyword search.
- **Rates:** solid for a maintenance skill ($55 to $86/hour contract), reflecting scarcity rather than growth. Scarcity pricing, not hot-market pricing.

**Inference:** demand is real, durable, and scarcity-priced, but shallow. A person who learns Natural is not entering a large job market; they are becoming eligible for a small pool of well-paid, often contract, often government/insurance maintenance roles. That is a legitimate career angle for someone already in the mainframe world, and a weak one for a career-changer with no mainframe background.

---

## 5. Audience for a "learn Natural" course

Ranked by realistic willingness and ability to pay:

1. **Existing mainframe / COBOL developers cross-training (strongest segment).** People already fluent in JCL, z/OS, and COBOL who want to add Natural/ADABAS to stay employable at a shop that runs it, or to qualify for scarcity-priced contract work. They understand the platform, so the course only needs to teach the language and ADABAS access model, not the mainframe from scratch. They or their employers can pay.
2. **Staff at current Natural/ADABAS employers (corporate/B2B angle).** State agencies, insurers, and banks facing retirements need to onboard replacements fast. Software AG's own mitigation playbook is "structured onboarding and role-based learning paths." An independent, cheaper, well-produced course is a plausible purchase for these teams, especially where Software AG's official training is priced high. This is the segment most able to pay real money, and it argues for a B2B/site-license posture over consumer pricing.
3. **Offshore / nearshore support teams.** System-integrator and outsourcing shops (Cognizant appears in the usage data) staff legacy maintenance from lower-cost geographies and need to spin up Natural skills quickly. High volume of learners, low per-seat willingness to pay, price-sensitive.
4. **New grads / career-changers entering government IT (weakest as a standalone pitch).** A real but small trickle of people take government IT jobs and inherit Natural systems. They are a poor primary audience because they usually will not seek out Natural training before being hired; the employer trains them (see segment 2). They may buy a cheap course after the fact.

Market-size reality check:

- The paying universe is bounded by the installed base (a few thousand detectable sites globally per Enlyft; 757 organizations per Sumble) times a small number of Natural developers each, minus everyone content with vendor training. That is a **small four-figure to low five-figure** total addressable population worldwide, shrinking slowly as sites migrate but replenished slowly by retirements creating backfill needs.
- Willingness to pay is bimodal: individuals are price-sensitive (this is "legacy" and unglamorous), while employers backfilling a retirement are far less price-sensitive because the cost of an unmaintained mission-critical system is enormous.

**Inference / recommendation for the go decision:** build for segments 1 and 2. Price and package for the corporate-upskilling and cross-training buyer, not the mass-market self-learner. Keep production cost low and content evergreen (the language barely changes and is supported to 2050+), so the course can earn over a decade from a thin but steady stream. Do not model this on COBOL-course enrollment numbers; model it as a small, defensible, long-tail niche with almost no direct competition and a genuine retirement-driven tailwind. If the business case requires large enrollments to break even, the market will not deliver them.

---

## Distinguishing fact from inference (quick reference)

- **Fact (sourced):** UT Austin runs its degree-audit system on Natural/ADABAS; UT Austin is the top-ranked Natural/ADABAS user in Sumble data; California CDT, CalSTRS, Alaska, Erste Group, Morgan Stanley are Natural/ADABAS users; Adabas & Natural are supported to 2050+; job-board counts on 2026-07-19 were low tens; contractor rates ~$55 to $86/hour; Forrester 2018 and Futurum 2024 skills-gap figures.
- **Inference (reasonable, not proven):** that "Natural is taught at UT" means internal staff enablement rather than a for-credit course; that the paying course audience is dominated by cross-training mainframe devs and employers backfilling retirements; that the total addressable learner population is small four-figure to low five-figure worldwide; that competition for an independent Natural course is nearly absent.
- **Not verified / gaps:** any Natural for-credit university curriculum; a named Texas state Natural workforce pipeline; a precise global customer count or total lines-of-code figure.

---

## Sources

All accessed 2026-07-19.

- EDUCAUSE, "Storing Degree Audit Data" (UT Austin IDA on Software AG Natural/ADABAS, Amdahl/OS-390, since 1985): https://www.educause.edu/ir/library/html/cmr9907/cmr9907.html (supports the UT Austin Natural/ADABAS production fact; note: page returned 403 on direct fetch but content surfaced via search index).
- UT Austin Registrar, Interactive Degree Audit: https://registrar.utexas.edu/about/us/ida ; UT Austin OneStop IDA: https://onestop.utexas.edu/registration-and-degree-planning/degree-planning/interactive-degree-audit/ (supports IDA as a live service).
- Sumble, Natural/ADABAS technology usage (757 orgs; UT Austin ranked first; Erste Group, Cognizant, Morgan Stanley): https://sumble.com/tech/natural-adabas (supports named users and org count).
- georgenet.net, US ADABAS sites list (~2000; Texas agencies, Colorado, Idaho, FBI, EPA, NJ Transit, UT San Antonio, Brown, Cincinnati): http://georgenet.net/misc/adabas.html (supports historical government/university footprint; dated).
- California Department of Technology, ADABAS service (>1M commands/sec; managed z Systems service; pricing): https://www.cdt.ca.gov/services/database-adabas/ (supports California state government use).
- AppsRunTheWorld, Software AG Adabas & Natural customers (CalSTRS; AAFMAA; 21 industries; US/UK concentration): https://www.appsruntheworld.com/customers-database/products/view/software-ag-adabas-natural (supports named customers and industry spread).
- Alaska Online Public Notices, RFI for z/OS Adabas and Natural licensing/maintenance: https://aws.state.ak.us/OnlinePublicNotices/Notices/View.aspx?id=200753 (supports active state deployment).
- GovTech, "Collaboration on Unemployment Systems Creates Efficiencies": https://www.govtech.com/policy/collaboration-on-unemployment-systems-creates-efficiencies.html (supports state UI systems and modernization drivers).
- Hawaii UI RFP glossary (Adabas/Natural in UI systems): https://hiepro.ehawaii.gov/resources/116592/Attachment%20C%20Glossary%20of%20Terms%20and%20Abbreviations.pdf (supports UI-system usage).
- Treehouse Software case studies (state-government Adabas modernization): https://treehousesoftware.wordpress.com/2022/05/24/treehouse-software-customer-case-study-a-state-government-agencys-real-time-data-synchronization-between-ibm-mainframe-adabas-and-aws/ (supports government maintenance/modernization demand).
- Software AG, "Building the next generation of Adabas & Natural experts": https://www.softwareag.com/en/blog/skills/building-the-next-generation-of-adabas-natural-experts/ (supports skills gap and vendor mitigations).
- Software AG, "Your experts will retire. Their knowledge doesn't have to." / DevOps modernization: https://www.softwareag.com/en/blog/skills/modernizing-natural-development-devops/ (supports retirement-risk framing).
- Modern Systems / OneAdvanced, Natural and Adabas modernization eBook (Forrester 2018: 23% staff loss, 63% unfilled; universities no longer teach): https://modernsystems.oneadvanced.com/globalassets/modern-systems-assets/resources/ebook/natural-and-adabas-modernization-ebook.pdf (supports skills-gap statistics).
- Software AG education portal and courses (Natural Programming Basic, Adabas Basic; free VM/tutorials): https://education.softwareag.com/adabas-and-natural ; https://learn.softwareag.com/course/info.php?id=1467 ; https://github.com/SoftwareAG/adabas-natural-education-package (supports vendor training and academic package).
- Software AG University Relations / Academic Alliance (program, named partners historically Yale/MIT/Berkeley): https://www.softwareag.com/en_corporate/resources/university-relations.html ; press: http://www1.softwareag.com/us/Press/pressreleases/20131105_University_Relations_Program.asp (supports academic-alliance existence; note portfolio-wide, not Natural-specific).
- Software AG Adabas & Natural 2050+ (support commitment through 2050): https://www.softwareag.com/en/resources/adabas-natural/mission-critical-applications/ (supports platform longevity).
- IBM Newsroom / Open Mainframe Project, COBOL skills initiatives; IBM Z Academic Initiative (120+ schools; Master the Mainframe 4,286 students / 600+ schools): https://newsroom.ibm.com/2020-04-09-IBM-and-Open-Mainframe-Project-Mobilize-to-Connect-States-with-COBOL-Skills ; https://openmainframeproject.org/projects/cobol-programming-course/ (supports COBOL-vs-Natural academic-pipeline contrast).
- ZipRecruiter, Adabas Natural jobs (~59 listings; ~$57-$86/hr; contract-heavy): https://www.ziprecruiter.com/Jobs/Adabas-Natural (supports job-market volume and rates; direct fetch returned 403, figures via search index).
- Glassdoor, Natural Adabas jobs (~28 national; ~8 remote-developer): https://www.glassdoor.com/Job/natural-adabas-jobs-SRCH_KO0,14.htm (supports job-market volume).
- SimplyHired, natural adabas developer jobs (~11-21): https://www.simplyhired.com/search?q=natural+adabas+developer (supports job-market volume).
- Enlyft, Software AG Adabas market share (~0.18% of DBMS): https://enlyft.com/tech/products/software-ag-adabas (supports small installed-base universe; direct fetch returned 403, figure via search index).
- Texas HHSC / TxDOT careers (generic programmer roles at Natural-using agencies): https://careers.hhs.texas.gov/ ; https://www.txdot.gov/about/careers.html (supports government-employer presence; roles keyword-undercounted).
