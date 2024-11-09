use std::collections::HashMap;
use std::sync::Mutex;

pub struct State {
    data: Mutex<HashMap<String, String>>,
}

impl State {
    pub fn new() -> Self {
        State {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }
}

pub fn skylarkhello() -> String {
    "Hello from skylark!".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        let result = skylarkhello();
        assert_eq!(result, "Hello from skylark!");
    }

    #[test]
    fn set_and_get_key() {
        let state = State::new();
        state.set("key1".to_string(), "value1".to_string());
        assert_eq!(state.get("key1"), Some("value1".to_string()));
    }
}
