
use tumblr_api_derive::Builder;

#[test]
fn foo() {
    #[derive(Builder)]
    #[builder()]
    #[allow(unused)]
    struct Foo {
        #[builder(set(ctor(into)))]
        bar: String,
        #[builder(set(ctor()))]
        qux: u32,
        #[builder(set(setter(into, wrap_with = Option::Some, arg_type = "String")))]
        abcd: Option<String>,
    }
}
