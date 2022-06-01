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

impl Category {
  fn get_default() -> Category {
    Category {
      id: String::from("0"),
      title: String::from("default"),
      color: String::from("orange"),
    }
  }
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
  pub fn get_categories(&mut self, user_id: String) -> Vec<Category> {
    // let mut user_categories = Vec::new();
    let user_categories = self.values.get(&user_id);
    match user_categories {
      Some(v) => return v.to_vec(),
      None => {
        // If there were no categories, it means, it's user's first login.
        // Thus, insert default category for this user and return it

        let new_category_vec = self.insert_default_value_to_look_map(user_id);
        //
        return new_category_vec;
      }
    }
  }

  fn insert_default_value_to_look_map(&mut self, user_id: String) -> Vec<Category> {
    let mut new_category_vec = Vector::<Category>::new(b"c");
    new_category_vec.push(&Category::get_default());
    self.values.insert(&user_id, &new_category_vec);
    //
    return new_category_vec.to_vec();
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
        // if user does not have any category, create default one and push new category to the new vector
        // basically it is "never" case because on frontend, user always should get_categories first,
        // which would generate default category for this user.
        // This None handling is made for testing
        self.insert_default_value_to_look_map(user_id.clone());
        // ...and add some recursiveness ✨✨✨
        self.add_category(user_id.clone(), category)
      }
    }
  }
}
