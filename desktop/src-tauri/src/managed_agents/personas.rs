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

const HONEY_AVATAR: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAADDUlEQVR4nOzcS0hUUQCH8VOIo04+sheBSqGgKCTSRona2INq0wsiioIoWrgpWhjUQnpAEFFBLYQWBYX0ICKQogcRBlO4EIuEHkYwEYnWOO+Z0OpWMN2hcO7uf6a+H7O493DPLObj3Lu4hymI9V0z0CkwkCKAGAHECCBGADECiBFAjABiBBAjgBgBxAggRgAxAogRQIwAYgQQI4AYAcQIIEYAMQKIEUCMAGIEECOAGAHECCBGADECiBFAjABiBBAjgBgBxAggRgAxAogRQIwAYgQQI4AYAcQIIEYAMQKIEUCMAGIEELMowPbOU5/Go16urKuZf+bg7szpu/cjHUe6jTc7N7RvXLXEWIMVIEYAMQKIWRRg3fLWRDLlHDx99uptcOTPC5rqqhfVL3AOKivK3OPlpf4ta5c6B5OT367eefzXL29va55b+WNWQ221sck0C/+2MhxLdJ64EPw45h5sWFh1bN+2Il/h1HMHhoa7zvVMTHx1D65f0bpr00pjpenGPuUzSo7v35E1Uuo/undrzl/f0dJYu2fzavdIW3O9tb++sTOAo6LM7y/2ZU5LS4qKi3we586ZmXWDml1ZbizGQ1iMAGIEECOAGAHECCCWHwEiscSlWw89XvxhNGTyR54EiCd7evvMv4hbkBgBxAgglh8BqubN6j7c4fHi/uevu872mDzBChAjgBgBxAggRgAxAogRQIwAYpa+lB+PxOPJdOY0mkglU2mPc0dDEffp2OewsZiNAcKxxIGTF7NGovFDpy+n0l9yzh0YGu6+cts9Ehh8ef76XWMrizZm3bgX8L4zbs2yxZnxUDjW+6jfeNsZ19JY21RXY6xh0TPg5v0nU29Pf/Em6HzMz+3p7gDO+sj5tuBBYPDXQbGvkAD4jQBiBBCzcXf0f4UVIEYAMQKIEUCMAGIEECOAGAHECCBGADECiBFAjABiBBAjgBgBxAggRgAxAogRQIwAYgQQI4AYAcQIIEYAMQKIEUCMAGIEECOAGAHECCBGADECiBFAjABiBBAjgBgBxAggRgCx7wAAAP//nFtCPgAAAAZJREFUAwAXVKWJVw3xDAAAAABJRU5ErkJggg==";

const BUMBLE_AVATAR: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAEjklEQVR4nOzcX0yVdRzH8W8lCAfxHP4of0QKEInCpTEUJGiZsuViq2Y3LTY2ZZkbsFquC71o1dZF2k1Lt5rhhZtjnrlFyla6ypGaDkuDxBzICeiA/A84/Dmg9Ew39vjghXjz+Z6fn9fOzfPjec7F8z57fjy/88Aib7BXCGeREBQDgDEAGAOAMQAYA4AxABgDgDEAGAOAMQAYA4AxABgDgDEAGAOAMQAYA4AxABgDgDEAGAOAMQAYA4AxABgDgDEAGAOAMQAYA4AxABgDgDEAGAOAMQAYA4AxABgDgDEAGAOAMQAYA4AxABgDgDEAGAOAMQAYA4AxAJiWAOODQ4c2bZOFC3NFRMTEuOJi3MmJ8VkZy57JTN2QK6HjMSX/suyhA8wXHuXKeLlobdm2+Mx0Ue9xMU4wMN5S98PRNytO7fksOD4huhkYYM61k6e9ZZWBvgFRTMslaHpi4tLh2rnN29PTV44cnwkG7fskrVuTsn6t48DgaGByZDQ4Fhjt7u3/u3X+O2dsKtz6xceilZZJOCwyMv/dcvtI4nM5J6v32keSn1/j2MdhuMPf7K37s7bu1tTU3GDbT2e7Lv6Rsn6dqKT3EpT+YkHR7l0LOsSTmvzC+zvfOLTfMd7y/Y+ileo5YEnCclm4xJzsvHfK7CPtZ86LVmZOwtmlJfbNqZFR6yUqmRnAnZLsGBkfHBaVzFyKmBhynu7wJVGikpkB2n4+a990xXmi4mNFJQMDBPoHLxyosY+sfmWzaGVUAOs+oOnYd1eP11urEXOD1sc/r+Jt0SqUAlj3utYtlX1kdnY20NdvjQ93/DvQ6utrue44JDopofTLTyPc0aJVKAW4Xn/aej3gztYyddarJRurdizWOv3eZeYkHJeRVlC9I604X9Qz8z5goK39RNUeb3lVT9NV0c3k5ejuy38dK6ts2HdQFAulS1Du9rc2Vm6fP27NwGM9vcNd/v86/b6Gi4OtN+w/vXzEOzM5+dLe90QlE+YAT+oK63V3wbmwumLoRkdjzdFrthXQZu+JlRtyV20pFn0MvATFpKdu+eRDxzcH57/6VlQydg6wVqStm4C5zWFfZ1fjFdHH5En4ycI8+6bv1wuij8kPZrlX3rMoPdLlF31MDhAM3PNMyhNh4aKPyQFuNrfYN13LNK5IGxtgsO2fjnON9pGE7EzRR3WAsZsP+czSkK+zfvdHjsGnigtEH70B2n851/D5AVmggTZf66kzl2pq7Y8GWVZv3Rwe5RJ9tAS475Nxjn38vzf9dvCwY/D2zK1AT+9Y38BIV/eIv+e+b754aXTRBztFJQOfjnawPvivf71v+bNZopLJN2Jy5xux177Zr/bsi8G/BVmn/unSkvxd5aKbCQGsS3yEZ6kr1uNekRSXmR67Ki0hJyvS45ZQoGUOeGTxj/TAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgAjAHAGACMAcAYAIwBwBgA7H8AAAD//zIwLvYAAAAGSURBVAMAMLE5V3leTgUAAAAASUVORK5CYII=";

const FIZZ_SYSTEM_PROMPT: &str = "You are Fizz, an energetic maker who turns ideas into action. Be upbeat, practical, and decisive. Help users plan, create, solve problems, and finish work. Keep the voice cheeky and second person, skip exclamation marks, and leave emoji to whoever you are talking to.";

const HONEY_SYSTEM_PROMPT: &str = "You are Honey, a warm and thoughtful communicator. Help users write clearly, organize ideas, brainstorm, summarize, and prepare for conversations. Be kind, creative, and concise. Keep the voice warm and second person, skip exclamation marks, and leave emoji to whoever you are talking to.";

const BUMBLE_SYSTEM_PROMPT: &str = "You are Bumble, a curious and adventurous researcher. Explore questions, compare options, check assumptions, and explain what you find clearly. Be candid when uncertain and favor useful evidence. Keep the voice dry and second person, skip exclamation marks, and leave emoji to whoever you are talking to.";

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
    BuiltInPersona {
        id: "builtin:honey",
        display_name: "Honey",
        avatar_url: Some(HONEY_AVATAR),
        system_prompt: HONEY_SYSTEM_PROMPT,
        name_pool: &["Honey"],
        model: None,
        runtime: None,
        default_active: true,
    },
    BuiltInPersona {
        id: "builtin:bumble",
        display_name: "Bumble",
        avatar_url: Some(BUMBLE_AVATAR),
        system_prompt: BUMBLE_SYSTEM_PROMPT,
        name_pool: &["Bumble"],
        model: None,
        runtime: None,
        default_active: true,
    },
];

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
