pub enum RangesRelation {
    /// The two ranges have no relation.
    ///
    /// Can happen if for example one is the empty range.
    None,

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
