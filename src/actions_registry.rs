use std::{collections::HashMap, sync::Arc};

struct Action<HzObject> {
    handler: Arc<dyn Fn(Vec<HzObject>) -> Option<HzObject>>,
}

struct Layer<Word, HzObject> {
    action: Action<HzObject>,
    with_gap: Option<Box<Layer<Word, HzObject>>>,
    with_word: HashMap<Word, Layer<Word, HzObject>>,
}

struct Root<Word, HzObject> {
    with_gap: Option<Box<Layer<Word, HzObject>>>,
    with_word: HashMap<Word, Layer<Word, HzObject>>,
}

pub struct ActionsRegistry<Word, HzObject> {
    root: Root<Word, HzObject>,
}

impl<Word, HzObject> ActionsRegistry<Word, HzObject> {
    pub fn new() -> Self {
        Self {
            root: Root {
                with_gap: None,
                with_word: HashMap::new(),
            },
        }
    }

    pub fn 
}
