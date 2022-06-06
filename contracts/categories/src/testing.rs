use super::*;

use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId, VMContext};

fn get_test_categories_empty_case() -> Categories {
  let lm = LookupMap::<AccountId, Vector<Category>>::new(b"l");
  Categories { values: lm }
}

fn get_context(account_id: &str) -> VMContext {
  let test_user = String::from(account_id);
  let account = AccountId::try_from(test_user).unwrap();
  let context = VMContextBuilder::new().signer_account_id(account).build();
  return context;
}

/// formats Vec<Category\> to readable format
///
/// ## Example
///
/// ### Base usage
/// #### Input
/// ```
/// println!( "get user categories: {:?}",user_categories)
/// ```
/// #### Output
/// ```
/// get user categories: Vector { len: 1, prefix: [99] }
/// ```
/// ### Usage with categories_format_output
/// #### Input
/// ```
/// println!(
///   "get user categories: {}",
///   categories_format_output(user_categories)
/// );
/// ```
///
/// #### Output
///
/// ```
/// get user categories:
/// [{
///   id: 0;
///   title: default;
///   color: orange;
///  }]
/// ```
fn categories_format_output(categories: &Vec<Category>) -> String {
  let mut output = String::from("\n[");
  for category in categories.iter() {
    output = format!("{output}{{\n", output = output);
    output = format!(
      "\t{output} {field_name}: {field_value};\n",
      output = output,
      field_name = "id",
      field_value = category.id
    );
    output = format!(
      "\t{output} {field_name}: {field_value};\n",
      output = output,
      field_name = "title",
      field_value = category.title
    );
    output = format!(
      "\t{output} {field_name}: {field_value};\n",
      output = output,
      field_name = "color",
      field_value = category.color
    );
    output = format!("{output}}}", output = output)
  }
  output = format!("\n{output}]\n", output = output);
  return output;
}

fn get_testing_category() -> Category {
  Category {
    id: 0,
    title: String::from("testing_title"),
    color: String::from("testing_color"),
  }
}

#[test]
fn add_category() {
  let test_user = String::from("unicorn.testnet");
  let context = get_context(&test_user);
  testing_env!(context.clone());
  let mut contract = get_test_categories_empty_case();

  let categories_before = contract.get_categories();
  println!(
    "categories before: {}",
    categories_format_output(&categories_before)
  );

  let test_category = get_testing_category();
  println!("adding category: {:?}", test_category);
  contract.add_category(test_category.title, test_category.color);
  let categories_after = contract.get_categories();

  println!(
    "categories after: {}",
    categories_format_output(&categories_after)
  );
  //
  assert_eq!(categories_before.len() < categories_after.len(), true);
}

#[test]
fn get_categories() {
  let test_user = String::from("unicorn.testnet");
  let context = get_context(&test_user);
  testing_env!(context.clone());
  let contract = get_test_categories_empty_case();

  let got_categories = contract.get_categories();
  println!(
    "got categories: {}",
    categories_format_output(&got_categories)
  );
  //
  assert_eq!(got_categories.len() == 0, true);
}
