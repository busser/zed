use edit_prediction_types::{
    EditPrediction, EditPredictionDelegate, EditPredictionDiscardReason, EditPredictionIconSet,
};
use gpui::{App, Context, Entity};
use icons::IconName;
use language::{Anchor, Buffer};
use std::path::{Path, PathBuf};

pub struct AugmentEditPredictionDelegate;

impl EditPredictionDelegate for AugmentEditPredictionDelegate {
    fn name() -> &'static str {
        "augment"
    }

    fn display_name() -> &'static str {
        "Augment"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn icons(&self, _cx: &App) -> EditPredictionIconSet {
        // TODO(busser): replace with Augment icons
        EditPredictionIconSet::new(IconName::Copilot)
            .with_disabled(IconName::CopilotDisabled)
            .with_error(IconName::CopilotError)
    }

    fn is_refreshing(&self, _cx: &App) -> bool {
        false
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, _cx: &App) -> bool {
        false
    }

    fn refresh(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: Anchor,
        _debounce: bool,
        _cx: &mut Context<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {}

    fn discard(&mut self, _reason: EditPredictionDiscardReason, _cx: &mut Context<Self>) {}

    fn suggest(
        &mut self,
        _buffer: &Entity<Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        None
    }
}

pub fn resolve_server_path(custom_path: Option<&str>, default_path: &Path) -> PathBuf {
    custom_path
        .map(PathBuf::from)
        .unwrap_or_else(|| default_path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_server_path_uses_default_when_no_custom_path() {
        let result = resolve_server_path(None, Path::new("/default/server.js"));
        assert_eq!(result, PathBuf::from("/default/server.js"));
    }

    #[test]
    fn test_resolve_server_path_uses_custom_path_when_provided() {
        let result =
            resolve_server_path(Some("/custom/server.js"), Path::new("/default/server.js"));
        assert_eq!(result, PathBuf::from("/custom/server.js"));
    }
}
