#!/usr/bin/env node
/**
 * route-parity-check.mjs — fails CI if this SDK drifts from the 25 documented
 * Robinhood Chain API routes.
 *
 * This standalone crate is NOT scanned by the monorepo's sdk-route-parity guard,
 * so it ships its own. The 25 `/rhc/*` routes (23 GET + 2 batch POST) are pinned
 * below (from the "MadeOnSol — Robinhood Chain API" OpenAPI spec). The check:
 *
 *   1. Every `/rhc/*` path literal in src/ MUST resolve to one of the 25 routes.
 *   2. All 25 routes MUST be referenced by the client (coverage).
 *
 * Non-`/rhc/*` infra paths (WebSocket-token issuance under /stream/*) are
 * allow-listed — they are shared endpoints, not RHC-specific.
 *
 *   node scripts/route-parity-check.mjs
 *
 * Exit 0 = in parity · 1 = drift (unknown route or missing coverage) · 2 = setup error.
 */
import { readdirSync, statSync, readFileSync } from "node:fs";
import path from "node:path";

const SRC = process.env.SRC_DIR || "src";

// The 25 documented Robinhood Chain routes, normalized ({param} -> :p).
const RHC_ROUTES = [
  "/rhc/kol/feed",
  "/rhc/kol/leaderboard",
  "/rhc/kol/hot-tokens",
  "/rhc/kol/coordination",
  "/rhc/kol/first-touches",
  "/rhc/kol/:p",
  "/rhc/trades",
  "/rhc/tokens",
  "/rhc/tokens/:p",
  "/rhc/tokens/:p/candles",
  "/rhc/tokens/:p/kol-consensus",
  "/rhc/tokens/:p/buyer-quality",
  "/rhc/tokens/:p/bundle",
  "/rhc/token/batch",
  "/rhc/tokens/batch/buyer-quality",
  "/rhc/deployer-hunter/leaderboard",
  "/rhc/deployer-hunter/alerts",
  "/rhc/deployer-hunter/best-tokens",
  "/rhc/deployer-hunter/recent-bonds",
  "/rhc/deployer-hunter/stats",
  "/rhc/deployer-hunter/:p",
  "/rhc/deployer-hunter/:p/trajectory",
  "/rhc/deployer-hunter/:p/tokens",
  "/rhc/deployer-hunter/:p/history",
  "/rhc/alpha-wallets",
];
const ROUTES = new Set(RHC_ROUTES);

// Shared infra paths that are legitimately not `/rhc/*` (WS streaming issuance).
const ALLOW = new Set([
  "/stream/token",
  "/stream/sessions",
  "/stream/sessions/:p",
]);

function normalize(p) {
  let s = p.split(/[?#]/)[0];
  s = s.replace(/\$\{[^}]*\}/g, ":p").replace(/\{[^}]*\}/g, ":p"); // ${x}, {x}, {} (rust format!) -> :p
  s = s.replace(/\/+$/, "");
  return s;
}

function walk(dir) {
  const out = [];
  for (const e of readdirSync(dir)) {
    if (["node_modules", "target", "dist", "build", "examples", "tests", "test"].includes(e)) continue;
    const f = path.join(dir, e);
    if (statSync(f).isDirectory()) out.push(...walk(f));
    else if (f.endsWith(".rs")) out.push(f);
  }
  return out;
}

const STR = /"([^"]*)"|`([^`]*)`/g;
const problems = [];
const seen = new Set();
let count = 0;
for (const file of walk(SRC)) {
  readFileSync(file, "utf8")
    .split("\n")
    .forEach((line, i) => {
      for (const m of line.matchAll(STR)) {
        const raw = m[1] ?? m[2] ?? "";
        const c = raw.replace(/^\/api\/(?:v1|x402)/, ""); // strip prefix if present
        if (!/^\/[a-z][a-z0-9-]*(\/|$)/.test(c)) continue; // API-path-shaped only
        count++;
        const norm = normalize(c);
        if (!norm || norm === "/") continue;
        if (ALLOW.has(norm)) continue;
        if (ROUTES.has(norm)) {
          seen.add(norm);
          continue;
        }
        problems.push(`${path.relative(".", file)}:${i + 1}  "${raw}"  ->  ${norm}`);
      }
    });
}

if (count < 5) {
  console.error(`route-parity: only ${count} path literals found in ${SRC}/ — extractor may be broken`);
  process.exit(2);
}

const missing = RHC_ROUTES.filter((r) => !seen.has(r));
const uniqProblems = [...new Set(problems)];

let failed = false;
if (uniqProblems.length) {
  failed = true;
  console.error(`\n❌ route-parity: ${uniqProblems.length} SDK path(s) not among the ${RHC_ROUTES.length} RHC routes:\n  ${uniqProblems.join("\n  ")}\n`);
}
if (missing.length) {
  failed = true;
  console.error(`\n❌ route-parity: ${missing.length} documented RHC route(s) not referenced by the client:\n  ${missing.join("\n  ")}\n`);
}
if (failed) process.exit(1);

console.log(`✅ route-parity: all ${seen.size}/${RHC_ROUTES.length} RHC routes covered; ${count} path literals resolve.`);
