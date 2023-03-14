use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, cut, opt, recognize, success},
    error::{context, convert_error, ContextError, ParseError, VerboseError},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    Finish, IResult, Parser,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequisiteType {
    Pre,
    Co,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Requisite<'a> {
    pub typ: RequisiteType,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Primary<'a> {
    Req(Requisite<'a>),
    Expr(Expression<'a>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrExpression<'a>(pub Vec<Primary<'a>>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AndExpression<'a>(pub Vec<OrExpression<'a>>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Expression<'a>(pub AndExpression<'a>);

fn is_uppercase(c: char) -> bool {
    matches!(c, 'A'..='Z')
}

fn is_numeric(c: char) -> bool {
    matches!(c, '0'..='9')
}

trait Err<'a>: ParseError<&'a str> + ContextError<&'a str> {}
impl<'a, T> Err<'a> for T where T: ParseError<&'a str> + ContextError<&'a str> {}

fn class_name<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let department = take_while1(is_uppercase);
    let space = multispace1;
    let num = take_while1(is_numeric);
    let modifier = take_while_m_n(0, 1, is_uppercase);

    let name = tuple((department, space, num, modifier));
    let name = preceded(multispace0, recognize(name));
    context("class_name", name)(input)
}

fn requesite_type<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, RequisiteType, E> {
    let concurrent = pair(tag("or concurrent"), opt(tag("ly")));
    let corequesite = concurrent.map(|_| RequisiteType::Co);
    let corequesite = context("corequesite", corequesite);

    let prerequesite = success(RequisiteType::Pre);

    let requesite_type = alt((corequesite, prerequesite));
    let requesite_type = preceded(multispace0, requesite_type);
    context("requesite_type", requesite_type)(input)
}

fn requisite<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, Requisite<'a>, E> {
    let requesite = pair(class_name, requesite_type);
    let requesite = requesite.map(|(name, typ)| Requisite { name, typ });
    context("requesite", requesite)(input)
}

fn primary<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, Primary<'a>, E> {
    let req = requisite.map(Primary::Req);
    let and = parenthesis_helper(expression).map(Primary::Expr);

    let expr = alt((req, and));
    context("primary", expr)(input)
}

fn parenthesis_helper<'a, T, E: Err<'a>>(
    parser: impl Parser<&'a str, T, E>,
) -> impl Parser<&'a str, T, E> {
    let parenthesis = delimited(tag("("), parser, cut(tag(")")));
    context("parenthesis", parenthesis)
}

fn expression_helper<'a, T, E: Err<'a>>(
    word: &'static str,
    parser: impl Parser<&'a str, T, E>,
) -> impl Parser<&'a str, Vec<T>, E> {
    separated_list1(delimited(multispace0, tag(word), multispace0), parser)
}

fn or_expression<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, OrExpression<'a>, E> {
    let expr = expression_helper("or", primary).map(OrExpression);
    context("or_expression", expr)(input)
}

fn and_expression<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, AndExpression<'a>, E> {
    let expr = expression_helper("and", or_expression).map(AndExpression);
    context("and_expression", expr)(input)
}

fn expression<'a, E: Err<'a>>(input: &'a str) -> IResult<&'a str, Expression<'a>, E> {
    let expression = and_expression.map(Expression);
    context("expression", preceded(multispace0, expression))(input)
}

fn verbose<'a, O>(
    parser: impl Parser<&'a str, O, VerboseError<&'a str>>,
    input: &'a str,
) -> Result<O, String> {
    let mut parser = all_consuming(terminated(parser, multispace0));
    match parser.parse(input).finish() {
        Ok((_, res)) => Ok(res),
        Err(e) => Err(convert_error(input, e)),
    }
}

#[test]
fn run_tests() {
    macro_rules! run {
        ($parser:ident, $value:expr) => {
            let res = verbose($parser, $value);
            let v = match res.as_ref() {
                Ok(v) => format!("{v:#?}"),
                Err(e) => e.to_owned(),
            };
            println!("---\n{}() on \"{}\": {}", stringify!($parser), $value, v);
            res.unwrap();
        };
    }

    run!(expression, "PHY 183B");
    run!(expression, "PHY 183B");
    run!(expression, "CSE 232");
    run!(expression, "CSE 220 or concurrently");

    run!(expression, "(CSE 220 or concurrently)");
    run!(expression, "FOO 100 or (CSE 220)");
    run!(expression, "FOO 100 and FOO 200 and FOO 300");

    let data = "((PHY 183 or concurrently) or (PHY 193H or concurrently) or PHY 183B) or (PHY 231 and (PHY 233B or concurrently)) or (PHY 231C and (PHY 233B or concurrently))
    MTH 234 or MTH 254H or LB 220
    (ECE 201) and ((MTH 235 or concurrently) or MTH 340 or MTH 347H)
    ECE 202 or concurrently
    (CSE 231 or concurrently) or (CSE 220 or concurrently)
    (MTH 234 or MTH 254H) and (ECE 201 or concurrently)
    ECE 202
    ECE 203 and (ECE 302 or concurrently) and (ECE 280 or concurrent)
    (CSE 220 or CSE 232) and (ECE 230 and (ECE 203 or concurrently))";

    for line in data.split('\n') {
        run!(expression, line);
    }
}

pub fn parse<'a>(input: &'a str) -> Expression<'a> {
    verbose(expression, input).unwrap()
}