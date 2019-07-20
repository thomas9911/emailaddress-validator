//! # Email validator
//!
//! Form validates email address based on rfc822 spec
//!
//!
//! ```
//! use email_parser::PestError;
//! use email_parser::Rule;
//! use email_parser::Email;
//!
//! fn main() -> Result<(), PestError<Rule>> {
//!     let email_result = Email::parse("test@example.com")?;
//!     Ok(())
//! }
//! ```


extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;

const _GRAMMAR: &'static str = include_str!("../data/emailaddress.pest");

#[derive(Parser)]
#[grammar = "../data/emailaddress.pest"]
struct EmailParser;

/// ```
/// use email_parser::PestError;
/// use email_parser::Rule;
/// use email_parser::Email;
///
/// fn main() -> Result<(), PestError<Rule>> {
///     let email_result = Email::parse("test@example.com")?;
///     Ok(())
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct Email {
    pub address: String,
    pub group: Option<String>,
    pub mailbox: Option<String>,
    pub mailboxes: Option<Vec<String>>,
    pub displayname: Option<String>,
    pub routeaddr: Option<String>,
    pub route: Option<String>,
    pub localpart: Option<String>,
    pub domain: Option<String>,
    pub comment: Option<String>,
}

impl Email {
    /// Creates a new Email struct without logic. You probably want to use `parse`.
    pub fn new(email: String) -> Self {
        Email {
            address: email,
            ..Default::default()
        }
    }

    /// Parses `email` and returns on success an `Email` struct with `email` split into its components.
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
            /// ping pong
            address: String::new(),
            group: None,
            mailbox: None,
            mailboxes: None,
            displayname: None,
            routeaddr: None,
            route: None,
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
        Rule::mailboxes => match &mut email.mailboxes {
            Some(x) => {
                let tmp = String::from(pair.as_str());
                if &tmp != "," {
                    x.push(tmp)
                }
            }
            None => email.mailboxes = Some(vec![String::from(pair.as_str())]),
        },
        Rule::displayname => email.displayname = Some(String::from(pair.as_str())),
        Rule::routeaddr => email.routeaddr = Some(String::from(pair.as_str())),
        Rule::route => email.route = Some(String::from(pair.as_str())),
        Rule::localpart => email.localpart = Some(String::from(pair.as_str())),
        Rule::domain => email.domain = Some(String::from(pair.as_str())),
        Rule::comment => email.comment = Some(String::from(pair.as_str())),
        _ => {}
    }
}

fn parse_email_pair(email: &mut Email, pair: Pair<Rule>) {
    let inner: Vec<Pair<Rule>> = pair.clone().into_inner().collect();
    set_attribute(email, pair);
    for t in inner {
        parse_email_pair(email, t)
    }
}

#[cfg(test)]
macro_rules! test_parse {
    ($t:expr) => {
        match Email::parse($t) {
            Ok(_x) => (),
            Err(e) => {
                println!("{}", e);
                panic!()
            }
        }
    };
}

#[cfg(test)]
macro_rules! test_not_parse {
    ($t:expr) => {
        match Email::parse($t) {
            Ok(x) => {
                println!("{}", x);
                panic!()
            }
            Err(_e) => (),
        }
    };
}

#[test]
fn valid() {
    // from https://en.wikipedia.org/wiki/Email_address
    const VALID: [&'static str; 17] = [
        "simple@example.com",
        "very.common@example.com",
        "disposable.style.email.with+symbol@example.com",
        "other.email-with-hyphen@example.com",
        "fully-qualified-domain@example.com",
        "user.name+tag+sorting@example.com",
        "x@example.com",
        "example-indeed@strange-example.com",
        "admin@mailserver1",
        "example@s.example",
        r#"" "@example.org"#,
        r#""john..doe"@example.org"#,
        "jsmith@[IPv6:2001:db8::1]",
        "examples:test1@example.com,test2@example.com;",
        "test<test@example.com>",
        r#""Test Example"<@domain,@domain2:test@example.com>"#,
        r#"examples:"Test Example"<test@example.com>,peter<peter@example.com>;"#,
    ];

    for v in VALID.iter() {
        test_parse!(*v)
    }
}

#[test]
fn invalid() {
    const INVALID: [&'static str; 6] = [
        "Abc.example.com",
        "A@b@c@example.com",
        r#"a"b(c)d,e:f;g<h>i[j\k]l@example.com"#,
        r#"just"not"right@example.com"#,
        r#"this is"not\allowed@example.com"#,
        r#"this\ still\"not\\allowed@example.com"#,
    ];

    for v in INVALID.iter() {
        test_not_parse!(*v)
    }
}

#[test]
fn parse_correct() {
    let email = Email::parse("jsmith@[IPv6:2001:db8::1]").unwrap();

    let expect = Email {
        address: String::from("jsmith@[IPv6:2001:db8::1]"),
        mailbox: Some(String::from("jsmith@[IPv6:2001:db8::1]")),
        localpart: Some(String::from("jsmith")),
        domain: Some(String::from("[IPv6:2001:db8::1]")),
        ..Default::default()
    };

    assert_eq!(email, expect);
}

#[test]
fn parse_correct_2() {
    let email = Email::parse("examples:test1@example.com,test2@example.com;").unwrap();

    let expect = Email {
        address: String::from("examples:test1@example.com,test2@example.com;"),
        group: Some(String::from(
            "examples:test1@example.com,test2@example.com;",
        )),
        mailboxes: Some(vec![
            String::from("test1@example.com"),
            String::from("test2@example.com"),
        ]),
        mailbox: Some(String::from("test2@example.com")),
        displayname: Some(String::from("examples")),
        localpart: Some(String::from("test2")),
        domain: Some(String::from("example.com")),
        ..Default::default()
    };

    assert_eq!(email, expect);
}

#[test]
fn parse_correct_3() {
    let email = Email::parse(r#""Test Example"<@domain,@domain2:test@example.com>"#).unwrap();
    let expect = Email {
        address: String::from(r#""Test Example"<@domain,@domain2:test@example.com>"#),
        mailbox: Some(String::from(
            r#""Test Example"<@domain,@domain2:test@example.com>"#,
        )),
        route: Some(String::from("@domain,@domain2:")),
        routeaddr: Some(String::from("<@domain,@domain2:test@example.com>")),
        localpart: Some(String::from("test")),
        domain: Some(String::from("example.com")),
        ..Default::default()
    };

    assert_eq!(email, expect);
}

#[test]
fn parse_correct_4() {
    let email =
        Email::parse(r#"examples:"Test Example"<test@example.com>,test2<test2@example.com>;"#)
            .unwrap();
    let expect = Email {
        address: String::from(
            r#"examples:"Test Example"<test@example.com>,test2<test2@example.com>;"#,
        ),
        group: Some(String::from(
            r#"examples:"Test Example"<test@example.com>,test2<test2@example.com>;"#,
        )),
        mailboxes: Some(vec![
            String::from("\"Test Example\"<test@example.com>"),
            String::from("test2<test2@example.com>"),
        ]),
        mailbox: Some(String::from("test2<test2@example.com>")),
        routeaddr: Some(String::from("<test2@example.com>")),
        displayname: Some(String::from("examples")),
        localpart: Some(String::from("test2")),
        domain: Some(String::from("example.com")),
        ..Default::default()
    };

    assert_eq!(email, expect);
}
