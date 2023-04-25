use hfst::implementations::{HfstBasicTransducer, HfstBasicTransition};

#[test]
fn test_examples() {
    let mut tr1 = HfstBasicTransducer::default();
    tr1.add_state(1);
    tr1.set_final_weight(1, 0.0);
    tr1.add_transition(
        0,
        HfstBasicTransition::new(1, "@_UNKNOWN_SYMBOL_@".into(), "foo".into(), 0.0),
    );
    // tr1 is [ @_UNKNOWN_SYMBOL_@:foo ]

    let mut tr2 = HfstBasicTransducer::default();
    tr2.add_state(1);
    tr2.add_state(2);
    tr2.set_final_weight(2, 0.0);
    tr2.add_transition(
        0,
        HfstBasicTransition::new(
            1,
            "@_IDENTITY_SYMBOL_@".into(),
            "@_IDENTITY_SYMBOL_@".into(),
            0.0,
        ),
    );
    tr2.add_transition(
        1,
        HfstBasicTransition::new(2, "bar".into(), "bar".into(), 0.0),
    );
    // tr2 is [ [ @_IDENTITY_SYMBOL_@:@_IDENTITY_SYMBOL_@ ] [ bar:bar ] ]

    // The disjunction should be
    let mut disj = HfstBasicTransducer::default();
    disj.add_state(1);
    disj.add_state(2);
    disj.set_final_weight(2, 0.0);

    disj.add_transition(
        0,
        HfstBasicTransition::new(
            1,
            "@_IDENTITY_SYMBOL_@".into(),
            "@_IDENTITY_SYMBOL_@".into(),
            0.0,
        ),
    );
    disj.add_transition(
        0,
        HfstBasicTransition::new(1, "foo".into(), "foo".into(), 0.0),
    );

    disj.add_transition(
        0,
        HfstBasicTransition::new(2, "@_UNKNOWN_SYMBOL_@".into(), "foo".into(), 0.0),
    );
    disj.add_transition(
        0,
        HfstBasicTransition::new(2, "bar".into(), "foo".into(), 0.0),
    );

    disj.add_transition(
        1,
        HfstBasicTransition::new(2, "bar".into(), "bar".into(), 0.0),
    );
}
