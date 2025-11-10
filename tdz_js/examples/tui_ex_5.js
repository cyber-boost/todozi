/* tag_assistant.js - Example 5: Tag Assistant and Report Generator
import fs from 'fs';
import path from 'path';
import os from 'os';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

 *
 * This example demonstrates:
 * - Using TagManager and TagSearchEngine from tags.js
 * - Loading tasks from storage.js
 * - Extracting candidate tags from task text
 * - Fuzzy suggestions and auto-merge of duplicate tags
 * - Generating a Markdown report and optional JSON export
 *
 * Usage:
 *   node tag_assistant.js
 *   node tag_assistant.js --out report.md --json tags_export.json --minUsage 2 --fuzzy 2 --add --verbose
 */


/* Dynamic local imports so this example stays self-contained and executable.
 * If tags.js or models.js are not present, it will still run and report what it needs.
 */
async function importLocal(moduleName) {
  try {
    return await import(moduleName);
  } catch (e) {
    throw new Error(`Missing local module ${moduleName}. Place tags.js and models.js next to this script.\n${e.message}`);
  }
}

function stopwords() {
  return new Set([
    'a','an','and','the','is','to','of','in','for','on','with','at','from','by','as','be','are','or','if','then','than','that',
    'this','it','its','into','over','under','between','within','without','about','around','through','after','before',
    'can','could','should','would','may','might','will','won\'t','shouldn\'t','need','needs','just','also','not','no','yes',
    'do','does','did','done','has','have','had','was','were','been','being','i','we','you','they','he','she','them','our','your',
    'make','made','making','get','got','getting','go','went','gone','going','see','saw','seen','seein','take','took','taken',
    'work','works','working','used','using','use','uses','add','adding','added','create','creates','created','creating',
    'build','built','building','update','updates','updated','updating','fix','fixes','fixed','fixing','refactor','refactoring',
    'implement','implements','implemented','implementing','design','designs','designed','designing','review','reviews','reviewed',
    'reviewing','test','tests','testing','tested','deploy','deploys','deployed','deploying','ship','ships','shipped','shipping',
    'note','notes','notes:','todo','task','tasks','item','items','thing','things'
  ]);
}

function tokenizeText(text) {
  if (!text) return [];
  return text
    .toLowerCase()
    .replace(/[^a-z0-9_\-#\s]/g, ' ')
    .split(/\s+/)
    .filter(Boolean)
    .filter(t => !stopwords().has(t))
    .map(t => t.replace(/^#+/, ''))
    .filter(t => t.length >= 3);
}

function topKeywords(text, max = 8) {
  const freq = new Map();
  for (const tok of tokenizeText(text)) {
    freq.set(tok, (freq.get(tok) || 0) + 1);
  }
  return [...freq.entries()]
    .sort((a, b) => b[1] - a[1])
    .slice(0, max)
    .map(([t]) => t);
}

function normalizeTagName(name) {
  return name
    .toLowerCase()
    .replace(/[_\s]+/g, '-')
    .replace(/[^a-z0-9\-]/g, '')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '');
}

function dedupeTags(tags) {
  const seen = new Set();
  const out = [];
  for (const t of tags) {
    const n = normalizeTagName(t);
    if (!n) continue;
    if (seen.has(n)) continue;
    seen.add(n);
    out.push(n);
  }
  return out;
}

function suggestTagMerge(candidates) {
  const pairs = [];
  const syn = [
    ['dev', 'development', 'develop'],
    ['qa', 'testing', 'tests'],
    ['ops', 'operate', 'operations'],
    ['ui', 'ux', 'design'],
    ['security', 'secure', 'sec'],
    ['database', 'db', 'sql', 'postgres', 'mysql', 'sqlite'],
    ['cli', 'terminal', 'console'],
    ['server', 'backend', 'api'],
    ['bug', 'fix', 'fixes'],
  ];

  for (const group of syn) {
    const set = new Set(group.map(g => normalizeTagName(g)));
    const found = candidates.filter(c => set.has(c));
    if (found.length > 1) {
      const primary = found.sort((a, b) => a.length - b.length)[0];
      const dups = found.filter(x => x !== primary);
      if (dups.length) pairs.push([primary, dups]);
    }
  }
  return pairs;
}

async function loadTasksFromStorage(storageDir) {
  const containers = await listProjectTaskContainers(storageDir);
  const tasks = [];
  for (const c of containers) {
    for (const t of c.getAllTasks()) {
      tasks.push(t);
    }
  }
  return tasks;
}

async function listProjectTaskContainers(storageDir) {
  const projectTasksDir = path.join(storageDir, 'project_tasks');
  const containers = [];
  if (!fs.existsSync(projectTasksDir)) return containers;
  const files = fs.readdirSync(projectTasksDir).filter(f => f.endsWith('.json'));
  for (const file of files) {
    const content = fs.readFileSync(path.join(projectTasksDir, file), 'utf8');
    const data = JSON.parse(content);
    // Heuristic: reconstruct a container with a getAllTasks method
    const container = {
      projectName: data.projectName || file.replace('.json', ''),
      getAllTasks: () => {
        const all = [
          ...Object.values(data.activeTasks || {}),
          ...Object.values(data.completedTasks || {}),
          ...Object.values(data.archivedTasks || {}),
          ...Object.values(data.deletedTasks || {}),
        ];
        return all;
      }
    };
    containers.push(container);
  }
  return containers;
}

function parseArgs(argv) {
  const args = { _: [] };
  for (let i = 2; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--out') args.out = argv[++i];
    else if (a === '--json') args.json = argv[++i];
    else if (a === '--minUsage') args.minUsage = parseInt(argv[++i], 10) || 1;
    else if (a === '--fuzzy') args.fuzzy = parseInt(argv[++i], 10) || 2;
    else if (a === '--add') args.add = true;
    else if (a === '--verbose') args.verbose = true;
    else if (a === '--help') args.help = true;
    else args._.push(a);
  }
  return args;
}

function printUsage() {
  console.log('Tag Assistant (Example 5)');
  console.log('');
  console.log('Usage: node tag_assistant.js [options]');
  console.log('Options:');
  console.log('  --out <file>      Write Markdown report to file');
  console.log('  --json <file>     Write JSON export to file');
  console.log('  --minUsage <n>    Minimum suggested tag usage (default 1)');
  console.log('  --fuzzy <n>       Fuzzy max distance for suggestions (default 2)');
  console.log('  --add             Apply suggestions and persist to storage');
  console.log('  --verbose         Print extra diagnostics');
  console.log('  --help            Show this help');
}

async function main() {
  const args = parseArgs(process.argv);
  if (args.help) {
    printUsage();
    process.exit(0);
  }

  // Import modules from todozi
  const tagsMod = await import('../todozi/tags.js');
  const TagManager = tagsMod.TagManager;
  const TagSearchEngine = tagsMod.TagSearchEngine;

  const { getStorageDir, initStorage, loadConfig } = await import('../todozi/storage.js');

  let storageDir;
  try {
    storageDir = await getStorageDir();
  } catch (e) {
    console.log('Storage directory not found. Initializing default storage...');
    await initStorage();
    storageDir = await getStorageDir();
  }

  if (args.verbose) console.log(`Storage dir: ${storageDir}`);

  // Ensure config exists
  await loadConfig();

  // Load existing tags into the manager
  const tagManager = TagManager.new();
  // Optionally load persisted tags if you have a persistence layer; here we simulate none.

  // Load tasks from project task containers
  const tasks = await loadTasksFromStorage(storageDir);

  if (args.verbose) console.log(`Loaded ${tasks.length} tasks from storage`);

  // Build candidate tag set from tasks
  const tagCounts = new Map();
  const seenTags = new Set();

  for (const t of tasks) {
    const text = [
      t.action,
      t.context_notes,
      t.parent_project,
      t.priority,
      t.status,
      t.assignee
    ].filter(Boolean).join(' ');

    const words = topKeywords(text, 10);
    for (const w of words) {
      const tag = normalizeTagName(w);
      if (!tag) continue;
      if (seenTags.has(tag)) {
        tagCounts.set(tag, (tagCounts.get(tag) || 0) + 1);
      } else {
        seenTags.add(tag);
        tagCounts.set(tag, 1);
      }
    }
  }

  // Create Tag instances for existing tag candidates
  // In a real system, you'd load these from persistence; here we create ephemeral tags
  for (const [tag, count] of tagCounts.entries()) {
    if (count > 0) {
      const tagObj = new (tagsMod.Tag)({
        name: tag,
        category: null,
        usage_count: count
      });
      try {
        await tagManager.createTag(tagObj);
      } catch (_) {
        // Ignore duplicates or errors for ephemeral tags
      }
    }
  }

  // Merge obvious duplicates
  const primaryMap = new Map(); // duplicate -> primary
  const candidates = dedupeTags([...tagCounts.keys()]);
  const mergePairs = suggestTagMerge(candidates);
  for (const [primary, dups] of mergePairs) {
    for (const d of dups) {
      primaryMap.set(d, primary);
    }
  }
  if (args.verbose && mergePairs.length) {
    console.log('Auto-merge pairs:', mergePairs);
  }

  // Use TagSearchEngine for fuzzy suggestions based on a seed set
  const seedTags = [...tagManager.getAllTags()].map(t => t.name);
  const searchEngine = TagSearchEngine.new(tagManager);

  // For each seed tag, find fuzzy neighbors and suggest
  const suggestedTags = new Map(); // tag -> { count, reason: 'new'|'fuzzy', sources: [] }
  for (const seed of seedTags) {
    const matches = searchEngine.fuzzySearch(seed, args.fuzzy || 2);
    for (const [tag, dist] of matches) {
      const t = tag.name;
      if (t === seed) continue;
      const existing = suggestedTags.get(t) || { count: 0, reason: 'fuzzy', sources: [] };
      existing.count += 1;
      existing.sources.push(`${seed}@${dist}`);
      suggestedTags.set(t, existing);
    }
  }

  // Add brand-new tags based on frequency
  for (const [t, c] of tagCounts.entries()) {
    if (!suggestedTags.has(t) && c >= (args.minUsage || 1)) {
      suggestedTags.set(t, { count: c, reason: 'new', sources: [] });
    }
  }

  // Convert merge duplicates to primary
  for (const [dup, primary] of primaryMap.entries()) {
    if (suggestedTags.has(dup)) {
      const s = suggestedTags.get(dup);
      suggestedTags.delete(dup);
      const cur = suggestedTags.get(primary) || { count: 0, reason: 'existing', sources: [] };
      cur.count += s.count;
      cur.sources.push(...s.sources);
      suggestedTags.set(primary, cur);
    }
  }

  // Sort suggestions
  const sortedSuggestions = [...suggestedTags.entries()]
    .sort((a, b) => b[1].count - a[1].count);

  // Generate report
  const lines = [];
  lines.push(`# Tag Assistant Report`);
  lines.push('');
  lines.push(`Generated: ${new Date().toISOString()}`);
  lines.push('');
  lines.push(`- Tasks scanned: ${tasks.length}`);
  lines.push(`- Unique tokens found: ${tagCounts.size}`);
  lines.push(`- Tag candidates suggested: ${sortedSuggestions.length}`);
  lines.push('');

  if (sortedSuggestions.length === 0) {
    lines.push(`No tag suggestions at this time.`);
  } else {
    lines.push(`## Suggested Tags`);
    lines.push('');
    lines.push(`| Tag | Count | Reason | Sources |`);
    lines.push(`| --- | ---: | --- | --- |`);
    for (const [tag, meta] of sortedSuggestions) {
      const sources = meta.sources.slice(0, 5).join(', ');
      lines.push(`| ${tag} | ${meta.count} | ${meta.reason} | ${sources} |`);
    }
  }

  lines.push('');
  lines.push(`## Top Keywords (candidates)`);
  lines.push('');
  const top = [...tagCounts.entries()].sort((a, b) => b[1] - a[1]).slice(0, 25);
  for (const [t, c] of top) {
    lines.push(`- ${t} (${c})`);
  }

  lines.push('');
  lines.push(`## How to apply`);
  lines.push('');
  lines.push(`- Review suggested tags above.`);
  lines.push(`- Use your tag manager to create, merge, or remove tags.`);
  if (args.add) {
    lines.push(`- This run used --add. It would apply suggestions in a real persistence layer.`);
  } else {
    lines.push(`- To auto-apply, run with --add. (Requires persistence wiring)`);
  }

  const report = lines.join('\n');

  // Print to console
  console.log(report);

  // Optional: write files
  if (args.out) {
    fs.writeFileSync(args.out, report, 'utf8');
    console.log(`\nWrote report to: ${args.out}`);
  }

  if (args.json) {
    const exportObj = {
      generatedAt: new Date().toISOString(),
      tasksScanned: tasks.length,
      uniqueTokens: tagCounts.size,
      suggestions: sortedSuggestions.map(([tag, meta]) => ({ tag, ...meta })),
      topKeywords: top.map(([t, c]) => ({ tag: t, count: c })),
      mergePairs
    };
    fs.writeFileSync(args.json, JSON.stringify(exportObj, null, 2), 'utf8');
    console.log(`Wrote JSON export to: ${args.json}`);
  }
}

main().catch(err => {
  console.error('Error:', err.message);
  if (process.argv.includes('--verbose')) {
    console.error(err.stack);
  }
  process.exit(1);
});