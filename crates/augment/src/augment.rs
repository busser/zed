use edit_prediction_types::{
    EditPrediction, EditPredictionDelegate, EditPredictionDiscardReason, EditPredictionIconSet,
};
use gpui::{App, Context, Entity};
use icons::IconName;
use language::{Anchor, Buffer};

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
