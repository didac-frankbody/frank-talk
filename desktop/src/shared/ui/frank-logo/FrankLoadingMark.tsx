import { cn } from "@/shared/lib/cn";
import { AppIcon } from "@/shared/ui/frank-logo/AppIcon";

/**
 * The waiting mark: the app icon, breathing.
 *
 * This replaced the Buzz bee that used to fly on the boot splash, the
 * onboarding gates and the ACP transcript. frank talk has no mascot — the brand
 * is typographic and does not shout — so a still mark with a slow pulse carries
 * "working on it" instead of a character animation.
 *
 * The icon is theme-matched (see {@link AppIcon}), so it keeps a defined edge on
 * both the pale page and the ink shell.
 */
export function FrankLoadingMark({
  ariaLabel,
  className,
}: {
  /** Announced to screen readers; the element carries `role="status"`. */
  ariaLabel: string;
  className?: string;
}) {
  return (
    <div
      aria-label={ariaLabel}
      className={cn("animate-pulse", className)}
      data-testid="frank-loading-mark"
      role="status"
    >
      <AppIcon alt="" className="h-full w-full rounded-[22%]" />
    </div>
  );
}

/**
 * Three rising dots, for the inline "an agent is mid-turn" indicator.
 *
 * The mark itself is too dense to read at this size — the old bee was a
 * silhouette that survived scaling down, the `tk` tile is not. Dots in Original
 * Pink carry the same beat, and pink as a fill (never as type) is the brand
 * rule.
 */
export function FrankActivityDot({ className }: { className?: string }) {
  return (
    <span
      className={cn("block size-1.5 rounded-full bg-primary", className)}
      aria-hidden="true"
    />
  );
}
