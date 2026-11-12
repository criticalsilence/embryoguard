# 🧬 EmbryoGuard: IVF Outcome Assurance Vault & Embryologist Advisory on NEAR

> **Built by a Senior Clinical Embryologist ([@criticalsilence](https://github.com/criticalsilence)) for the NEAR Hackathon 2026**  
> **Live Interactive Demo:** **[https://cukka.org/embryoguard/](https://cukka.org/embryoguard/)**  
> **Repository:** **[https://github.com/criticalsilence/embryoguard](https://github.com/criticalsilence/embryoguard)**  
> **Tracks:** `Real-World Money` • `Autonomous Business` • `Privacy & Private AI` • `Unstoppable DeFi`

---

## 🌍 Problem Statement: Why Cross-Border IVF & Fertility Tourism Needs NEAR

Every year, hundreds of thousands of couples travel abroad (Türkiye, Cyprus, Czechia, Spain) for **In Vitro Fertilization (IVF)** treatments, spending **$5,000 – $15,000+ per cycle**. Today, patients face three severe pain points:

1. **Zero Financial Protection ("What if it fails?"):** Standard IVF clinics require 100% upfront wire/cash payment before treatment begins. If the cycle fails, the patient loses 100% of their savings.
2. **Laboratory Opacity:** Patients are rarely shown the real biological attrition funnel—how many retrieved oocytes (OPU) are mature Metaphase II (MII), how many fertilize via ICSI (2PN), and the true Day-5 Blastocyst Gardner grade (`5AA`, `4AA`, `3BB`) with cryptographic Time-Lapse (`EmbryoScope+`) proof.
3. **Privacy & Cross-Border Payment Friction:** Sensitive fertility labs (AMH, sperm DNA fragmentation DFI, genetic PGT-A reports) are sent over unencrypted WhatsApp chats, and international medical wires incur heavy delays and fees.

---

## 💡 The Solution: EmbryoGuard on NEAR Protocol

**EmbryoGuard** is an **Embryologist-led Outcome Assurance Vault & Advisory Platform** powered by **NEAR Intents**, **NEAR Private AI (TEE)**, and **NEAR Smart Contracts**:

1. **Transparent Embryology Funnel & Shared-Risk Actuarial Engine (`src/embryology_engine.js`):**
   - Evaluates maternal age, AMH ovarian reserve, prior failed IVF cycles, sperm factor (ICSI / IMSI / DFI), Day-5 Gardner Blastocyst grade (`5AA`–`2BC`), and PGT-A euploidy screening inside a privacy-preserving **NEAR AI TEE Enclave**.
   - Visualizes the 6-stage biological funnel (`OPU Oocytes` → `Mature MII` → `ICSI 2PN` → `Day-3 Cleavage` → `Day-5 Blastocyst` → `Euploid Embryo`) and calculates a personalized **50%–55% Automated Refund Guarantee**.
2. **Direct Clinical Embryologist Consultation & Verified Partner Clinic Matching:**
   - Patients consult directly with the Lead Clinical Embryologist + Private AI to understand laboratory standards (`Piezo-ICSI`, `Microfluidics Sperm Chip`, `EmbryoScope+ Time-Lapse`, `RI Witness RFID`) and get referred to verified partner clinics.
3. **NEAR Intents Cross-Chain Milestone Escrow & Automated Outcome Refund (`contracts/ivf_outcome_vault/src/lib.rs`):**
   - Patients fund their IVF package using any asset (`USDC`, `USDT`, `NEAR`, `SOL`, or `BTC` via Chain Signatures) settled via **NEAR Intents**.
   - **Milestone 1 (20%):** Released upon verified Oocyte Pick-Up (OPU).
   - **Milestone 2 (30%):** Released only when the **Lead Embryologist Oracle (`caner-embryologist.near`)** signs the Day-5 Blastocyst Gardner report and `EmbryoScope+` time-lapse SHA-256 hash on-chain.
   - **Milestone 3 (50% — Outcome Assurance Lock):** Held in the vault until the clinical pregnancy ($\beta$-hCG) outcome.
     - **If Positive:** Released to the partner clinic.
     - **If Negative:** **Automatically refunded back to the patient's wallet** in seconds via **NEAR Intents**—zero bureaucracy, zero middlemen.

---

## 🏗️ Architecture & Repository Structure

```text
embryoguard-near/
├── index.html                                # Standalone & Server-ready Interactive Web App (TR / EN)
├── server.js                                 # Zero-dependency Node.js HTTP & REST API Server
├── src/
│   └── embryology_engine.js                  # Clinical Embryology Funnel, Gardner Grading & NEAR Intents Engine
├── contracts/
│   └── ivf_outcome_vault/
│       ├── Cargo.toml                        # NEAR SDK v5 Rust Smart Contract Manifest
│       └── src/lib.rs                        # Milestone Escrow, Embryologist Oracle & Auto-Refund Contract
├── test/
│   └── verify.js                             # Automated Verification Test Suite for AI & Human Judges
├── .env.example                              # Environment configuration template
└── package.json                              # Zero-dependency scripts (`npm start`, `npm test`)
```

---

## 🚀 Quick Start (Zero External Dependencies — Runs in 5 Seconds)

### Option A: Open Directly in Browser (No Node.js Required)
Simply open `index.html` in any modern web browser. All interactive calculators, Embryologist chat, Partner Clinic cards, and the NEAR Smart Contract Vault simulator run immediately.

### Option B: Run with Node.js (For Judges & Automated AI Screening)
```bash
# 1. Run automated clinical & NEAR Intents verification tests
npm test

# 2. Start the web & API server on http://localhost:3000
npm start
```

### Option C: Compile & Test the NEAR Rust Smart Contract
```bash
cd contracts/ivf_outcome_vault
cargo test
```

---

## 🧪 Live Demo Flow (2-Minute Hackathon Walkthrough)

1. **Step 1 (Transparency Funnel):** Adjust Age, AMH, Sperm Factor, and Day-5 Blastocyst Gardner Grade (`5AA` / `4AA`) to see the live 6-stage biological funnel and personalized **50%–55% Refund Guarantee**.
2. **Step 2 (Embryologist Advisory Chat):** Ask clinical questions (or switch to **Embryologist Role** to post direct clinical guidance and clinic referrals).
3. **Step 3 (Verified Partner Clinics):** Inspect laboratory scorecards (Blastocyst conversion rate, Vitrification survival rate, `EmbryoScope+` status) or add a new Partner Clinic using the **+ Yeni Anlaşmalı Klinik Ekle** modal.
4. **Step 4 (NEAR Outcome Assurance Vault):** Select any cross-chain asset (`USDC`, `USDT`, `NEAR`, `SOL`, `BTC`), fund the vault, sign the Day-5 Blastocyst Oracle report, and click **🛡️ Gebelik Negatif (Hastaya Otomatik İade Tetikle!)** to watch the smart contract execute an instant automated refund with `EVENT_JSON` logs.
