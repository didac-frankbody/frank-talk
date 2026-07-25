# Brand fonts — install these locally

This directory is intentionally **empty of font binaries**.

frank talk's type is Pitch Semibold and Founders Grotesk Text, both licensed
commercial faces from [Klim Type Foundry](https://klim.co.nz/). This repository
is a **public** fork, so committing the files here would redistribute them to
anyone who clones it — which typical foundry web licences do not permit. The
`@font-face` declarations live in
`desktop/src/shared/styles/globals/fonts.css` and point at this directory, so
the app picks the fonts up automatically once the files are in place.

Until then the stacks in `tailwind.config.js` fall through to the next family
and the app renders in a system sans. Everything works; it just isn't on brand.

## Installing

Copy these 14 files from the design handoff's `fonts/` folder into this
directory (no subfolders):

```
Pitch-Semibold.woff2
Pitch-Semibold.woff
FoundersGrotesk-Light.woff2
FoundersGrotesk-Light.woff
FoundersGroteskText-Light.otf
FoundersGroteskText-LightItalic.otf
FoundersGroteskText-Regular.otf
FoundersGroteskText-RegularItalic.otf
FoundersGroteskText-Medium.otf
FoundersGroteskText-MediumItalic.otf
FoundersGroteskText-Semibold.otf
FoundersGroteskText-SemiboldItalic.otf
FoundersGroteskText-Bold.otf
FoundersGroteskText-BoldItalic.otf
```

Then restart the dev server (Vite only scans `public/` at startup).

`.gitignore` in this directory keeps the binaries out of commits, so you can
drop them in without risk of pushing them.

## Notes for whoever productionises this

- **Pitch Semibold (600) is the only licensed Pitch weight.** Never let the
  browser synthesise bold or italic from it — declare only 600, which
  `fonts.css` does.
- Founders Grotesk ships here as **OTF for weights 400–700**, which are large
  and uncompressed compared to the two woff2 faces. Converting them to woff2
  (and subsetting to the Latin range the app actually renders) would cut a few
  hundred KB off first paint in the Tauri webview. Worth doing before a real
  release; not required for the theme to be correct.
- If this app later moves to a private repo or a licensed asset pipeline, the
  files can be committed and this README replaced.
