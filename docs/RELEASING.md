# Releasing a new version

Releases are built by GitHub Actions (`.github/workflows/release.yml`) on GitHub's own Windows machines, straight from the tagged source. Nobody uploads an `.exe` from their own PC, and anyone can check which commit a download was built from.

Every release attaches the same file name, `Rastro-windows-x64.exe`, so this link always downloads the newest version and is the one the README uses:

```
https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe
```

## Steps

1. **Bump the version** in these three places (they must match):
   - `Cargo.toml`: `version = "0.5.0"`
   - `web/package.json`: `"version": "0.5.0"`
   - `rastro.rc`: `FILEVERSION 0,5,0,0`, `PRODUCTVERSION 0,5,0,0`, `"FileVersion", "0.5.0"` and `"ProductVersion", "0.5.0"`

2. **Update the changelog and the release notes.**
   - `CHANGELOG.md`: a new section at the top. The app shows it in *What's new*.
   - `.github/release-notes/v0.5.0.md`: the text of the release page, written for players. The workflow adds the SHA-256 at the end on its own.

3. **Build locally once** with `build.bat`, so `Cargo.lock` picks up the new version and you know everything compiles.

4. **Commit, tag and push:**

   ```powershell
   git add -A
   git commit -m "Release v0.5.0"
   git push
   git tag v0.5.0
   git push origin v0.5.0
   ```

5. **Wait for the build.** The *Actions* tab shows a "Release" run. It takes about 10 minutes the first time and less afterwards. When it finishes, the release is published and marked as *Latest*: the README download link, the badge and the in-app update notice all point to it.

6. **Check the release page** once: the notes, `Rastro-windows-x64.exe` and `SHA256SUMS.txt` should all be there.

## If something goes wrong

- **The build failed.** Nothing is published. Open the run in *Actions* to see which step failed, fix it, and move the tag to the fixed commit:

  ```powershell
  git tag -d v0.5.0
  git push origin :refs/tags/v0.5.0
  git tag v0.5.0
  git push origin v0.5.0
  ```

- **"Tag does not match Cargo.toml version"**: the tag and the version in `Cargo.toml` differ. Fix the version, commit, and move the tag as above.
- **Rebuilding an existing tag**: *Actions → Release → Run workflow*, and type the tag.
- **A published release is broken.** Don't reuse its number. Fix the problem and release the next patch version (0.5.1). If it's serious, you can mark the broken release as a pre-release on GitHub so the download link falls back to the previous one.
- **The download link still opens an old version.** The link follows the release marked *Latest*. Check that the new release isn't a draft or a pre-release.

## Code signing

The `.exe` is not signed, so Windows SmartScreen warns the first time someone runs it until the file builds up reputation. Signing removes that warning. [SignPath Foundation](https://signpath.org/) offers free code signing to open-source projects, and it can be added to this workflow later.
