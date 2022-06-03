use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};

#[cfg(test)]
#[path = "./testing.rs"]
mod categories;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Category {
  id: String,
  title: String,
  color: String,
}

// struct Categories stores individual list of categories for every user
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Categories {
  values: LookupMap<String, Vector<Category>>,
}

impl Default for Categories {
  fn default() -> Self {
    env::panic_str("The contract should be initialized before usage")
  }
}

#[near_bindgen]
impl Categories {
  /// # Description
  /// Creates the contract and inits empty categories LookupMap
  #[init]
  pub fn new() -> Self {
    assert!(!env::state_exists(), "Already initialized");

    let map = LookupMap::<String, Vector<Category>>::new(b"l");

    Self { values: map }
  }

  pub fn get_categories(&self, user_id: String) -> Vec<Category> {
    // let mut user_categories = Vec::new();
    let user_categories = self.values.get(&user_id);
    match user_categories {
      Some(v) => return v.to_vec(),
      None => {
        // Return empty array if there is no categories, related to this user
        return Vec::new();
      }
    }
  }

  fn create_empty_tasks_vector(&mut self, user_id: String) {
    let new_category_vec = Vector::<Category>::new(b"t");
    self.values.insert(&user_id, &new_category_vec);
  }

  pub fn add_category(&mut self, user_id: String, category: Category) {
    // get existing user categories
    let user_categories = self.values.get(&user_id);

    match user_categories {
      Some(mut v) => {
        // push new category to old vector, then replace old value
        v.push(&category);
        self.values.insert(&user_id, &v);
      }
      None => {
        // if user does not have any category (key is missing), create empty vector & try to add new category again
        self.create_empty_tasks_vector(user_id.clone());
        // ...and add some recursiveness ✨✨✨
        self.add_category(user_id.clone(), category)
      }
    }
  }
}
