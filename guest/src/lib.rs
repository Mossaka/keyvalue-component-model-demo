

wit_bindgen_guest_rust::generate!("../keyvalue.wit");

struct Component {}

export_results!(Component);

impl results::Results for Component {
    fn handler() -> String {
        keyvalue::set("hello", "world".as_bytes()).unwrap();
        let res: String = String::from_utf8(keyvalue::get("hello").unwrap()).unwrap();
        println!("res from guest: {res}");
        res
    }
}
