# Handover — the Buzz → frank talk rebrand

Written for whoever picks this up next. Everything described here is **already
merged to `main`**; there is no branch to land and no open PR. This is a map of
what changed, which decisions are load-bearing, and what is deliberately still
open.

State at handover: `main` at `37a925d`, working tree clean, zero open PRs.

---

## What shipped

Six squash-merged PRs, oldest first:

| PR | Commit | What |
|---|---|---|
| #1 | `3b0a437` | Re-skin the desktop client into frank talk (theme, tokens, wordmark, layout) |
| #2 | `4994a3a` | Rebrand the README with real screenshots |
| #3 | `1b07052` | The 21 user-visible `Buzz.` strings the first sweep's regex skipped |
| #4 | `53122f5` | Apply the second design handoff (specified light/dark tokens, official lockup) |
| #5 | `ade14ed` | App icon, Buzz artwork out of onboarding, OS/mobile/web/admin/CLI surfaces, `just installer` |
| #6 | `37a925d` | Rename the Honey and Bumble starter agents to Rosie and Clay |

---

## The five things most likely to be broken by accident

Read this section before touching the theme, the icon, or anything named `buzz`.

### 1. Theme tokens cannot live in `theme.css`'s `:root`

Both design handoffs asked for the palette to go in `globals/theme.css`. That
does not work here. `applyTheme()` → `createThemeVars()` derives ~35 CSS custom
properties at runtime and writes every one as an **inline style on `:root`**,
which beats any stylesheet rule; `applyAccentColor()` then overwrites the
`--primary` / `--sidebar-active` family on top.

So the brand palette enters through the same runtime pass, from
`desktop/src/shared/theme/frank-theme.ts`, and the accent pass is pinned for
frank themes (see `isAccentPinnedTheme` in `ThemeProvider.tsx`). Values in
`theme.css` are a pre-hydration fallback only.

### 2. `buzz` identifiers are load-bearing — do not "finish the rename"

These are storage keys, wire values and public contracts. Renaming any of them
breaks existing installs, and several look like leftovers but are not:

- Crate / package / binary names (`buzz-relay`, `buzz-cli`, `buzz-acp`, …)
- The `buzz://` deep-link scheme and the bundle id `xyz.block.buzz.app`
- `BUZZ_*` env vars, `--buzz-*` CSS custom properties, `.buzz-*` class prefixes,
  `data-buzz-sidebar`, localStorage keys (`buzz-theme`, `buzz-accent-color`)
- The ACP runtime id `buzz-agent`
- Theme ids `buzz` / `buzz-dark` — these are the *original Buzz colour themes*,
  deliberately kept and shown in Appearance as "Classic" / "Classic Dark"
- Persona ids `builtin:honey` / `builtin:bumble` — the personas display as Rosie
  and Clay; the slugs are in every install's agent records and message history
- `[Buzz event: …]` — the harness's wire marker. The desktop and mobile clients
  both parse it, so an older sidecar against a newer app has to keep working.
  The rename happens at the render boundary: `displaySectionTitle()` in
  `agentSessionTranscriptHelpers.ts` and `transcript_builder.dart`.

### 3. Both handoff bundles were read from a pre-re-skin `main`

If you get another handoff, expect it to describe the app as it was before PR
#1. Conflicts already identified and resolved, with reasoning in code comments:

- `--sidebar-accent` / `--sidebar-border: 0 0% 100%` (plain white) is wrong —
  `bg-sidebar-accent` appears 14× with no alpha at the call site, so white paints
  solid white blocks on the ink sidebar. They stay pre-composited.
- The spec reuses `data-buzz-sidebar` on the assumption frank *replaces* Buzz.
  It doesn't — Classic is still selectable, so frank uses its own
  `data-frank-theme` marker and leaves the `--buzz-*` values olive.
- "frank talk ships one theme, `.dark` is a fallback" — no. `frank` /
  `frank-dark` are a registered `THEME_PAIRS` pair, dark is the default, Light
  and System are in Appearance.
- `body { font-weight: 300 }` was not applied: this app is nearly all small UI
  chrome, where the brand's own guidance is 400–600.
- `overflow-x: auto` on the shell row was not applied: the shell must never
  scroll sideways. The layout floors are honoured by stacking to single-column
  at 740px.

### 4. `Info.plist` overrides `productName`

`tauri.conf.json` says `frank talk`, but `CFBundleName` / `CFBundleDisplayName`
in `desktop/src-tauri/Info.plist` win for the menu bar, Finder, About box and
permission prompts. Both must agree. A knock-on: Tauri names the executable
after `productName`, so anything matching on the process name or the
`.app` bundle path has to follow — see `runtime/desktop_process.rs` and the
`frank talk.app` paths in `.github/workflows/release.yml`.

### 5. Brand rules that are easy to violate

- **Original Pink `#FFB6A5` is a fill, never a text colour.** ~100 `text-primary`
  call sites are remapped to ink once, in `globals/frank.css`.
- **There is no red.** `--destructive` is ink with off-white type in light, and
  pink on ink in dark. The web client's token was Catppuccin red and was fixed.
- **No exclamation marks** in UI copy.
- **No emoji in chrome** — emoji are user content only.
- Text sizes must be rem, never px (`pnpm check:px-text` enforces it).

---

## Where things live

```
desktop/src/shared/theme/frank-theme.ts        brand token maps (light + dark)
desktop/src/shared/theme/ThemeProvider.tsx     theme application, accent pinning
desktop/src/shared/styles/globals/frank.css    frank-scoped rules with no token
desktop/src/shared/styles/globals/fonts.css    @font-face for the brand faces
desktop/src/shared/ui/frank-logo/
  FrankWordmark.tsx        the supplied lockup + the 88px sidebar logo zone
  AppIcon.tsx              theme-matched app icon (useAppIcon / <AppIcon/>)
  FrankLoadingMark.tsx     the pulsing waiting mark + the pink activity dot
desktop/src-tauri/icons/source/                the two icon variants (regenerate from these)
desktop/public/frank-talk-wordmark.png         the lockup, 872×200
desktop/src/shared/styles/globals/components.css   the onboarding palette block
```

To change the app icon: replace the two files in `icons/source/`, then

```bash
cd desktop && pnpm exec tauri icon src-tauri/icons/source/frank-talk-icon-light.png -o src-tauri/icons
```

and regenerate `public/app-icon*{@2x,@3x}.png`, `web/src/assets/app-icon@3x.png`
and the 38 mobile PNGs at their existing sizes and colour types (iOS app icons
must have no alpha channel).

---

## Still open, deliberately

1. **Brand fonts are not committed.** Pitch Semibold and Founders Grotesk Text
   are licensed Klim faces and this is a public fork, so `desktop/public/fonts/`
   has a `.gitignore` that blocks the binaries. Drop the 14 supplied files in
   and the app picks them up; without them the stacks fall through to a system
   sans and a monospace stand-in for Pitch. Everything works either way.
2. **Internal docs still say Buzz** — `VISION*.md`, `CONTRIBUTING.md`,
   `ARCHITECTURE.md`, `AGENTS.md`, `CLAUDE.md`, `RELEASING.md`, `TESTING.md`,
   `desktop/README.md`, most of `docs/`. Deferred on purpose; the root
   `README.md` is done.
3. **`docs/welcome-kickoff-silent-failures.md` keeps Honey and Bumble.** It is a
   post-mortem of a real incident. Renaming the participants would make the
   record false.
4. **The mobile app's theme is still Catppuccin**, not the brand palette. Its
   *identity* (name, icons, splash) is rebranded; its colours are not. That is a
   design decision nobody has taken yet.
5. **`data_dir().join("Buzz")`** in `managed_node_paths.rs` is the app-data
   directory name. Changing it would orphan existing installs' managed Node
   runtimes; it needs a migration, not a rename.
6. **Never run in the real Tauri shell.** Everything was verified against the web
   build in headless Chromium. Native title bar, macOS vibrancy and the
   `backdrop-filter` header are unverified.

---

## Verifying your work

```bash
. ./bin/activate-hermit

cd desktop
pnpm exec biome check --write .   # lint + format
pnpm check                        # file-size, px-text, pubkey-truncation guards
pnpm typecheck
pnpm test                         # 3494 unit tests
pnpm test:e2e:smoke               # ~710 tests, ~35 min
pnpm exec playwright test --project=integration   # needs the mock bridge only

cd ../mobile && flutter analyze && flutter test   # 568 tests
cd .. && cargo fmt --all --check
cargo check -p buzz-relay -p buzz-cli -p buzz-acp -p buzz-agent -p buzz-admin -p buzz-dev-mcp -p sprig
```

**GitHub Actions never runs on this fork** — the workflows are registered but
produce no check runs, so every PR merges with zero CI. The local gate above is
the only gate. Two things could not be verified in the cloud container and are
worth running first on a real machine:

- `cargo clippy` and `cargo test` for `desktop/src-tauri` — the container has no
  GTK dev headers, so the Tauri crate cannot build at all. The Rust changes in
  PRs #5 and #6 are verified by `cargo fmt` and by reading every caller, **not
  by a compiler**. Run these before trusting them.
- `just installer` end to end.

### e2e failures that are not yours

These fail under parallel load in a cloud container and pass in isolation; four
were confirmed identical on the pre-change baseline:

`virtualization` (2 tests) · `relay-reconnect` · `channel-controls` ·
`community-rail` (a different test each run — dnd keyboard sensor) ·
`video-attachment` · `channel-browser` · `onboarding` "name-only community
profile save"

If you see exactly these, don't chase them. If you see anything else, it's real.

### Screenshots

`just desktop-screenshot --name foo --active-channel general`. In a container
whose Chromium doesn't match the pinned Playwright build, set
`BUZZ_CHROMIUM_PATH` to the installed binary — on a normal machine you won't
need it.

---

## Building something installable

```bash
just installer
```

Compiles the five sidecar binaries for real, then bundles: `.dmg` on macOS,
`.deb`/`.AppImage` on Linux, `.msi` on Windows.

`just desktop-release-build` is **not** the same thing — it `touch`es empty stub
sidecars, so the bundle launches but its agent features cannot work.

The bundle is unsigned, so macOS blocks it on first open:

```bash
xattr -dr com.apple.quarantine "/Applications/frank talk.app"
```

The installed app still needs a relay: `just relay` locally, or point it at a
deployed one.
