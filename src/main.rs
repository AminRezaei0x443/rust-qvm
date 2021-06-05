use quick_js::{Context, JsValue, ExecutionError};
mod http;

use http::*;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let context = Context::new().unwrap();
    addHttpModule(&context);

    let res = context.eval_as::<String>(r#"
        var x = http.get("https://google.com");
        x.toString()
    "#).unwrap();
        println!("{}", res);
    Ok(())
}
