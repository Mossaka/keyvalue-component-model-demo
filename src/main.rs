wasmtime::component::bindgen!("./keyvalue.wit");
wasmtime::component::bindgen!("./wasi_snapshot_preview1/testwasi.wit");

use std::{collections::HashMap};

use keyvalue::{Keyvalue, KeyvalueError};

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};


struct Redis {
    inner: HashMap<String, Vec<u8>>,
}

impl Keyvalue for Redis {
    fn get(&mut self, key: String) -> anyhow::Result<Result<Vec<u8>, KeyvalueError>> {
        let val = self.inner.get(&key).map(|v| v.to_vec());
        Ok(val.ok_or(KeyvalueError::KeyNotFound(String::from("key not found"))))
    }

    fn set(&mut self, key: String, value: Vec<u8>) -> anyhow::Result<Result<(), KeyvalueError>> {
        self.inner.insert(key, value);
        Ok(Ok(()))
    }

    fn delete(&mut self, key: String) -> anyhow::Result<Result<(), KeyvalueError>> {
        self.inner.remove(&key);
        Ok(Ok(()))
    }
}

struct HostTestwasi {}

impl testwasi::Testwasi for HostTestwasi {
    fn log(&mut self, bytes: Vec<u8>) -> anyhow::Result<()> {
        println!("{}", String::from_utf8(bytes)?);
        Ok(())
    }

    fn log_err(&mut self, bytes: Vec<u8>) -> anyhow::Result<()> {
        eprintln!("{}", String::from_utf8(bytes)?);
        Ok(())
    }
}

pub struct Ctx {
    keyvalue: Redis,
    testwasi: HostTestwasi,
}

fn main() -> anyhow::Result<()> {
    let keyvalue = Redis {
        inner: HashMap::new(),
    };
    let testwasi = HostTestwasi {};
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, Ctx { keyvalue, testwasi });
    let mut linker = Linker::new(&engine);
    keyvalue::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.keyvalue)?;
    testwasi::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.testwasi)?;
    let component = Component::from_file(&engine, "./target/guest.component.wasm")?;

    let (results, _) = Results::instantiate(&mut store, &component, &linker)?;
    let res = results.handler(&mut store)?;
    println!("res from host: {res:?}");

    Ok(())
}
