// Enums are types which have a few definite values.

enum Movement {
    // Variants.
    Up,
    Down,
    Left,
    Right
}

fn move_player(m: Movement) {
    // Perform an action depending on m: Movement.

    // Similar to a switch-case.
    match m {
        Movement::Up => println!("Move up!"),
        Movement::Down => println!("Move down!"),
        Movement::Left => println!("Move left!"),
        Movement::Right => println!("Move right!")
    }
}

pub fn run() {
    let player1 = Movement::Left;
    let player2 = Movement::Up;
    let player3 = Movement::Right;
    let player4 = Movement::Down;

    move_player(player1);
    move_player(player2);
    move_player(player3);
    move_player(player4);
}
