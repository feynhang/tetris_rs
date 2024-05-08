use crate::const_vals::INFO_TEXT_POINT;

pub(crate) fn text() -> String {
    format!("{} Help [h]", INFO_TEXT_POINT.to_moving_string())
}
