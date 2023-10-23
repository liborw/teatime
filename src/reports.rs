use std::{error::Error, fmt::Display, str::FromStr, fs::File, io};

use crate::cli::ReportArgs;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupBy {
    Day,
    Week,
    Month,
    Label
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseGroupByError {
    UnknownGroupByKeyword
}

impl Error for ParseGroupByError {}

impl Display for ParseGroupByError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for GroupBy {
    type Err = ParseGroupByError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s.to_lowercase().as_str() {
            "day" => Ok(GroupBy::Day),
            "week" => Ok(GroupBy::Week),
            "month" => Ok(GroupBy::Month),
            "label" => Ok(GroupBy::Label),
            _ => Err(ParseGroupByError::UnknownGroupByKeyword)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::reports::{GroupBy, ParseGroupByError};


    #[test]
    fn test_group_by_from_str() {
        assert_eq!(GroupBy::from_str("day"), Ok(GroupBy::Day));
        assert_eq!(GroupBy::from_str("week"), Ok(GroupBy::Week));
        assert_eq!(GroupBy::from_str("month"), Ok(GroupBy::Month));
        assert_eq!(GroupBy::from_str("label"), Ok(GroupBy::Label));
        assert_eq!(GroupBy::from_str("_"), Err(ParseGroupByError::UnknownGroupByKeyword));
    }

}

