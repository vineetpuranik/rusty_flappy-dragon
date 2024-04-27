use bracket_lib::prelude::*;

//available game modes
enum GameMode {
    Menu,
    Playing, 
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

//struct to store the Game state
struct State {
    player: Player,
    frame_time: f32, //tracks time accumulate between frames to control the game's speed
    mode: GameMode,
}

impl State {
    // constructor for our struct
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    // restart the game
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    // play function
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        //ctx.frame_time_ms provides the time elapsed since the last tick function was called.
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);

        ctx.print(0,0,"Prace SPACE to flap");

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
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

//struct for dragon attributes

struct Player {
    x: i32,
    y: i32, 
    velocity: f32,
}


impl Player {

    // constructor new player
    fn new(x: i32, y: i32) -> Self {
        Player {
            x, 
            y, 
            velocity: 0.0,
        }
    }

    // render player on the screen
    fn render(&mut self, ctx:  &mut BTerm) {
        ctx.set(
            0, 
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }

    // gravity and move
    // velocity represents vertical momemntum
    // for each turn add a value representing gravity to the velocity
    // modify players y position by that amount
    fn gravity_and_move(&mut self) {

        // only apply gravity if downward momentum is less than 2     
        if self.velocity < 2.0 {
            self.velocity += 0.2
        }

        self.y += self.velocity as i32;
        self.x += 1;
        
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
        // negative number so flapping will move the dragon upwards
        // 0 is top of the screen
    }


}


fn main() -> BError {
    let context= BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State::new())
}
