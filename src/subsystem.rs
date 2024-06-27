pub trait Subsystem: std::fmt::Debug {
    fn is_eq(&self, other: &'static dyn Subsystem) -> bool;
}
