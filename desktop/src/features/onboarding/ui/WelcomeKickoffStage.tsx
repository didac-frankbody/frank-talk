import * as React from "react";

import {
  isWelcomeKickoffStageExiting,
  type WelcomeKickoffStagePhase,
} from "@/features/onboarding/useWelcomeKickoffStage";
import { cn } from "@/shared/lib/cn";

type StageCharacter = {
  name: string;
};

/**
 * One marker per starter agent.
 *
 * These were animated bee characters — Buzz's mascots. frank talk has no
 * mascot, so the same three staggered slots carry Original Pink markers
 * instead: the arrival still reads as "your team is turning up" without
 * borrowing another brand's characters.
 */
const STAGE_CHARACTERS: readonly StageCharacter[] = [
  { name: "Fizz" },
  { name: "Honey" },
  { name: "Bumble" },
];

const STAGE_EXIT_ANIMATION = "motion-kickoff-stage-exit";

/**
 * The welcome team characters standing on top of the Welcome composer banner
 * while the team is being set up. Positioned relative to the banner wrapper
 * (`bottom-full` = feet on the banner's top edge) and purely decorative —
 * the banner's own copy carries the setup status for screen readers.
 *
 * Placeholder choreography: staggered rise-from-below entrance per character
 * (CSS `motion-kickoff-character-enter`, delay via `--stagger-index`), whole
 * row crossfades out on either resolution — the first agent message landing,
 * or the wait timing out. The characters must not linger after a timeout: a
 * stage that stays up implies a team is still coming when none is.
 */
export function WelcomeKickoffStage({
  onExitComplete,
  phase,
}: {
  onExitComplete: () => void;
  phase: WelcomeKickoffStagePhase;
}) {
  const handleAnimationEnd = React.useCallback(
    (event: React.AnimationEvent<HTMLDivElement>) => {
      if (event.animationName === STAGE_EXIT_ANIMATION) {
        onExitComplete();
      }
    },
    [onExitComplete],
  );

  if (phase === "hidden" || phase === "done") return null;

  return (
    <div
      aria-hidden
      className={cn(
        "pointer-events-none absolute bottom-full left-10 z-10 flex items-end gap-4",
        isWelcomeKickoffStageExiting(phase) && "motion-kickoff-stage-exit",
      )}
      data-phase={phase}
      data-testid="welcome-kickoff-stage"
      onAnimationEnd={handleAnimationEnd}
    >
      {STAGE_CHARACTERS.map((character, index) => (
        <span
          className="motion-kickoff-character-enter block size-10 rounded-full bg-primary"
          data-testid={`welcome-kickoff-stage-${character.name.toLowerCase()}`}
          key={character.name}
          style={{ "--stagger-index": index } as React.CSSProperties}
        />
      ))}
    </div>
  );
}
