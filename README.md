# droppable_process

A wrapper around `std::process::Child` that kills the child when the wrapper is dropped.

Also provides a shorthand `.kill()` function for the wrapper that returns `()` instead of `std::io::Result<()>` if you don't care about whether the child was already dead.

Also provides a `.kill()` function for `Option<DroppableProcess>` that replaces the Option with a None if it isn't already, and kills the child.
