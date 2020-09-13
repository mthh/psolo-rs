mod board;
mod cell;

use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, VectorFont},
    input::Event,
    run, Graphics, Input, Settings, Window,
};

use crate::board::Board;
use crate::cell::Cell;

pub const ENGLISH_BOARD: &'static str =
    "  XXX  \n  XXX  \nXXXXXXX\nXXXOXXX\nXXXXXXX\n  XXX  \n  XXX  ";
pub const EUROPEAN_BOARD: &'static str =
    "  XXX  \n XXXXX \nXXXXXXX\nXXXOXXX\nXXXXXXX\n XXXXX \n  XXX  ";
pub const WIEGLEB_BOARD: &'static str =
    "   XXX   \n   XXX   \n   XXX   \nXXXXXXXXX\nXXXXOXXXX\nXXXXXXXXX\n   XXX   \n   XXX   \n   XXX   ";
pub const ASYMETRIC_BOARD: &'static str =
    "  XXX   \n  XXX   \n  XXX   \nXXXXXXXX\nXXXOXXXX\nXXXXXXXX\n  XXX   \n  XXX   ";

struct ScreenBoard {
    board: Board,
    cell_size: f32,
    cell_margin: f32,
    d_cell_peg_size: f32,
    d_cell_hole_size: f32,
    hole_size: f32,
    peg_size: f32,
    cell_with_margin: f32,
    board_size: (f32, f32),
    board_margin_top: f32,
    board_margin_left: f32,
}

impl ScreenBoard {
    pub fn new(
        board: Board,
        cell_size: f32,
        cell_margin: f32,
        d_cell_peg_size: f32,
        d_cell_hole_size: f32,
    ) -> Self {
        let cell_with_margin = cell_size + cell_margin;
        let board_size = (
            board.width() as f32 * cell_with_margin,
            board.height() as f32 * cell_with_margin,
        );
        ScreenBoard {
            board,
            cell_size,
            cell_margin,
            d_cell_peg_size,
            d_cell_hole_size,
            board_size,
            hole_size: (cell_size / 2.) - d_cell_hole_size,
            peg_size: (cell_size / 2.) - d_cell_peg_size,
            cell_with_margin: cell_with_margin,
            board_margin_top: (600. - board_size.1) / 1.8,
            board_margin_left: (600. - board_size.0) / 2.,
        }
    }

    fn get_row_col_cell_clicked(&self, position: Vector) -> Option<(u32, u32)> {
        let mut x = position.x;
        let mut y = position.y;
        x -= self.board_margin_left;
        y -= self.board_margin_top;
        x /= self.cell_size + self.cell_margin;
        y /= self.cell_size + self.cell_margin;
        x = x.floor();
        y = y.floor();
        if x < self.board.width() as f32 && y < self.board.height() as f32 && x >= 0. && y >= 0. {
            Some((x as u32, y as u32))
        } else {
            None
        }
    }
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> quicksilver::Result<()> {
    let mut screenboard: Option<ScreenBoard> = None;
    let bg_color = Color::from_rgba(128, 128, 128, 1.);
    let make_peg = |sb: &ScreenBoard,
                    i: u32,
                    j: u32,
                    color_rect: Color,
                    color_circle: Color,
                    g: &mut Graphics| {
        g.fill_rect(
            &Rectangle::new(
                Vector::new(
                    sb.board_margin_left + i as f32 * sb.cell_with_margin,
                    sb.board_margin_top + j as f32 * sb.cell_with_margin,
                ),
                Vector::new(sb.cell_size, sb.cell_size),
            ),
            color_rect,
        );
        g.fill_circle(
            &Circle::new(
                Vector::new(
                    sb.board_margin_left
                        + i as f32 * sb.cell_with_margin
                        + sb.peg_size
                        + sb.d_cell_peg_size,
                    sb.board_margin_top
                        + j as f32 * sb.cell_with_margin
                        + sb.peg_size
                        + sb.d_cell_peg_size,
                ),
                sb.peg_size,
            ),
            color_circle,
        );
    };

    let make_hole = |sb: &ScreenBoard, i: u32, j: u32, g: &mut Graphics| {
        g.fill_rect(
            &Rectangle::new(
                Vector::new(
                    sb.board_margin_left + i as f32 * sb.cell_with_margin,
                    sb.board_margin_top + j as f32 * sb.cell_with_margin,
                ),
                Vector::new(sb.cell_size, sb.cell_size),
            ),
            Color::BLUE,
        );
        g.fill_circle(
            &Circle::new(
                Vector::new(
                    sb.board_margin_left
                        + i as f32 * sb.cell_with_margin
                        + sb.hole_size
                        + sb.d_cell_hole_size,
                    sb.board_margin_top
                        + j as f32 * sb.cell_with_margin
                        + sb.hole_size
                        + sb.d_cell_hole_size,
                ),
                sb.hole_size,
            ),
            Color::INDIGO,
        );
    };
    let mut selected_src: Option<(u32, u32)> = None;
    let mut selected_dest: Option<(u32, u32)> = None;


    let ttf = VectorFont::load("font.ttf").await?;
    let mut font_title = ttf.to_renderer(&gfx, 56.0)?;
    let mut font_menu = ttf.to_renderer(&gfx, 28.0)?;
    let mut font_other = ttf.to_renderer(&gfx, 16.0)?;
    loop {
        gfx.clear(bg_color);
        font_title.draw(
            &mut gfx,
            "Peg Solitaire",
            Color::BLACK,
            Vector::new(20., 50.),
        )?;
        if screenboard.is_some() {
            let mut sb = screenboard.take().unwrap();
            let n_peg_left = sb.board.count_peg();
            let mut restart = false;
            if n_peg_left > 1 {
                font_other.draw(
                    &mut gfx,
                    &format!("{} pieces left", n_peg_left),
                    Color::BLACK,
                    Vector::new(
                        sb.board_margin_left + sb.board_size.0 - 100.,
                        sb.board_margin_top + sb.board_size.1 + 20.,
                    ),
                )?;
                font_other.draw(
                    &mut gfx,
                    &format!("Press [R] to restart."),
                    Color::BLACK,
                    Vector::new(240., 585.),
                )?;

                while let Some(ev) = input.next_event().await {
                    match ev {
                        Event::KeyboardInput(k_ev) => {
                            let key_pressed = k_ev.key();
                            if key_pressed == quicksilver::blinds::event::Key::R {
                                restart = true;
                            }
                        },
                        Event::PointerInput(p_ev) => {
                            // Left click : select a peg or select the destination of the previously selected_src peg
                            if p_ev.button() == quicksilver::blinds::MouseButton::Left
                                && p_ev.is_down()
                            {
                                let position =
                                    gfx.screen_to_camera(&window, input.mouse().location());

                                if let Some((i_clicked, j_clicked)) =
                                    sb.get_row_col_cell_clicked(position)
                                {
                                    let cell_clicked = sb.board.get_cell(i_clicked, j_clicked);
                                    if cell_clicked == Cell::Peg {
                                        // User was clicking to select a source peg
                                        selected_src = Some((i_clicked, j_clicked));
                                    } else if selected_src.is_some()
                                        && sb.board.is_valid_move(
                                            selected_src.unwrap(),
                                            (i_clicked, j_clicked),
                                        )
                                    {
                                        // User was clicking to select a destination peg and the move was valid
                                        selected_dest = Some((i_clicked, j_clicked));
                                    }
                                } else {
                                    // User clicked outside of the board
                                    selected_src = None;
                                }

                            // Right click : deselect the current selected_src peg if any
                            } else if p_ev.button() == quicksilver::blinds::MouseButton::Right
                                && p_ev.is_down()
                            {
                                selected_src = None;
                            }
                        }
                        _ => {}
                    }
                }
                for i in 0..sb.board.width() {
                    for j in 0..sb.board.height() {
                        let cell = sb.board.get_cell(i, j);
                        match cell {
                            Cell::Peg => {
                                make_peg(&sb, i, j, Color::BLUE, Color::YELLOW, &mut gfx);
                            }
                            Cell::Hole => {
                                make_hole(&sb, i, j, &mut gfx);
                            }
                            _ => {} // _ => {
                                    //     gfx.fill_rect(
                                    //         &Rectangle::new(
                                    //             Vector::new(
                                    //                 board_margin_left + i as f32 * cell_with_margin,
                                    //                 board_margin_top + j as f32 * cell_with_margin,
                                    //             ),
                                    //             Vector::new(cell_size, cell_size),
                                    //         ),
                                    //         Color::BLACK,
                                    //     );
                                    // }
                        };
                    }
                }

                if let Some((i, j)) = selected_src {
                    make_peg(&sb, i, j, Color::BLUE, Color::RED, &mut gfx);
                    let mouse = gfx.screen_to_camera(&window, input.mouse().location());
                    gfx.fill_circle(&Circle::new(mouse, 12.0), Color::RED);
                }

                if let Some((i, j)) = selected_dest {
                    make_peg(&sb, i, j, Color::RED, Color::BLUE, &mut gfx);
                }

                if let Some(dest_coords) = selected_dest {
                    if let Some(src_coords) = selected_src {
                        sb.board.make_move(src_coords, dest_coords);
                        selected_src = None;
                        selected_dest = None;
                    }
                }

                if !sb.board.has_valid_move_left() {
                    font_other.draw(
                        &mut gfx,
                        &format!("No valid move left !"),
                        Color::RED,
                        Vector::new(240., 565.),
                    )?;
                }
            } else {
                font_title.draw(
                    &mut gfx,
                    "YOU WIN !!",
                    Color::RED,
                    Vector::new(200.0, 200.0),
                )?;
            }
            if !restart {
                screenboard = Some(sb);
            } else {
                selected_src = None;
                selected_dest = None;
            }
        } else {
            font_menu.draw(
                &mut gfx,
                "Board selection:",
                Color::BLACK,
                Vector::new(190.0, 200.0),
            )?;
            gfx.fill_rect(
                &Rectangle::new(Vector::new(210., 230.), Vector::new(160., 40.)),
                Color::WHITE,
            );
            font_other.draw(
                &mut gfx,
                &format!("English Board"),
                Color::BLACK,
                Vector::new(230., 255.),
            )?;
            gfx.fill_rect(
                &Rectangle::new(Vector::new(210., 280.), Vector::new(160., 40.)),
                Color::WHITE,
            );
            font_other.draw(
                &mut gfx,
                &format!("European Board"),
                Color::BLACK,
                Vector::new(230., 305.),
            )?;
            gfx.fill_rect(
                &Rectangle::new(Vector::new(210., 330.), Vector::new(160., 40.)),
                Color::WHITE,
            );
            font_other.draw(
                &mut gfx,
                &format!("Asymetric Board"),
                Color::BLACK,
                Vector::new(230., 355.),
            )?;
            gfx.fill_rect(
                &Rectangle::new(Vector::new(210., 380.), Vector::new(160., 40.)),
                Color::WHITE,
            );
            font_other.draw(
                &mut gfx,
                &format!("Wiegleb Board"),
                Color::BLACK,
                Vector::new(230., 405.),
            )?;

            while let Some(ev) = input.next_event().await {
                match ev {
                    Event::PointerInput(p_ev) => {
                        // Left click : select a peg or select the destination of the previously selected_src peg
                        if p_ev.button() == quicksilver::blinds::MouseButton::Left && p_ev.is_down()
                        {
                            let position = gfx.screen_to_camera(&window, input.mouse().location());
                            let board_type = if position.y >= 380.
                                && position.y <= 420.
                                && position.x >= 210.
                                && position.x <= 370.
                            {
                                Some(WIEGLEB_BOARD)
                            } else if position.y >= 330.
                                && position.y <= 370.
                                && position.x >= 210.
                                && position.x <= 370.
                            {
                                Some(ASYMETRIC_BOARD)
                            } else if position.y >= 280.
                                && position.y <= 320.
                                && position.x >= 210.
                                && position.x <= 370.
                            {
                                Some(EUROPEAN_BOARD)
                            } else if position.y >= 230.
                                && position.y <= 270.
                                && position.x >= 210.
                                && position.x <= 370.
                            {
                                Some(ENGLISH_BOARD)
                            } else {
                                None
                            };
                            if let Some(b) = board_type {
                                screenboard = Some(ScreenBoard::new(
                                    Board::new(b)
                                        .expect("Unable to make board from the provided string"),
                                    50.,
                                    2.,
                                    3.,
                                    6.,
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        gfx.present(&window)?;
    }
}

fn main() {
    run(
        Settings {
            title: "Peg Solitaire",
            size: Vector::new(600., 600.),
            // resizable: true,
            ..Settings::default()
        },
        app,
    );
}
