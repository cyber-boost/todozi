#!/usr/bin/env python3
"""
example1_error_manager.py

A clear, practical example demonstrating:
- ErrorManager usage (create, resolve, filter, stats, export)
- Parsing <error>...</error> strings into Error objects
- Using TodoziError factories (validation, storage, api, etc.)
"""

from __future__ import annotations

import json
import logging
import sys
from datetime import datetime, timezone
from typing import List

# Import the classes from the provided error.py
from error import (
    Error,
    ErrorManager,
    ErrorManagerConfig,
    ErrorSeverity,
    ErrorCategory,
    parse_error_format,
    TodoziError,
)


def setup_logging():
    # Configure logging so ERROR manager doesn't spam stdout/stderr
    logger = logging.getLogger("example1")
    logger.setLevel(logging.INFO)
    handler = logging.StreamHandler(sys.stdout)
    handler.setFormatter(logging.Formatter("%(levelname)s - %(name)s - %(message)s"))
    logger.addHandler(handler)
    return logger


def print_errors(errors: List[Error], title: str = "Errors"):
    print(f"\n{title} ({len(errors)}):")
    for e in errors:
        sev = e.severity.value
        cat = e.category.value
        print(
            f"  - {e.id[:8]} | {sev.upper():8} | {cat:16} | {e.title} | "
            f"{'resolved' if e.resolved else 'unresolved'}"
        )


def main():
    logger = setup_logging()

    # 1) Create an ErrorManager with a small cap to show auto-eviction behavior
    config = ErrorManagerConfig(max_errors=5, auto_cleanup_resolved=True, cleanup_interval_hours=24)
    mgr = ErrorManager(config=config, logger=logger)

    # 2) Create some errors
    e1 = Error(
        title="Database connection failed",
        description="Unable to connect to PostgreSQL database",
        severity=ErrorSeverity.CRITICAL,
        category=ErrorCategory.NETWORK,
        source="database-service",
        context="Connection timeout after 30 seconds",
        tags=["database", "postgres", "connection"],
    )
    e2 = Error(
        title="Invalid API payload",
        description="Missing required field 'email'",
        severity=ErrorSeverity.MEDIUM,
        category=ErrorCategory.VALIDATION,
        source="api-gateway",
        context="POST /v1/users",
        tags=["api", "validation"],
    )
    e3 = Error(
        title="Disk full",
        description="No space left on device",
        severity=ErrorSeverity.URGENT,
        category=ErrorCategory.STORAGE,
        source="fsWatcher",
        context="Mount: /var/data",
        tags=["disk", "storage"],
    )

    id1 = mgr.create_error(e1)
    id2 = mgr.create_error(e2)
    id3 = mgr.create_error(e3)

    # 3) List unresolved and needing attention
    unresolved = mgr.get_unresolved_errors()
    print(f"Unresolved errors: {len(unresolved)}")
    print_errors(unresolved, title="Unresolved")

    needing = mgr.get_errors_needing_attention()
    print_errors(needing, title="Needing attention (CRITICAL or URGENT)")

    # 4) Stats
    stats = mgr.stats()
    print(f"\nStats by category: {stats}")

    # 5) Resolve one
    mgr.resolve_error(id2, "Fixed payload validation to include email")
    print(f"\nResolved error {id2[:8]} with: 'Fixed payload validation to include email'")

    # 6) Create a few more to trigger auto-eviction (max_errors=5)
    for i in range(4):
        extra = Error(
            title=f"Extra error {i+1}",
            description="Filler error to test eviction",
            severity=ErrorSeverity.LOW,
            category=ErrorCategory.SYSTEM,
            source="test",
            tags=["eviction", "test"],
        )
        _ = mgr.create_error(extra)

    # 7) Export all (include resolved)
    exported = mgr.export_errors_json(include_resolved=True)
    print("\nExported JSON (first 500 chars):")
    print(exported[:500] + ("..." if len(exported) > 500 else ""))

    # 8) Demonstrate parsing from <error>...</error> strings
    text = (
        "<error>"
        "Cache miss rate too high; "
        "Redis cache miss rate exceeded threshold; "
        "high; "
        "system; "
        "cache-service; "
        "miss_rate=0.42, threshold=0.25; "
        "cache,redis,performance"
        "</error>"
    )
    try:
        parsed = parse_error_format(text)
        print("\nParsed error from string:")
        print(f"  title: {parsed.title}")
        print(f"  severity: {parsed.severity.value}")
        print(f"  category: {parsed.category.value}")
        print(f"  context: {parsed.context}")
        print(f"  tags: {parsed.tags}")

        # Add it to the manager
        pid = mgr.create_error(parsed)
        print(f"  -> created in manager with id {pid[:8]}")
    except TodoziError as pe:
        print(f"Failed to parse: {pe}")

    # 9) Show final unresolved/needing attention
    print("\nAfter parsing and adding:")
    print_errors(mgr.get_unresolved_errors(), title="Unresolved")
    print_errors(mgr.get_errors_needing_attention(), title="Needing attention")

    # 10) Demonstrate TodoziError factories
    try:
        # Validation example
        raise TodoziError.validation({"field": "email", "value": "not-an-email", "message": "Invalid email format"})
    except TodoziError as ve:
        print(f"\nCaught validation error: {ve.error_code} | {ve.message}")
        print(f"  context: {ve.context}")

    # Storage example
    try:
        raise TodoziError.storage({"path": "/var/data", "message": "No space left on device"})
    except TodoziError as se:
        print(f"\nCaught storage error: {se.error_code} | {se.message}")
        print(f"  context: {se.context}")

    # API example
    try:
        raise TodoziError.api({"endpoint": "/v1/users", "status_code": 400, "message": "Bad request"})
    except TodoziError as ae:
        print(f"\nCaught API error: {ae.error_code} | {ae.message}")
        print(f"  context: {ae.context}")

    print("\n✅ Example completed successfully.")


if __name__ == "__main__":
    main()