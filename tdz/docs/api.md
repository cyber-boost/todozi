# `todozi api`

Manage API keys that the Todozi server uses for authentication and authorization.
The `api` command provides a simple way to create, list, verify, activate, deactivate, and remove API keys without hand‑editing the `tdz.hlx` configuration file.

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **register** `--user-id <USER_ID>` | Generate a new API key pair (public & private). If `USER_ID` is omitted a random UUID is used. |
| **list** `--active-only` | Show all stored keys; with `--active-only` only keys that are currently active are displayed. |
| **check** `--public-key <PUB> [--private-key <PRIV>]` | Validate an API key (public key is mandatory, private key optional). Returns the associated user ID and whether the key has admin privileges. |
| **deactivate** `--user-id <USER_ID>` | Mark the key belonging to `USER_ID` as inactive – it can no longer be used for write operations. |
| **activate** `--user-id <USER_ID>` | Reactivate a previously deactivated key. |
| **remove** `--user-id <USER_ID>` | Permanently delete the key pair from the configuration. This action cannot be undone. |

## Usage examples

```sh
# Create a new key (random user ID)
todozi api register
# → Public: abc123… Private: xyz789…

# Create a key for a known user
todozi api register --user-id my-service

# List all keys
todozi api list

# List only active keys
todozi api list --active-only

# Verify a key (public key only)
todozi api check --public-key abc123…

# Verify a key (both public and private)
todozi api check --public-key abc123… --private-key xyz789…

# Deactivate a key
todozi api deactivate --user-id my-service

# Reactivate a key
todozi api activate --user-id my-service

# Remove a key permanently
todozi api remove --user-id my-service
```

## How it works

1. **Storage** – API keys are stored in `~/.todozi/tdz.hlx` under the `api_keys` namespace. Each entry contains:
   - `user_id` – Identifier chosen at registration.
   - `public_key` – UUID‑like string used for read‑only access.
   - `private_key` – Secret used for admin operations.
   - `active` – Boolean flag indicating whether the key may be used.
   - `created_at` / `updated_at` – Timestamps for audit purposes.

2. **Security** – The private key is never written to logs. When a key is generated, the private portion is displayed **once** on the console and must be saved by the operator.

3. **Authentication flow** – The server expects an HTTP header:
   ```
   Authorization: Bearer <public_key>
   ```
   If a request attempts a mutating operation (`POST`, `PUT`, `DELETE`) and the corresponding key has `active: true` **and** includes the private key in the header:
   ```
   X-Private-Key: <private_key>
   ```
   the request is allowed; otherwise it is rejected with a 403.

4. **Lifecycle management** –
   - `register` creates a fresh pair and writes it to the HLX file.
   - `list` reads all entries and formats a table (using `tabled`).
   - `check` looks up the public key, optionally validates the private key, and reports the user ID and admin status.
   - `activate` / `deactivate` toggle the `active` flag.
   - `remove` deletes the entry entirely.

## When to use `api`

- **Automation** – When you run external services (CI pipelines, bots, or custom integrations) that need to interact with the Todozi server programmatically.
- **Least‑privilege** – Create separate keys for read‑only vs. admin use‑cases; deactivate or rotate them when no longer needed.
- **Auditing** – The timestamps allow you to track when keys were issued, modified, or revoked.

## Related commands

- `todozi server` – The server validates incoming requests using the keys managed here.
- `todozi register` – Registers the client with `todozi.com` (unrelated to local API keys but useful for cloud‑based features).
- `todozi backup` – Take a backup of `tdz.hlx` before bulk changes to API keys.

For a complete list of top‑level commands, see **`docs/cmd.md`**.
