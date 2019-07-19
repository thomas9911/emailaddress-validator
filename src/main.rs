use email_parser::Email;

static HELP: &'static str = "email-validator

Form validates email address based on rfc822 spec

Usage:
    email-validator [EMAIL]
";

fn main() -> Result<(), ()> {
    let mut input = std::env::args();
    input.next().unwrap(); // binname
    let email_result = Email::parse(match input.next() {
        Some(x) => {
            if ["--help", "-h", "help", "/?"].contains(&x.as_str()) {
                return Ok(println!("{}", HELP));
            } else {
                x
            }
        }
        None => return Ok(println!("{}", HELP)),
    });
    match email_result {
        Ok(x) => Ok(println!("Valid email: {}", x)),
        Err(e) => Err(println!("{}", e)),
    }
}
