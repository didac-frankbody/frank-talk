import { cn } from "@/shared/lib/cn";

/**
 * The frank talk wordmark: two capsules reading `frank` `talk`.
 *
 * The brand has no supplied logo asset for in-app sub-brand marks, so the
 * design system specifies this capsule-pair pattern instead (design.md §4):
 * `frank` outlined, the module name filled. Rendered in CSS rather than as an
 * image so it inherits the theme's ink/pink tokens and stays crisp at any zoom.
 *
 * If frank body ever supplies an approved `frank talk` logo file, replace this
 * component's internals with it — the brand guidelines say an approved asset
 * always wins over a recreation.
 */
export function FrankWordmark({ className }: { className?: string }) {
  return (
    <div
      aria-label="frank talk"
      className={cn("flex w-full items-stretch gap-1.5", className)}
      role="img"
    >
      <span
        className={cn(
          "flex flex-1 items-center justify-center rounded-md border-2 py-1",
          "border-sidebar-foreground text-sidebar-foreground",
          "font-display text-wordmark font-semibold lowercase tracking-[0.04em]",
        )}
      >
        frank
      </span>
      <span
        className={cn(
          "flex flex-1 items-center justify-center rounded-md py-1",
          "bg-sidebar-primary text-sidebar-primary-foreground",
          "font-display text-wordmark font-semibold lowercase tracking-[0.04em]",
        )}
      >
        talk
      </span>
    </div>
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
        "flex h-[5.5rem] shrink-0 flex-col justify-center gap-1.5 px-4",
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
