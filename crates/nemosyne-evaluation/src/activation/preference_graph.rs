use nemosyne_core::activation::{ActivationCandidate, CandidateId};

use super::{EvaluationError, ExpectedPreference, ScenarioId};

#[derive(Clone, Copy)]
struct ResolvedPreference {
    preferred_index: usize,
    other_index: usize,
}

#[derive(Clone, Copy)]
struct OutgoingEdge {
    edge_index: usize,
    other_index: usize,
}

struct PreferenceGraph {
    edges: Box<[ResolvedPreference]>,
    outgoing: Vec<Vec<OutgoingEdge>>,
    indegrees: Box<[usize]>,
}

impl PreferenceGraph {
    fn new(candidate_count: usize, edges: Vec<ResolvedPreference>) -> Self {
        let mut outgoing = vec![Vec::new(); candidate_count];
        let mut indegrees = vec![0; candidate_count];
        for (edge_index, edge) in edges.iter().enumerate() {
            outgoing[edge.preferred_index].push(OutgoingEdge {
                edge_index,
                other_index: edge.other_index,
            });
            indegrees[edge.other_index] += 1;
        }

        Self {
            edges: edges.into_boxed_slice(),
            outgoing,
            indegrees: indegrees.into_boxed_slice(),
        }
    }

    fn contains_cycle(&self) -> bool {
        let mut indegrees = self.indegrees.to_vec();
        let mut available: Vec<usize> = indegrees
            .iter()
            .enumerate()
            .filter_map(|(index, degree)| (*degree == 0).then_some(index))
            .collect();
        let mut visited = 0;
        while let Some(index) = available.pop() {
            visited += 1;
            for edge in &self.outgoing[index] {
                indegrees[edge.other_index] -= 1;
                if indegrees[edge.other_index] == 0 {
                    available.push(edge.other_index);
                }
            }
        }

        visited != self.outgoing.len()
    }

    fn has_alternate_path(
        &self,
        excluded_edge_index: usize,
        preference: ResolvedPreference,
    ) -> bool {
        let mut visited = vec![false; self.outgoing.len()];
        let mut pending = vec![preference.preferred_index];
        visited[preference.preferred_index] = true;

        while let Some(candidate_index) = pending.pop() {
            for edge in &self.outgoing[candidate_index] {
                if edge.edge_index == excluded_edge_index {
                    continue;
                }
                if edge.other_index == preference.other_index {
                    return true;
                }
                if !visited[edge.other_index] {
                    visited[edge.other_index] = true;
                    pending.push(edge.other_index);
                }
            }
        }

        false
    }
}

pub(super) fn validate_preferences(
    scenario_id: ScenarioId,
    candidates: &[ActivationCandidate],
    preferences: &[ExpectedPreference],
) -> Result<(), EvaluationError> {
    if preferences.is_empty() {
        return Err(EvaluationError::NoPreferences { scenario_id });
    }

    let mut resolved = Vec::with_capacity(preferences.len());
    for preference in preferences {
        if preference.preferred() == preference.other() {
            return Err(EvaluationError::SelfPreference {
                scenario_id,
                candidate_id: preference.preferred(),
            });
        }

        let preferred_index = resolve_candidate(candidates, preference.preferred()).ok_or(
            EvaluationError::UnknownPreferenceCandidate {
                scenario_id,
                candidate_id: preference.preferred(),
            },
        )?;
        let other_index = resolve_candidate(candidates, preference.other()).ok_or(
            EvaluationError::UnknownPreferenceCandidate {
                scenario_id,
                candidate_id: preference.other(),
            },
        )?;
        resolved.push(ResolvedPreference {
            preferred_index,
            other_index,
        });
    }

    if let Some(pair) = preferences.windows(2).find(|pair| pair[0] == pair[1]) {
        return Err(EvaluationError::DuplicatePreference {
            scenario_id,
            preferred: pair[0].preferred(),
            other: pair[0].other(),
        });
    }
    let graph = PreferenceGraph::new(candidates.len(), resolved);
    if graph.contains_cycle() {
        return Err(EvaluationError::CyclicPreferences { scenario_id });
    }
    if let Some((edge_index, _)) = graph
        .edges
        .iter()
        .enumerate()
        .find(|(edge_index, edge)| graph.has_alternate_path(*edge_index, **edge))
    {
        let preference = preferences[edge_index];
        return Err(EvaluationError::RedundantPreference {
            scenario_id,
            preferred: preference.preferred(),
            other: preference.other(),
        });
    }

    Ok(())
}

fn resolve_candidate(
    candidates: &[ActivationCandidate],
    candidate_id: CandidateId,
) -> Option<usize> {
    candidates
        .binary_search_by_key(&candidate_id, ActivationCandidate::candidate_id)
        .ok()
}
