#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_node {
    use js_sys::JSON;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn it_transpiles() {
        let collection: &'static str = include_str!("./fixtures/wasm/collection.json");
        let openapi: &'static str = include_str!("./fixtures/wasm/openapi.json");

        match postman2openapi::transpile(JSON::parse(collection).unwrap()) {
            Ok(oas) => assert_eq!(
                JSON::stringify(&JSON::parse(openapi).unwrap()).unwrap(),
                JSON::stringify(&oas).unwrap()
            ),
            Err(err) => panic!("Couldn't convert collection to OpenAPI: {:?}", err),
        };
    }
}
