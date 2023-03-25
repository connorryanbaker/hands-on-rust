use std::io::stdin;

#[derive(Debug)] // deriving requires every member field support feature being derived
struct Visitor {
  name: String,
  action: VisitorAction,
  age: i8
}

impl Visitor {
  fn new(name: &str, action: VisitorAction, age: i8) -> Self {
    Self {
      name: name.to_lowercase(),
      action,
      age
    }
  }

  fn greet_visitor(&self) {
    match &self.action {
      VisitorAction::Accept => println!("Welcome to the treehouse, {}!", self.name),
      VisitorAction::AcceptWithNote { note } => {
        println!("Welcome to the treehouse, {}!", self.name);
        println!("{}", note);
        if self.age < 21 {
          println!("No booze for u!");
        }
      },
      VisitorAction::Probation => println!("{} is now a probationary member.", self.name),
      VisitorAction::Refuse => println!("{} is not allowed. Scram!", self.name),
    }
  }
}

#[derive(Debug)] // deriving debug allows rust formatters to print enum value by name
enum VisitorAction {
  Accept,
  AcceptWithNote { note: String },
  Refuse,
  Probation,
}

fn main() {
  let mut visitor_list = vec![
    Visitor::new("bert", VisitorAction::Accept, 45),
    Visitor::new("steve", VisitorAction::AcceptWithNote{note: String::from("Juice in the fridge!")}, 11),
    Visitor::new("fred", VisitorAction::Refuse, 30),
  ];
  loop {
    println!("Hello, what is your name?");
    let name = read_name();
    let known_visitor = visitor_list
      .iter()
      .find(|visitor| visitor.name == name);
    // find returns an Option type
    // Options have 2 possible values: Some(x) or None
    // There are many ways to interact w/ options, we'll use match to start

    match known_visitor {
      Some(visitor) => visitor.greet_visitor(),
      None => {
        if name.is_empty() {
          break;
        } else {
          println!("{} is not on the visitor list.", name);
          visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
      }
    }
  }

  println!("Visitor list:");
  println!("{:#?}", visitor_list);
}

fn read_name() -> std::string::String {
  let mut name = String::new();
  stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
  name
    .trim()
    .to_lowercase()
}
