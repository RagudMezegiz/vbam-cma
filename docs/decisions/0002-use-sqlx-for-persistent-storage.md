# Use SQLx for Persistent Storage

## Context and Problem Statement

Some form of persistent storage is required to hold campaign data. Said storage should not require a server or special user permissions. SQLite is a suitable cross-platform format that uses a single file.

## Considered Options

* SQLx
* rusqlite
* sqlite

## Decision Outcome

Chosen option: "SQLx", because it supports not just SQLite but also other databases and is asynchronous to not interfere with the UI thread.
