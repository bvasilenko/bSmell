#!/usr/bin/env node
const { spawnSync } = require('node:child_process');
const path = require('node:path');
const os = require('node:os');
const fs = require('node:fs');
const candidates = [
  path.join(os.homedir(), '.cargo', 'bin', 'bsmell'),
  '/usr/local/bin/bsmell',
  'bsmell',
];
let exe = candidates.find((p) => p === 'bsmell' || (fs.existsSync(p) && fs.statSync(p).isFile())) || 'bsmell';
const result = spawnSync(exe, process.argv.slice(2), { stdio: 'inherit' });
process.exit(result.status ?? 1);
