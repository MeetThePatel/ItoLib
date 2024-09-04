use thiserror::Error;

use crate::Float;

#[repr(transparent)]
#[derive(Debug)]
#[derive(Error)]
#[derive(Clone)]
pub struct DomainError(pub Float);

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} does not fit into the domain.", self.0)
    }
}
