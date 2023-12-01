use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Thing {}

impl FromStr for Thing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Thing {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_thing_from_str() {
        assert_eq!(get_input::<Thing>(Input::Test1)[0], Thing {});
    }
}
