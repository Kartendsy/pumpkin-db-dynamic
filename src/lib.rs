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
    let db_lock = self.data.lock().unwrap();
    db_lock.clone()
  }

  pub fn update<F>(&self, update_fn:F) where F:FnOnce(&mut T) {
    let mut db_lock = self.data.lock().unwrap();

    update_fn(&mut *db_lock);

    let data_clone = db_lock.clone();
    let path_clone = self.file_path.clone();

    std::thread::spawn(move || {
      if let Ok(json_str) = serde_json::to_string_pretty(&data_clone) {
        let _ = fs::write(path_clone, json_str);
      }
    });
  }

  pub fn force_save(&self) {
    let db_lock = self.data.lock().unwrap();
    if let Ok(json_str) = serde_json::to_string_pretty(&*db_lock) {
      let _ = fs::write(&self.file_path, json_str);
    }
  }
}
