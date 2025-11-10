/**
 * Example 5: API key lifecycle and authentication check
 *
 * This example demonstrates how to:
 * - Initialize storage so the API key collection file exists
 * - Create API keys (with and without a userId)
 * - List API keys (all and active only)
 * - Retrieve keys by userId and by publicKey
 * - Check authentication using public/private key pair
 * - Deactivate, activate, and remove an API key
 *
 * How to run:
 * 1) Save this file as: examples/example5-api-usage.js
 * 2) Run with Node.js (v14+): node examples/example5-api-usage.js
 */

import path from 'path';
import { fileURLToPath } from 'url';
import { initStorage } from '../todozi/storage.js';
import {
  createApiKey,
  createApiKeyWithUserId,
  getApiKey,
  getApiKeyByPublic,
  listApiKeys,
  listActiveApiKeys,
  checkApiKeyAuth,
  deactivateApiKey,
  activateApiKey,
  removeApiKey
} from '../todozi/api.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// ---------- Utilities ----------
const sleep = (ms) => new Promise(res => setTimeout(res, ms));
const pretty = (obj) => JSON.stringify(obj, null, 2);

function log(...args) {
  console.log('[example5]', ...args);
}

function logJson(label, obj) {
  console.log(`[example5] ${label}:\n${pretty(obj)}\n`);
}

async function safe(fn, ...args) {
  try {
    return await fn(...args);
  } catch (err) {
    log('Error:', err?.message || err);
    return null;
  }
}

// ---------- Demo ----------
async function main() {
  log('Initializing storage (creates ~/.todozi and supporting files)...');
  await initStorage();

  // Create an API key without specifying a userId
  log('\n1) Creating a new API key (auto userId)...');
  const keyA = await createApiKey();
  logJson('Created keyA', keyA);

  // Create an API key with a specific userId
  log('\n2) Creating API key for user "alice"...');
  const keyB = await createApiKeyWithUserId('alice');
  logJson('Created keyB (for alice)', keyB);

  // Small delay to show updatedAt differences if filesystem timestamps are granular
  await sleep(50);

  // Create a third key for demonstration
  log('\n3) Creating API key for user "bob"...');
  const keyC = await createApiKeyWithUserId('bob');
  logJson('Created keyC (for bob)', keyC);

  // List all keys
  log('\n4) Listing all API keys...');
  const allKeys = await listApiKeys();
  logJson('All API keys (clones)', allKeys);

  // List only active keys
  log('\n5) Listing only active API keys...');
  const activeKeys = await listActiveApiKeys();
  logJson('Active API keys (clones)', activeKeys);

  // Retrieve by userId
  log('\n6) Getting API key for user "alice" by userId...');
  const aliceKey = await getApiKey('alice');
  logJson('Alice key (clone)', aliceKey);

  // Retrieve by publicKey
  log('\n7) Getting API key by publicKey (bob)...');
  const bobByPublic = await getApiKeyByPublic(keyC.publicKey);
  logJson('Bob key by public (clone)', bobByPublic);

  // Auth checks
  log('\n8) Checking auth for Alice with PUBLIC key only (read-only)...');
  const [userIdA, isAdminA] = await checkApiKeyAuth(aliceKey.publicKey, null);
  log(`auth result => userId=${userIdA}, isAdmin=${isAdminA}`);

  log('\n9) Checking auth for Alice with PUBLIC + PRIVATE key (admin)...');
  const [userIdA2, isAdminA2] = await checkApiKeyAuth(aliceKey.publicKey, aliceKey.privateKey);
  log(`auth result => userId=${userIdA2}, isAdmin=${isAdminA2}`);

  // Deactivate a key
  log('\n10) Deactivating "alice" API key...');
  await deactivateApiKey('alice');
  log('Deactivated alice.');

  // Verify inactive key cannot authenticate
  log('\n11) Attempting auth with deactivated Alice key (should fail)...');
  const aliceCheckAfterDeactivate = await safe(checkApiKeyAuth, aliceKey.publicKey, aliceKey.privateKey);
  if (!aliceCheckAfterDeactivate) {
    log('Auth failed as expected: invalid/disabled key');
  }

  // Activate the key again
  log('\n12) Re-activating "alice" API key...');
  await activateApiKey('alice');
  log('Activated alice.');

  // Remove a key
  log('\n13) Removing "bob" API key...');
  const removed = await removeApiKey('bob');
  logJson('Removed key (bob)', removed);

  // Final state: show active keys
  log('\n14) Final list of active keys...');
  const finalActive = await listActiveApiKeys();
  logJson('Final active keys', finalActive);

  // Auth again for alice after re-activation
  log('\n15) Final auth check for alice (admin)...');
  const [finalUserId, finalIsAdmin] = await checkApiKeyAuth(aliceKey.publicKey, aliceKey.privateKey);
  log(`final auth => userId=${finalUserId}, isAdmin=${finalIsAdmin}`);

  log('\n✅ Example 5 completed successfully.');
}

main().catch((err) => {
  console.error('[example5] Fatal error:', err?.stack || err);
  process.exit(1);
});