// Thread-panel geometry from the frank talk design handoff: `flex: 0 1 372px`
// with a 280px floor. The panel stays user-resizable — these set where it opens
// and how far it can be dragged in.
export const AUXILIARY_PANEL_DEFAULT_WIDTH_PX = 372;
export const AUXILIARY_PANEL_MIN_WIDTH_PX = 280;
/**
 * Floor for the message column beside an open auxiliary panel (handoff: message
 * column `min-width: 460px`).
 */
export const MAIN_PANE_MIN_WIDTH_PX = 460;

/**
 * Below this content width the shell drops to a single panel.
 *
 * This is the sum of the two floors, which is how the handoff's layout
 * constraints get satisfied here. The handoff asks for a 740px floor on the
 * shared wrapper plus `overflow-x: auto` on the outer row — that solution suits
 * the prototype, but this app must never scroll horizontally: `globals/theme.css`
 * pins `overflow: hidden` and `overscroll-behavior: none` on the document
 * precisely because a horizontal pan once escaped through the shell. Collapsing
 * to one column before the floors can overflow honours the same intent — the
 * message column never gets squeezed — using the responsive mechanism the app
 * already has.
 */
export const AUXILIARY_PANEL_SINGLE_COLUMN_BREAKPOINT_PX =
  MAIN_PANE_MIN_WIDTH_PX + AUXILIARY_PANEL_MIN_WIDTH_PX;

/**
 * Content width below which a channel header's action cluster collapses into a
 * single menu (while an auxiliary panel is open).
 *
 * Lives here rather than beside its one consumer because it is meaningless
 * except in relation to {@link AUXILIARY_PANEL_SINGLE_COLUMN_BREAKPOINT_PX}: it
 * must sit comfortably ABOVE it, so there is a band of widths where the actions
 * have already collapsed but the two-pane layout still holds. The header should
 * degrade before the layout does.
 *
 * The previous value of 760 was tuned against a 600px single-column breakpoint.
 * That breakpoint is now 740 (the sum of the two pane floors), which left only a
 * 20px band — so this moved up to keep roughly the ~160px of headroom the
 * original pairing had.
 */
export const HEADER_ACTIONS_COMPACT_BREAKPOINT_PX = 900;
export const AUXILIARY_PANEL_MAX_WIDTH_PX = 720;

/**
 * Upper bound for the auxiliary panel width clamp, given the current viewport width.
 *
 * On ultrawide displays the static {@link AUXILIARY_PANEL_MAX_WIDTH_PX} is too small,
 * so the panel is allowed to grow with the viewport while always reserving at least
 * {@link AUXILIARY_PANEL_MIN_WIDTH_PX} for the main pane. The static cap acts as a
 * floor, so narrow viewports keep their existing behavior.
 */
export function getAuxiliaryPanelMaxWidth(viewportWidth: number): number {
  return Math.max(
    AUXILIARY_PANEL_MAX_WIDTH_PX,
    viewportWidth - AUXILIARY_PANEL_MIN_WIDTH_PX,
  );
}

/** Clamp a stored panel width into the allowed range for the current viewport. */
export function clampAuxiliaryPanelWidth(
  width: number,
  viewportWidth: number,
): number {
  return Math.max(
    AUXILIARY_PANEL_MIN_WIDTH_PX,
    Math.min(getAuxiliaryPanelMaxWidth(viewportWidth), width),
  );
}
