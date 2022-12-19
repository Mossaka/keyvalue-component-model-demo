wasmtime::component::bindgen!({
    path: "./keyvalue.wit",
    async: true,
});
use std::collections::HashMap;

use keyvalue::{Keyvalue, KeyvalueError};

use host::{add_to_linker, Wasi, WasiCtx};
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

struct Memory {
    inner: HashMap<String, Vec<u8>>,
}

#[async_trait::async_trait]
impl Keyvalue for Memory {
    async fn get(
        &mut self,
        key: String,
    ) -> std::result::Result<Vec<u8>, wasmtime::component::Error<KeyvalueError>> {
        let val = self.inner.get(&key).map(|v| v.to_vec());
        if let Some(val) = val {
            Ok(val)
        } else {
            Err(wasmtime::component::Error::new(KeyvalueError::KeyNotFound(
                String::from("key not found"),
            )))
        }
    }

    async fn set(
        &mut self,
        key: String,
        value: Vec<u8>,
    ) -> std::result::Result<(), wasmtime::component::Error<KeyvalueError>> {
        self.inner.insert(key, value);
        Ok(())
    }

    async fn delete(
        &mut self,
        key: String,
    ) -> std::result::Result<(), wasmtime::component::Error<KeyvalueError>> {
        self.inner.remove(&key);
        Ok(())
    }
}

pub struct Ctx {
    keyvalue: Memory,
    wasi: WasiCtx,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let keyvalue = Memory {
        inner: HashMap::new(),
    };
    let wasi = WasiCtxBuilder::new().build();
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, Ctx { keyvalue, wasi });
    let mut linker = Linker::new(&engine);
    keyvalue::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.keyvalue)?;
    add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi)?;

    let component = Component::from_file(&engine, "./target/guest.component.wasm")?;
    let (results, _) = Results::instantiate_async(&mut store, &component, &linker).await?;
    let res = results.handler(&mut store).await?;
    println!("res from host: {res:?}");

    Ok(())
}
