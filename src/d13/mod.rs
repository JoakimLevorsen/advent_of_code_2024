struct Position {
    x: u32,
    y: u32,
}

struct Machine {
    a: Position,
    b: Position,
    prize: Position,
}

// cost = ap * 1 + bp * 3 where px = ap * ax + bp * bx & py = ap * ay + bp * by

// Isolated = ap = ( px -  bp * bx ) / ax
// bp = (py - ap * ay)/by

// Which means
// ap = (px - ((py - ap * ay)/by) * bx) / ax

#[test]
fn test_equation() {
    let ax = 94.0;
    let ay = 34.0;

    let bx = 22.0;
    let by = 67.0;

    let px = 8400.0;
    let py = 5400.0;

    // let ap = (px - ((py - ap * ay) / by) * bx) / ax;

    let ap = (px - ((bx * py) / by)) / (ax - ((ay * bx) / by));
    let bp = (py - ap * ay) / by;

    let cost = 3.0 * ap + bp;

    println!("Cost for {ap} and {bp} presses was {cost}");
}
