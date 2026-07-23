//! Public-contract tests for the revision-1 coding-agent evidence corpus.

use nemosyne_evaluation::activation::{EvaluationReport, evaluate_parameters};
use nemosyne_evaluation_corpus::activation::{
    ActivationEvidenceCorpus, CorpusPartition, coding_agent_v1,
};

#[path = "coding_agent_v1/provenance.rs"]
mod provenance;
#[path = "coding_agent_v1/references.rs"]
mod references;
#[path = "coding_agent_v1/revision.rs"]
mod revision;
#[path = "coding_agent_v1/structure.rs"]
mod structure;

fn corpus() -> ActivationEvidenceCorpus {
    coding_agent_v1().expect("revision-1 corpus must satisfy its contract")
}

fn report(
    corpus: &ActivationEvidenceCorpus,
    reference_key: &str,
    partition: &CorpusPartition,
) -> EvaluationReport {
    let reference = corpus
        .references()
        .iter()
        .find(|reference| reference.key() == reference_key)
        .expect("named reference must exist");
    evaluate_parameters(reference.parameters(), partition.suite())
        .expect("reference must evaluate the complete partition")
}

fn assert_strictly_sorted<T: Ord + Copy + std::fmt::Debug>(values: impl IntoIterator<Item = T>) {
    let values: Vec<_> = values.into_iter().collect();
    assert!(
        values.windows(2).all(|pair| pair[0] < pair[1]),
        "values are not strictly sorted: {values:?}"
    );
}
