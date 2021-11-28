use serde::{Deserialize, Serialize};
use std::{cmp::PartialEq, fmt::Debug};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Suppressor {
    IsAcOn,
    IsAcOff,
}

impl Suppressor {
    pub fn is_enabled(&self, is_ac: bool) -> bool {
        match self {
            Self::IsAcOn if is_ac => true,
            Self::IsAcOff if !is_ac => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(Suppressor::IsAcOn, true => true)]
    #[test_case(Suppressor::IsAcOn, false => false)]
    #[test_case(Suppressor::IsAcOff, true => false)]
    #[test_case(Suppressor::IsAcOff, false => true)]
    fn test_is_suppressor_enabled(suppressor: Suppressor, is_ac: bool) -> bool {
        suppressor.is_enabled(is_ac)
    }
}
