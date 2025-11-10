#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Demo script that shows a typical “day in the life” of a Todozi user:

1.  Starts the async HTTP server (in the background).
2.  Creates a project, a few tasks and an agent.
3.  Sends a rich chat message that contains many different Todozi tags.
4.  Stores the parsed objects in the persistent Storage implementation.
5.  Runs a few queries: list tasks, search by text, show tag statistics.
6.  Adds a tiny CLI extension (`export‑tasks`) that writes all tasks to CSV.

Everything is self‑contained – you can copy‑paste this file into the repo and run