use crate::recurrence_relation::RecurrenceRelation;
use std::cmp::max;

#[derive(Debug)]
pub enum ParseRecurrenceError {
    NoRecurrence,
    MultipleRecurrence,
    TermMatchesNothing,
    MultipleBaseCase,
    NoBaseCase,
    MissingEquals,
    ParseFloatError,
    ParseIntError,
    BaseCaseError,
    RecurrenceError,
}

impl From<std::num::ParseFloatError> for ParseRecurrenceError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseRecurrenceError::ParseFloatError
    }
}

impl From<std::num::ParseIntError> for ParseRecurrenceError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseRecurrenceError::ParseIntError
    }
}

fn parse_base_case(s: &str) -> Result<(f64, usize), ParseRecurrenceError> {
    let mut parts = s.split('=');
    let left = parts.next().ok_or(ParseRecurrenceError::BaseCaseError)?;
    let right = parts.next().ok_or(ParseRecurrenceError::BaseCaseError)?;
    if parts.next().is_some() {
        return Err(ParseRecurrenceError::BaseCaseError);
    }
    let lparen = left.find('(').ok_or(ParseRecurrenceError::BaseCaseError)?;
    let rparen = left.find(')').ok_or(ParseRecurrenceError::BaseCaseError)?;
    let index: usize = left[lparen + 1..rparen].trim().parse()?;

    let val: f64 = right.trim().parse()?;
    Ok((val, index))
}

fn parse_recurrence(s: &str) -> Result<Vec<f64>, ParseRecurrenceError> {
    let mut parts = s.split('=');
    //todo: check that left has correct format, return error if it does not
    let _left = parts.next().ok_or(ParseRecurrenceError::RecurrenceError)?;
    let right = parts.next().ok_or(ParseRecurrenceError::RecurrenceError)?;
    if parts.next().is_some() {
        return Err(ParseRecurrenceError::RecurrenceError);
    }

    //todo: support minus sign as separator as well
    let parts = right.split('+');
    let mut degree = 0;
    let mut pairs: Vec<(f64, usize)> = Vec::new();
    for part in parts {
        let part = part.trim();
        let lparen_index = part
            .find('(')
            .ok_or(ParseRecurrenceError::RecurrenceError)?;
        let rparen_index = part
            .find(')')
            .ok_or(ParseRecurrenceError::RecurrenceError)?;
        let minus_index = part
            .find('-')
            .ok_or(ParseRecurrenceError::RecurrenceError)?;

        let index: usize = part[minus_index+1..rparen_index].trim().parse()?;
        degree = max(degree, index);
        match (lparen_index, part[..lparen_index-1].trim().parse()) {
            (1, _) => pairs.push((1.0, index)),
            (_, Ok(coefficient)) => pairs.push((coefficient, index)),
            _ => {
                let times_index = part
                    .find('*')
                    .ok_or(ParseRecurrenceError::RecurrenceError)?;
                let coefficient = part[..times_index].trim().parse()?;
                pairs.push((coefficient, index))
            }
        }
    }

    let mut res = vec![0.0; degree];
    for (coefficient, index) in pairs {
        res[index - 1] = coefficient;
    }
    Ok(res)
}

pub fn parse_recurrence_relation(s: &str) -> Result<RecurrenceRelation, ParseRecurrenceError> {
    let mut recurrence = None;
    let mut base_case_pairs = Vec::new();
    let equations = s.split(',').map(|x| x.trim());

    for equation in equations {
        if let Ok(parsed_base_case) = parse_base_case(equation) {
            base_case_pairs.push(parsed_base_case);
        } else if let Ok(parsed_recurrence) = parse_recurrence(equation) {
            if recurrence.is_some() {
                return Err(ParseRecurrenceError::MultipleRecurrence);
            }
            recurrence = Some(parsed_recurrence);
        } else {
            return Err(ParseRecurrenceError::TermMatchesNothing);
        }
    }

    let recurrence = recurrence.ok_or(ParseRecurrenceError::NoRecurrence)?;

    let degree = recurrence.len();
    let mut base_cases = vec![None; degree];
    for (num, index) in base_case_pairs {
        if base_cases[index].is_some() {
            return Err(ParseRecurrenceError::MultipleBaseCase);
        } else {
            base_cases[index] = Some(num);
        }
    }
    let base_cases = base_cases
        .iter()
        .map(|x| x.ok_or(ParseRecurrenceError::NoBaseCase))
        .collect::<Result<Vec<f64>, ParseRecurrenceError>>()?;
    Ok(RecurrenceRelation::new(base_cases, recurrence))
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_base_case() {
        assert_eq!(parse_base_case("f(0) = 1.0").unwrap(), (1.0, 0));
        assert_eq!(parse_base_case("f(0) = 1").unwrap(), (1.0, 0));
        assert_eq!(parse_base_case("f(0) = -1").unwrap(), (-1.0, 0));
        assert_eq!(parse_base_case("f(2) = 3.24").unwrap(), (3.24, 2));
        assert_eq!(parse_base_case("a(2) = 3.24").unwrap(), (3.24, 2));
    }

    #[test]
    fn test_parse_recurrence() {
        assert_eq!(parse_recurrence("f(n) = 1.0f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = 1f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = 3.24f(n-1)").unwrap(), vec![3.24]);
        assert_eq!(parse_recurrence("f(n) = 1.0*f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = 1  *f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = 3.24  *        f(n-1)").unwrap(), vec![3.24]);
        assert_eq!(
            parse_recurrence("f(n) = f(n-1) + f(n-2)").unwrap(),
            vec![1.0, 1.0]
        );
        assert_eq!(
            parse_recurrence("f(n) = 3*f(n-1) + 5*f(n-3) + 10.67*f(n-6)").unwrap(),
            vec![3.0, 0.0, 5.0, 0.0, 0.0, 10.67]
        );
    }

    #[test]
    fn test_parse_recurrence_relation() {
        let relation: RecurrenceRelation = "f(n) = f(n-1) + f(n-2), f(0) = 0, f(1) = 1"
            .parse()
            .unwrap();
        assert_eq!(
            relation,
            RecurrenceRelation::new(vec![0.0, 1.0], vec![1.0, 1.0])
        );
        let relation: RecurrenceRelation = "a(n) = 5a(n-4) + 6.7*a(n-3), a(3)=5.0, a(1)=4   ,   a(0)=1.00,a(2)=3".parse().unwrap();
        assert_eq!(
            relation,
            RecurrenceRelation::new(vec![1.0, 4.0, 3.0, 5.0], vec![0.0, 0.0, 6.7, 5.0])
        );
    }
        
}
