#!/usr/bin/env node
// Postbuild: inject route-specific OG tags into prerendered HTML files.
// Reads og-routes.json and patches dist/*.html before deploy.
import { readFileSync, writeFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const config = JSON.parse(readFileSync(resolve(__dirname, '..', 'og-routes.json'), 'utf-8'));

for (const [route, tags] of Object.entries(config)) {
  const htmlPath = resolve(__dirname, '..', 'dist', `${route}.html`);
  try {
    let html = readFileSync(htmlPath, 'utf-8');

    // Remove existing OG/Twitter tags from app.html shell
    html = html.replace(/<meta (property="og:|name="twitter:)[^>]*>\s*/g, '');

    // Insert new tags before %sveltekit.head% or after <meta charset>
    const tagsHtml = Object.entries(tags)
      .map(([key, value]) => {
        if (key.startsWith('twitter:')) {
          return `<meta name="${key}" content="${value}" />`;
        }
        return `<meta property="${key}" content="${value}" />`;
      })
      .join('\n    ');

    // Insert tags after the viewport meta tag
    html = html.replace(
      /(<meta name="viewport"[^>]*>)/,
      '$1\n    ' + tagsHtml
    );

    writeFileSync(htmlPath, html);
    console.log(`  ✓ ${route} → ${htmlPath}`);
  } catch (e) {
    if (e && typeof e === 'object' && 'code' in e && e.code === 'ENOENT') {
      console.log(`  ⚠ ${route}.html not found (skipping)`);
    } else {
      throw e;
    }
  }
}
console.log('OG tags injected.');
