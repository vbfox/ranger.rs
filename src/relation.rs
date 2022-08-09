#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum RangesRelation {
    /// The first range is strictly before the second one with no overlap
    ///
    /// ```text
    /// [ A ]
    ///       [ B ]
    /// ```
    StrictlyBefore,

    /// The first range is strictly after the second one with no overlap
    ///
    /// ```text
    ///       [ A ]
    /// [ B ]
    /// ```
    StrictlyAfter,

    ///*
    /// ```text
    /// [ A ]
    ///     [ B ]
    /// ```
    ////
    Meets,

    ///*
    /// ```text
    ///     [ A ]
    /// [ B ]
    /// ```
    ////
    IsMet,

    ///*
    /// ```text
    /// [ A ]
    ///   [ B ]
    /// ```
    ////
    Overlaps,

    ///*
    /// ```text
    ///   [ A ]
    /// [ B ]
    /// ```
    ////
    IsOverlapped,

    ///*
    /// ```text
    /// [ A ]
    /// [   B   ]
    /// ```
    ////
    Starts,

    ///*
    /// ```text
    /// [   A   ]
    /// [ B ]
    /// ```
    ////
    IsStarted,

    ///*
    /// ```text
    /// [   A   ]
    ///   [ B ]
    /// ```
    ////
    StrictlyContains,

    ///*
    /// ```text
    ///   [ A ]
    /// [   B   ]
    /// ```
    ////
    IsStrictlyContained,

    ///*
    /// ```text
    ///     [ A ]
    /// [   B   ]
    /// ```
    ////
    Finishes,

    ///*
    /// ```text
    /// [   A   ]
    ///     [ B ]
    /// ```
    ////
    IsFinished,

    ///*
    /// ```text
    /// [ A ]
    /// [ B ]
    /// ```
    ////
    Equal,
}

impl RangesRelation {
    pub fn intersects(&self) -> bool {
        match self {
            RangesRelation::StrictlyBefore | RangesRelation::StrictlyAfter => false,

            RangesRelation::Overlaps
            | RangesRelation::IsOverlapped
            | RangesRelation::Meets
            | RangesRelation::IsMet
            | RangesRelation::Starts
            | RangesRelation::IsStarted
            | RangesRelation::StrictlyContains
            | RangesRelation::IsStrictlyContained
            | RangesRelation::Finishes
            | RangesRelation::IsFinished
            | RangesRelation::Equal => true,
        }
    }
}
