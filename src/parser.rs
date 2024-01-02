use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, one_of},
    combinator::{map, map_res, recognize},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

use crate::ast::{InsertStatement, SelectStatement, Statement, StatsStatement};

pub fn parse(input: &str) -> Result<Statement, String> {
    match parse_statement(input) {
        Ok((_, statement)) => Ok(statement),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((insert_statement, select_statement, stats_statement))(input)
}

fn insert_statement(input: &str) -> IResult<&str, Statement> {
    map(insert_statement_base, |s| {
        return Statement::Insert(InsertStatement {
            id: s.1,
            name: s.2,
            email: s.3,
        });
    })(input)
}

fn select_statement(input: &str) -> IResult<&str, Statement> {
    map(select_statement_base, |_| {
        return Statement::Select(SelectStatement {});
    })(input)
}

fn stats_statement(input: &str) -> IResult<&str, Statement> {
    map(stats_statement_base, |_| {
        return Statement::Stats(StatsStatement {});
    })(input)
}

fn select_statement_base(input: &str) -> IResult<&str, ()> {
    map(tag("select"), |_| ())(input)
}

fn stats_statement_base(input: &str) -> IResult<&str, ()> {
    map(tag("stats"), |_| ())(input)
}

fn insert_statement_base(input: &str) -> IResult<&str, (&str, i32, &str, &str)> {
    tuple((
        insert_tag,
        terminated(id, multispace0),
        terminated(alpha1, multispace0),
        terminated(alpha1, multispace0),
    ))(input)
}

fn insert_tag(input: &str) -> IResult<&str, &str> {
    terminated(tag("insert"), multispace0)(input)
}

fn id(input: &str) -> IResult<&str, i32> {
    map_res(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse::<i32>()
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_tag() {
        assert_eq!(
            insert_tag("insert 123 foo bar"),
            Ok(("123 foo bar", "insert"))
        );
        assert!(insert_tag("test insert").is_err());
    }

    #[test]
    fn test_id() {
        assert_eq!(id("123"), Ok(("", 123)));
        assert_eq!(id("12345"), Ok(("", 12345)));
        assert_eq!(id("123_456"), Ok(("_456", 123)));
        assert!(id("a_123").is_err());
    }

    #[test]
    fn test_insert_statement() {
        assert!(matches!(
            parse_statement("insert 123 foo bar"),
            Ok((
                "",
                Statement::Insert(InsertStatement {
                    id: 123,
                    name: "foo",
                    email: "bar"
                })
            ))
        ))
    }

    #[test]
    fn test_select_statement() {
        assert!(matches!(
            parse_statement("select"),
            Ok(("", Statement::Select(SelectStatement {})))
        ))
    }

    #[test]
    fn test_stats_statement() {
        assert!(matches!(
            parse_statement("stats"),
            Ok(("", Statement::Stats(StatsStatement {})))
        ))
    }
}
