#![allow(dead_code)]

// 1.variable
fn define_variable() {
  println!("define variable");
  let str = "hello rust";
  println!("define a str variable : {} ",str);
  println!("====")
}

// 2. variability
fn define_variability(){
  println!("define variability");
  let mut str = "hello rust";
  println!("define variability: {}",str);
  str = "hello rust variability";
  println!("changed variable: {}",str);
  println!("====")
}

// 3. hiding mechanism
fn hiding_mechanism(){
  println!("variable hiding mechanism");
  let str = "hello hiding mechanism";
  println!("hiding mechanism first define: {}",str);
  let str = 10;
  println!("hiding mechanism changed: {}",str);
  println!("====")
}

// 4. define data
fn define_number_data(){
  println!("define basic data");
  let num:i8 = -128;
  println!("The data is a signed integer: {} ",num);
  let num:u8 = 127;
  println!("The data is a unsigned integer: {} ",num);
  let bool = false;
  println!("The data is a boolean type: {}",bool);
  let str = "string type";
  println!("The data is a string type: {}",str);
  let tuple = (1,3,"string");
  println!("The data is a tuple type: {:#?}",tuple);
  let arr = [1,2,3,4];
  println!("The data is a array type: {:#?}",arr);
  println!("====")
}

// 5. define a return value function
fn define_fn(){
  let double_val = return_new_val(10);
  println!("The val is function return: {}",double_val);
  let bad_double_val = return_new_val_bad(&double_val);
  println!("Bad example,The val is function return: {}",bad_double_val);

  println!("====")
}
// The function can operate x
fn return_new_val(x:u32) -> u32{
  x * x
}
// The function dot't can operate x
fn return_new_val_bad(x:&u32) -> u32{
  x * x
}

// 6. define control flow
fn define_control_flow(){
  define_if_expression();
  define_loop();
  define_while();
  define_for();
}

// Usage if statement，the conditional expression must be a Boolean
fn define_if_expression() {
  println!("if statement demo:");
  let bool = true;
  println!("define a boolean val: {}",bool);
  if bool {
    println!("bool is true print")
  }

  let bool = false;
  println!("change bool to false");

  if !bool {
     println!("bool is false print");
  }

  // Bad statement
  // let num = 3;
  // if num {
  //    println!("This a compile error code block")
  // }


  // Assign an if statement
  let sum = if bool {
     10
  } else {
    100
  };
  println!("Usage if statement,assign value to sum: {}",sum);

  println!("====")
}

fn define_loop() {
  println!("=== loop example === ");
  println!("If sum < 10 loop go on else stop loop");
  let mut sum = 1;
  loop {
     if sum > 10 {
        // If break is not used, the loop statement will continue to loop
        break;
     }
     println!("Looping value: {}",sum);
     sum = sum + 1;
  }
}

fn define_while(){
  println!("=== while example ===");
  let mut sum = 10;
  println!("If sum > 5 loop go on else stop loop");
  while sum > 5 {
    println!("While changed value: {}",sum);
    sum = sum - 1;
  }
}

fn define_for() {
  println!("=== for example === ");
  println!("Iteration arr");
  let arr = [1,2,3,5,6123,123];
  for val in arr.iter() {
     println!("The val is: {}",val);
  }


  println!("Normal loop");
  // range
  // 1..10 not 10 or 1.=10 have 10
  for val in 1..10 {
     println!("normal loop val: {}",val);
  }
}


fn main() {
  // define_variable();
  // define_variability();
  // hiding_mechanism();
  // define_number_data();
  // define_fn();
  define_control_flow();
}
