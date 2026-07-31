<!-- ABOUTME: Adversarial fact-check of research/03-academia-job-market.md, verifying every factual claim against primary sources.
ABOUTME: Records verdicts, required corrections, and the sources consulted, so the market case rests on traceable evidence. -->

# Verification: 03-academia-job-market.md

Verification date: 2026-08-01
Verifier posture: adversarial. Every claim treated as wrong until a primary source proved otherwise.
Source file spike date: 2026-07-19

## Headline result

Of 24 checked claims: 12 CONFIRMED, 6 PARTIALLY CORRECT, 4 REFUTED, 2 UNVERIFIED.

The two most serious defects are both misuse of cited research. The Futurum 2024 statistic is misquoted and the report's actual headline conclusion contradicts the argument the file uses it to support. The "universities no longer teach this" quote is attributed to the platform vendor when it actually comes from a competitor selling migration off the platform.

Separately, the file's competition analysis is wrong in the direction that matters commercially. Independent Natural training already exists in volume, and the vendor's own foundational Natural course is free with a certification badge.

---

## Verdict table

| Claim (quoted) | Verdict | What is actually true | Source URL | Accessed |
|---|---|---|---|---|
| "UT Austin runs its Interactive Degree Audit (IDA) system on Software AG Natural and ADABAS and has since the mid-1980s" | PARTIALLY CORRECT | Every element is verbatim in the source, but the source is from 1999. It reads: "UT Austin has had an automated degree audit system since 1985. This system currently runs on an Amdahl Millenium 785 mainframe running the OS/390 (revision 2.5) operating system. The degree audit system is written in Software AG's NATURAL programming language and uses their ADABAS database." The word "currently" means 1999. The file states this in the present tense on a 27-year-old source. Independent current corroboration exists but the file does not cite it: a UT Austin "Sr Software Developer/Analyst" posting requires "proven experience developing in the UT environment with Natural and ADABAS", and Sumble's June 2026 profile still shows UT Austin activity. | https://web.archive.org/web/20240912205110/https://www.educause.edu/ir/library/html/cmr9907/cmr9907.html | 2026-08-01 |
| "a TN3270 mainframe version shipped in 1996 and was later exposed over the web" | CONFIRMED | Verbatim: "a mainframe (TN3270) version of the new student access system went into service in 1996, under the name of IDA (Interactive Degree Audit). More recently, access to IDA has been made available over the World Wide Web." | https://web.archive.org/web/20240912205110/https://www.educause.edu/ir/library/html/cmr9907/cmr9907.html | 2026-08-01 |
| "Sumble ranks UT Austin as the single most Natural/ADABAS-heavy organization it tracks (12 teams)" | PARTIALLY CORRECT | Sumble is a real company (sales-intelligence platform founded by Kaggle founders Anthony Goldbloom and Ben Hamner, emerged from stealth October 2025, $68.5M raised). The page does rank UT Austin first at 12 teams. Three caveats the file omits. First, Sumble infers technographics from job postings and people profiles; it is a lead-generation tool, not a census, and publishes no methodology for the team counts. Second, Sumble never defines "team". Third, on the headcount column UT Austin shows only 5 people while Cognizant shows 14, so "most Natural/ADABAS-heavy" holds only on an undefined metric. Full ranking: UT Austin 12 teams/5 people, Erste Group 8/0, Cognizant 6/14, Morgan Stanley 4/9, ISM 3/0. Page last updated 2026-06-08. | https://sumble.com/tech/natural-adabas | 2026-08-01 |
| "Sumble counts 757 organizations that mention Natural/ADABAS" | CONFIRMED | Page states "757 organizations using Natural/ADABAS on Sumble". | https://sumble.com/tech/natural-adabas | 2026-08-01 |
| "I found no course catalog entry, CS/MIS syllabus, or continuing-education listing showing UT (Austin or any UT System campus) teaches Natural for credit" | CONFIRMED | Re-tested independently and more broadly than the original: US and German university catalogs, Software AG academic alliance pages, community college and state workforce programs, and edX. No for-credit Natural course found at any institution worldwide. edX has an "Adabas" landing page that carries zero actual Adabas or Natural courses; it is a search-engine landing page describing the technology. The file's NO stands. | https://www.edx.org/learn/adabas | 2026-08-01 |
| "Software AG runs a University Relations / Academic Alliance program... historically named partners such as Yale, MIT, and UC Berkeley... oriented to Software AG's broader modern portfolio... not specifically to teaching legacy Natural/ADABAS" | CONFIRMED | Program is real and the named partners check out. The file's own caveat that it is portfolio-wide rather than Natural-specific is accurate and correctly stated. | http://www1.softwareag.com/us/Press/pressreleases/20131105_University_Relations_Program.asp | 2026-08-01 |
| "Software AG's own skills blog states plainly that 'most universities no longer offer related instruction, since no one would dream of using these technologies for greenfield development projects.'" | **REFUTED** | Misattributed and misquoted. The sentence is from the Modern Systems / OneAdvanced eBook "Natural Selection", not Software AG. Modern Systems is a modernization vendor that sells migration **off** Natural and Adabas, so this is an adversarial commercial source, not the platform vendor conceding the point. The actual wording is "Most universities no longer offer **mainframe** instruction since no one would dream of using these technologies for greenfield development projects." The file changed "mainframe instruction" to "related instruction" inside quotation marks, which narrows a general mainframe claim into a Natural-specific one. Neither Software AG blog the file cites contains the sentence: "Building the next generation of Adabas & Natural experts" (2026-03-24) and "Your experts will retire" (2026-04-22) were both fetched in full and contain no such claim. The file's own Sources section attributes the quote correctly to Modern Systems, contradicting its body text. | https://modernsystems.oneadvanced.com/globalassets/modern-systems-assets/resources/ebook/natural-and-adabas-modernization-ebook.pdf | 2026-08-01 |
| "Forrester (2018), commissioned study: enterprises lost an average of 23% of specialized mainframe staff over the prior five years, and 63% of those vacated roles remained unfilled." | CONFIRMED | Verified against the primary press release, not the secondary eBook. Verbatim: "Enterprises have lost an average 23 percent of specialized mainframe staff in the last five years" and "63 percent of those positions have not been filled." Forrester Consulting on behalf of Compuware, published 2018-03-22, study titled "Modern Mainframe KPIs Are Key To A Successful Digital Strategy", surveying mainframe decision-makers and developers in the US and Europe. Caveats the file should carry: it is vendor-commissioned by a company selling mainframe DevOps tooling, it is now 8 years old, and it concerns mainframe staff generally, not Natural. | https://www.einpresswire.com/article/438209464/compuware-survey-shows-critical-mainframe-workloads-increasing-while-mainframe-staff-losses-remain-unfilled | 2026-08-01 |
| "Futurum Group, 2024 Global Mainframe Skills Report: 79% of organizations are struggling to fill mid-career legacy roles as the aging workforce retires." | **REFUTED** | Two separate defects. **The statistic is misquoted.** The report says "there is also a pronounced demand for mid-career professionals, with 79% of employers focusing recruitment efforts on this group." That is a demand measure, not a difficulty measure. "Struggling to fill" is not in the report. Neither is "legacy roles". The word "retire" does not appear anywhere in the report text. **The report's conclusion is the opposite of the file's use of it.** Verbatim from the executive summary: "This report... addresses the prevailing narrative surrounding the mainframe workforce, that it is an aging field with dwindling new talent. Contrary to this perception, our findings reveal a dynamic and evolving landscape where mainframe skills continue to grow and diversify." And: "The key research findings do not align with an aging workforce narrative. A key takeaway is that finding mainframe skills is no harder for enterprises than finding cybersecurity or AI skills, which will no doubt shock those that seek to decry the mainframe." And: "The influx of early-career professionals into the mainframe industry challenges the myth of an aging workforce." The report also finds a 65% reported rise in skilled mainframe workers over 5 years and 91% of employers planning to hire mainframe talent within 2 years. The strings "Natural" and "COBOL" appear zero times. Sponsored by IBM, Broadcom and 21CS. Methodology: three surveys 2024-02-20 to 2024-03-03 covering 750 businesses (>5000 employees, >$100M revenue), 200 universities (screened to schools that already offer mainframe curriculum, so the 65% is self-selected), and 200 students; Japan and China excluded. The file additionally sources it as "Cited via Software AG skills material", and neither Software AG blog it cites contains the figure. | https://devops.com/wp-content/uploads/2024/08/TFG_Mainframe-60-Skills-Research-Report_v1.pdf | 2026-08-01 |
| "Software AG committed in 2016 to support Adabas and Natural through 2050 and beyond (the 'Adabas & Natural 2050+' program)" | CONFIRMED, with a material omission | The commitment is real and dated: press release of 2016-08-31 announcing the "Adabas & Natural 2050+" agenda to support and further develop the portfolio until beyond the year 2050. The omission is that the committing entity no longer exists in that form. Software AG has been broken up: webMethods and StreamSets went to IBM in July 2024, and since January 2025 Adabas & Natural has run as a standalone business under Silver Lake ownership via the Software GmbH holding (announced 2025-01-07). The live 2050+ landing page no longer carries the explicit commitment wording. A 2016 durability pledge from a delisted, dismembered, private-equity-owned vendor is weaker evidence than the file presents, and the file calls it "the single most important durability signal" without noting any of this. | https://www.pressebox.com/pressrelease/software-ag-en/Software-AG-Announces-New-Adabas-Natural-2050-Agenda/boxid/812403 ; https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html | 2026-08-01 |
| "continues to invest (NaturalONE IDE, Natural for VS Code, a 'Natural AI Code Assist' tool announced as coming)" | PARTIALLY CORRECT | Both products are real announcements but neither has shipped, and the names are wrong. Correct names are "Natural for Visual Studio Code" and "Natural AI Code Assistant (working title)". Both are listed under the Co-Innovation Program with a planned release date of **October 2026**. The file's phrasing implies Natural for VS Code already exists alongside NaturalONE. | https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504 | 2026-08-01 |
| "the California Department of Technology (CDT) offers ADABAS as a managed z Systems service and states it 'delivers extremely high transaction levels, over 1 million commands per second,' billing roughly $613/hour of processing and $3.10/GB-month storage" | CONFIRMED | Page states "over 1 million commands per second" and "Capable of processing 1+ million ADABAS commands per second", $613.00 per hour of processing normalized to IBM CPU 2064-116, and $3.10 monthly per GB. Service is actively offered with current request forms and support hours. | https://www.cdt.ca.gov/services/database-adabas/ | 2026-08-01 |
| "CalSTRS... is a named Adabas & Natural customer" and "American Armed Forces Mutual Aid Association is a named customer" | CONFIRMED (secondary source only) | Both are listed. Caveat: AppsRunTheWorld is a lead-generation database with no publication date; only three named customers surface (CalSTRS, AAFMAA, Nissan Motor Corporation UK) and the implementation dates range 2015 to 2020. This is a secondary aggregator, not a vendor or customer statement. | https://www.appsruntheworld.com/customers-database/products/view/software-ag-adabas-natural | 2026-08-01 |
| "adoption 'spanning over 21 industries,' most concentrated in the US and UK" | CONFIRMED (secondary source only) | Page states "spanning over 21 industries", "most concentrated in United States and United Kingdom", concentrated in Banking and Financial Services, Non Profit, and Automotive. | https://www.appsruntheworld.com/customers-database/products/view/software-ag-adabas-natural | 2026-08-01 |
| "Alaska issued a Request for Information for IBM mainframe z/OS Adabas and Natural licensing and maintenance, confirming an active state deployment." | PARTIALLY CORRECT | The RFI is real: Department of Administration, Office of Information Technology, seeking "Software AG Maintenance and Support" for IBM mainframe z/OS Adabas and Natural. But it was published **2020-12-11** and archived 2020-12-22. The file presents a nearly six-year-old procurement notice as evidence of a currently active deployment, with no date given. | https://aws.state.ak.us/OnlinePublicNotices/Notices/View.aspx?id=200753 | 2026-08-01 |
| "A March-2000 list of US ADABAS sites names 'University of Texas (San Antonio)'... alongside Brown University and the University of Cincinnati" | CONFIRMED | List is dated "LAST UPDATED: 3/6/00". Contains "TX San Antonio / Univ. of Texas / type of industry: education - university / environment (Natural, Supernatural, ADASQL, Adamints, etc): natural/adabas", plus "TX Austin? Univ. of Texas", Brown University (Providence RI), University of Cincinnati, Colorado Dept. of Labor and Employment, Idaho Health and Welfare, FBI, EPA OIRM, NJ Transit, and the named Texas agencies. Caveat: it is an anonymous crowd-sourced hobbyist list compiled by one individual (George Lewycky) with self-flagged uncertainty in the entries, not an authoritative census. The file correctly labels it dated. | https://web.archive.org/web/20210505220230/http://georgenet.net/misc/adabas.html | 2026-08-01 |
| "State unemployment-insurance (UI) systems are a documented Adabas/Natural stronghold." | PARTIALLY CORRECT | The association is documented, but the file omits that the stronghold is actively being dismantled in the state it highlights most. Texas Workforce Commission selected Sagitec's Neosurance COTS product to replace its mainframe UI system, with a three-year implementation plus five years of maintenance and operations. Presenting state UI as durable demand without noting live displacement overstates the tailwind. | https://www.sagitec.com/pension-software-company/press-releases/texas-workforce-commission-chooses-sagitec-to-modernize-their-unemployment-insurance-system | 2026-08-01 |
| "Software AG case material cites an insurance client where Adabas & Natural applications were 'directly responsible for over $2 billion in annual revenue.'" | CONFIRMED | Software AG case study confirms the figure. The customer is unnamed. Additional detail: over 207 GB of Adabas data, over 2,000 Natural objects, 10,000 COBOL objects. | https://www.softwareag.com/en/resources/adabas-natural/insurance-risk-reduction/ | 2026-08-01 |
| "Enlyft... has historically pegged Software AG Adabas at roughly 0.18% database-management-system market share" | CONFIRMED | Enlyft states 0.18% share and reports data on 3,873 companies using Software AG Adabas, most often US-based, IT services, >10000 employees. The 3,873 figure supports the file's "few thousand detectable sites" characterization and should be cited explicitly. | https://enlyft.com/tech/products/software-ag-adabas | 2026-08-01 |
| "The IBM Z Academic Initiative partners with 120-plus US schools" | CONFIRMED | Verbatim from the IBM release of 2020-04-09: "Through the IBM Z Academic Initiative program, IBM actively partners with over 120 schools across the United States located in the vicinity of our clients to integrate critical Enterprise Computing content into curriculum." Same release adds "Over 45 of these schools have specific courses dedicated to COBOL programming." Note the live URL cited in the file returns HTTP 404; only the Wayback capture resolves. | https://web.archive.org/web/20260316184710/https://newsroom.ibm.com/2020-04-09-IBM-and-Open-Mainframe-Project-Mobilize-to-Connect-States-with-COBOL-Skills | 2026-08-01 |
| "Master the Mainframe reached 4,286 students across 600-plus schools" | CONFIRMED, but stale | Verbatim: "our Master the Mainframe program which reached 4,286 students from over 600 schools across the US last year." The release is dated 2020-04-09, so "last year" means 2019. The program no longer exists: Master the Mainframe was sunset in September 2021 and shut down 2021-12-30, replaced by IBM Z Xplore. A 2026 document should not present it as a current pipeline without saying so. | https://web.archive.org/web/20260316184710/https://newsroom.ibm.com/2020-04-09-IBM-and-Open-Mainframe-Project-Mobilize-to-Connect-States-with-COBOL-Skills ; https://community.ibm.com/community/user/ibmz-and-linuxone/blogs/salisu-ali/2022/01/10/the-sunsetting-of-a-great-project-ibm-master-mainf | 2026-08-01 |
| "Total live US openings that name Natural/ADABAS number in the low tens at any given moment (roughly 8 to 60...)" | CONFIRMED | Reproduced on 2026-08-01. ZipRecruiter "Adabas Natural": 59 currently hiring. Glassdoor "Natural adabas" US: 28; "Natural adabas developer" US: 8; "natural adabas" remote: 19; bare "Adabas" US: 46. SimplyHired: 16 to 21 depending on phrasing. Dice "adabas": 17 results (2 new). The 8 to 60 band holds. Worth noting as evidence for the file's own "noisy" caveat: Jooble advertises "5440 Adabas natural developer vacancies", which is a loose-match artifact and demonstrates why board totals cannot be taken at face value. | https://www.ziprecruiter.com/Jobs/Adabas-Natural ; https://www.glassdoor.com/Job/natural-adabas-jobs-SRCH_KO0,14.htm ; https://www.dice.com/jobs/q-adabas-jobs | 2026-08-01 |
| "contractor rates cluster around $55 to $86/hour" / "stated pay bands in the ~$57 to ~$86/hour range" | PARTIALLY CORRECT | The top of the range is not currently reproducible and the bottom is too high. Observed on 2026-08-01: ZipRecruiter's "Adabas Natural" band is $57 to $84/hr; the narrower "Adabas Natural Developer" band is $47 to $69/hr; "Adabas Natural Programming" is $47 to $84/hr; a November 2025 capture of the same page showed $57 to $69/hr, so the band moves substantially. No $86 figure surfaced. The Atlanta detail does reproduce exactly: "As of Jul 9, 2026, the average hourly pay for remote adabas natural in Atlanta, GA is $64.25... Most workers in this role earn between $55.72 and $80.67 per hour." Defensible restatement: roughly $47 to $84/hr, clustering near $64/hr. | https://www.ziprecruiter.com/Jobs/Adabas-Natural ; https://www.ziprecruiter.com/Jobs/Adabas-Natural-Developer ; https://www.ziprecruiter.com/Salaries/Adabas-Natural-Salary | 2026-08-01 |
| "some permanent/senior roles quoted $103K to $120K/year; one 'senior part-time' aggregate skewed lower (~$80K average...)" | UNVERIFIED | Could not reproduce these specific figures on any board. ZipRecruiter's "Senior Part Time Adabas Natural Developer" page exists but the quoted salary values did not surface. Treat as unsourced until a dated screenshot or capture backs them. | https://www.ziprecruiter.com/Jobs/Senior-Part-Time-Adabas-Natural-Developer | 2026-08-01 |
| "A well-made independent Natural course would have very little direct competition" / "almost no competing structured instruction outside Software AG's own paywalled/vendor materials" | **REFUTED** | Independent commercial Natural/ADABAS training already exists in volume. Verified live providers: MaxMunus, igmGuru, Koenig Solutions, Verhoef Training (UK, "3 days. Hands on.", on-site or virtual via Zoom with remote labs), Vistasparks Solutions, and Nisa Trainings. Shiksha aggregates a listing titled "Top 56 natural adabas Courses & Certifications Online". This is a thin market with several incumbents, not an empty one. | https://verhoef-training.co.uk/system-z-programming/adabas-natural-programming ; https://www.maxmunus.com/page/NATURAL-ADABAS-Training ; https://www.igmguru.com/it/natural-adabas-training ; https://www.koenig-solutions.com/natural-adabas-training | 2026-08-01 |
| "especially where Software AG's official training is priced high" | **REFUTED** | The vendor's foundational Natural training is free. Software AG's Digital Essentials program offers free self-paced modules covering Adabas, Natural Programming, NaturalONE and webMethods ApplinX, and awards a "Software AG Certified digital badge" on completion. This is the strongest single competitor to a paid independent course, because it is free **and** carries vendor certification that an independent course cannot issue. The file's Section 1 acknowledges the free vendor materials and then Section 5 argues the opposite; the document contradicts itself. Partial mitigation: user comments on the announcement report that some courses labelled free were actually instructor-led and paid, and the post was corrected in February 2023, so "free" is not universal across the catalog. | https://techcommunity.softwareag.com/t/introducing-free-training-essentials-for-adabas-natural/259320 | 2026-08-01 |
| "a small four-figure to low five-figure total addressable population worldwide" | UNVERIFIED (correctly labeled) | This is the file's own inference and it labels it as such, which is honest. The only anchors are Enlyft's 3,873 companies and Sumble's 757 organizations, both produced by inference-based vendor trackers with no published methodology. No primary installed-base figure exists. The file's statement that "No public figure gives a precise global customer count" is accurate and should stay. | https://enlyft.com/tech/products/software-ag-adabas | 2026-08-01 |

---

## Corrections required

Ordered by severity. Items 1 through 4 are defects that misrepresent cited research or invert a commercial conclusion. Items 5 onward are accuracy and dating fixes.

**1. Line 75. Rewrite the Futurum entry. It is currently a misquote that inverts the source.**

Replace:
> - **Futurum Group, 2024 Global Mainframe Skills Report:** 79% of organizations are struggling to fill mid-career legacy roles as the aging workforce retires. (Cited via Software AG skills material.)

With:
> - **Futurum Group, 2024 Global Mainframe Skills Report:** 79% of employers are focusing recruitment on mid-career mainframe professionals, which the report frames as demand and "a shortage of deep experience in the field," not as difficulty filling roles. Read the whole report before leaning on it: its headline conclusion cuts against the retirement narrative. It states that "the key research findings do not align with an aging workforce narrative," that "finding mainframe skills is no harder for enterprises than finding cybersecurity or AI skills," and that early-career inflow "challenges the myth of an aging workforce." It also reports a 65% rise in available skilled mainframe workers over five years and 91% of employers planning mainframe hires within two years. The report never mentions Natural or COBOL, and never uses the word "retire." Sponsored by IBM, Broadcom and 21CS; surveys of 750 businesses, 200 universities (screened to schools already offering mainframe curriculum) and 200 students, 2024-02-20 to 2024-03-03. (Source: Futurum Group report PDF, verified directly 2026-08-01.)

**2. Line 34. Fix the misattribution and the altered quote.**

Replace:
> Software AG's own skills blog states plainly that "most universities no longer offer related instruction, since no one would dream of using these technologies for greenfield development projects."

With:
> Modern Systems / OneAdvanced, a vendor that sells migration off the platform, writes that "Most universities no longer offer mainframe instruction since no one would dream of using these technologies for greenfield development projects." Weigh it accordingly: the claim comes from a party with a commercial interest in the platform looking obsolete, and it concerns mainframe instruction generally rather than Natural specifically. Neither Software AG skills blog makes this claim.

**3. Sections 1, 5 and the executive summary. Retract the "almost no competition" claim.**

The file currently asserts near-absent competition twice (line 39 and line 119) and additionally asserts that Software AG training is "priced high" (line 110). All three are wrong.

- Line 39. Replace "A well-made independent Natural course would have very little direct competition" with a statement that at least six independent providers sell Natural/ADABAS training today (MaxMunus, igmGuru, Koenig Solutions, Verhoef Training, Vistasparks, Nisa Trainings), that Shiksha aggregates a "Top 56 natural adabas Courses" listing, and that the competitive gap is quality, price and self-serve format rather than absence.
- Line 110. Delete "especially where Software AG's official training is priced high." Replace with the fact that Software AG's Digital Essentials program gives away self-paced Adabas, Natural Programming and NaturalONE modules and issues a Software AG Certified digital badge, so a paid independent course must beat free-plus-vendor-certification on quality and format. Note the caveat that some courses labelled free are in fact instructor-led and paid.
- Line 119. Amend "almost no direct competition" in the recommendation to match.

**4. Line 61 and line 125. Add the ownership change beside the 2050+ commitment.**

Keep the commitment (it is real, dated 2016-08-31), but append: webMethods and StreamSets were sold to IBM in July 2024, and since January 2025 Adabas & Natural has operated as a standalone business under Silver Lake ownership through the Software GmbH holding. The 2016 pledge was made by a publicly listed company that no longer exists in that form, and the live 2050+ page no longer carries the explicit wording. Calling it "the single most important durability signal" without this context overstates it.

**5. Line 22 and line 125. Date the UT Austin evidence and add the current corroboration.**

Change the present-tense "UT Austin runs its Interactive Degree Audit (IDA) system on Software AG Natural and ADABAS" to make the provenance explicit: the Natural/ADABAS/Amdahl/1985 detail comes from a 1999 EDUCAUSE paper by Mark Long and Brent Heustess of the UT Austin Registrar, where "currently runs" means 1999. Then add the current corroboration the file is missing: a UT Austin "Sr Software Developer/Analyst" posting requiring "proven experience developing in the UT environment with Natural and ADABAS", plus Sumble's 2026-06-08 profile. Move UT Austin from "Fact (sourced)" to "well-corroborated, primary source is historical" in the line 125 quick-reference.

**6. Line 23 and line 125. Qualify the Sumble ranking.**

State that Sumble is a sales-intelligence platform inferring technology use from job postings and people profiles, that it publishes no methodology for the "teams" metric and never defines "team", and that UT Austin leads only on that metric (12 teams, 5 people) while Cognizant leads on headcount (6 teams, 14 people). Drop or soften "single most Natural/ADABAS-heavy organization it tracks".

**7. Line 50. Date the Alaska RFI.**

Add "issued 2020-12-11 by the Department of Administration, Office of Information Technology." Remove or soften "confirming an active state deployment" to "confirming a deployment active as of late 2020."

**8. Line 88, 92, 99, 125. Correct the rate range.**

Replace "$55 to $86/hour" and "~$57 to ~$86/hour" with "roughly $47 to $84/hour, clustering near $64/hour." Keep the Atlanta figures, which reproduce exactly, and cite them as "as of 2026-07-09 per ZipRecruiter." Note that ZipRecruiter's band for the same query was $57 to $69/hr in November 2025, so the band is volatile and should always be dated.

**9. Line 92. Mark the annual-salary figures as unverified or drop them.**

"$103K to $120K/year" and the "~$80K average" senior part-time figure could not be reproduced. Either capture a dated source or remove them.

**10. Line 36. Add the sunset note to Master the Mainframe.**

Both IBM figures are verbatim correct but are 2019 numbers from an April 2020 release. Add that Master the Mainframe was sunset in September 2021 and shut down 2021-12-30, succeeded by IBM Z Xplore.

**11. Line 52. Balance the UI-systems claim.**

Add that Texas Workforce Commission is replacing its mainframe UI system with Sagitec's Neosurance COTS product. State UI is both a stronghold and an active migration target; presenting only the first half overstates the durability of that segment.

**12. Sources section. Replace two dead links and add archive links.**

- The EDUCAUSE URL returns 403 on direct fetch. The file already notes this. Add the working archive URL: `https://web.archive.org/web/20240912205110/https://www.educause.edu/ir/library/html/cmr9907/cmr9907.html`
- The IBM newsroom URL now returns **404**, not just a fetch block. Replace with `https://web.archive.org/web/20260316184710/https://newsroom.ibm.com/2020-04-09-IBM-and-Open-Mainframe-Project-Mobilize-to-Connect-States-with-COBOL-Skills`
- georgenet.net direct is unreliable; add the 2021 archive capture.
- Add the Forrester primary source (Compuware press release, 2018-03-22) rather than routing the statistic through the Modern Systems eBook.
- Add the Futurum report PDF as a direct source, since the file currently cites it only through Software AG material that does not contain it.

**13. Line 61. Fix the two product names.**

"Natural for VS Code" and "Natural AI Code Assist" should be "Natural for Visual Studio Code" and "Natural AI Code Assistant (working title)", and both should be marked as unreleased with a planned October 2026 release under the Co-Innovation Program.

**14. Line 57. Add the Enlyft company count.**

Enlyft reports 3,873 companies using Software AG Adabas. Citing it makes the "few thousand detectable sites" inference traceable instead of asserted.

---

## What survived unchanged

These held up and need no edit:

- The core UT Austin technical facts, verbatim, subject to the dating fix.
- The Forrester 2018 23%/63% statistic, now verified against the primary press release.
- The job-volume characterization of "low tens", reproduced across five boards.
- The finding that no university teaches Natural for credit. Re-tested independently and more broadly; it stands.
- CDT's ADABAS service figures, verbatim.
- Enlyft 0.18%, Sumble 757 organizations, the $2 billion insurance case study, the IBM 120-schools and 4,286-students figures, the georgenet site list contents, and the CalSTRS and AAFMAA customer names.
- The file's honest labeling of the addressable-population estimate as inference, and its statement that no public installed-base figure exists.

## Effect on the go/no-go

The demand side of the thesis is intact. The supply-of-competition side is not.

Volume, rates, concentration in government and insurance, contract-heavy hiring, and the absence of any academic feeder all verified. What changed is that two of the three pillars supporting "build it" are weaker than written. The acute-retirement framing leaned on a Futurum report whose actual conclusion is that mainframe skills are no harder to hire than cybersecurity or AI skills. The "no competition" claim is false, and the most important competitor is the vendor's own free certified course.

That does not sink the project, but it does move it. The differentiator can no longer be "nobody else teaches this." It has to be format and quality: an interactive browser-based course with a live interpreter, against incumbents selling instructor-led classroom days and a free vendor course that is slide-and-video self-study. That is a defensible position, and it is a narrower one than the file currently claims.

---

## Sources

Primary and archival:

- EDUCAUSE CMR9907, Mark Long and Brent Heustess, UT Austin Office of the Registrar, "Storing Degree Audit Data" (1999): https://web.archive.org/web/20240912205110/https://www.educause.edu/ir/library/html/cmr9907/cmr9907.html
- IBM Newsroom, "IBM and Open Mainframe Project Mobilize to Connect States with COBOL Skills", 2020-04-09 (live URL now 404): https://web.archive.org/web/20260316184710/https://newsroom.ibm.com/2020-04-09-IBM-and-Open-Mainframe-Project-Mobilize-to-Connect-States-with-COBOL-Skills
- Compuware / Forrester Consulting, "Modern Mainframe KPIs Are Key To A Successful Digital Strategy", 2018-03-22: https://www.einpresswire.com/article/438209464/compuware-survey-shows-critical-mainframe-workloads-increasing-while-mainframe-staff-losses-remain-unfilled
- The Futurum Group, 2024 Global Mainframe Skills Report (full PDF, text extracted and searched): https://devops.com/wp-content/uploads/2024/08/TFG_Mainframe-60-Skills-Research-Report_v1.pdf
- Futurum report landing page: https://futurumgroup.com/research-reports/global-mainframe-skills-report-insights-from-industry-and-educational-experts/
- Software AG, "Adabas & Natural 2050" agenda press release, 2016-08-31: http://www1.softwareag.com/us/Press/pressreleases/20160831_Adabas_Natural_2050_Agenda.asp and https://www.pressebox.com/pressrelease/software-ag-en/Software-AG-Announces-New-Adabas-Natural-2050-Agenda/boxid/812403
- Software GmbH, Adabas & Natural and ARIS launch as standalone businesses, 2025-01-07: https://www.prnewswire.com/news-releases/software-gmbh-announces-adabas--natural-and-aris-will-launch-as-standalone-businesses-closing-of-sales-of-alfabet-and-cumulocity-departure-of-group-ceo-sanjay-brahmawar-and-new-central-leadership-302344983.html
- Software AG Tech Community, Adabas & Natural release information Oct. 2025 (Natural for Visual Studio Code, Natural AI Code Assistant, October 2026): https://techcommunity.softwareag.com/t/adabas-natural-release-information-oct-2025/311504
- Software AG Tech Community, "Introducing Free Training Essentials for Adabas & Natural", 2022-07-28: https://techcommunity.softwareag.com/t/introducing-free-training-essentials-for-adabas-natural/259320
- California Department of Technology, ADABAS service: https://www.cdt.ca.gov/services/database-adabas/
- Alaska Online Public Notices, RFI for Software AG maintenance and support, 2020-12-11: https://aws.state.ak.us/OnlinePublicNotices/Notices/View.aspx?id=200753
- georgenet.net, "Sites using ADABAS in the U.S.", last updated 2000-03-06: https://web.archive.org/web/20210505220230/http://georgenet.net/misc/adabas.html
- IBM Community, "Sunset of Master the Mainframe, Dawn of IBM Z Xplore": https://community.ibm.com/community/user/ibmz-and-linuxone/blogs/salisu-ali/2022/01/10/the-sunsetting-of-a-great-project-ibm-master-mainf

Vendor and case material:

- Software AG, "Insurance Leader Reduces Risk with Software AG": https://www.softwareag.com/en/resources/adabas-natural/insurance-risk-reduction/
- Software AG, "Building the next generation of Adabas & Natural experts", 2026-03-24: https://www.softwareag.com/en/blog/skills/building-the-next-generation-of-adabas-natural-experts/
- Software AG, "Your experts will retire. Their knowledge doesn't have to.", 2026-04-22: https://www.softwareag.com/en/blog/skills/modernizing-natural-development-devops/
- Modern Systems / OneAdvanced, "Natural Selection" eBook: https://modernsystems.oneadvanced.com/globalassets/modern-systems-assets/resources/ebook/natural-and-adabas-modernization-ebook.pdf
- Modern Systems / OneAdvanced, "How to easily evolve beyond Software AG's Natural and Adabas": https://modernsystems.oneadvanced.com/news-and-opinion/how-to-easily-evolve-beyond-software-ags-natural-and-adabas/
- Sagitec, Texas Workforce Commission UI modernization: https://www.sagitec.com/pension-software-company/press-releases/texas-workforce-commission-chooses-sagitec-to-modernize-their-unemployment-insurance-system

Third-party trackers (secondary, inference-based, no published methodology):

- Sumble, Natural/ADABAS, last updated 2026-06-08: https://sumble.com/tech/natural-adabas
- Sumble company background: https://techcrunch.com/2025/10/22/sumble-emerges-from-stealth-with-38-5m-to-bring-ai-powered-context-to-sales-intelligence/
- Enlyft, Software AG Adabas: https://enlyft.com/tech/products/software-ag-adabas
- AppsRunTheWorld, Software AG Adabas & Natural customers: https://www.appsruntheworld.com/customers-database/products/view/software-ag-adabas-natural

Job boards, all queried 2026-08-01:

- ZipRecruiter Adabas Natural (59 listings, $57 to $84/hr): https://www.ziprecruiter.com/Jobs/Adabas-Natural
- ZipRecruiter Adabas Natural Developer ($47 to $69/hr): https://www.ziprecruiter.com/Jobs/Adabas-Natural-Developer
- ZipRecruiter Adabas Natural salary (Atlanta $64.25/hr average, $55.72 to $80.67): https://www.ziprecruiter.com/Salaries/Adabas-Natural-Salary
- Glassdoor Natural adabas, United States (28): https://www.glassdoor.com/Job/natural-adabas-jobs-SRCH_KO0,14.htm
- Glassdoor Natural adabas developer, United States (8): https://www.glassdoor.com/Job/natural-adabas-developer-jobs-SRCH_KO0,24.htm
- Glassdoor Adabas, United States (46): https://www.glassdoor.com/Job/adabas-jobs-SRCH_KO0,6.htm
- SimplyHired natural adabas developer (16 to 21): https://www.simplyhired.com/search?q=natural+adabas+developer
- Dice adabas (17 results): https://www.dice.com/jobs/q-adabas-jobs

Competing training providers:

- Verhoef Training, Adabas Natural Programming (3 days, hands on): https://verhoef-training.co.uk/system-z-programming/adabas-natural-programming
- MaxMunus NATURAL ADABAS Training: https://www.maxmunus.com/page/NATURAL-ADABAS-Training
- igmGuru Natural ADABAS Training: https://www.igmguru.com/it/natural-adabas-training
- Koenig Solutions, Natural and Adabas System Administration Training: https://www.koenig-solutions.com/natural-adabas-training
- Shiksha, "Top 56 natural adabas Courses & Certifications Online": https://www.shiksha.com/online-courses/natural-adabas-certification
- edX Adabas landing page (no actual Adabas or Natural courses): https://www.edx.org/learn/adabas
