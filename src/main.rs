use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{io::stdout, thread, time::Duration};

const PADDLE_HEIGHT: i32 = 4;
const WIDTH: i32 = 60;
const HEIGHT: i32 = 20;

struct Game {
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    left_paddle_y: i32,
    right_paddle_y: i32,
    left_score: i32,
    right_score: i32,
}

impl Game {
    fn new() -> Self {
        Game {
            ball_x: WIDTH as f32 / 2.0,
            ball_y: HEIGHT as f32 / 2.0,
            ball_dx: 1.0,
            ball_dy: 0.5,
            left_paddle_y: HEIGHT / 2,
            right_paddle_y: HEIGHT / 2,
            left_score: 0,
            right_score: 0,
        }
    }

    fn update(&mut self) {
        // Update ball position
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        // Ball collision with top and bottom walls
        if self.ball_y <= 0.0 || self.ball_y >= HEIGHT as f32 {
            self.ball_dy = -self.ball_dy;
        }

        // Ball collision with paddles
        if self.ball_x <= 1.0
            && (self.ball_y as i32 >= self.left_paddle_y - PADDLE_HEIGHT / 2)
            && (self.ball_y as i32 <= self.left_paddle_y + PADDLE_HEIGHT / 2)
        {
            self.ball_dx = -self.ball_dx;
            self.ball_dx *= 1.1; // Increase speed
        }

        if self.ball_x >= WIDTH as f32 - 1.0
            && (self.ball_y as i32 >= self.right_paddle_y - PADDLE_HEIGHT / 2)
            && (self.ball_y as i32 <= self.right_paddle_y + PADDLE_HEIGHT / 2)
        {
            self.ball_dx = -self.ball_dx;
            self.ball_dx *= 1.1; // Increase speed
        }

        // Score points
        if self.ball_x <= 0.0 {
            self.right_score += 1;
            self.reset_ball();
        }
        if self.ball_x >= WIDTH as f32 {
            self.left_score += 1;
            self.reset_ball();
        }
    }

    fn reset_ball(&mut self) {
        self.ball_x = WIDTH as f32 / 2.0;
        self.ball_y = HEIGHT as f32 / 2.0;
        self.ball_dx = if rand::random() { 1.0 } else { -1.0 };
        self.ball_dy = if rand::random() { 0.5 } else { -0.5 };
    }

    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), Clear(ClearType::All))?;

        // Draw scores
        execute!(
            stdout(),
            MoveTo(WIDTH as u16 / 4, 1),
            crossterm::style::Print(self.left_score),
            MoveTo(3 * WIDTH as u16 / 4, 1),
            crossterm::style::Print(self.right_score)
        )?;

        // Draw ball
        execute!(
            stdout(),
            MoveTo(self.ball_x as u16, self.ball_y as u16),
            crossterm::style::Print("●")
        )?;

        // Draw left paddle
        for i in -PADDLE_HEIGHT/2..=PADDLE_HEIGHT/2 {
            execute!(
                stdout(),
                MoveTo(0, (self.left_paddle_y + i) as u16),
                crossterm::style::Print("█")
            )?;
        }

        // Draw right paddle
        for i in -PADDLE_HEIGHT/2..=PADDLE_HEIGHT/2 {
            execute!(
                stdout(),
                MoveTo(WIDTH as u16 - 1, (self.right_paddle_y + i) as u16),
                crossterm::style::Print("█")
            )?;
        }

        // Draw borders
        for x in 0..WIDTH {
            execute!(stdout(), MoveTo(x as u16, 0), crossterm::style::Print("═"))?;
            execute!(
                stdout(),
                MoveTo(x as u16, HEIGHT as u16 - 1),
                crossterm::style::Print("═")
            )?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), Hide)?;

    let mut game = Game::new();

    loop {
        // Handle input
        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') if game.left_paddle_y > PADDLE_HEIGHT/2 => {
                        game.left_paddle_y -= 1
                    }
                    KeyCode::Char('s') if game.left_paddle_y < HEIGHT - PADDLE_HEIGHT/2 => {
                        game.left_paddle_y += 1
                    }
                    KeyCode::Up if game.right_paddle_y > PADDLE_HEIGHT/2 => {
                        game.right_paddle_y -= 1
                    }
                    KeyCode::Down if game.right_paddle_y < HEIGHT - PADDLE_HEIGHT/2 => {
                        game.right_paddle_y += 1
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        game.update();
        game.draw()?;
        thread::sleep(Duration::from_millis(50));
    }

    execute!(stdout(), Show)?;
    disable_raw_mode()?;
    Ok(())
}
