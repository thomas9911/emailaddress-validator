extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;

const _GRAMMAR: &'static str = include_str!("../data/emailaddress.pest");

#[derive(Parser)]
#[grammar = "../data/emailaddress.pest"]
struct EmailParser;

#[derive(Debug)]
pub struct Email {
    address: String,
    group: Option<String>,
    mailbox: Option<String>,
    routeaddr: Option<String>,
    route: Option<String>,
    addrspec: Option<String>,
    localpart: Option<String>,
    domain: Option<String>,
    comment: Option<String>,
}

impl Email {
    pub fn new(email: String) -> Self {
        Email {
            address: email,
            ..Default::default()
        }
    }

    pub fn parse<S>(email: S) -> Result<Self, PestError<Rule>>
    where
        S: Into<String>,
    {
        let string_email = email.into();
        let mut email_object = Email::new(string_email.clone());
        let pairs = EmailParser::parse(Rule::emailaddress, &string_email)?;
        for pair in pairs {
            parse_email_pair(&mut email_object, pair)
        }
        Ok(email_object)
    }
}

impl Default for Email {
    fn default() -> Self {
        Email {
            address: String::new(),
            group: None,
            mailbox: None,
            routeaddr: None,
            route: None,
            addrspec: None,
            localpart: None,
            domain: None,
            comment: None,
        }
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

fn set_attribute(email: &mut Email, pair: Pair<Rule>) {
    match pair.as_rule() {
        Rule::group => email.group = Some(String::from(pair.as_str())),
        Rule::mailbox => email.mailbox = Some(String::from(pair.as_str())),
        Rule::routeaddr => email.routeaddr = Some(String::from(pair.as_str())),
        Rule::route => email.route = Some(String::from(pair.as_str())),
        Rule::addrspec => email.addrspec = Some(String::from(pair.as_str())),
        Rule::localpart => email.localpart = Some(String::from(pair.as_str())),
        Rule::domain => email.domain = Some(String::from(pair.as_str())),
        Rule::comment => email.comment = Some(String::from(pair.as_str())),
        _ => {}
    }
}

fn parse_email_pair(email: &mut Email, pair: Pair<Rule>) {
    let inner: Vec<Pair<Rule>> = pair.clone().into_inner().collect();
    if inner.len() == 0 {
        set_attribute(email, pair)
    };
    for t in inner {
        parse_email_pair(email, t)
    }
}

#[cfg(test)]
macro_rules! test_parse {
    ($t:expr) => {
        match Email::parse($t){
            Ok(_x) => (),
            Err(e) => {println!("{}", e); panic!()}
        }
    };
}

// tests based on examples from antlr https://github.com/antlr/grammars-v4/tree/master/rfc822-emailaddress
#[test]
fn example1() {
    test_parse!("tom@khubla.com");
}

#[test]
fn example2() {
    test_parse!("very.common@example.com")
}
#[test]
fn example3() {
    test_parse!("disposable.style.email.with+symbol@example.com")
}

#[test]
fn example4() {
    test_parse!("other.email-with-dash@example.com")
}

#[test]
fn example5() {
    test_parse!("x@example.com")
}

#[test]
fn example6() {
    test_parse!(r#""much.more unusual"@example.com"#)
}
#[test]
fn example7() {
    test_parse!("example-indeed@strange-example.com")
}
#[test]
fn example8() {
    test_parse!("admin@mailserver1")
}
#[test]
fn example9() {
    test_parse!("example@s.solutions")
}
