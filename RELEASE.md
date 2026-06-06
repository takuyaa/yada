# Releasing yada

This document is for maintainers cutting a new release.

## Prerequisites (one-time)

- crates.io Trusted Publishing configured for this repo:
  Owner `takuyaa` / Repo `yada` / Workflow `release.yml` / Environment `release`
  (Crate Settings → Trusted Publishing on crates.io)

## Release procedure

1. Decide the new version `X.Y.Z` following [SemVer](https://semver.org/).
   Optionally run `cargo semver-checks check-release` to detect API breakage.
2. Update `CHANGELOG.md`:
   - Replace `## [Unreleased]` with `## [X.Y.Z] - YYYY-MM-DD`
   - Add a new empty `## [Unreleased]` above
   - Add a new comparison link at the bottom and update the `[Unreleased]` link
3. Update `version` in `Cargo.toml` to `X.Y.Z`.
4. Commit and push to `master`:

   ```
   git commit -am "Release X.Y.Z"
   git push
   ```
5. Tag and push:

   ```
   git tag X.Y.Z
   git push --tags
   ```
6. The `Release` workflow will:
   - Verify the tag matches `Cargo.toml`
   - Extract the matching `CHANGELOG.md` section as release notes
   - Run tests, then `cargo publish` via Trusted Publishing
   - Create the GitHub Release
7. Verify the published version appears on
   <https://crates.io/crates/yada> and the GitHub Release page.

## Recovering from a failed release

- **Publish failed before the tag was used**: fix the issue on `master`,
  delete the tag locally and on the remote, then recreate it:

  ```
  git push --delete origin X.Y.Z
  git tag -d X.Y.Z
  ```
- **`CHANGELOG.md` section missing for the tag**: the workflow aborts before
  publishing. Add the section, recreate the tag.
- **Already published to crates.io**: crates.io versions are immutable.
  Yank the bad version (`cargo yank --version X.Y.Z`) and release `X.Y.Z+1`.
