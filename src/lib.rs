use std::process::Child;

pub mod prelude {
    pub use super::{DroppableProcess, OptionDroppableProcessTrait};
}

pub struct DroppableProcess(pub Child);

impl Drop for DroppableProcess {
    // Attribute up here because putting above the individual line wasn't working for some reason
    // There's also only one line of code in the function so it's not like it will accidentals hide other unused results
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        // It's ok to ignore this result because it only returns an error if the child is already dead.
        // The purpose of this function is just to ensure it's dead on drop, so if it is dead already, that's fine.
        self.0.kill();
    }
}

impl DroppableProcess {
    // Just an easier way to do `let _ = variable_name.0.kill()`
    // or an easier to understand and read way of doing `drop(variable_name)`
    pub fn kill(self) {
        // Why `drop(self)` instead of `self.0.kill()`?
        // Because kill requires mutability, and doing it this way lets the function be called on immutable child processes
        drop(self)
    }
}

pub trait OptionDroppableProcessTrait: private::Sealed {
    fn kill(&mut self);
}

impl OptionDroppableProcessTrait for Option<DroppableProcess> {
    fn kill(&mut self) {
        self.take();
    }
}

// https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed
mod private {
    pub trait Sealed {}
    impl Sealed for Option<super::DroppableProcess> {}
}
