# Releasing yada

This document is for maintainers cutting a new release.

## Prerequisites (one-time)

- crates.io Trusted Publishing configured for this repo:
  Owner `takuyaa` / Repo `yada` / Workflow `release.yml` / Environment `release`
  (Crate Settings → Trusted Publishing on crates.io)

## Release procedure

1. Decide the new version `X.Y.Z` following [SemVer](https://semver.org/).
   Optionally run `cargo semver-checks check-release` (install with
   `cargo install cargo-semver-checks`) to detect API breakage.
2. Create a release branch off `master` (e.g. `git switch -c release-X.Y.Z`
   or `wt switch -c release-X.Y.Z --base master`).
3. Update `CHANGELOG.md`:
   - Replace `## [Unreleased]` with `## [X.Y.Z] - YYYY-MM-DD`
   - Add a new empty `## [Unreleased]` above
   - Add a new comparison link at the bottom and update the `[Unreleased]` link
4. Update `version` in `Cargo.toml` to `X.Y.Z`.
5. Commit, push the branch, open a `Release X.Y.Z` PR against `master`,
   wait for CI to pass, then merge it.
6. Pull the merged `master` and tag it:

   ```
   git switch master
   git pull --ff-only
   git tag -a X.Y.Z -m "Release X.Y.Z"
   git push origin X.Y.Z
   ```
7. The `Release` workflow will:
   - Verify the tag matches `Cargo.toml`
   - Extract the matching `CHANGELOG.md` section as release notes
   - Run tests, then `cargo publish` via Trusted Publishing
   - Create the GitHub Release
8. Verify the published version appears on
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
