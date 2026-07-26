/**
 * Inline "New" divider rendered directly above the oldest unread top-level
 * message, mirroring Slack's read/unread boundary. Computed from the
 * channel's read frontier as it stood when the channel was opened.
 */
export function UnreadDivider() {
  return (
    <section
      aria-label="New messages"
      className="relative flex items-center py-1"
      data-testid="message-unread-divider"
    >
      {/*
       * The label is ink on a solid brand pill rather than brand-coloured text:
       * under the frank theme `--primary` is Original Pink, a background-only
       * colour that would all but vanish as type on the pale message surface.
       * The hairlines carry the brand colour at full strength instead of a
       * tint, so the boundary still reads as one continuous pink rule.
       */}
      <div className="h-px flex-1 bg-primary" />
      <span className="mx-2 shrink-0 rounded-full bg-primary px-2 py-0.5 text-badge font-semibold uppercase tracking-[0.14em] text-primary-foreground">
        New
      </span>
      <div className="h-px flex-1 bg-primary" />
    </section>
  );
}
