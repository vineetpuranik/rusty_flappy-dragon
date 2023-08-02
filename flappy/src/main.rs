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
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
}

//implement the GameState trait
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Bracket Terminal");
    }
}

fn main() -> BError {
    let context= BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State::new())
}
