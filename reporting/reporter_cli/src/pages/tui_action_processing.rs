use std::iter::empty;

use reporter_state::stores::store_action_processing::StoreAction;

/// Converts a `TUI`` action into a `Store`` action.
#[derive(Debug, Default)]
pub(crate) struct TuiActionProcessing {
    
}

#[derive(Clone, Debug)]
pub(crate) enum TuiAction {

}

impl TuiActionProcessing {
    pub(crate) fn process_action(action: TuiAction) -> impl Iterator<Item = StoreAction> {
        todo!();
        empty()
    }
}