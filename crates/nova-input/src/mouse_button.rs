#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[cfg(feature = "winit")]
impl From<winit::event::MouseButton> for MouseButton {
    #[inline]
    fn from(button: winit::event::MouseButton) -> Self {
        use winit::event::MouseButton as Button;

        match button {
            Button::Left => MouseButton::Left,
            Button::Right => MouseButton::Right,
            Button::Middle => MouseButton::Middle,
            Button::Other(button) => MouseButton::Other(button),
        }
    }
}
