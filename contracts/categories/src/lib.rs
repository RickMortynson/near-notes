use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

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
  values: LookupMap<AccountId, Vector<Category>>,
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

    let map = LookupMap::<AccountId, Vector<Category>>::new(b"l");

    Self { values: map }
  }

  pub fn get_categories(&self) -> Vec<Category> {
    // let mut user_categories = Vec::new();
    let user_id = env::signer_account_id();
    let user_categories = self.values.get(&user_id);
    match user_categories {
      Some(v) => return v.to_vec(),
      None => {
        // Return empty array if there is no categories, related to this user
        return Vec::new();
      }
    }
  }

  pub fn add_category(&mut self, category: Category) {
    // get existing user categories
    let user_id = env::signer_account_id();
    let user_categories = self.values.get(&user_id);

    match user_categories {
      Some(mut v) => {
        // push new category to old vector, then replace old value
        v.push(&category);
        self.values.insert(&user_id, &v);
      }
      None => {
        // if user does not have any category (key is missing), create empty vector & try to add new category again
        let base_vector = Vector::<Category>::new(b"t");
        self.values.insert(&user_id, &base_vector);
        // ...and add some recursiveness ✨✨✨
        self.add_category(category)
      }
    }
  }
}
