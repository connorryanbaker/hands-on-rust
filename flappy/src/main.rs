use bracket_lib::prelude::*;

enum GameMode {
  Menu,
  Playing,
  End,
}

struct State{
  mode: GameMode,
}

impl State {
  fn new() -> Self {
    State {
      mode: GameMode::Menu,
    }
  }

  fn play(&mut self, ctx: &mut BTerm) {
    // TODO: unstub

    self.mode = GameMode::End;
  }

  fn restart(&mut self) {
    self.mode = GameMode::Playing;
  }

  fn main_menu(&mut self, ctx: &mut BTerm) {
    // display menu, respond to user input
    // change mode to playing, reset all game state
    ctx.cls();
    ctx.print_centered(5, "Welcome to Flappy Dragon");
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");
    // ctx BTerm object has Key variable to hold keyboard input state represented as Option type
    // Rust provides shorthand version of matching single case w/ `if let`
    // Option can contain None or Some(data)

    if let Some(key) = ctx.key { // key pressed will be extracted into `key` variable
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn dead(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "You are dead!");
    ctx.print_centered(8, "(P) Play again");
    ctx.print_centered(9, "(Q) Quit game");

    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    match self.mode {
      GameMode::Menu => self.main_menu(ctx),
      GameMode::End => self.dead(ctx),
      GameMode::Playing => self.play(ctx),
    }
  }
}

fn main() -> BError {
  let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?; // ? "unwraps valid values or returns erroneous values"
  
  main_loop(context, State::new())
}
