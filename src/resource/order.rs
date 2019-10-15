use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::{map, map_res, opt},
    multi::separated_list,
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Order {
    pub field: String,
    pub ordering: Ordering,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ordering {
    Descending,
    Ascending,
}

/* impl Ordering {
 *     pub fn apply<Q, C, T, E, Db>(
 *         &self,
 *         query: Q,
 *         column: C,
 *     ) -> Box<dyn diesel::expression::BoxableExpression<T, Db, SqlType = ()>>
 *     where
 *         Q: diesel::query_builder::Query + diesel::query_dsl::methods::ThenOrderDsl<E>,
 *         C: diesel::query_source::Column<Table = T>
 *             + diesel::expression_methods::ExpressionMethods,
 *         T: diesel::query_source::Table,
 *         E: diesel::expression::BoxableExpression<T, Db, SqlType = ()>,
 *         Db: diesel::backend::Backend,
 *     {
 *         match self {
 *             Ordering::Descending => query.then_order_by(column.desc()).as_query(),
 *             Ordering::Ascending => query.then_order_by(column.asc()).as_query(),
 *         }
 *     }
 * }
 */

impl std::str::FromStr for Ordering {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let o = match s {
            "desc" => Ordering::Descending,
            "asc" => Ordering::Ascending,
            _ => return Err("no such ordering"),
        };

        Ok(o)
    }
}

#[macro_export]
macro_rules! order_query {
    ($query:expr, $col:expr, $order:expr) => {{
        match $order {
            crate::resource::order::Ordering::Descending => $query.then_order_by($col.desc()),
            crate::resource::order::Ordering::Ascending => $query.then_order_by($col.asc()),
        }
    }}
}

pub fn parse_orders(input: &str) -> Result<Vec<Order>, String> {
    match orders(input) {
        Ok(("", v)) => Ok(v),
        Ok((_, _)) => Err("trailing characters in parse_orders".to_string()),
        Err(e) => Err(format!("Error in parse_orders: {:?}", e)),
    }
}

// field1[:ordering1],field2[:ordering2], ...
fn orders(input: &str) -> IResult<&str, Vec<Order>> {
    separated_list(tag(","), order)(input)
}

// field[:ordering]
fn order(input: &str) -> IResult<&str, Order> {
    map(
        pair(field, opt_ordering),
        |(field, ordering)| Order {
            field: field.to_string(),
            ordering,
        },
    )(input)
}

// A-Za-z0-9_
fn field(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_')(input)
}

// parse a colon followed by an ordering if present, or default to Ordering::Descending
fn opt_ordering(input: &str) -> IResult<&str, Ordering> {
    map(opt(preceded(tag(":"), ordering)), |o| o.unwrap_or(Ordering::Descending))(input)
}

// asc|desc
fn ordering(input: &str) -> IResult<&str, Ordering> {
    use std::str::FromStr;
    map_res(alt((tag("desc"), tag("asc"))), Ordering::from_str)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        assert_eq!(ordering("desc"), Ok(("", Ordering::Descending)));
        assert_eq!(ordering("asc"), Ok(("", Ordering::Ascending)));
        assert!(ordering("something_else").is_err());
    }

    #[test]
    fn test_order_single_well_formed() {
        #[rustfmt::skip]
        let input_output = vec![
            ("field1", Order { field: "field1".to_string(), ordering: Ordering::Descending } ),
            ("field1:desc", Order { field: "field1".to_string(), ordering: Ordering::Descending } ),
            ("field1:asc", Order { field: "field1".to_string(), ordering: Ordering::Ascending } ),
            ("field_1", Order { field: "field_1".to_string(), ordering: Ordering::Descending } ),
            ("field_1:desc", Order { field: "field_1".to_string(), ordering: Ordering::Descending } ),
            ("field_1:asc", Order { field: "field_1".to_string(), ordering: Ordering::Ascending } ),
            ("field1", Order { field: "field1".to_string(), ordering: Ordering::Descending } ),
            ("field1:desc", Order { field: "field1".to_string(), ordering: Ordering::Descending } ),
            ("field1:asc", Order { field: "field1".to_string(), ordering: Ordering::Ascending } ),
        ];

        for (input, output) in input_output {
            assert_eq!(parse_orders(input).unwrap(), vec![output]);
        }
    }
}
