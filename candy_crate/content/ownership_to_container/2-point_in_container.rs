
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {

    let mut container: Vec<Point> = Vec::new();    
    let point = Point { x: 4, y: 4, };
    container.push(point);
    println!("added {};{}", point.x, point.y);
	
}
