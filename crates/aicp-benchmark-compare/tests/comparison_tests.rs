use aicp_benchmark_compare::suspicious_scale_change;

#[test]
fn catches_thousand_fold_scale_bug() {
    assert!(suspicious_scale_change(18_428.0, 18.4));
}
