//! Tree-walking interpreter for Diracord, with execution limits.
//!
//! The engine **never talks to Discord**. When code calls `reply` or `ban`,
//! the interpreter asks a [`Host`] to do it. The TypeScript side (or a test)
//! implements the host. Saved data goes through a [`diracord_value::Store`].
//!
//! ```
//! use diracord_interpreter::{block_on, Context, Interpreter, Options, Trigger, testing::TestHost};
//! use diracord_parser::{parse, ParseOptions};
//! use diracord_value::MemoryStore;
//!
//! let program = parse("command(\"ping\") { reply \"Pong!\" }", &ParseOptions::default()).program;
//! let engine = Interpreter::new(program, Options::default());
//! let mut host = TestHost::new();
//! let mut store = MemoryStore::new();
//! let trigger = Trigger::command("ping");
//! block_on(engine.run(&trigger, Context::default(), &mut host, &mut store)).unwrap();
//! assert_eq!(host.replies(), ["Pong!"]);
//! ```
//!
//! # Async without a runtime
//!
//! [`Host::perform`] returns a boxed future, so a host can wait for Discord.
//! The interpreter needs no async runtime: [`block_on`] is a tiny executor
//! (park/unpark), and an embedding can poll the future from its own loop.
//!
//! # Limits
//!
//! Every run is bounded by [`Limits`]: steps, CPU time (time spent waiting on
//! the host does not count), memory held in variables, nesting and number of
//! host actions. Hitting a limit stops the run and cannot be caught by `try`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod check;
mod context;
mod engine;
mod env;
mod error;
mod eval;
mod exec;
mod executor;
mod host;
mod layout;
mod limits;
pub mod testing;

pub use check::check;
pub use context::{Context, Trigger};
pub use engine::{Interpreter, Options, Outcome};
pub use error::RuntimeError;
pub use executor::block_on;
pub use host::{BoxFuture, Host, HostError};
pub use limits::Limits;
