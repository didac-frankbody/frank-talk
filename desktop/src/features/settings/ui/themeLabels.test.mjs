import assert from "node:assert/strict";
import test from "node:test";

import { formatThemeLabel, pairedThemeLabel } from "./SettingsPanels.tsx";

// The theme picker is a user-visible surface, so the labels it renders are brand
// copy. Theme *ids* are persisted preferences and deliberately never renamed —
// these tests pin the mapping from those stable ids to the displayed names.

test("the brand pair renders lowercase, never title-cased", () => {
  assert.equal(formatThemeLabel("frank"), "frank talk");
  assert.equal(formatThemeLabel("frank-dark"), "frank talk dark");
  // The paired tile (shown under "Follow system") derives its label from the
  // light variant, so it must resolve through the same override.
  assert.equal(pairedThemeLabel("frank"), "frank talk");
});

test("the upstream pair does not surface the old product name", () => {
  assert.equal(formatThemeLabel("buzz"), "Classic");
  assert.equal(formatThemeLabel("buzz-dark"), "Classic Dark");
  assert.equal(pairedThemeLabel("buzz"), "Classic");
  for (const id of ["buzz", "buzz-dark"]) {
    assert.doesNotMatch(formatThemeLabel(id), /buzz/i);
  }
});

test("Shiki bundle names still title-case generically", () => {
  assert.equal(formatThemeLabel("rose-pine"), "Rose Pine");
  assert.equal(formatThemeLabel("github-light"), "Github Light");
  assert.equal(pairedThemeLabel("rose-pine-dawn"), "Rose Pine");
});
