#[cfg(target_os = "linux")]
mod linux;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalInput {
    KeyPress,
    MouseLeft,
    MouseRight,
}

pub fn start<F>(on_event: F)
where
    F: Fn(GlobalInput) + Send + Sync + 'static,
{
    #[cfg(target_os = "linux")]
    linux::start(on_event);
}
