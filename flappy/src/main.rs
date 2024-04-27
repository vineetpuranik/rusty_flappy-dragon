use bracket_lib::prelude::*;

//available game modes
enum GameMode {
    Menu,
    Playing, 
    End,
}

//struct to store the Game state
struct State {
    mode: GameMode,
}

impl State {
    // constructor for our struct
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    // restart the game
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    // play function
    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stub later
        self.mode = GameMode::End;
    }

    // main_menu function
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to flappy dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = GameMode::Menu,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }

    }


}

//implement the GameState trait
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // match the current mode and call appropriate functions.
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context= BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State::new())
}
