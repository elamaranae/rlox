use rlox::vm::VM;

#[test]
fn expr() {
    let mut vm: VM = Default::default();
    let mut out = Vec::new();

    let expr = String::from("(-1 + 2) * 3 - -4");

    vm.interpret(expr, &mut out);

    let output = String::from_utf8(out).unwrap();
    
    assert_eq!(output, "7.0");
}
