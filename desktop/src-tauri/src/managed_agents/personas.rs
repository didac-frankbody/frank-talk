use std::fs;

use tauri::AppHandle;

use crate::{managed_agents::AgentDefinition, util::now_iso};

struct BuiltInPersona {
    id: &'static str,
    display_name: &'static str,
    avatar_url: Option<&'static str>,
    system_prompt: &'static str,
    name_pool: &'static [&'static str],
    model: Option<&'static str>,
    runtime: Option<&'static str>,
    default_active: bool,
}

// Built-in persona avatars: a brand initial on a brand fill, one colour each so
// the three stay distinguishable in a member list. These replaced inlined
// portraits of Buzz's bee characters, which were both off-brand and ~400KB of
// this file; a 128px flat tile is ~1KB.
//
// They are avatars rather than `None` on purpose. `replace_builtin_avatar` in
// migration.rs upgrades a stored legacy avatar to whatever the built-in
// currently has, and bails when there is nothing to upgrade *to* — so dropping
// these would strand every existing install on its old bee portrait.
const FIZZ_AVATAR: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAC0ElEQVR4nOzaS0hUUQCA4ZO5SdQBM3Nj5qMSssweUAZF0IMIol0Q5UIoItq1aVPROheBBOEuSTCIQkgRCrIiNQh6WKQiLYLKXBjWrhxtoBjMEiYh/nuH/+Mu5pwZZjE/5x7m3ps7090exMkNQhkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgEUrQPPNnjsDz8N/s2VNxaXjh0OUuAJgBoAZABatANtqKosK8tLDb9+THb1P5nymrrKsrqosZKD3xdC78YkQbdEKsL12deqYPbO+ouzctVvJ6en0TOrXb9q3I5NvO7i1/vSV62MTkyHCckK0NaytPnFgZ1iQ4kTB5ZNHQrTFYA8oSRSGhSotSoRocxOGGQCWJQEevhw+33b75+tddTUXjh0KMeEKgBkAliUB+t+MzvdWb/PZEGGxDzA1lbzY3vlocCTEU/wCtN3tSx0hW7gHwAwAMwAsfgH2bqrdv3ldejg1nWzpvBf9y87ziV+A0qLC+lXls2daTh0903pj9MOnEENRvxydiUR+XuPuhhBP7gEwA8CyJEB5ydLGPb/OQhXLl4X4yJYApcVNpRndKI4aT0EwA8BiEGB88stvw89fQxaJ+v+AvtejrV0PZs/0PB3suD8QskW0VsDjVyPD78fSw78+GZdytav348RkIn/JnPmN1Ss3VK0IsRKtAP1DbzN8PL2z/9mfk7k5iw2gf2MAmAFgi2a624M4rgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWA/AAAA///8T0i9AAAABklEQVQDAAcyd43/LXKFAAAAAElFTkSuQmCC";

const ROSIE_AVATAR: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAEiUlEQVR4nOzcf0zUdRzH8Y94gHh6B+fxy0MEA8VAYoLUNGm5KI1ls1DnLLcWsmir1qppm/xTLv+pWGvLrVbLtdmkn7acbtpW3pWuIRsVIRKNH3rkddx5nNwPWHfdVqMPR8PhP6/P5+vrsfvj7s19+YPn7u7D5/vdma47PxGEYxIExQBgDADGAGAMAMYAYAwAxgBgDADGAGAMAMYAYAwAxgBgDADGAGAMAMYAYAwAxgBgDADGAGAMAMYAYAwAxgBgDADGAGAMAMYAYAwAxgBgDADGAGAMAMYAYAwAxgBgDADGAGAMAMYAYAwAxgBgDADGAGAMAMYAYAwAplCAPfvaRq8F53TIYvMCi9mcaVmYl20rcuSsKMitWr1CaEXvV0BwPJK4XfGMdv82/M8kIz21urzkvvVV69aUCh0Y7S0oHJ10dfYkbquKHM8/8fCyPLtQW4owqN6BK/vfODI84hVqm6fOtyZ+fvpcKBxJ3Dn9Q5fXPyb/qDA/e8PasuQD4vPGQuFQKBKKRL3+YP/wyMzfac+yvN3abDEvFKpSKMAUrz/Q+tbRoZE/pyYba27fv7fxBkf5xtpPOk+cvZA0r6sp37f3UaEqFd+C7FnWAy07xBzZbZandzccemHPoowF8vxsR3fiJSJUpehnQJrpJlcHlSuLXmp6JGn4Y1evUJUBP4RrKkpybFZ5ctUXEKoy5ioo154pP/QFrgtVGXMrIhKdkB+mmuYLVRkwQHA83Dc4bUlqsy4WqjJggPaTrqTJqmKHUJXRAhz9+rvEP3TyJMtqLi8pFKoyQgC3xzfk9pzv6v2+8+LMJf+uhjqhMD0CODt+dXa8Iuauuvy2LRurhcKMfEKmfv0dLbu2pKQovdQ2ZoClOVlN2x+4s3KlUJ7RAlSWFW2qrazfUCU0oUeApO3oSwPuC9398hMSf/dndjcszVkidKNHgOWO7Me23jv1cGJi8tnX3pNPtvx0caB/6A8dA2i5F5SWltrasjM9LVUeth35anDEI3Sj62acI3fJc48/JE+iE5MH3zmWtAukPo13Q++prWiom7bGd3v8bR8eF1rRezu6eefm4oJceeLq7PnyzHmhD70DmEzzDzy1Pekc5AefnekbcAtNaH9CJi/b9uKT2+TJX7HYq4ePBYLjQgdGOCO2bk3ptvq75MnoteChdz+NxWJCeYoG8PinncX1+sZmf35T4/1lxQXy5OdLgx8d/1YoT8UAl696Dx5ulyc9v19+/f0v4vH4LEe93NxoMWfIk/ZTro5f+oTaVLwy7pSr0x/4n3fwu9euXpb/77WetkzLg3XJ+8zfnOt6c/oy1JyRvnVTrTypKF2u1BXUCm1FJJaPs1+enlhiTt0vKcyfGcBmXZQ0GQ9HPz7hlCc7NscZgP7DAGAMAKbi1dG3FL4CwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAOxvAAAA//+kyBGMAAAABklEQVQDABQZJEz0bN9EAAAAAElFTkSuQmCC";

const CLAY_AVATAR: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAFSElEQVR4nOzdfUzUdRzA8c8BwcXB8Xhyx7OQFWQIZCSYTw0rth4o0SxmaUql5rIMXCsr29q0dGsWs+ZW2MPMMEGH6QozBLZSKAEBFQGJh+PhOuAeuDtQ6dfun7sfZdzV9vn8bp/X+ON+393vn+97d9zv9/0e+BwaHwSGxwcYKg6AjAMg4wDIOAAyDoCMAyDjAMg4ADIOgIwDIOMAyDgAMg6AjAMg4wDIOAAyDoCMAyDjAMg4ADIOgIwDIOMAyDgAMg6AjAMg4wDIOAAyDoCMAyDjAMg4ADIOgIwDIOMAyDgAMg6AjAMg4wDIOAAyDoCMAyDjAMg4ADIOgIwDIOMAyKQXYOhCW1dtnVmns46abAYDTE76h4cpNRHqOcmxmXNBaiQTYEw/XF9ysKOy2tDX/0/PCYzSpK/KS1mZaz/87tW32yur7Y/lQcqCqjKgxwukoKn06Je5a859XnqD2RcYe7VVOz4syXlq6GI7SAT1V8D4mOXY5m09Z36b/ilG7UBZwZYFhRtGOnuAPNIBrKPG8heKhlovgYtsBmPltp0gBXQDTFgsZQWv6C51iMaFd/PE7IUJi7OUkWqFKlTm7a3v7Oqra7z8Y81AYwtIDd0Ap3cWT539uevyM198VjSonp0k/KSvfkLb0PzTux9MPYsyor+E20/VtpQfdxzxkcvz9u+ZOvuONHPuePKbfUmPPgDSQTTA6fc+Eo08uON1YX6ncSpkby+67aGlIBEUA7QcOWHSOv0939Snl89cnAXTtuSNzQFqFUgBxQD1nx5wPFSowuZtWA2uuEkuX7R1E0gBuQDCR/6RLqfP76n5y4QJBRclLJkfd28GkEcuQPfZc46HMpns9ofvB7dkPL8KyCP3MbSvvtHxMPKuFP+wEHCL+s7kW5YusuiHhce+gQFAEq0AV222/oZmxxFV8q3wH+S8/ybQRiuAoaf/+rVrjiMRSbPAo9EKYDOZRCPKaA14NGIBDOIAfgqi793/F1oBJixW0Yi33Bc8Gq0AfoEK0ci4yQwejVYAebBSNCIsCYBHo3UhJg8OEo2MdPeCR6MVICBC5e3n5zgy2Ozycpi00Arg5eUVleZ0z3ngfCt4NHL3gtSpsx0PdRfb9Z3d4LnIBYhKTxGNNB+uAM9FLkB0Rpro6vd8aYVZpwd39TeRXqmnuCCTmp/neHjVav3l4/3glmNb3iorKNQ63+AjhWIAYVXdz/nucfOhiis1Z8BFxwvf6ThZI/Q7uvE1snvlKAbw9b953sY1osEThdsHp71Dyzz0x+G1L1/+ocp+KFxOH1m/deT3PqCH6K6IlJW5sVlOW52F20TfPvNS9e69lpHRG5w4bh77teTgV8vW9jov7Mi8ZDB5HeiRkf1/wqZB3dcrnvvb6U68b35i9sLQxJkKVah/6F/rZaM9fQNNrW3fV3Wcqp36fMWM8Mf37Q6OiwZ66AYQDF/pLl9fJNqi4qqg2MjcT3YpNRFAEunt6SHxMXkle4Jio8Bd8QvuWf5FMdnZB+KvADubyfxz8WeNB1z7eoV/eGjmpnXJ5LcpSiCA3dCFtupde3vrGv71mSEJcSkrHkl+LMfH+b4eTZIJYDemH24/Wd1Ve9aoHbAajLZRw8SYNTg+JihaE6ieETYrIebutJCEWJAOiQXwPPw1VWQcABkHQMYBkHEAZBwAGQdAxgGQcQBkHAAZB0DGAZBxAGQcABkHQMYBkHEAZBwAGQdAxgGQcQBkHAAZB0DGAZBxAGQcABkHQMYBkHEAZBwAGQdAxgGQcQBkHAAZB0DGAZBxAGQcABkHQMYBkHEAZH8CAAD//9AOUfkAAAAGSURBVAMAfaN397eueJgAAAAASUVORK5CYII=";

const FIZZ_SYSTEM_PROMPT: &str = "You are Fizz, an energetic maker who turns ideas into action. Be upbeat, practical, and decisive. Help users plan, create, solve problems, and finish work. Keep the voice cheeky and second person, skip exclamation marks, and leave emoji to whoever you are talking to.";

const ROSIE_SYSTEM_PROMPT: &str = "You are Rosie, a warm and thoughtful communicator. Help users write clearly, organize ideas, brainstorm, summarize, and prepare for conversations. Be kind, creative, and concise. Keep the voice warm and second person, skip exclamation marks, and leave emoji to whoever you are talking to.";

const CLAY_SYSTEM_PROMPT: &str = "You are Clay, a curious and adventurous researcher. Explore questions, compare options, check assumptions, and explain what you find clearly. Be candid when uncertain and favor useful evidence. Keep the voice dry and second person, skip exclamation marks, and leave emoji to whoever you are talking to.";

const BUILT_IN_PERSONAS: &[BuiltInPersona] = &[
    BuiltInPersona {
        id: "builtin:fizz",
        display_name: "Fizz",
        avatar_url: Some(FIZZ_AVATAR),
        system_prompt: FIZZ_SYSTEM_PROMPT,
        // Botanical, not apiary: the pool used to run on bee words (Nectar,
        // Pollen, Hive, Waxwing, and "Buzz" itself), which surfaced as agent
        // display names.
        name_pool: &[
            "Bramble", "Comet", "Clover", "Amber", "Daisy", "Thistle", "Meadow", "Juniper",
            "Aster", "Sage", "Willow", "Orchard", "Sorrel", "Fennel", "Cedar", "Poppy", "Reed",
        ],
        model: None,
        runtime: None,
        default_active: true,
    },
    // Rosie and Clay are frank body's own vocabulary — the rosehip oil and the
    // clay mask. They shipped as Honey and Bumble, bee names inherited from Buzz.
    // The ids keep the old slugs: they are stored in every existing install's
    // agent records, team references and message history. See RENAMED_BUILT_INS
    // for how a stored record catches up to the new display name.
    BuiltInPersona {
        id: "builtin:honey",
        display_name: "Rosie",
        avatar_url: Some(ROSIE_AVATAR),
        system_prompt: ROSIE_SYSTEM_PROMPT,
        name_pool: &["Rosie"],
        model: None,
        runtime: None,
        default_active: true,
    },
    BuiltInPersona {
        id: "builtin:bumble",
        display_name: "Clay",
        avatar_url: Some(CLAY_AVATAR),
        system_prompt: CLAY_SYSTEM_PROMPT,
        name_pool: &["Clay"],
        model: None,
        runtime: None,
        default_active: true,
    },
];

/// Built-ins whose display name changed after they shipped, as
/// `(persona id, the name it shipped with)`.
///
/// `merge_personas` renames a stored record that still carries the old name.
/// One the user renamed themselves is left alone — same conservative shape as
/// the legacy-avatar refresh, which only replaces an avatar it recognises.
const RENAMED_BUILT_INS: &[(&str, &str)] =
    &[("builtin:honey", "Honey"), ("builtin:bumble", "Bumble")];

pub(crate) fn built_in_persona_avatar_url(id: &str) -> Option<&'static str> {
    BUILT_IN_PERSONAS
        .iter()
        .find(|persona| persona.id == id)
        .and_then(|persona| persona.avatar_url)
}

const RETIRED_PERSONAS: &[(&str, &str)] = &[
    (
        "builtin:solo",
        "",
    ),
    (
        "builtin:kit",
        "",
    ),
    (
        "builtin:scout",
        "",
    ),
    (
        "builtin:orchestrator",
        "You are an orchestration agent. Coordinate multi-step work across specialized agents, keep the overall plan moving, and synthesize results into a clear final outcome. When another agent should take a task, @mention them explicitly with the assignment, expected deliverable, and any relevant constraints or deadlines.",
    ),
    (
        "builtin:researcher",
        "You are a research agent. Gather relevant information, compare sources, call out uncertainty, and return concise findings with evidence.",
    ),
    (
        "builtin:planner",
        "You are a planning agent. Turn ambiguous requests into structured plans with milestones, dependencies, risks, and clear next actions. Do not implement the work yourself unless asked.",
    ),
    (
        "builtin:implementer",
        "You are a builder agent. Execute tasks directly, make code and configuration changes carefully, validate the result, and explain important decisions and follow-up items.",
    ),
    (
        "builtin:refactor",
        "You are a refactoring agent. Improve structure, naming, duplication, and module boundaries without changing externally observable behavior. Keep changes incremental, preserve compatibility, and add or update validation when behavior could drift.",
    ),
    (
        "builtin:reviewer",
        "You are a review agent. Inspect plans, code, and outputs for bugs, regressions, edge cases, security issues, and missing tests. Prioritize findings by severity, cite concrete evidence, and keep summaries secondary to the actual review.",
    ),
];

fn built_in_persona_records(now: &str) -> Vec<AgentDefinition> {
    BUILT_IN_PERSONAS
        .iter()
        .map(|persona| AgentDefinition {
            id: persona.id.to_string(),
            display_name: persona.display_name.to_string(),
            avatar_url: persona.avatar_url.map(|s| s.to_string()),
            system_prompt: persona.system_prompt.to_string(),
            runtime: persona.runtime.map(|s| s.to_string()),
            model: persona.model.map(|s| s.to_string()),
            provider: None,
            name_pool: persona.name_pool.iter().map(|s| s.to_string()).collect(),
            is_builtin: true,
            is_active: persona.default_active,
            source_team: None,
            source_team_persona_slug: None,
            env_vars: std::collections::BTreeMap::new(),
            respond_to: None,
            respond_to_allowlist: Vec::new(),
            parallelism: None,
            created_at: now.to_string(),
            updated_at: now.to_string(),
        })
        .collect()
}

pub(crate) fn built_in_persona_definition(id: &str, now: &str) -> Option<AgentDefinition> {
    built_in_persona_records(now)
        .into_iter()
        .find(|persona| persona.id == id)
}

fn built_in_order(id: &str) -> Option<usize> {
    BUILT_IN_PERSONAS
        .iter()
        .position(|persona| persona.id == id)
}

fn sort_personas(records: &mut [AgentDefinition]) {
    records.sort_by(|left, right| {
        let left_builtin = if left.is_builtin { 0 } else { 1 };
        let right_builtin = if right.is_builtin { 0 } else { 1 };

        left_builtin
            .cmp(&right_builtin)
            .then_with(
                || match (built_in_order(&left.id), built_in_order(&right.id)) {
                    (Some(left_order), Some(right_order)) => left_order.cmp(&right_order),
                    _ => std::cmp::Ordering::Equal,
                },
            )
            .then_with(|| {
                left.display_name
                    .to_lowercase()
                    .cmp(&right.display_name.to_lowercase())
            })
            .then_with(|| left.id.cmp(&right.id))
    });
}

fn merge_personas(mut stored: Vec<AgentDefinition>, now: &str) -> (Vec<AgentDefinition>, bool) {
    let mut changed = false;

    for built_in in built_in_persona_records(now) {
        if let Some(existing) = stored.iter_mut().find(|record| record.id == built_in.id) {
            if !existing.is_builtin {
                existing.is_builtin = true;
                changed = true;
            }
            // Catch a stored built-in up to a renamed display name. Nothing else
            // about a stored record is refreshed from the built-in definition, so
            // without this an install that predates the rename keeps the old name
            // for good.
            for (renamed_id, shipped_name) in RENAMED_BUILT_INS {
                if built_in.id == *renamed_id
                    && existing.display_name == *shipped_name
                    && existing.display_name != built_in.display_name
                {
                    existing.display_name = built_in.display_name.clone();
                    existing.updated_at = now.to_string();
                    changed = true;
                }
            }
        } else {
            stored.push(built_in);
            changed = true;
        }
    }

    // Demote any stored persona still flagged as built-in whose id is no
    // longer in BUILT_IN_PERSONAS (e.g. a built-in that has been retired).
    // The record stays so existing managed-agent and team references keep
    // working; the user can delete it from the catalog like any custom
    // persona once they no longer need it.
    for record in stored.iter_mut() {
        if record.is_builtin && built_in_order(&record.id).is_none() {
            record.is_builtin = false;
            record.updated_at = now.to_string();
            changed = true;
        }
    }

    // Soft-deprecate retired built-in personas that were replaced by
    // Fizz. Runs after demotion so the records are already
    // marked as non-builtin.
    if migrate_retired_personas(&mut stored, now) {
        changed = true;
    }

    sort_personas(&mut stored);
    (stored, changed)
}

/// Soft-deprecate retired built-in personas by appending " (retired)" to
/// their display name and marking them inactive. Never removes records —
/// the cost is extra records for pre-transition users, but this
/// eliminates dangling `persona_id` references in managed-agents.json
/// and teams.json.
fn migrate_retired_personas(stored: &mut [AgentDefinition], now: &str) -> bool {
    let mut changed = false;

    for record in stored.iter_mut() {
        if let Some((_, original_prompt)) = RETIRED_PERSONAS.iter().find(|(id, _)| *id == record.id)
        {
            let retired_suffix = " (retired)";
            let needs_suffix = !record.display_name.ends_with(retired_suffix);
            if needs_suffix || record.is_active {
                let was_unmodified = record.system_prompt == *original_prompt;
                eprintln!(
                    "buzz-desktop: persona-migration: retiring {} persona '{}' → '{} (retired)'",
                    if was_unmodified {
                        "unmodified"
                    } else {
                        "customized"
                    },
                    record.display_name,
                    record.display_name,
                );
                if needs_suffix {
                    record.display_name = format!("{}{}", record.display_name, retired_suffix);
                }
                record.is_active = false;
                record.updated_at = now.to_string();
                changed = true;
            }
        }
    }

    changed
}

pub fn ensure_persona_is_active(
    personas: &[AgentDefinition],
    persona_id: &str,
) -> Result<(), String> {
    let persona = personas
        .iter()
        .find(|candidate| candidate.id == persona_id)
        .ok_or_else(|| format!("agent {persona_id} not found"))?;

    if !persona.is_active {
        return Err(format!(
            "{} is not in My Agents. Choose it from Agent Catalog first.",
            persona.display_name
        ));
    }

    Ok(())
}

pub fn ensure_persona_ids_are_active(
    personas: &[AgentDefinition],
    persona_ids: &[String],
) -> Result<(), String> {
    for persona_id in persona_ids {
        ensure_persona_is_active(personas, persona_id)?;
    }

    Ok(())
}

pub fn validate_persona_deletion(
    persona: &AgentDefinition,
    referenced_by_team: bool,
) -> Result<(), String> {
    if persona.is_builtin {
        return Err("Built-in agents cannot be deleted.".to_string());
    }

    if persona.source_team.is_some() {
        return Err(format!(
            "{} belongs to a team. Delete the team to remove all team agents together.",
            persona.display_name
        ));
    }

    if referenced_by_team {
        return Err(format!(
            "{} is still referenced by a team. Remove it from those teams first.",
            persona.display_name
        ));
    }

    Ok(())
}

pub fn validate_persona_activation_change(
    persona: &AgentDefinition,
    active: bool,
    referenced_by_managed_agent: bool,
    referenced_by_team: bool,
) -> Result<(), String> {
    if !persona.is_builtin {
        return Err("Only built-in agents can be added to or removed from My Agents.".to_string());
    }

    if !active && referenced_by_managed_agent {
        return Err(format!(
            "{} is still assigned to a managed agent. Remove or reassign those agents first.",
            persona.display_name
        ));
    }

    if !active && referenced_by_team {
        return Err(format!(
            "{} is still referenced by a team. Remove it from those teams first.",
            persona.display_name
        ));
    }

    Ok(())
}

pub fn load_personas(app: &AppHandle) -> Result<Vec<AgentDefinition>, String> {
    let now = now_iso();

    // Post-fold: definitions live in the unified agent store, presented in
    // the legacy shape. Pre-fold stores are converted by
    // `fold_personas_into_agent_store` in boot migrations before any caller
    // reaches this shim.
    let records = crate::managed_agents::storage::load_agent_definitions(app)?
        .iter()
        .filter_map(|record| record.to_definition_view())
        .collect();

    let (records, changed) = merge_personas(records, &now);
    if changed {
        save_personas(app, &records)?;
    }

    Ok(records)
}

/// Read the raw persona records at `path` — no built-in merge, no write-back.
/// The single disk-read seam for persona definitions: `load_personas` layers
/// the built-in merge on top, and the boot-time readers that need raw records
/// without an `AppHandle` (`event_sync`, `migration::load_persona_runtimes`)
/// call it directly. The A2 store fold retargets THIS function at the unified
/// store; its callers stay unchanged.
pub(crate) fn load_personas_from_path(
    path: &std::path::Path,
) -> Result<Vec<AgentDefinition>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read persona store: {error}"))?;
    serde_json::from_str::<Vec<AgentDefinition>>(&content)
        .map_err(|error| format!("failed to parse persona store: {error}"))
}

pub fn save_personas(app: &AppHandle, records: &[AgentDefinition]) -> Result<(), String> {
    let mut sorted = records.to_vec();
    sort_personas(&mut sorted);

    // Post-fold: persona saves write key-less definition records into the
    // unified agent store (instances preserved by `save_agent_definitions`).
    let definitions: Vec<_> = sorted
        .into_iter()
        .map(|persona| persona.into_agent_record())
        .collect();
    crate::managed_agents::storage::save_agent_definitions(app, &definitions)
}

#[cfg(test)]
mod tests;
