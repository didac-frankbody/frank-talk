import { cn } from "@/shared/lib/cn";

/** The supplied frank talk wordmark, served from `public/`. */
const WORDMARK_SRC = "/frank-talk-wordmark.png";

/** Intrinsic size of the supplied asset, used to reserve layout space. */
const WORDMARK_WIDTH = 872;
const WORDMARK_HEIGHT = 200;

/**
 * The frank talk wordmark.
 *
 * This renders the **supplied logo asset** rather than recreating it in CSS —
 * the brand guidelines are explicit that an approved asset always wins over a
 * recreation, and this one came from the brand owner. Replacing the file at
 * `desktop/public/frank-talk-wordmark.png` updates the logo everywhere it
 * appears; no component changes needed.
 *
 * Note the asset carries its own colours (`#2A2A2A` block, `#F8E6E6` field) and
 * is deliberately NOT tinted by the theme — a logo is not a themed surface. It
 * therefore looks the same in frank light and frank dark, which is what a
 * wordmark should do.
 *
 * `width`/`height` are set to the intrinsic pixel size so the browser reserves
 * the right box before the image decodes and the logo zone does not reflow on
 * load.
 */
export function FrankWordmark({ className }: { className?: string }) {
  return (
    <img
      alt="frank talk"
      className={cn("block h-auto w-full select-none", className)}
      data-testid="frank-wordmark"
      draggable={false}
      height={WORDMARK_HEIGHT}
      src={WORDMARK_SRC}
      width={WORDMARK_WIDTH}
    />
  );
}

/**
 * The sidebar's logo zone: the wordmark plus the app tagline, in an 88px block
 * with a hairline rule beneath it.
 *
 * The tagline is deliberately copy, not chrome — "the internal chat" is the
 * brand voice explaining what this app is, in the same slot where frankHUB puts
 * "B2B GROWTH ENGINE". Keep it lowercase-sourced and let CSS uppercase it, and
 * keep it free of exclamation marks (the brand never shouts).
 */
export function FrankLogoZone({ className }: { className?: string }) {
  return (
    <div
      className={cn(
        "flex h-[5.5rem] shrink-0 flex-col justify-center gap-2 px-4",
        className,
      )}
      data-frank-logo-zone=""
      data-testid="frank-logo-zone"
    >
      <FrankWordmark />
      <span className="text-center text-badge font-semibold uppercase tracking-[0.16em] text-sidebar-foreground/50">
        the internal chat
      </span>
    </div>
  );
}
