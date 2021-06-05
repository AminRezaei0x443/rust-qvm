use quick_js::{Context, JsValue};
use tokio;
use std::error::Error;
use std::{convert::TryFrom, error, fmt};
use std::collections::HashMap;
use reqwest;

#[derive(PartialEq, Debug)]
pub enum ExecError {
    Internal(String)
}
impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ExecError::*;
        match self {
            Internal(e) => write!(f, "Internal error: {}", e),
        }
    }
}
impl Error for ExecError {}

fn get(url: JsValue) -> Result<JsValue, Box<dyn std::error::Error + Send + Sync>>{
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let u = url.into_string().unwrap();
        let resp = reqwest::get(u)
        .await?
            .text().await?;

        Ok::<String, Box<dyn std::error::Error>>(resp)
    }).unwrap();

    Ok(JsValue::String(result))
}

pub fn addHttpModule(context: &Context) -> Result<(), Box<dyn Error>>{
    let o = context.new_object()?;
    context.add_function(&o, "get", get).unwrap();
    context.addGlobalObject("http", o);

    Ok(())
}