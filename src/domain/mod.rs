pub mod amount;
pub mod currency;
pub mod market;
pub mod offer;
pub mod statistics;

use crate::prelude::*;
pub enum CommandResult {
    Accepted,
    Ignored,
}
pub trait FutureCommandResult: Future<Item = CommandResult, Error = MailboxError> {}
impl<F> FutureCommandResult for F where F: Future<Item = CommandResult, Error = MailboxError> {}
