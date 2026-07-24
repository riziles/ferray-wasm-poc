#!/usr/bin/env node
// Convert og SVGs to PNGs using sharp.
import sharp from 'sharp';
import { readFileSync, writeFileSync } from 'fs';
import { resolve, dirname, basename } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const staticDir = resolve(__dirname, '..', 'static');

const files = process.argv.slice(2);
if (files.length === 0) {
  console.log('Usage: node scripts/svg2png.js file1.svg [file2.svg ...]');
  process.exit(0);
}

for (const file of files) {
  const svgPath = resolve(staticDir, file);
  const pngPath = svgPath.replace(/\.svg$/, '.png');
  try {
    const svg = readFileSync(svgPath);
    await sharp(svg).png().toFile(pngPath);
    console.log(`  ✓ ${file} → ${basename(pngPath)}`);
  } catch (e) {
    console.error(`  ✗ ${file}: ${e.message}`);
  }
}
