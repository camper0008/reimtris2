// Window size in pixels
const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 460.0;

// Offset of the game board in pixels
const GAME_OFFSET_X: f32 = 140.0;
const GAME_OFFSET_Y: f32 = 30.0;

// Game size in blocks
const GAME_WIDTH: i32 = 10;
const GAME_HEIGHT: i32 = 20;

// Block size in pixels
const SIZE: f32 = 20.0;

const FPS: u32 = 60;

// Resources
const BACKGROUND_MUSIC: &'static [u8] = include_bytes!("res/music.ogg");
const HARD_DROP_SOUND: &'static [u8] = include_bytes!("res/hard_drop.ogg");
const LINE_CLEAR_SOUND: &'static [u8] = include_bytes!("res/line_clear.ogg");
const MOVE_SOUND: &'static [u8] = include_bytes!("res/move.ogg");
const ROTATION_SOUND: &'static [u8] = include_bytes!("res/rotation.ogg");
const DEFAULT_CONFIG: &'static [u8] = include_bytes!("res/default_config.toml");
const FONT: &'static [u8] = include_bytes!("res/josenfin_sans_regular.ttf");

#[derive(Clone)]
struct Tetromino {
    name: char,
    color: Color,
    rotations: [[[i8; 4]; 4]; 4],
}

impl Debug for Tetromino {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

lazy_static! {
    static ref TETROMINOS: [Tetromino; 7] = [
        Tetromino {
            name: 'I',
            color: Color::from_rgb(0, 255, 255),
            rotations: [
                [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
                [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]]
            ]
        },
        Tetromino {
            name: 'J',
            color: Color::from_rgb(0, 0, 255),
            rotations: [
                [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0]]
            ]
        },
        Tetromino {
            name: 'L',
            color: Color::from_rgb(255, 128, 0),
            rotations: [
                [[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0]],
                [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]]
            ]
        },
        Tetromino {
            name: 'O',
            color: Color::from_rgb(255, 255, 0),
            rotations: [
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
            ]
        },
        Tetromino {
            name: 'S',
            color: Color::from_rgb(0, 255, 0),
            rotations: [
                [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0],],
                [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]]
            ]
        },
        Tetromino {
            name: 'T',
            color: Color::from_rgb(255, 0, 255),
            rotations: [
                [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]]
            ]
        },
        Tetromino {
            name: 'Z',
            color: Color::from_rgb(255, 0, 0),
            rotations: [
                [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0]]
            ]
        }
    ];
}

struct Coords {
    x: i8,
    y: i8,
}

impl Coords {
    pub fn new(x: i8, y: i8) -> Coords {
        Coords { x, y }
    }
}

// Get wall kick data for this rotation
fn wall_kicks(tetromino: &Tetromino, old_rotation: i8, new_rotation: i8) -> [Coords; 5] {
    const DEFAULT: i8 = 0;
    const RIGHT: i8 = 1;
    const UPSIDE_DOWN: i8 = 2;
    const LEFT: i8 = 3;

    match tetromino.name {
        'J' | 'L' | 'S' | 'T' | 'Z' => match old_rotation {
            DEFAULT => match new_rotation {
                RIGHT => [
                    Coords::new(0, 0),
                    Coords::new(-1, 0),
                    Coords::new(-1, 1),
                    Coords::new(0, -2),
                    Coords::new(-1, -2),
                ],
                LEFT => [
                    Coords::new(0, 0),
                    Coords::new(1, 0),
                    Coords::new(1, 1),
                    Coords::new(0, -2),
                    Coords::new(1, -2),
                ],
                _ => panic!("Invalid rotation"),
            },
            RIGHT => match new_rotation {
                DEFAULT => [
                    Coords::new(0, 0),
                    Coords::new(1, 0),
                    Coords::new(1, -1),
                    Coords::new(0, 2),
                    Coords::new(1, 2),
                ],
                UPSIDE_DOWN => [
                    Coords::new(0, 0),
                    Coords::new(1, 0),
                    Coords::new(1, -1),
                    Coords::new(0, 2),
                    Coords::new(1, 2),
                ],
                _ => panic!("Invalid rotation"),
            },
            UPSIDE_DOWN => match new_rotation {
                RIGHT => [
                    Coords::new(0, 0),
                    Coords::new(-1, 0),
                    Coords::new(-1, 1),
                    Coords::new(0, -2),
                    Coords::new(-1, -2),
                ],
                LEFT => [
                    Coords::new(0, 0),
                    Coords::new(1, 0),
                    Coords::new(1, 1),
                    Coords::new(0, -2),
                    Coords::new(1, -2),
                ],
                _ => panic!("Invalid rotation"),
            },
            LEFT => match new_rotation {
                DEFAULT => [
                    Coords::new(0, 0),
                    Coords::new(-1, 0),
                    Coords::new(-1, -1),
                    Coords::new(0, 2),
                    Coords::new(-1, 2),
                ],
                UPSIDE_DOWN => [
                    Coords::new(0, 0),
                    Coords::new(1, 0),
                    Coords::new(1, 1),
                    Coords::new(0, -2),
                    Coords::new(1, -2),
                ],
                _ => panic!("Invalid rotation"),
            },
            _ => panic!("Invalid rotation state"),
        },
        'I' => match old_rotation {
            DEFAULT => match new_rotation {
                RIGHT => [
                    Coords::new(0, 0),
                    Coords::new(-2, 0),
                    Coords::new(1, 0),
                    Coords::new(-2, -1),
                    Coords::new(1, 2),
                ],
                LEFT => [
                    Coords::new(0, 0),
                    Coords::new(-1, 0),
                    Coords::new(2, 0),
                    Coords::new(-1, 2),
                    Coords::new(2, -1),
                ],
                _ => panic!("Invalid rotation"),
            },
            RIGHT => match new_rotation {
                DEFAULT => [
                    Coords::new(0, 0),
                    Coords::new(2, 0),
                    Coords::new(-1, 0),
                    Coords::new(2, 1),
                    Coords::new(-1, -2),
                ],
                UPSIDE_DOWN => [
                    Coords::new(0, 0),
                    Coords::new(-1, 0),
                    Coords::new(2, 0),
                    Coords::new(1, 2),
                    Coords::new(2, -1),
                ],
                _ => panic!("Invalid rotation"),
            },
            UPSIDE_DOWN => match new_rotation {
                RIGHT => [
                    Coords::new(0, 0),
                    Coords::new(1, 0),
                    Coords::new(-2, 0),
                    Coords::new(1, -2),
                    Coords::new(-2, 1),
                ],
                LEFT => [
                    Coords::new(0, 0),
                    Coords::new(2, 0),
                    Coords::new(-1, 0),
                    Coords::new(2, 1),
                    Coords::new(-1, -2),
                ],
                _ => panic!("Invalid rotation"),
            },
            LEFT => match new_rotation {
                DEFAULT => [
                    Coords::new(0, 0),
                    Coords::new(1, 0),
                    Coords::new(-2, 0),
                    Coords::new(1, -2),
                    Coords::new(-2, 1),
                ],
                UPSIDE_DOWN => [
                    Coords::new(0, 0),
                    Coords::new(-2, 0),
                    Coords::new(1, 0),
                    Coords::new(-2, -1),
                    Coords::new(1, 2),
                ],
                _ => panic!("Invalid rotation"),
            },
            _ => panic!("Invalid rotation state"),
        },
        'O' => [
            Coords::new(0, 0),
            Coords::new(0, 0),
            Coords::new(0, 0),
            Coords::new(0, 0),
            Coords::new(0, 0),
        ],
        _ => panic!("Invalid tetromino"),
    }
}

#[derive(Clone)]
struct CurrentTetromino {
    tetromino: Tetromino,
    rotation: i8,
    x: i8,
    y: i8,
}

impl CurrentTetromino {
    fn new(tetromino: Tetromino) -> CurrentTetromino {
        CurrentTetromino {
            tetromino,
            rotation: 0,
            x: (GAME_WIDTH / 2) as i8 - 2,
            y: -1,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    muted: bool,
    controls: Controls,
}

#[derive(Serialize, Deserialize)]
struct Controls {
    move_left: String,
    move_right: String,
    rotate_cw: String,
    rotate_ccw: String,
    soft_drop: String,
    hard_drop: String,
    hold: String,
    pause: String,
    mute: String,
    restart: String,
}

impl Config {
    fn update(&mut self, context: &mut Context) {
        let toml = toml::to_string_pretty(&self).unwrap();
        let mut config_path = user_config_dir(context).to_owned();
        config_path.push("config.toml");
        let mut config_file = File::create(config_path).unwrap();
        config_file.write_all(toml.as_bytes()).unwrap();
    }
}

struct GameState {
    board: [[Color; GAME_WIDTH as usize]; GAME_HEIGHT as usize], // The tetris play field with all the colors
    bag: Vec<Tetromino>, // The tetromino bag with up to 7 tetrominos
    current_tetromino: Option<CurrentTetromino>,
    next_tetromino: Option<Tetromino>,
    held_tetromino: Option<Tetromino>,
    has_swapped: bool, // Whether or not the player has swapped the held tetromino
    level: i32,
    score: i32,
    lines: i32, // Number of lines cleared since last levelup
    combo: i32,
    draw: bool,      // Whether or not to draw the next frame
    game_over: bool, // Whether or not the game has ended
    paused: bool,
    focused: bool,
    bgm: Source,
    back_to_back: bool,
    fps: u8, // The FPS value to draw
    ticks: usize,
    held_keys: HashMap<KeyCode, usize>, // Which keys are currently held down and at which tick the player started holding them down
    config: Config,
}

impl GameState {
    pub fn new(context: &mut Context) -> Self {
        // Get config file
        let mut config_str = String::new();
        let mut config_path = user_config_dir(context).to_owned();
        config_path.push("config.toml");

        // If the file exists, read it
        if config_path.exists() {
            let mut config_file =
                File::open(config_path.clone()).expect("Could not open config file");
            config_file
                .read_to_string(&mut config_str)
                .expect("Could not read config file");

        // If not, create it with default values
        } else {
            let mut config_file =
                File::create(config_path.clone()).expect("Could not create config file");
            config_file
                .write_all(DEFAULT_CONFIG)
                .expect("Could not write to config file");
            config_str = String::from_utf8(DEFAULT_CONFIG.to_vec()).unwrap();
        }

        let config: Config = match toml::from_str(&*config_str) {
            Ok(result) => result,
            Err(err) => {
                remove_file(config_path).expect(&*format!("Could not parse config file. {}", err));
                panic!("Could not parse config file. It has been reset now, try running the program again.");
            }
        };

        let mut result = GameState {
            board: [[BLACK; GAME_WIDTH as usize]; GAME_HEIGHT as usize],
            bag: Vec::with_capacity(7),
            current_tetromino: None,
            next_tetromino: None,
            held_tetromino: None,
            has_swapped: false,
            level: 1,
            score: 0,
            lines: 0,
            combo: 0,
            draw: true,
            game_over: false,
            paused: false,
            focused: true,
            bgm: play_sound(context, BACKGROUND_MUSIC, 1.0, true, false),
            back_to_back: false,
            fps: 0,
            ticks: 0,
            held_keys: HashMap::new(),
            config,
        };
        if result.config.muted {
            result.bgm.pause();
        }
        new_current_tetromino(&mut result);
        result
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if check_update_time(context, FPS) {
            if self.game_over || self.paused || !self.focused {
                return Ok(());
            }

            self.ticks += 1;

            // Falling delay, faster when soft drop key is held down
            let mut delay = 32 - self.level as usize * 2;
            if is_key_pressed(context, KeyCode::Down) {
                delay /= 10;
            }

            // Make tetrominos fall down
            if self.ticks % delay == 0 {
                let mut current_tetromino = self.current_tetromino.clone().unwrap();
                current_tetromino.y += 1;

                if collides(&current_tetromino, self.board) {
                    current_tetromino.y -= 1;
                    get_new_tetromino(self);
                    check_line_clears(self, context);
                } else {
                    self.current_tetromino = Some(current_tetromino);
                    if is_key_pressed(context, KeyCode::Down) {
                        self.score += 1;
                    }
                }

                self.draw = true;
            }

            // Holding down left/right after delay
            for key in [KeyCode::Left, KeyCode::Right].iter() {
                if self.held_keys.contains_key(key) && self.held_keys[key] < self.ticks - 15 {
                    let mut tetromino = self.current_tetromino.clone().unwrap();
                    tetromino.x += match key {
                        KeyCode::Left => -1,
                        KeyCode::Right => 1,
                        _ => 0,
                    };
                    if !collides(&tetromino, self.board) {
                        self.current_tetromino = Some(tetromino);
                        if !self.config.muted {
                            play_sound(context, MOVE_SOUND, 1.0, false, true);
                        }
                    }
                    self.draw = true;
                }
            }

            // Update FPS counter
            if self.ticks % 60 == 0 {
                self.fps = fps(context).floor() as u8;
                self.draw = true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        if !self.draw {
            return Ok(());
        }

        self.draw = false;

        clear(context, BLACK);

        let font = Font::new_glyph_font_bytes(context, FONT).unwrap_or(Font::default());

        let mut mesh = MeshBuilder::new();

        // Draw grid lines
        for i in 0..self.board.len() {
            mesh.line(
                &[
                    Point2::from([GAME_OFFSET_X, GAME_OFFSET_Y + i as f32 * SIZE]),
                    Point2::from([
                        GAME_OFFSET_X + GAME_WIDTH as f32 * SIZE,
                        GAME_OFFSET_Y + i as f32 * SIZE,
                    ]),
                ],
                1.0,
                Color::from_rgb(50, 50, 50),
            )
            .unwrap();
        }

        for i in 0..self.board[0].len() {
            mesh.line(
                &[
                    Point2::from([GAME_OFFSET_X + i as f32 * SIZE, GAME_OFFSET_Y]),
                    Point2::from([
                        GAME_OFFSET_X + i as f32 * SIZE,
                        GAME_OFFSET_Y + GAME_HEIGHT as f32 * SIZE,
                    ]),
                ],
                1.0,
                Color::from_rgb(50, 50, 50),
            )
            .unwrap();
        }

        // Draw board frame
        mesh.rectangle(
            DrawMode::Stroke(StrokeOptions::default()),
            Rect::new(
                GAME_OFFSET_X,
                GAME_OFFSET_Y,
                1.0 + SIZE * GAME_WIDTH as f32,
                1.0 + SIZE * GAME_HEIGHT as f32,
            ),
            WHITE,
        );

        // Draw board content
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                // Don't draw invisible blocks
                if i >= GAME_HEIGHT as usize || self.board[i][j] == BLACK {
                    continue;
                }

                let offset = if self.board[i][j] == BLACK { 1.0 } else { 0.0 };
                let color = if self.game_over {
                    Color::from_rgb(200, 200, 200)
                } else {
                    self.board[i][j]
                };
                mesh.rectangle(
                    DrawMode::fill(),
                    Rect::new(
                        GAME_OFFSET_X + j as f32 * SIZE + offset,
                        GAME_OFFSET_Y + i as f32 * SIZE + offset,
                        SIZE - offset,
                        SIZE - offset,
                    ),
                    color,
                );
            }
        }

        // Draw ghost tetromino
        let mut ghost_tetromino = self.current_tetromino.clone().unwrap();
        loop {
            ghost_tetromino.y += 1;
            if collides(&ghost_tetromino, self.board) {
                ghost_tetromino.y -= 1;
                draw_tetromino(&ghost_tetromino, Color::from_rgb(100, 100, 100), &mut mesh);
                break;
            }
        }

        // Draw current tetromino
        let current_tetromino = self.current_tetromino.clone().unwrap();
        let color = if self.game_over {
            Color::from_rgb(200, 200, 200)
        } else {
            current_tetromino.tetromino.color
        };
        draw_tetromino(&current_tetromino, color, &mut mesh);

        if self.paused || self.game_over {
            // Draw box behind pause/game over text
            let height = 60.0;
            mesh.rectangle(
                DrawMode::fill(),
                Rect::new(
                    GAME_OFFSET_X,
                    GAME_OFFSET_Y + (GAME_HEIGHT as f32 * SIZE / 2.0) - height / 2.0,
                    GAME_WIDTH as f32 * SIZE,
                    height,
                ),
                Color::from_rgba(0, 0, 0, 90),
            );

            // Draw text
            let mut text: Text;
            if self.paused {
                text = Text::new("PAUSED");
            } else {
                text = Text::new("GAME OVER");
            }
            text.set_font(font, Scale::uniform(28.0));
            let width = text.width(context);
            let height = text.height(context);
            queue_text(
                context,
                &text,
                Point2 {
                    x: WINDOW_WIDTH / 2.0 - width as f32 / 2.0,
                    y: WINDOW_HEIGHT / 2.0 - height as f32 / 2.0,
                },
                Some(WHITE),
            );
        }

        // Draw 'next' text
        let mut text = Text::new("NEXT");
        text.set_font(font, Scale::uniform(20.0));
        queue_text(
            context,
            &text,
            Point2 {
                x: GAME_OFFSET_X + (GAME_WIDTH as f32 * SIZE) + 20.0,
                y: GAME_OFFSET_Y,
            },
            Some(WHITE),
        );

        // Draw next piece
        let text_height = text.height(context);
        let next_tetromino = self.next_tetromino.as_ref().unwrap();
        let tetromino = next_tetromino.rotations[0];
        for i in 1..tetromino.len() {
            for j in 0..tetromino[i].len() {
                if tetromino[i][j] != 1 {
                    continue;
                }

                mesh.rectangle(
                    DrawMode::fill(),
                    Rect::new(
                        GAME_OFFSET_X + (GAME_WIDTH as f32 * SIZE) + 20.0 + SIZE * j as f32,
                        GAME_OFFSET_Y + text_height as f32 + 2.0 + SIZE * i as f32,
                        SIZE,
                        SIZE,
                    ),
                    next_tetromino.color,
                );
            }
        }

        // Draw 'hold' text
        let mut text = Text::new("HOLD");
        text.set_font(font, Scale::uniform(20.0));
        let text_width = text.width(context);
        let text_height = text.height(context);
        queue_text(
            context,
            &text,
            Point2 {
                x: GAME_OFFSET_X - 20.0 - text_width as f32,
                y: GAME_OFFSET_Y,
            },
            Some(WHITE),
        );

        // Draw held piece
        if self.held_tetromino.is_some() {
            let held_tetromino = self.held_tetromino.as_ref().unwrap();
            let tetromino = held_tetromino.rotations[0];
            for i in 1..tetromino.len() {
                for j in 0..tetromino[i].len() {
                    if tetromino[i][j] != 1 {
                        continue;
                    }

                    let mut offset = 3.0;
                    if held_tetromino.name == 'I' {
                        offset = 4.0;
                    }

                    mesh.rectangle(
                        DrawMode::fill(),
                        Rect::new(
                            GAME_OFFSET_X - 20.0 - SIZE * offset + SIZE * j as f32,
                            GAME_OFFSET_Y + text_height as f32 + 2.0 + SIZE * i as f32,
                            SIZE,
                            SIZE,
                        ),
                        held_tetromino.color,
                    );
                }
            }
        }

        let mesh = mesh.build(context).unwrap();
        draw(context, &mesh, DrawParam::default()).unwrap();

        // Draw 'level' text
        let mut text = Text::new("LEVEL");
        text.set_font(font, Scale::uniform(20.0));
        let text_width = text.width(context);
        let text_height = text.height(context);
        queue_text(
            context,
            &text,
            Point2 {
                x: GAME_OFFSET_X - 20.0 - text_width as f32,
                y: GAME_OFFSET_Y + 200.0,
            },
            Some(WHITE),
        );

        // Draw level
        let mut text = Text::new(self.level.to_string());
        text.set_font(font, Scale::uniform(20.0));
        let level_text_width = text.width(context);
        queue_text(
            context,
            &text,
            Point2 {
                x: GAME_OFFSET_X - 20.0 - text_width as f32 / 2.0 - level_text_width as f32 / 2.0,
                y: GAME_OFFSET_Y + 210.0 + text_height as f32,
            },
            Some(WHITE),
        );

        // Draw 'score' text
        let mut text = Text::new("SCORE");
        text.set_font(font, Scale::uniform(20.0));
        let text_height = text.height(context);
        queue_text(
            context,
            &text,
            Point2 {
                x: GAME_OFFSET_X + (SIZE * GAME_WIDTH as f32) + 20.0,
                y: GAME_OFFSET_Y + 200.0,
            },
            Some(WHITE),
        );

        // Draw score
        let mut text = Text::new(self.score.to_string());
        text.set_font(font, Scale::uniform(20.0));
        let score_text_width = text.width(context);
        queue_text(
            context,
            &text,
            Point2 {
                x: GAME_OFFSET_X + (SIZE * GAME_WIDTH as f32) + 20.0 + text_width as f32 / 2.0
                    - score_text_width as f32 / 2.0,
                y: GAME_OFFSET_Y + 210.0 + text_height as f32,
            },
            Some(WHITE),
        );

        // Draw FPS counter
        let mut text = Text::new("FPS: ".to_string() + &self.fps.to_string());
        text.set_font(font, Scale::uniform(12.0));
        draw(
            context,
            &text,
            DrawParam::default().dest(Point2 { x: 2.0, y: 2.0 }),
        )
        .unwrap();

        sleep(Duration::from_millis(1));

        draw_queued_text(context, DrawParam::default(), None, FilterMode::Linear).unwrap();
        present(context).unwrap();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        key_code: KeyCode,
        _key_mods: KeyMods,
        _repeat: bool,
    ) {
        let key = serde_json::to_string(&key_code)
            .expect(&*format!("Invalid key: {:?}", key_code))
            .replace("\"", "");

        self.held_keys.insert(key_code, self.ticks);

        if is_key_repeated(context) {
            return ();
        }

        let controls = &self.config.controls;

        // Enter will restart if the game has ended
        if self.game_over {
            if key == controls.restart {
                self.board = [[BLACK; GAME_WIDTH as usize]; GAME_HEIGHT as usize];
                self.bag = Vec::with_capacity(7);
                self.current_tetromino = None;
                self.next_tetromino = None;
                self.held_tetromino = None;
                self.draw = true;
                self.game_over = false;
                self.back_to_back = false;
                self.score = 0;
                self.combo = 0;
                self.level = 1;
                new_current_tetromino(self);
                if !self.config.muted {
                    self.bgm.play().unwrap();
                }
            }
            return ();
        }

        let mut current_tetromino = self.current_tetromino.clone().unwrap();
        let mut draw = true;

        if key == controls.mute {
            // Mute/unmute
            if self.config.muted {
                self.bgm.resume();
            } else {
                self.bgm.pause();
            }
            self.config.muted = !self.config.muted;
            self.config.update(context);
        } else if key == controls.pause {
            // Pause / unpause
            self.paused = !self.paused;
            if self.paused {
                self.bgm.pause();
            } else if !self.config.muted {
                self.bgm.resume();
            }
        } else if self.paused {
            // The controls below should not work if the game is paused
            return ();
        } else if key == controls.move_left {
            // Move left
            current_tetromino.x -= 1;
            if !collides(&current_tetromino, self.board) {
                self.current_tetromino = Some(current_tetromino);
                if !self.config.muted {
                    play_sound(context, MOVE_SOUND, 1.0, false, true);
                }
            }
        } else if key == controls.move_right {
            // Move right
            current_tetromino.x += 1;
            if !collides(&current_tetromino, self.board) {
                self.current_tetromino = Some(current_tetromino);
                if !self.config.muted {
                    play_sound(context, MOVE_SOUND, 1.0, false, true);
                }
            }
        } else if key == controls.rotate_cw {
            // Rotate CW
            current_tetromino.rotation = (1 + current_tetromino.rotation) % 4;
            attempt_rotation(self, current_tetromino, context);
        } else if key == controls.rotate_ccw {
            // Rotate CCW
            current_tetromino.rotation -= 1;
            if current_tetromino.rotation < 0 {
                current_tetromino.rotation = 3;
            }
            attempt_rotation(self, current_tetromino, context);
        } else if key == controls.hard_drop {
            // Hard drop
            loop {
                current_tetromino.y += 1;
                if collides(&current_tetromino, self.board) {
                    current_tetromino.y -= 1;

                    self.score += (current_tetromino.y - self.current_tetromino.as_ref().unwrap().y)
                        as i32
                        * 2;
                    self.current_tetromino = Some(current_tetromino);
                    get_new_tetromino(self);
                    check_line_clears(self, context);
                    break;
                }
            }
        } else if key == controls.hold {
            // Hold piece
            if !self.has_swapped {
                if self.held_tetromino.is_some() {
                    // Swap current and held tetromino
                    let temp = self.current_tetromino.clone().unwrap();
                    self.current_tetromino =
                        Some(CurrentTetromino::new(self.held_tetromino.clone().unwrap()));
                    self.held_tetromino = Some(temp.tetromino);
                } else {
                    self.held_tetromino = Some(self.current_tetromino.clone().unwrap().tetromino);
                    new_current_tetromino(self);
                }

                self.has_swapped = true;
                if !self.config.muted {
                    play_sound(context, ROTATION_SOUND, 1.0, false, true);
                }
            }
        } else {
            draw = false; // Don't draw to the screen if nothing was changed
        }

        self.draw = draw;
    }

    fn key_up_event(&mut self, _context: &mut Context, key_code: KeyCode, _key_mods: KeyMods) {
        self.held_keys.remove(&key_code);
    }

    fn focus_event(&mut self, _context: &mut Context, gained: bool) {
        self.focused = gained;
        self.draw = true;
    }
}

fn play_sound(
    context: &mut Context,
    sound: &[u8],
    volume: f32,
    repeat: bool,
    detached: bool,
) -> Source {
    let mut sound = Source::from_data(context, SoundData::from_bytes(sound)).unwrap();
    sound.set_repeat(repeat);
    if detached {
        sound.play_detached().unwrap();
    } else {
        sound.play().unwrap();
    }
    sound.set_volume(volume);
    return sound;
}

fn attempt_rotation(
    game_state: &mut GameState,
    mut new_tetromino: CurrentTetromino,
    context: &mut Context,
) {
    let mut can_rotate = true;
    if collides(&new_tetromino, game_state.board) {
        can_rotate = false;
        let wall_kicks = wall_kicks(
            &new_tetromino.tetromino,
            game_state.current_tetromino.clone().unwrap().rotation,
            new_tetromino.rotation,
        );
        for wall_kick in wall_kicks.iter() {
            let mut tetromino = new_tetromino.clone();
            tetromino.x += wall_kick.x;
            tetromino.y += wall_kick.y;
            if !collides(&tetromino, game_state.board) {
                new_tetromino = tetromino;
                can_rotate = true;
                break;
            }
        }
    }
    if can_rotate {
        game_state.current_tetromino = Some(new_tetromino);
        check_collision(game_state, context);
        if !game_state.config.muted {
            play_sound(context, ROTATION_SOUND, 1.0, false, true);
        }
    }
}

fn new_current_tetromino(game_state: &mut GameState) {
    if game_state.bag.is_empty() {
        game_state.bag = TETROMINOS
            .choose_multiple(&mut thread_rng(), 7)
            .cloned()
            .collect()
    }

    if game_state.next_tetromino.is_some() {
        game_state.current_tetromino = Some(CurrentTetromino::new(
            game_state.next_tetromino.clone().unwrap(),
        ));
        game_state.next_tetromino = Some(game_state.bag.remove(0));
    } else {
        game_state.current_tetromino = Some(CurrentTetromino::new(game_state.bag.remove(0)));
        game_state.next_tetromino = Some(game_state.bag.remove(0));
    }
}

// Returns whether or not the current tetromino is colliding
fn collides(
    current_tetromino: &CurrentTetromino,
    board: [[Color; GAME_WIDTH as usize]; GAME_HEIGHT as usize],
) -> bool {
    let tetromino = &current_tetromino.tetromino.rotations[current_tetromino.rotation as usize];
    for i in 0..tetromino.len() {
        for j in 0..tetromino[i].len() {
            if tetromino[i][j] == 0 {
                continue;
            }

            let x = j as i8 + current_tetromino.x;
            let y = i as i8 + current_tetromino.y;

            if y < 0 {
                continue;
            }

            // Collides with floor        Collides with walls               Collides with other pieces
            if y as i32 >= GAME_HEIGHT
                || x >= GAME_WIDTH as i8
                || x < 0
                || board[y as usize][x as usize] != BLACK
            {
                return true;
            }
        }
    }
    false
}

fn check_collision(game_state: &mut GameState, context: &mut Context) {
    let mut current_tetromino = game_state.current_tetromino.clone().unwrap();
    current_tetromino.y += 1;

    if collides(&current_tetromino, game_state.board) {
        get_new_tetromino(game_state);
        check_line_clears(game_state, context);
    }
}

fn check_line_clears(game_state: &mut GameState, context: &mut Context) {
    let mut line_clears: Vec<usize> = Vec::with_capacity(GAME_HEIGHT as usize);

    // Add the indexes of the cleared lines to array
    for i in 0..game_state.board.len() {
        if !game_state.board[i].contains(&BLACK) {
            line_clears.push(i);
        }
    }

    let mut line_clears_num = 0;
    for i in (0..game_state.board.len()).rev() {
        // Move lines above cleared lines down
        if i + line_clears_num < GAME_HEIGHT as usize {
            game_state.board[i + line_clears_num] = game_state.board[i];
        }

        if line_clears.contains(&i) {
            line_clears_num += 1;
        }
    }

    // Level up
    game_state.lines += line_clears_num as i32;
    if game_state.lines > game_state.level * 5 {
        game_state.level += 1;
        game_state.lines = 0;
    }

    // Award score (stored as float so it can be multiplied by 1.5)
    let mut score: f32 = game_state.level as f32
        * match line_clears_num {
            0 => 0.0,
            1 => 100.0,
            2 => 300.0,
            3 => 500.0,
            4 => 800.0,
            _ => panic!("You somehow cleared a number of lines that is below 0 or above 4"),
        };
    // Back to back tetris
    if game_state.back_to_back && line_clears_num == 4 {
        score *= 1.5;
    }
    // Combos
    if line_clears_num > 0 {
        score += (game_state.combo * 50 * game_state.level) as f32;
    }
    game_state.score += score as i32;

    if line_clears_num == 4 {
        game_state.back_to_back = true;
    } else if line_clears_num > 0 {
        game_state.back_to_back = false;
    }

    if line_clears_num > 0 {
        game_state.combo += 1;
        if !game_state.config.muted {
            play_sound(
                context,
                LINE_CLEAR_SOUND,
                1.0 + ((line_clears_num - 1) as f32 * 2.0),
                false,
                true,
            );
        }
    } else {
        game_state.combo = 0;
        if !game_state.config.muted {
            play_sound(context, HARD_DROP_SOUND, 1.0, false, true);
        }
    }
}

fn get_new_tetromino(game_state: &mut GameState) {
    let current_tetromino = game_state.current_tetromino.clone().unwrap();
    let tetromino = &current_tetromino.tetromino.rotations[current_tetromino.rotation as usize];

    // Add tetromino to board and get new current tetromino
    for i in 0..tetromino.len() {
        for j in 0..tetromino[i].len() {
            if current_tetromino.tetromino.rotations[current_tetromino.rotation as usize][i][j] == 1
            {
                game_state.board[(current_tetromino.y + i as i8) as usize]
                    [(current_tetromino.x + j as i8) as usize] = current_tetromino.tetromino.color;
            }
        }
    }

    game_state.has_swapped = false;

    new_current_tetromino(game_state);

    if current_tetromino.y < 1 {
        game_state.bgm.pause();
        game_state.game_over = true;
        game_state.draw = true;
    }
}

fn draw_tetromino(current_tetromino: &CurrentTetromino, color: Color, mesh: &mut MeshBuilder) {
    let tetromino = &current_tetromino.tetromino.rotations[current_tetromino.rotation as usize];

    for i in 0..tetromino.len() {
        for j in 0..tetromino[i].len() {
            if tetromino[i][j] == 1 {
                mesh.rectangle(
                    DrawMode::fill(),
                    Rect::new(
                        GAME_OFFSET_X + SIZE * (current_tetromino.x + j as i8) as f32,
                        GAME_OFFSET_Y + SIZE * (current_tetromino.y + i as i8) as f32,
                        SIZE,
                        SIZE,
                    ),
                    color,
                );
            }
        }
    }
}

fn main() {
    let (context, event_loop) = &mut ContextBuilder::new("Reimtris", "Reimar")
        .window_setup(WindowSetup {
            title: "Reimtris".to_string(),
            samples: Zero,
            vsync: false,
            icon: "".to_string(),
            srgb: false,
        })
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .unwrap();

    // Crash handler
    let user_data_path = user_data_dir(context).to_owned();
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info: &panic::PanicInfo| {
        let mut path = user_data_path.clone();
        path.push("crash.txt");
        let mut file = File::create(path.clone()).unwrap();
        file.write_all(info.to_string().as_bytes()).unwrap();

        // Create popup window if using windows
        unsafe {
            create_popup_message(info, path);
        }

        panic_hook(info);
    }));

    let game_state = &mut GameState::new(context);
    event::run(context, event_loop, game_state).unwrap();
}

#[cfg(windows)]
unsafe fn create_popup_message(info: &panic::PanicInfo, path: PathBuf) {
    let location = info.location().unwrap();
    let title: Vec<u16> = "Reimtris crashed :(\0".encode_utf16().collect();
    let message: Vec<u16> = (format!(
        "{}\n\n{}:{}:{}\n\nFull error at: {}\0",
        info.payload()
            .downcast_ref::<&str>()
            .unwrap_or(&"Unknown error"),
        location.file(),
        location.line(),
        location.column(),
        path.to_str().unwrap_or("Unknown path")
    ))
    .encode_utf16()
    .collect();
    winapi::um::winuser::MessageBoxW(
        null_mut(),
        message.as_ptr(),
        title.as_ptr(),
        winapi::um::winuser::MB_OK | winapi::um::winuser::MB_ICONERROR,
    );
}

#[cfg(not(windows))]
fn create_popup_message(info: &panic::PanicInfo) {
    // Do nothing
}
