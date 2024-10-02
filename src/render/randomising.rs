use skia_safe::Rect;
use crate::app_state::AppState;
use crate::lib::skia::Skia;

pub fn randomising(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(app_state);
}
