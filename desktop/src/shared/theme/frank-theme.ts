/**
 * frank talk theme tokens.
 *
 * The frank body brand palette, expressed as the app's shadcn CSS custom
 * properties. Values come from the design handoff's "Token map" table.
 *
 * ## Why these live in TypeScript and not in `globals/theme.css`
 *
 * The handoff asks for these tokens to be added to `theme.css`'s `:root`
 * block. That does not work in this codebase: {@link applyTheme} derives the
 * whole token set at runtime via `createThemeVars()` and writes every one of
 * them as an **inline style on `:root`**, which beats any stylesheet rule.
 * The Catppuccin values in `theme.css` are effectively dead code after boot —
 * they are only a pre-hydration fallback. `applyAccentColor()` then overwrites
 * the `--primary` / `--sidebar-active` family on top of that.
 *
 * So a themed palette has to enter through the same runtime path. These maps
 * are applied *over* the derived vars for the frank themes, and the accent
 * pass is pinned (see `resolveEffectiveAccent`) so it cannot repaint pink.
 *
 * ## Brand rules encoded here
 *
 * - **Original Pink `#FFB6A5` is a background only, never a text colour.**
 *   That is why `--primary-foreground` is ink rather than white: anything
 *   filled pink carries ink type.
 * - **There is no red in this brand.** `--destructive` is ink with white
 *   type; destructive intent is carried by weight and fill, not hue.
 * - Alpha is deliberately *not* baked into the border/accent tokens even
 *   though the handoff writes them as `351 20% 21% / .10`. Several consumers
 *   compose their own alpha on top (`hsl(var(--border) / 0.8)` in
 *   `tailwind.config.js`, `hsl(var(--sidebar-border) / 0.45)` in `theme.css`),
 *   and a token that already carries a slash produces invalid CSS there. The
 *   values below are the same colours pre-composited against their surface,
 *   so they render identically and stay composable.
 */

/** Raw brand palette, for the few places that need a hex rather than a token. */
export const FRANK_PALETTE = {
  /** Black 5C — all text, primary buttons, sidebar, emphasis borders. */
  ink: "#3F2A2D",
  /** Original Pink, PMS 2337C — active states, badges, accents. Background only. */
  pink: "#FFB6A5",
  /** Chips, avatars, scrollbar thumb. */
  pinkSoft: "#FFD0C6",
  /** Hovers, chips, page tint. */
  blush: "#FFEFEA",
  /** Message surface. */
  offWhite: "#FFFBFA",
  /** Community rail — one step darker than the sidebar. */
  railInk: "#2F1F21",
  /** Healthy / connected status fill. */
  acid: "#ACFAEA",
  /** Avatar fill variant. */
  tan: "#F4C3AB",
} as const;

/**
 * frank talk (light). The sidebar is ink even in this theme — the brand's
 * shell is dark and the content surface is the pale one.
 */
export const FRANK_LIGHT_VARS: Readonly<Record<string, string>> = {
  // 8px. Applied through the runtime var pass rather than `theme.css`'s `:root`
  // so it scopes to frank instead of reshaping every other theme's radii.
  "--radius": "0.5rem",
  "--background": "12 100% 99%",
  "--foreground": "351 20% 21%",
  "--card": "0 0% 100%",
  "--card-foreground": "351 20% 21%",
  "--popover": "0 0% 100%",
  "--popover-foreground": "351 20% 21%",
  "--primary": "11 100% 82%",
  "--primary-foreground": "351 20% 21%",
  "--secondary": "14 100% 96%",
  "--secondary-foreground": "351 20% 21%",
  "--muted": "14 100% 96%",
  "--muted-foreground": "351 20% 45%",
  "--accent": "11 100% 89%",
  "--accent-foreground": "351 20% 21%",
  "--destructive": "351 20% 21%",
  "--destructive-foreground": "0 0% 100%",
  // Pre-composited ink/10 and ink/14 over the off-white surface — see the
  // note above on why these carry no slash.
  "--border": "0 11% 91%",
  "--input": "9 10% 88%",
  "--ring": "11 100% 82%",

  "--sidebar": "351 20% 21%",
  "--sidebar-background": "351 20% 21%",
  "--sidebar-foreground": "0 0% 100%",
  /** New token: the community rail, one step darker than the sidebar. */
  "--sidebar-rail": "353 21% 15%",
  "--sidebar-primary": "11 100% 82%",
  "--sidebar-primary-foreground": "351 20% 21%",
  "--sidebar-active": "11 100% 82%",
  "--sidebar-active-foreground": "351 20% 21%",
  // Pre-composited white/9 and white/12 over the ink sidebar.
  "--sidebar-accent": "351 13% 28%",
  "--sidebar-accent-foreground": "0 0% 100%",
  "--sidebar-border": "353 12% 30%",
  "--sidebar-ring": "11 100% 82%",

  // Status fills carry ink type; `--status-deleted` is ink because the brand
  // has no red.
  "--status-added": FRANK_PALETTE.acid,
  "--status-modified": FRANK_PALETTE.pinkSoft,
  "--status-deleted": FRANK_PALETTE.ink,
  "--ui-warning": FRANK_PALETTE.ink,
  "--ui-warning-bg": "#FFACB8",

  // The huddle chrome is a dark overlay in every theme; tint it to brand ink
  // rather than leaving it on the derived neutral greys.
  "--huddle-drawer-surface": "351 20% 12%",
  "--huddle-control-surface": "351 16% 22%",
  "--huddle-control-hover-surface": "351 14% 27%",
  "--huddle-control-chevron-surface": "351 18% 17%",
  "--huddle-control-chevron-hover-surface": "351 16% 22%",
  "--huddle-control-foreground": "0 0% 98%",
  "--huddle-popover-surface": "351 18% 17%",
  "--huddle-popover-border": "351 14% 27%",
  "--huddle-tooltip-surface": "351 16% 22%",
  "--huddle-tooltip-foreground": "0 0% 98%",
};

/**
 * frank talk (dark). **Derived, not specified.** The handoff's token map is
 * light-only, and its shell is already ink — so "dark" cannot be a simple
 * inversion. The rule applied here: the ink shell stays put and the *content*
 * surface descends to meet it, so the app reads as one continuous dark object
 * rather than a pale card in a dark frame. Pink keeps its exact brand hue and
 * stays a background; type goes to blush-white rather than pure white, which
 * would glare against ink.
 */
export const FRANK_DARK_VARS: Readonly<Record<string, string>> = {
  "--radius": "0.5rem",
  "--background": "351 18% 13%",
  "--foreground": "14 100% 96%",
  "--card": "351 19% 16%",
  "--card-foreground": "14 100% 96%",
  "--popover": "351 19% 18%",
  "--popover-foreground": "14 100% 96%",
  "--primary": "11 100% 82%",
  "--primary-foreground": "351 20% 21%",
  "--secondary": "351 16% 22%",
  "--secondary-foreground": "14 100% 96%",
  "--muted": "351 16% 22%",
  "--muted-foreground": "12 24% 72%",
  "--accent": "351 16% 26%",
  "--accent-foreground": "14 100% 96%",
  // Ink is the shell colour here, so an ink destructive fill would vanish.
  // Invert the pairing: pale brand type on the dark surface, still no red.
  "--destructive": "14 100% 96%",
  "--destructive-foreground": "351 20% 21%",
  "--border": "351 14% 25%",
  "--input": "351 14% 28%",
  "--ring": "11 100% 82%",

  "--sidebar": "353 21% 15%",
  "--sidebar-background": "353 21% 15%",
  "--sidebar-foreground": "14 100% 96%",
  "--sidebar-rail": "353 22% 11%",
  "--sidebar-primary": "11 100% 82%",
  "--sidebar-primary-foreground": "351 20% 21%",
  "--sidebar-active": "11 100% 82%",
  "--sidebar-active-foreground": "351 20% 21%",
  "--sidebar-accent": "351 15% 22%",
  "--sidebar-accent-foreground": "14 100% 96%",
  "--sidebar-border": "351 14% 25%",
  "--sidebar-ring": "11 100% 82%",

  "--status-added": FRANK_PALETTE.acid,
  "--status-modified": FRANK_PALETTE.pinkSoft,
  // Ink would disappear on a dark surface; the soft pink reads as the
  // "removed" fill without introducing a red.
  "--status-deleted": "#E9B4B4",
  "--ui-warning": "#FFACB8",
  "--ui-warning-bg": "rgba(255, 172, 184, 0.14)",

  "--huddle-drawer-surface": "351 20% 12%",
  "--huddle-control-surface": "351 16% 22%",
  "--huddle-control-hover-surface": "351 14% 27%",
  "--huddle-control-chevron-surface": "351 18% 17%",
  "--huddle-control-chevron-hover-surface": "351 16% 22%",
  "--huddle-control-foreground": "14 100% 96%",
  "--huddle-popover-surface": "351 18% 17%",
  "--huddle-popover-border": "351 14% 27%",
  "--huddle-tooltip-surface": "351 16% 22%",
  "--huddle-tooltip-foreground": "14 100% 96%",
};

/** frank talk theme name (light). */
export const FRANK_THEME_NAME = "frank";

/** frank talk theme name (dark). */
export const FRANK_DARK_THEME_NAME = "frank-dark";

/** Whether a theme name is one of the frank talk themes. */
export function isFrankTheme(themeName: string): boolean {
  return themeName === FRANK_THEME_NAME || themeName === FRANK_DARK_THEME_NAME;
}

/**
 * The brand token overrides for a frank theme, or `null` for anything else.
 *
 * Callers apply these *over* the derived `createThemeVars()` output, so any
 * token the brand does not speak to (chart colours, for instance) keeps a
 * sensible derived value instead of being dropped.
 */
export function getFrankVars(
  themeName: string,
): Readonly<Record<string, string>> | null {
  if (themeName === FRANK_THEME_NAME) return FRANK_LIGHT_VARS;
  if (themeName === FRANK_DARK_THEME_NAME) return FRANK_DARK_VARS;
  return null;
}
