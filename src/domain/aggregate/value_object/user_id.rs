use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(usize);

impl UserId {
    pub fn gen() -> Self {
        Self(rand::random::<usize>())
    }
}

impl std::convert::From<usize> for UserId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<UserId> for usize {
    fn from(user_id: UserId) -> usize {
        user_id.0
    }
}
