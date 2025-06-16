#[macro_export]
macro_rules! include_wai {
    () => {
        wai_bindgen_rust::export!("./target/plugin.wai");
    };
}

mod elements;
