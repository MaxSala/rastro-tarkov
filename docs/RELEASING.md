# Releasing a new version

Releases are built by GitHub Actions (`.github/workflows/release.yml`) on GitHub's own Windows machines, straight from the tagged source. Nobody has to upload an `.exe` from their PC, and anyone can check which commit a download was built from.

## Steps

1. **Bump the version** in these three places (they must match):
   - `Cargo.toml`: `version = "0.4.0"`
   - `web/package.json`: `"version": "0.4.0"`
   - `rastro.rc`: `FILEVERSION 0,4,0,0`, `PRODUCTVERSION 0,4,0,0`, `"FileVersion", "0.4.0"` and `"ProductVersion", "0.4.0"`

2. **Write the release notes** in `.github/release-notes/v0.4.0.md`. Keep them for players: what's new, what to do after updating, known issues. The workflow adds the SHA-256 at the end on its own.

3. **Build locally once** with `build.bat` so `Cargo.lock` picks up the new version and you know everything compiles.

4. **Commit, tag and push:**

   ```powershell
   git add -A
   git commit -m "Release v0.4.0"
   git tag v0.4.0
   git push
   git push origin v0.4.0
   ```

5. **Wait for the build.** The *Actions* tab shows a "Release" run. It takes about 10 minutes the first time and less afterwards. If it fails, open it to see the step that failed.

6. **Publish.** In *Releases* there is now a **draft** with the `.exe`, `SHA256SUMS.txt` and your notes. Check it, then press **Publish release**. Until then nobody else can see it.

## If something goes wrong

- **"Tag does not match Cargo.toml version"**: the tag and the version in `Cargo.toml` differ. Fix the version, then move the tag:

  ```powershell
  git tag -d v0.4.0
  git push origin :refs/tags/v0.4.0
  git tag v0.4.0
  git push origin v0.4.0
  ```

  Delete the leftover draft release, if any, before retrying.
- **Rebuilding an existing tag**: *Actions → Release → Run workflow*, and type the tag.
- **Never reuse a published version number.** If a published release is broken, fix it and release the next patch version (0.4.1).

## Code signing

The `.exe` is not signed, so Windows SmartScreen warns the first time someone runs it until the file builds up reputation. Signing removes that warning. [SignPath Foundation](https://signpath.org/) offers free code signing to open-source projects, and it can be added to this workflow later.
