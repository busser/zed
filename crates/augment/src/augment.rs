use feature_flags::FeatureFlag;

pub struct AugmentFeatureFlag;

impl FeatureFlag for AugmentFeatureFlag {
    const NAME: &'static str = "augment";

    fn enabled_for_staff() -> bool {
        true
    }
}
