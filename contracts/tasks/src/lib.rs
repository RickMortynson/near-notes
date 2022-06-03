use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[cfg(test)]
#[path = "./testing.rs"]
mod tasks;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
  id: String,
  text: String,
  category_id: String,
  timestamp: u64,
}

// struct Categories stores individual list of categories for every user
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tasks {
  values: LookupMap<String, Vector<Task>>,
}

impl Default for Tasks {
  fn default() -> Self {
    env::panic_str("The contract should be initialized before usage")
  }
}

#[near_bindgen]
impl Tasks {
  /// # Description
  /// Creates the contract and inits empty Tasks LookupMap
  #[init]
  pub fn new() -> Self {
    assert!(!env::state_exists(), "Already initialized");

    let map = LookupMap::<String, Vector<Task>>::new(b"t");

    Self { values: map }
  }

  pub fn get_tasks(&self, user_id: String) -> Vec<Task> {
    let user_tasks = self.values.get(&user_id);
    match user_tasks {
      Some(v) => return v.to_vec(),
      None => {
        // Return empty array if there is no notes, related to this user

        return Vec::new();
      }
    }
  }

  fn generate_task_fields(text: String, category_id: String) -> Task {
    let since_the_epoch = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards");

    Task {
      id: Uuid::new_v4().to_string(),
      text,
      category_id,
      timestamp: since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000,
    }
  }

  pub fn add_task(&mut self, user_id: String, text: String, category_id: String) {
    // get existing user categories
    let user_categories = self.values.get(&user_id);

    match user_categories {
      Some(mut v) => {
        // push new category to old vector, then replace old value
        v.push(&Tasks::generate_task_fields(text, category_id));
        self.values.insert(&user_id, &v);
      }
      None => {
        let base_vector = Vector::<Task>::new(b"t");
        self.values.insert(&user_id, &base_vector);
        // add some recursiveness ✨✨✨
        self.add_task(user_id.clone(), text, category_id)
      }
    }
  }
}
