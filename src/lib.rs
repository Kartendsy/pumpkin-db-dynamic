use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::fs;
use serde::{Serialize, de::DeserializeOwned};


pub struct PumpkinDB<T> {
  data: Arc<Mutex<T>>,
  file_path: String,
}

impl <T> PumpkinDB<T> where T: Serialize + DeserializeOwned + Default + Clone + Send + 'static
{
  pub fn open(path:&str) -> Self {
    let _ = fs::create_dir_all("Database");

    let initial_data = if let Ok(content) = fs::read_to_string(path) {
      serde_json::from_str(&content).unwrap_or_else(|_| T::default())
    } else {
      T::default()
    };
    Self {
      data: Arc::new(Mutex::new(initial_data)),
      file_path: path.to_string(),
    }
  }

  pub fn get_data(&self) -> T {
    let guard = self.data.lock().unwrap();
    guard.clone()
  }

  pub fn get_entry<K,V>(&self, key: &K) -> Option<V>
  where
    T: AsRef<HashMap<K, V>>,
    K: std::hash::Hash + Eq,
    V: Clone,
  {
    let guard = self.data.lock().unwrap();
    guard.as_ref().get(key).cloned()
  }

  pub fn update_entry<K,V,F>(&self, key:K, update_fn:F)
  where
    T: AsMut<HashMap<K,V>>,
    K: std::hash::Hash + Eq + Clone + Send + 'static,
    V: Default + Clone + Send + 'static,
    F: FnOnce(&mut V),
  {
    let mut guard = self.data.lock().unwrap();

    let entry = guard.as_mut().entry(key).or_insert_with(V::default);
    update_fn(entry);

    let data_clone = guard.clone();
    let path_clone = self.file_path.clone();

    std::thread::spawn(move || {
      if let Ok(json_str) = serde_json::to_string_pretty(&data_clone) {
        if let Err(e) = fs::write(path_clone, json_str) {
          let _ = ::std::io::stderr();
          eprintln!("[PumpkinDb Error] Failed to write data to disk: {:?}", e);

        }
      }
    });
  }

  pub fn force_save(&self) {
    let guard = self.data.lock().unwrap();
    if let Ok(json_str) = serde_json::to_string_pretty(&*guard) {
      let _ = fs::write(&self.file_path, json_str);
    }
  }
}
