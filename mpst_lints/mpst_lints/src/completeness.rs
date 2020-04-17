declare_lint!(COMPLETENESS_QUEUES, Deny, "check the correctness of a SessionMpst with respect to its queues");

declare_lint!(COMPLETENESS_SESSION, Deny, "check the correctness of a SessionMpst with respect to its session");

declare_lint!(DUALITT, Deny, "check the correctness of a SessionMpst with respect to the others");

#[derive(Copy, Clone)]
struct Incomplete {}

impl_lint_pass!(Incomplete => [COMPLETENESS_QUEUES, COMPLETENESS_SESSION]);

impl Incomplete {
    pub fn new() -> Incomplete {
        Incomplete {}
    }
}

