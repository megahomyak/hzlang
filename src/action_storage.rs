use std::{collections::HashMap, sync::Arc};

use anyhow::Context;

pub struct ActionStorage<O> {
    handlers: HashMap<hzlang_parser::Name, Arc<dyn Handler<Obj = O>>>,
}

pub trait Handler {
    type Obj;

    fn call(&mut self, args: Vec<Self::Obj>) -> Self::Obj;
}

impl<O> ActionStorage<O> {
    pub fn register(&mut self, name: hzlang_parser::Name, handler: Arc<dyn Handler<Obj = O>>) {
        self.handlers.insert(name, handler);
    }

    pub fn call(&mut self, name: hzlang_parser::Name, args: Vec<O>) -> anyhow::Result<O> {
        Ok(self
            .handlers
            .get(&name)
            .context("handler not found")?
            .call(args))
    }
}
