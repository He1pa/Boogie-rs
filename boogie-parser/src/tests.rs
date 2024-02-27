use crate::boogie;

#[test]
fn parse_test() {
    let prog = boogie::BoogieProgramParser::new().parse("var $abort_code: int;");
    println!("{:?}", prog);
}