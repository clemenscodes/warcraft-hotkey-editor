use crate::domain::default_config::DefaultConfig;

pub(crate) fn baseline_content() -> &'static str {
    DefaultConfig::content()
}
