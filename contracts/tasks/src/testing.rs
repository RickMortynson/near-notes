use super::*;

use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId, VMContext};

fn get_context(account_id: &str) -> VMContext {
  let test_user = String::from(account_id);
  let account = AccountId::try_from(test_user).unwrap();
  let context = VMContextBuilder::new().signer_account_id(account).build();
  return context;
}

fn get_test_tasks_empty_case() -> Tasks {
  let lm = LookupMap::<AccountId, Vector<Task>>::new(b"l");
  Tasks { values: lm }
}

fn tasks_format_output(tasks: &Vec<Task>) -> String {
  let mut output = String::from("\n[");
  for task in tasks.iter() {
    output = format!("{output}{{\n", output = output);
    output = format!(
      "\t{output} {field_name}: {field_value};\n",
      output = output,
      field_name = "id",
      field_value = task.id
    );
    output = format!(
      "\t{output} {field_name}: {field_value};\n",
      output = output,
      field_name = "text",
      field_value = task.text
    );
    output = format!(
      "\t{output} {field_name}: {field_value};\n",
      output = output,
      field_name = "category_id",
      field_value = task.category_id
    );
    output = format!(
      "\t{output} {field_name}: {field_value};\n",
      output = output,
      field_name = "timestamp",
      field_value = u64::from(task.timestamp)
    );
    output = format!("{output}}}", output = output)
  }
  output = format!("\n{output}]\n", output = output);
  return output;
}

#[test]
fn add_task() {
  let test_user = String::from("unicorn.testnet");
  let context = get_context(&test_user);
  let account_id = AccountId::try_from(test_user).unwrap();

  testing_env!(context.clone());
  let mut contract = get_test_tasks_empty_case();

  let tasks_before = contract.get_tasks(account_id.clone());

  println!("tasks before: {}", tasks_format_output(&tasks_before));

  contract.add_task(String::from("test task text"), 0);

  let tasks_after = contract.get_tasks(account_id.clone());

  println!("tasks after: {}", tasks_format_output(&tasks_after));
  //
  assert_eq!(tasks_before.len() < tasks_after.len(), true);
}

#[test]
fn get_tasks_empty_case() {
  let test_user = String::from("unicorn.testnet");
  let context = get_context(&test_user);
  let account_id = AccountId::try_from(test_user).unwrap();

  testing_env!(context.clone());
  let contract = get_test_tasks_empty_case();

  let got_tasks = contract.get_tasks(account_id);
  println!("got tasks: {}", tasks_format_output(&got_tasks));

  assert_eq!(got_tasks.len() == 0, true);
}

#[test]
fn get_tasks_non_empty_case() {
  let test_user = String::from("unicorn.testnet");
  let context = get_context(&test_user);
  let account_id = AccountId::try_from(test_user).unwrap();

  testing_env!(context.clone());
  let mut contract = get_test_tasks_empty_case();

  let mut tasks_vec = Vector::new(b"t");
  tasks_vec.push(&Task {
    id: 0,
    text: String::from("test-text"),
    category_id: 0,
    timestamp: 21,
  });

  contract
    .values
    .insert(&env::signer_account_id(), &tasks_vec);

  let got_tasks = contract.get_tasks(account_id);
  println!("got tasks: {}", tasks_format_output(&got_tasks));

  assert_eq!(got_tasks.len() == 1, true);
}
