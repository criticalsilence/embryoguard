/**
 * Automated Verification Suite for EmbryoGuard (NEAR Hackathon 2026)
 * Executed via `npm test` by Claude Code / Codex screening agents and human judges.
 */

const assert = require("assert");
const Engine = require("../src/embryology_engine.js");

console.log("🔬 Running EmbryoGuard Preliminary Assessment & Dynamic NEAR Refund Verification Suite...\n");

// Test 1: Preliminary Assessment & Dynamic Refund Rate for Normal Parameters
const caseHigh = Engine.calculatePreliminaryAssessment({
  age: 30,
  amh: 2.4,
  amhKnown: true,
  priorCycles: 0,
  spermCategory: "normal"
});

assert.strictEqual(caseHigh.isFinal, false, "Assessment must explicitly be marked as non-final preliminary estimate");
assert(caseHigh.preliminaryEstimatePct >= 55, "Preliminary estimate for age 30 / normal sperm should be >=55%");
assert(caseHigh.dynamicRefundPct >= 45, "Dynamic refund rate should scale with preliminary estimate");
console.log("✅ [PASS] Test 1: Preliminary Estimate & Dynamic Refund Rate:", caseHigh.preliminaryEstimatePct + "% / Refund:", caseHigh.dynamicRefundPct + "%");

// Test 2: Dynamic Refund Rate Changes Based on Parameters (Not Static!)
const caseLow = Engine.calculatePreliminaryAssessment({
  age: 41,
  amh: 0.5,
  amhKnown: true,
  priorCycles: 2,
  spermCategory: "severe_low"
});

assert(
  caseLow.preliminaryEstimatePct < caseHigh.preliminaryEstimatePct,
  "Lower reserve and reduced sperm motility/count must yield lower preliminary estimate"
);
assert(
  caseLow.dynamicRefundPct !== caseHigh.dynamicRefundPct,
  "Refund rate must be dynamic and change according to the calculated preliminary ratio"
);
console.log("✅ [PASS] Test 2: Dynamic Refund Scaling Verified:", caseLow.preliminaryEstimatePct + "% / Refund:", caseLow.dynamicRefundPct + "%");

// Test 3: NEAR Intents Quote with Dynamic Refund
const quote = Engine.createNearIntentQuote(5000, "USDC", caseHigh.dynamicRefundPct);
assert.strictEqual(quote.usdEquivalent, 5000);
assert.strictEqual(quote.dynamicRefundPct, caseHigh.dynamicRefundPct);
console.log("✅ [PASS] Test 3: NEAR Intents Quote & Dynamic Refund Amount: $" + quote.refundUsd);

console.log("\n🎉 ALL EMBRYOGUARD VERIFICATION TESTS PASSED SUCCESSFULLY!");
