#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn can_hold(&self, rect: &Rect) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: i32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rect::square(3);

    println!("{:?}", sq);
}
