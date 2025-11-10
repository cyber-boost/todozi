#!/usr/bin/env python3
"""
tag_cli.py – a tiny command‑line front‑end for todozi.tags.TagManager

Features demonstrated:
  * Persistent storage of tags (JSON file)
  * Create a tag with optional category / colour / description
  * List all tags
  * Full‑text / token search
  * Increment the usage counter (e.g. after a task has been tagged)
  * Merge duplicate tags into a primary one
"""

import argparse
import json
import os
import sys
from pathlib import Path
from typing import List

# ----------------------------------------------------------------------
# Import the TagManager from the Todozi repository
# ----------------------------------------------------------------------
# The repository layout puts `tags.py` in the top‑level package directory.
# Adjust the import path if you run the script from a different location.
repo_root = Path(__file__).resolve().parent
if str(repo_root) not in sys.path:
    sys.path.insert(0, str(repo_root))

from tags import TagManager, TagUpdate, TagSearchQuery, TagSortBy, TagStatistics

# ----------------------------------------------------------------------
# Persistence helpers – dump / load the whole manager state
# ----------------------------------------------------------------------
DB_PATH = repo_root / "tags.db.json"

def load_manager() -> TagManager:
    """Create a TagManager and, if a DB file exists, hydrate it."""
    manager = TagManager()
    if DB_PATH.is_file():
        try:
            with DB_PATH.open("r", encoding="utf-8") as f:
                raw = json.load(f)
            # The JSON format mirrors the internal dicts of TagManager.
            # We simply re‑insert the tags; indexes get rebuilt lazily.
            for tag_dict in raw.get("tags", []):
                # recreate the Tag dataclass (the Tag class lives in tags.py)
                from tags import Tag
                tag = Tag.from_dict(tag_dict)
                # Directly inject – we are inside the same process, so we can
                # bypass the async lock (the manager hasn’t been used yet).
                manager.tags[tag.id] = tag
                if tag.category:
                    manager._add_category_index(tag.category, tag.id)
                manager._add_name_index(tag.name, tag.id)
                manager._rebuild_search_index_for_tag(tag)
        except Exception as exc:
            print(f"⚠️  Failed to read tag DB ({DB_PATH}): {exc}", file=sys.stderr)
    return manager


def dump_manager(manager: TagManager) -> None:
    """Serialise the manager’s tags to JSON."""
    data = {
        "tags": [t.to_dict() for t in manager.get_all_tags()],
    }
    with DB_PATH.open("w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


# ----------------------------------------------------------------------
# CLI command implementations
# ----------------------------------------------------------------------
async def cmd_create(ns: argparse.Namespace) -> None:
    """Create a new tag."""
    mgr = load_manager()
    tag_id = await mgr.create_tag(
        name=ns.name,
        description=ns.description,
        color=ns.color,
        category=ns.category,
    )
    dump_manager(mgr)
    print(f"✅ Tag created – id={tag_id}, name={ns.name}")


def cmd_list(_: argparse.Namespace) -> None:
    """Print every tag (sorted by name)."""
    mgr = load_manager()
    tags = sorted(mgr.get_all_tags(), key=lambda t: t.name.lower())
    if not tags:
        print("📭 No tags stored yet.")
        return
    print(f"📋 {len(tags)} tag(s):")
    for t in tags:
        cat = f"[{t.category}]" if t.category else ""
        col = f"[{t.color}]" if t.color else ""
        print(
            f" • {t.id[:8]} – {t.name} {cat} {col}\n"
            f"   desc: {t.description or '—'} | used: {t.usage_count}"
        )


def cmd_search(ns: argparse.Namespace) -> None:
    """Search tags – tokenised by default, fallback to substring."""
    mgr = load_manager()
    results = mgr.search_tags(ns.query)
    if not results:
        print(f"🔍 No tags match: {ns.query!r}")
        return
    print(f"🔍 {len(results)} result(s) for {ns.query!r}:")
    for t in results:
        print(f" • {t.name} (id={t.id[:8]}, used={t.usage_count})")


async def cmd_inc(ns: argparse.Namespace) -> None:
    """Increment a tag’s usage counter (useful after you have applied it)."""
    mgr = load_manager()
    await mgr.increment_tag_usage(ns.name)
    dump_manager(mgr)
    print(f"🔢 Incremented usage for tag {ns.name!r}")


async def cmd_merge(ns: argparse.Namespace) -> None:
    """Merge a set of duplicate tags into the first one (primary)."""
    mgr = load_manager()
    # Resolve names → ids (TagManager stores tags by UUID)
    primary = mgr.get_tag_by_name(ns.primary)
    if not primary:
        print(f"❌ Primary tag not found: {ns.primary!r}", file=sys.stderr)
        return
    dup_ids: List[str] = []
    for dup_name in ns.duplicates:
        dup = mgr.get_tag_by_name(dup_name)
        if dup:
            dup_ids.append(dup.id)
        else:
            print(f"⚠️  Duplicate tag not found – ignored: {dup_name!r}")
    if not dup_ids:
        print("⚠️  No valid duplicate tags supplied.", file=sys.stderr)
        return

    await mgr.merge_tags(primary.id, dup_ids)
    dump_manager(mgr)
    print(f"🔀 Merged {len(dup_ids)} tag(s) into {primary.name!r}.")
    print(f"   New usage count: {primary.usage_count}")


def cmd_stats(_: argparse.Namespace) -> None:
    """Show simple tag‑statistics (total tags, categories, relationships, avg‑usage)."""
    mgr = load_manager()
    stats: TagStatistics = mgr.get_tag_statistics()
    print("📊 Tag statistics")
    print(f"  total tags          : {stats.total_tags}")
    print(f"  total categories    : {stats.total_categories}")
    print(f"  total relationships : {stats.total_relationships}")
    print(f"  average usage       : {stats.average_usage:.2f}")
    print(f"  relationships/tag   : {stats.relationships_per_tag():.2f}")


# ----------------------------------------------------------------------
# Build the small argparse tree
# ----------------------------------------------------------------------
def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="tag_cli.py",
        description="Tiny front‑end for todozi.tags.TagManager (persistent JSON DB)",
    )
    sub = p.add_subparsers(dest="cmd", required=True)

    # create -----------------------------------------------------------
    create = sub.add_parser("create", help="Create a new tag")
    create.add_argument("name", help="Tag name (must be unique)")
    create.add_argument("--description", "-d", help="Long description")
    create.add_argument("--color", "-c", help="Hex colour like #ff6600")
    create.add_argument("--category", "-g", help="Category bucket")
    create.set_defaults(func=cmd_create)

    # list -------------------------------------------------------------
    sub.add_parser("list", help="List all tags (sorted by name)").set_defaults(func=cmd_list)

    # search -----------------------------------------------------------
    search = sub.add_parser("search", help="Search tags (tokenised or substring)")
    search.add_argument("query", help="Search term")
    search.set_defaults(func=cmd_search)

    # inc --------------------------------------------------------------
    inc = sub.add_parser(
        "inc",
        help="Increment usage counter – call after you have applied the tag to a resource",
    )
    inc.add_argument("name", help="Tag name to increment")
    inc.set_defaults(func=cmd_inc)

    # merge ------------------------------------------------------------
    merge = sub.add_parser(
        "merge",
        help="Merge duplicate tags into a primary tag. The primary tag keeps the usage count.",
    )
    merge.add_argument("primary", help="Primary tag name (kept)")
    merge.add_argument(
        "duplicates",
        nargs="+",
        help="One or more duplicate tag names that will be merged *into* the primary",
    )
    merge.set_defaults(func=cmd_merge)

    # stats ------------------------------------------------------------
    sub.add_parser("stats", help="Show tag statistics").set_defaults(func=cmd_stats)

    return p


# ----------------------------------------------------------------------
# Main entry point – tiny shim that forwards to the async functions
# ----------------------------------------------------------------------
def main(argv: List[str] | None = None) -> int:
    parser = build_parser()
    ns = parser.parse_args(argv)

    # Some commands are async (create, inc, merge) – we run them in the event‑loop.
    # Others are pure sync.
    try:
        if ns.cmd in ("create", "inc", "merge"):
            import asyncio
            asyncio.run(ns.func(ns))
        else:
            ns.func(ns)
        return 0
    except KeyboardInterrupt:
        print("\n👋 Interrupted by user", file=sys.stderr)
        return 130
    except Exception as exc:
        print(f"❌ Error: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())