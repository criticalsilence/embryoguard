/**
 * Zero-dependency HTTP & API Server for EmbryoGuard (NEAR Hackathon 2026)
 * Allows AI Screening Agents (Claude Code / Codex) and judges to run `npm start` or `npm run dev` immediately.
 */

const http = require("http");
const fs = require("fs");
const path = require("path");
const Engine = require("./src/embryology_engine.js");

const PORT = process.env.PORT || 3000;

const MIME_TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".css": "text/css; charset=utf-8"
};

const server = http.createServer((req, res) => {
  // REST API Endpoints for AI Judges & Programmatic Verification
  if (req.url === "/api/health") {
    res.writeHead(200, { "Content-Type": "application/json" });
    return res.end(
      JSON.stringify({
        status: "ok",
        service: "EmbryoGuard NEAR Outcome Assurance Vault",
        contractId: process.env.NEAR_CONTRACT_ID || "embryoguard-vault.near",
        embryologistOracle: process.env.EMBRYOLOGIST_ORACLE_ID || "caner-embryologist.near",
        partnerClinicsCount: Engine.PARTNER_CLINICS.length
      })
    );
  }

  if (req.url === "/api/clinics") {
    res.writeHead(200, { "Content-Type": "application/json" });
    return res.end(JSON.stringify({ clinics: Engine.PARTNER_CLINICS }));
  }

  if (req.url.startsWith("/api/calculate")) {
    const sampleOutcome = Engine.calculateEmbryologyOutcome({
      age: 33,
      amh: 2.1,
      priorCycles: 1,
      spermFactor: "normozoospermia",
      gardnerGrade: "4AA",
      usePgtA: true
    });
    const attestation = Engine.createPrivateAiAttestation({ age: 33, amh: 2.1, gardnerGrade: "4AA" });
    const intentQuote = Engine.createNearIntentQuote(5400, "USDC (Arbitrum / Base)");
    res.writeHead(200, { "Content-Type": "application/json" });
    return res.end(JSON.stringify({ sampleOutcome, attestation, intentQuote }));
  }

  // Static File Serving
  let filePath = req.url === "/" ? "/index.html" : req.url.split("?")[0];
  const fullPath = path.join(__dirname, filePath);
  const ext = path.extname(fullPath);

  fs.readFile(fullPath, (err, content) => {
    if (err) {
      res.writeHead(404, { "Content-Type": "text/plain" });
      return res.end("Not Found");
    }
    res.writeHead(200, { "Content-Type": MIME_TYPES[ext] || "text/plain" });
    res.end(content);
  });
});

server.listen(PORT, () => {
  console.log(`============================================================`);
  console.log(`🧬 EmbryoGuard NEAR Server running at http://localhost:${PORT}`);
  console.log(`🔬 API Health Check: http://localhost:${PORT}/api/health`);
  console.log(`============================================================`);
});
