use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

#[cfg(test)]
#[path = "./testing.rs"]
mod tasks;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
  id: u64,
  text: String,
  category_id: u64,
  timestamp: u64,
}

// struct Categories stores individual list of categories for every user
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tasks {
  values: LookupMap<AccountId, Vector<Task>>,
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

    let map = LookupMap::<AccountId, Vector<Task>>::new(b"t");

    Self { values: map }
  }

  /// * resets current user' tasks LookupMap
  pub fn reset(&mut self) {
    self.values.remove(&env::signer_account_id());
  }

  pub fn get_tasks(&self, account_id: AccountId) -> Vec<Task> {
    let user_tasks = self.values.get(&account_id);

    match user_tasks {
      Some(v) => return v.to_vec(),
      None => {
        // Return empty array if there is no notes, related to this user

        return Vec::new();
      }
    }
  }

  fn generate_task_fields(id: u64, text: String, category_id: u64) -> Task {
    Task {
      id,
      text,
      category_id,
      timestamp: env::block_timestamp(), // timestamp: since_the_epoch
    }
  }

  pub fn add_task(&mut self, text: String, category_id: u64) {
    // get existing user categories
    let user_id = env::signer_account_id();
    let user_categories = self.values.get(&user_id);

    match user_categories {
      Some(mut v) => {
        // push new category to old vector, then replace old value
        let vector_length = v.len();
        let new_task = Tasks::generate_task_fields(vector_length, text, category_id);
        env::log_str(format!("generated task with id = {}", new_task.id.to_string()).as_str());
        v.push(&new_task);

        self.values.insert(&user_id, &v);
      }
      None => {
        // if user had no category with this name before
        let base_vector = Vector::<Task>::new(b"t");
        self.values.insert(&user_id, &base_vector);
        // add some recursiveness ✨✨✨
        self.add_task(text, category_id)
      }
    }
  }
}
