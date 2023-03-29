use bracket_lib::prelude::*;

const FLOOR_HEIGHT : i32 = 30;
const SCREEN_HEIGHT : i32 = 50;
const SCREEN_WIDTH : i32 = 80;
const FRAME_DURATION : f32 = 75.0;

enum GameMode {
  Menu,
  Playing,
  End,
}

struct Player {
  x: i32,
  y: i32,
  velocity: f32,
}

impl Player {
  fn new(x: i32, y: i32) -> Self {
    Player {
      x,
      y,
      velocity: 0.0,
    }
  }

  fn render(&mut self, ctx: &mut BTerm) {
    ctx.set(
      5,
      self.y,
      YELLOW,
      BLACK,
      to_cp437('A'),
    );
  }

  fn gravity_and_move(&mut self) {
    if self.velocity < 2.0 {
      self.velocity += 0.2;
    }
    self.y += self.velocity as i32; // todo: look into y as f32 - will bterm support fractional y values
    self.x += 1;
    if self.y > FLOOR_HEIGHT {
      self.y = FLOOR_HEIGHT;
    }
  }

  fn jump(&mut self) {
    if self.y == FLOOR_HEIGHT {
      self.velocity = -2.0;
    }
  }
}

struct State {
  mode: GameMode,
  frame_time: f32,
  player: Player,
}

impl State {
  fn new() -> Self {
    State {
      mode: GameMode::Menu,
      player: Player::new(5, FLOOR_HEIGHT),
      frame_time: 0.0,
    }
  }
  
  fn dead(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "Game Over!");
    ctx.print_centered(8, "(P) Play Again");
    ctx.print_centered(9, "(Q) Quit");

    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn main_menu(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "Welcome to Log Jump!");
    ctx.print_centered(8, "(P) Play");
    ctx.print_centered(9, "(Q) Quit");

    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn play(&mut self, ctx: &mut BTerm) {
    ctx.cls_bg(NAVY);
    self.frame_time += ctx.frame_time_ms;
    if self.frame_time > FRAME_DURATION {
      self.frame_time = 0.0;
      self.player.gravity_and_move();
    }
    if let Some(VirtualKeyCode::Space) = ctx.key {
      self.player.jump();
    }
    self.player.render(ctx);
    ctx.print(0, 0, "Press SPACE to jump!");
  }

  fn restart(&mut self) {
    self.player = Player::new(5, FLOOR_HEIGHT);
    self.frame_time = 0.0;
    self.mode = GameMode::Playing;
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
    .with_title("Log Jump")
    .build()?;

  main_loop(context, State::new())
}
