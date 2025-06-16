#[macro_export]
macro_rules! export_wai {
    () => {
        wai_bindgen_rust::export!("./target/plugin.wai");
    };
}

#[macro_export]
macro_rules! import_wai {
    () => {
        wai_bindgen_wasmer::import!("./target/plugin.wai");
    };
}

mod elements;
