use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::{map, map_res, opt},
    multi::separated_list,
    sequence::{pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Filter {
    pub field: String,
    pub op: FilterOp,
    pub val: String,
}

impl Filter {
    pub fn new<S>(field: S, op: FilterOp, val: S) -> Filter
    where
        S: AsRef<str>,
    {
        Filter {
            field: field.as_ref().to_string(),
            op,
            val: val.as_ref().to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FilterOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl std::str::FromStr for FilterOp {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FilterOp::*;
        let o = match s {
            "eq" => Eq,
            "ne" => Ne,
            "gt" => Gt,
            "lt" => Lt,
            "ge" => Ge,
            "le" => Le,
            _ => return Err("no such filter op"),
        };

        Ok(o)
    }
}

/// Apply a filter to a query.
///
/// In effect, applies a WHERE clause (or an AND, if one already exists) comparing the expression
/// in `$a` to the one in `$b` using the comparator `$op`.
#[macro_export]
macro_rules! filter_query {
    ($query:expr, $a:expr, $op:expr, $b:expr) => {{
        match $op {
            crate::resource::filter::FilterOp::Eq => $query.filter($a.eq($b)),
            crate::resource::filter::FilterOp::Ne => $query.filter($a.ne($b)),
            crate::resource::filter::FilterOp::Gt => $query.filter($a.gt($b)),
            crate::resource::filter::FilterOp::Lt => $query.filter($a.lt($b)),
            crate::resource::filter::FilterOp::Ge => $query.filter($a.ge($b)),
            crate::resource::filter::FilterOp::Le => $query.filter($a.le($b)),
        }
    }};
}

pub fn parse_filters(input: &str) -> Result<Vec<Filter>, String> {
    match filters(input) {
        Ok(("", v)) => Ok(v),
        Ok((r, _)) => Err(format!("trailing characters in parse_filters: {}", r)),
        Err(e) => Err(format!("Error in parse_filters: {:?}", e)),
    }
}

fn filters(input: &str) -> IResult<&str, Vec<Filter>> {
    separated_list(tag(","), filter)(input)
}

// field:[op:]val
fn filter(input: &str) -> IResult<&str, Filter> {
    map(
        pair(terminated(field, tag(":")), op_value),
        |(field, (op, val))| Filter {
            field: field.to_string(),
            op,
            val,
        },
    )(input)
}

fn field(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn op_value(input: &str) -> IResult<&str, (FilterOp, String)> {
    pair(opt_filter_op, map(value, |s| s.to_string()))(input)
}

fn opt_filter_op(input: &str) -> IResult<&str, FilterOp> {
    map(opt(terminated(filter_op, tag(":"))), |o| {
        o.unwrap_or(FilterOp::Eq)
    })(input)
}

fn filter_op(input: &str) -> IResult<&str, FilterOp> {
    use std::str::FromStr;
    map_res(
        alt((
            tag("eq"),
            tag("ne"),
            tag("gt"),
            tag("lt"),
            tag("ge"),
            tag("le"),
        )),
        FilterOp::from_str,
    )(input)
}

fn value(input: &str) -> IResult<&str, &str> {
    // TODO: expand this
    take_while(|c: char| c.is_alphanumeric() || c == '_')(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filter_single_well_formed() {
        let input_output = vec![
            ("field1:0", Filter::new("field1", FilterOp::Eq, "0")),
            ("field1:lt:0", Filter::new("field1", FilterOp::Lt, "0")),
            ("field1:ne:val", Filter::new("field1", FilterOp::Ne, "val")),
            ("Field1:gt:6", Filter::new("Field1", FilterOp::Gt, "6")),
        ];

        for (input, output) in input_output {
            assert_eq!(parse_filters(input).unwrap(), vec![output]);
        }
    }
}
