use crate::types::Visibility::Visibility::Visible;

#[derive(Debug,Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    IsShow,
    IsHide,
}


impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
            Visibility::IsShow => Visibility::IsShow,
            Visibility::IsHide => Visibility::IsHide,
        }
    }
}