# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/zbrox/linkding-rs/compare/v0.2.0...v0.3.0) - 2025-05-19

### Added

- some tweaks to the assets endpoints
- add a couple of examples
- rename error variant to be more fitting
- add support for bookmark assets

### Fixed

- query string parameters were not appended correctly

### Other

- use release-plz for publishing
- update dependencies
- change example downloaded asset extension
- replace ureq with blocking reqwest
- update dependencies
- some linting
- Update changelog
## 0.2.0 - 2025-03-29
### Bug Fixes
- Make some needed fields public

### Miscellaneous
- Bump up version to 0.2.0

## 0.1.0 - 2024-11-17
### Features
- Add some simple uniffi bindings setup
- Implement list arguments and archived bookmarks listing
- Add get_bookmark function
- Add checking of url, updating and creating a bookmark
- Add archive, unarchive, delete bookmark
- Add user profile and tag functions
- Small changes to the interface and visibility
- Add all possible variants for user profile enumerated values

### Bug Fixes
- Correct responses for unarchive/unarchive

### Miscellaneous
- Add MIT license file
- Add missing metadata in cargo.toml

### Documentation
- Add some inline documentation
- Add an initial readme
- Add more info in readme
- Fix CI badge in readme

### Refactor
- Split in several modules

