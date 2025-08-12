# Changelog

All notable changes to this project will be documented in this file.

## [0.1.6] - 2025-08-12

### 🚀 Features

- :zap: More extensive language color capture

### 📚 Documentation

- :memo: Update CHANGELOG.md

### ⚙️ Miscellaneous Tasks

- :bookmark: Release v0.1.6
## [0.1.5] - 2025-08-12

### 🐛 Bug Fixes

- :ambulance: Update colors URL
- :adhesive_bandage: Add language case variants

### ⚙️ Miscellaneous Tasks

- :memo: Update CHANGELOG.md
- :bookmark: Release v0.1.5
## [0.1.4] - 2025-08-10

### 🚀 Features

- :technologist: Better debug prints
- :sparkles: Add demo link
- :sparkles: Add why repo detail\

### 🐛 Bug Fixes

- :art: Clippy fix

### 🚜 Refactor

- :recycle: Clippy fix
- :rotating_light: Clippy fixes

### ⚙️ Miscellaneous Tasks

- :bookmark: Release v0.1.4
## [0.1.3] - 2025-08-07

### 📚 Documentation

- :memo: Update CHANGELOG.md

### ⚙️ Miscellaneous Tasks

- :arrow_up: Update dependencies
- :bookmark: Release v0.1.3
## [0.1.2] - 2024-08-23

### 🚀 Features

- :sparkles: Add Logo and Highlight images to metadata

### 🎨 Styling

- :art: Clear Clippy

### ⚙️ Miscellaneous Tasks

- :bookmark: Bump version
## [0.1.1] - 2024-08-22

### 🐛 Bug Fixes

- :ambulance: Don't uppercase meta values

### 📚 Documentation

- :memo: Update CHANGELOG.md

### ⚙️ Miscellaneous Tasks

- :bookmark: Bump version
## [0.1.0] - 2024-08-22

### 🚀 Features

- :tada: Initial Commit
- :sparkles: Initial rust workflow
- :sparkles: Pass github token to rust
- :sparkles: Delete artifact after deployment
- :sparkles: Cache any files in ./.cache
- :sparkles: Query Interface
- :sparkles: Implement query interface for github
- :sparkles: README text to repo struct
- :sparkles: object serialization and compression
- :construction: Project class for grouping Repos
- :sparkles: Add ExpandedCache to interpret repo sets at runtime
- :sparkles: Add self check for days old
- :sparkles: Fetch lanuage colors from GitHub
- :construction: Working example with Zola and Hyde theme
- :lipstick: Highlight links
- :sparkles: Use repo Logos
- :sparkles: Expand repo and projects with urls

### 🐛 Bug Fixes

- :construction_worker: use gh-pages for deployment
- :construction: remove cargo to rename
- :wrench: update to Cargo.toml
- :wrench: output to local public folder instead of root
- :wrench: add write permission
- :wrench: Update permissions
- :bug: Use default github-pages artifact name
- :bug: Delete cache before saving it
- :bug: Open test file in append mode
- :bug: pass token for auth
- :bug: update permissions
- :bug: Only match word chars for regex key
- :wrench: assume page directory
- :bug: Continue on delete error for cache
- :art: Use millis epoch everywhere
- :alien: Update for localsavefile
- :art: re-export lsf
- :art: Clean up Clippy warnings
- :bug: Fix async runtimes
- :pushpin: Remove dependency path

### 💼 Other

- :heavy_plus_sign: Add all dependencies
- :heavy_minus_sign: Remove unused dependencies
- *(deps)* Bump prantlf/delete-cache-action from 2 to 3
- :heavy_minus_sign: Remove slug
- :building_construction: Make library and update dependencies
- :pushpin: Up versions
- :heavy_minus_sign: Remove dev dependency

### 🚜 Refactor

- :art: Cleanup imports and comments
- Rename file in println
- :bug: Use epoch time u64 instead of date strings
- :recycle: Output single error on _load
- :recycle: Modularize and split up functions
- :recycle: Itemize cachable entries into their own structs
- :art: Further generalize caching
- :lipstick: Restructure site layout
- :wastebasket: Just use repos.html
- :recycle: Switch to using localsavefile for cache
- :recycle: Integrate GitHub adapter
- :wastebasket: remove bincode
- :recycle: Clear up Clippy warnings

### 📚 Documentation

- :memo: TODO: Optimize for memory in map used for colors

### 🎨 Styling

- :art: Format code

### 🧪 Testing

- :construction: Test page generation
- :construction: Test updating page
- Test updating cache
- :white_check_mark: Basic inital tests
- :construction: test main
- :construction: Add example usage
- :construction: Move cache test to own function
- :heavy_minus_sign: use GITHUB_TOKEN for query test
- :heavy_minus_sign: Remove more test dependencies
- :white_check_mark: Properly mark tests
- :white_check_mark: Properly mark tests
- :white_check_mark: Update tests
- :white_check_mark: Make test async
- :coffin: Remove full use case example

### ⚙️ Miscellaneous Tasks

- :see_no_evil: Ignore .env files
- :wrench: Add inital cargo.toml
- :see_no_evil: ignore page dir
- :wrench: Use github actions for pages
- :arrow_up: Update actions/cache to v4
- :wrench: Always update cache
- :wrench: Run workflow every two weeks
- :see_no_evil: ignore .env for testing
- Add TODO: reduce query using cache
- :see_no_evil: ignore .cache
- :art: Format code
- :see_no_evil: Ignore zola and related for now
- :building_construction: rename to reposcrape
- :wrench: Remove workflow
- :wrench: Add Cargo.lock
- :green_heart: Add default CI/CD
- :memo: Rename to reposcrape
