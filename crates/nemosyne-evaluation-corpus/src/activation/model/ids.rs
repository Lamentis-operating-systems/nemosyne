macro_rules! numeric_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u64);

        impl $name {
            /// Creates an identifier from its numeric value.
            #[must_use]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the underlying numeric value.
            #[must_use]
            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

numeric_id!(
    /// A version of one immutable evidence corpus.
    CorpusRevision
);
numeric_id!(
    /// A numeric identifier for one reference parameter set.
    ReferenceId
);
numeric_id!(
    /// A numeric identifier for one broad scenario category.
    ScenarioCategoryId
);
numeric_id!(
    /// A numeric identifier shared by related paired-contrast scenarios.
    SemanticCaseId
);
numeric_id!(
    /// A scenario-local identifier for one constructed fact.
    FactId
);

/// A declared evidence-corpus partition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CorpusSplit {
    /// Evidence available during parameter development.
    Calibration,
    /// Structurally reserved evidence; not a claim of statistical blindness.
    HeldOut,
}

/// The provenance class of one authored scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ScenarioProvenance {
    /// A project-authored synthetic situation.
    Constructed,
}

/// One value on the corpus authoring grid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EvidenceLevel {
    /// Numeric zero: no supporting applicability or fit, or the canonical
    /// encoding paired with an inactive candidate channel.
    Absent,
    /// Indirect or peripheral applicability or fit.
    Low,
    /// Material but secondary applicability or fit.
    Medium,
    /// Direct and important applicability or fit.
    High,
    /// Explicitly dominant applicability or exact fit.
    Maximal,
}

impl EvidenceLevel {
    /// Returns the exact unit-interval value represented by this level.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        match self {
            Self::Absent => 0.0,
            Self::Low => 0.25,
            Self::Medium => 0.5,
            Self::High => 0.75,
            Self::Maximal => 1.0,
        }
    }

    pub(in crate::activation) const fn index(self) -> usize {
        match self {
            Self::Absent => 0,
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
            Self::Maximal => 4,
        }
    }
}
