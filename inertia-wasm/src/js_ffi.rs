use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub(crate) fn log(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub(crate) fn debug(s: &str);
}
