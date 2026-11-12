# EmbryoGuard — Clinical Embryology Advisory & Outcome-Aligned IVF Escrow on NEAR

- **Live Web Application:** [https://cukka.org/embryoguard/](https://cukka.org/embryoguard/)
- **Repository:** [https://github.com/criticalsilence/embryoguard](https://github.com/criticalsilence/embryoguard)
- **Author & Lead Clinical Embryologist:** Caner Karaogul, M.Sc. ([@criticalsilence](https://github.com/criticalsilence) · `canerkaraogul@gmail.com`)
- **Hackathon Tracks:** `Real-World Money` · `Autonomous Business` · `Privacy & Private AI` · `Unstoppable DeFi`
- **License:** [MIT](./LICENSE)

---

## 1. Mission: Democratizing IVF Pricing & Outcome Fairness

In conventional Assisted Reproductive Technology (ART) and cross-border fertility care, **every patient is charged the exact same flat upfront fee ($4,500–$12,000+ per cycle) despite having vastly different biological probabilities of success**.

A 30-year-old couple with normal ovarian reserve and normozoospermia may have a >60% single-cycle prognosis, whereas a 41-year-old couple with diminished ovarian reserve (`AMH < 1.0 ng/mL`), severe male factor, and multiple prior failed cycles faces a significantly lower per-cycle probability—yet both couples are required to wire 100% of the clinic fee upfront and bear 100% of the financial loss if the cycle fails.

**EmbryoGuard** is built to democratize fertility treatment financing and restore clinical transparency through three pillars:

1. **Individualized, Outcome-Linked Refund Ratios (Not Fixed Fees):** Instead of a rigid all-or-nothing payment or a static refund tier, EmbryoGuard computes an individualized preliminary indicator from core clinical parameters and pairs it with a **dynamic outcome refund ratio ranging from 20% to 65%**.
2. **Independent Senior Embryologist Consultation & Clinic Matching:** Preliminary estimates are explicitly marked as non-final. Before any financial commitment is made, diagnostic records are reviewed directly by a Senior Clinical Embryologist who determines the appropriate laboratory protocol (`ICSI`, `NGS PGT-A`, `vitrification`) and refers the couple to an audited partner clinic.
3. **Staged Milestone Escrow & Automated Refunds on NEAR Protocol:** Patient funds are never wired upfront to a clinic bank account. Instead, payments are held in a programmable escrow contract (`embryoguard-vault.testnet`) settled via **NEAR Intents**, releasing funds according to each patient's **dynamic split**: if a patient's calculated outcome refund ratio is $R\%$ (`20%–65%`), then $R\%$ is locked in Stage 3 until pregnancy outcome (automatically refunded to the patient if negative), while the remaining $(100 - R)\%$ is split across Stage 1 (oocyte retrieval) and Stage 2 (embryo transfer).

---

## 2. About the Founder & Lead Clinical Embryologist

**Caner Karaogul, M.Sc.** is a Senior Clinical Embryologist with **over 15 years of international experience** in clinical embryology, andrology, micromanipulation, quality control, and IVF laboratory management.

### Academic & Professional Credentials
- **M.Sc. in Histology & Embryology** — Fırat University Medical Faculty, Turkey (2009–2011)
- **B.Sc. in Molecular Biology & Genetics** — Haliç University, Istanbul, Turkey (2001–2005)
- **Embryology Laboratory Certification** — Ministry of Health of the Republic of Turkey (2011–2012)
- **Scientific Congresses & Training** — ASRM Annual Meeting (Hawaii, 2014), IFFS/ASRM Joint Congress (Boston, 2013), ESHRE Annual Meeting (Istanbul, 2012)

### International Clinical & Laboratory Leadership
Over the past 15+ years, Caner Karaogul has established, supervised, and optimized high-volume IVF and genetic screening laboratories across Asia, Europe, and the Mediterranean:
- **Siam IVF, Paolo Memorial Hospital (Bangkok, Thailand):** Managed full embryology and andrology laboratory operations and integrated routine Next-Generation Sequencing (NGS) genetic screening.
- **Gynolife IVF & Egg Donation Center (Nicosia, Cyprus):** Served as Laboratory Supervisor overseeing a team of embryologists across 60–70 embryo transfer cycles per month and high-volume oocyte vitrification programs.
- **International Turkish Hospital (Tirana, Albania):** Led clinical embryology services, managing 30 oocyte pick-up (OPU) and transfer procedures per month in compliance with international standards.
- **Acıbadem Hospital Group, Bahçeci IVF Group & Fırat University Hospital (Turkey):** Established the university IVF laboratory at Fırat University and practiced in high-volume Turkish ART centers handling up to 400 OPU procedures, 50 TESE procedures, and 300 embryo transfers per month.

### Verified ART Laboratory Key Performance Indicators (KPIs)

| ART Laboratory KPI | Clinical Calculation Formula | Caner Karaogul Competency Value | Vienna Consensus Benchmark |
| :--- | :--- | :---: | :---: |
| **ICSI Damage Rate** | Damaged or degenerated oocytes / all injected oocytes × 100 | **0.004%** | ≤ 10% |
| **ICSI Normal Fertilization Rate** | Oocytes with 2PN and 2PB / MII oocytes injected × 100 | **84.6%** | ≥ 65% |
| **Cleavage Rate (Day 2)** | Cleaved embryos on Day 2 / 2PN oocytes on Day 1 × 100 | **98.8%** | ≥ 95% |
| **Day-3 Embryo Development Rate** | 8-cell embryos on Day 3 / normally fertilized oocytes × 100 | **82.8%** | ≥ 45% |
| **Day-5 Blastocyst Development Rate** | Blastocysts on Day 5 / normally fertilized oocytes × 100 | **63.2%** | ≥ 40% |
| **Blastocyst Cryosurvival Rate** | Intact blastocysts after warming / total warmed × 100 | **99.0%** | ≥ 90% |

---

## 3. Clinical Assessment Methodology & WHO (2021) Standards

EmbryoGuard guides patients through a calm, four-step clinical workflow (available in **English** by default with full **Turkish** localization):

### Step 1: Preliminary Assessment Parameters
Patients enter four foundational parameters without needing to interpret complex laboratory terminology:
1. **Maternal Age (20–45 years):** Primary biological determinant of oocyte euploidy and implantation potential.
2. **AMH (Anti-Müllerian Hormone, 0.2–5.5 ng/mL):** Quantitative marker of ovarian reserve (`Low < 1.0`, `Normal 1.0–3.5`, `High > 3.5 ng/mL`), including an option for patients who have not yet tested their AMH.
3. **Semen Analysis (WHO 6th Edition, 2021 Lower Reference Limits):** Categorized according to the **World Health Organization (WHO) Laboratory Manual for the Examination and Processing of Human Semen, 6th Edition (2021)** 5th percentile lower reference thresholds:
   - **Normal (Normozoospermia):** Concentration `≥ 16 million/mL` and progressive motility `≥ 30%` (total motility `≥ 42%`).
   - **Mildly reduced:** Concentration `5–15 million/mL` and progressive motility `15%–29%`.
   - **Significantly reduced:** Concentration `< 5 million/mL` and progressive motility `< 15%`.
   - **Azoospermia:** No spermatozoa observed in the ejaculate.
   - **Not tested / unknown:** Semen analysis has not been performed yet.
4. **Prior Unsuccessful IVF Cycles (`None`, `1`, `2`, `3+`):** Accounts for recurrent implantation failure or prior ovarian response history.

### Step 2: Non-Final Preliminary Indicator & Dynamic Refund Ratio
- Displays a clinical report table with the **Preliminary Success Indicator (%)** and the **Outcome Refund Ratio (%)** (scaling dynamically between `20%` and `65%`).
- Explicitly informs the patient that the preliminary calculation is **not a final medical prognosis** and will be refined after the Lead Embryologist reviews their full diagnostic file.

### Step 3: Direct Clinical Embryologist Consultation Request
- Attaches the patient's preliminary parameters to a structured consultation request sent directly to Caner Karaogul, M.Sc. via **Telegram** and **Email**, with strict client-side XSS sanitization (`createElement` / `textContent`) and data privacy consent.

### Step 4: Dynamic Staged Escrow & Outcome-Aligned Refund Schedule
- Breaks down how the treatment package (`USDC`, `USDT`, `NEAR`, `SOL`, or `BTC` via NEAR Intents) is dynamically partitioned in escrow according to the patient's calculated refund ratio $R\%$ (`20%–65%`):
  - **Stage 1 ($\lfloor(100 - R)/2\rfloor\%$):** Released upon treatment initiation and oocyte retrieval (OPU).
  - **Stage 2 ($\lceil(100 - R)/2\rceil\%$):** Released upon embryo culture and transfer procedure.
  - **Stage 3 — Outcome Lock ($R\%$):** Held in escrow until clinical pregnancy verification. If the outcome is positive, $R\%$ is released to the clinic; if negative, that exact $R\%$ (`20%–65%` of total package) is returned directly from escrow to the patient's wallet.

---

## 4. NEAR Ecosystem Integration ("The Tools to Bring Your Idea to Life")

EmbryoGuard integrates the official tools and infrastructure featured in the NEAR Hackathon builder toolkit:

| NEAR Ecosystem Tool | How EmbryoGuard Uses It |
| :--- | :--- |
| **NEAR Intents** | Enables patients to lock IVF escrow packages and receive automated outcome refunds in `USDC`, `USDT`, `NEAR`, `SOL`, or `BTC` without manual bridging. |
| **NEAR.AI** | Powers the Step 3 Consultation Concierge Chat Assistant, answering workflow and contact questions while strictly abstaining from unverified medical advice. |
| **HOT Protocol & NEAR Testnet (`near-api-js`)** | Provides chain-abstracted Testnet wallet onboarding (`patient-ivf.testnet` / `MyNearWallet Testnet`) and live JSON-RPC synchronization against `https://rpc.testnet.near.org`. |
| **Trezu, NEAR Business & SputnikDAO** | Multi-party escrow treasury structure governing the dynamic 3-stage milestone releases ($\lfloor(100-R)/2\rfloor\%$ OPU, $\lceil(100-R)/2\rceil\%$ Embryo Transfer, and $R\%$ Clinical Outcome Lock). |
| **OutLayer** | Enforces transaction guardrails on `embryoguard-vault.testnet` so that negative-outcome refunds (`20%–65%`) can only be routed back to the patient's verified account. |

---

## 5. Technical Architecture & Repository Structure

```text
embryoguard-near/
├── index.html                                # Bilingual (EN default / TR) 4-Step Clinical Web Interface
├── styles.css                                # Custom clinical design system (Source Serif 4, IBM Plex Sans/Mono)
├── admin.html                                # Standalone notification & contact configuration page
├── server.js                                 # Zero-dependency Node.js HTTP & REST API server
├── src/
│   └── embryology_engine.js                  # WHO 2021 Preliminary Assessment & Dynamic Refund Engine
├── contracts/
│   └── ivf_outcome_vault/
│       ├── Cargo.toml                        # NEAR SDK v5 Rust Smart Contract manifest
│       └── src/lib.rs                        # Staged Milestone Escrow, Embryologist Attestation & Refund Contract
├── test/
│   └── verify.js                             # Automated verification test suite (`npm test`)
└── package.json                              # Zero-dependency scripts (`npm start`, `npm test`)
```

---

## 5. Reproducible Build & Verification Instructions

### Run Automated Verification Tests (Node.js)
```bash
npm test
```

### Start the Local Web & API Server
```bash
npm start
# Server listens on http://localhost:3000
# Health endpoint: http://localhost:3000/api/health
```

### Run NEAR Rust Smart Contract Unit Tests
```bash
cd contracts/ivf_outcome_vault
cargo test
```
