use learn_v8::JsRuntime;

fn main() {
    JsRuntime::init();

    let mut runtime = JsRuntime::new(None);
    let code = r#"
        function hello() {
            let result = print({a:1,b:2});
            print(result);
            return fetch("https://www.rust-lang.org/");
        }
        hello();
     "#;
    let result = runtime.execute_script(code);
    println!("Result is: {:#?}", result)
}
