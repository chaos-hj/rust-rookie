struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u16, u16, u16);
struct CuboidTup(u32, u32, u32);

//std::fmt::Display
#[derive(Debug)]
struct Cuboid {
    length: u32,
    width: u32,
    height: u32,
}

impl Cuboid {
    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn can_hold(&self, other: &Cuboid) -> bool {
        self.length > other.length && self.width > other.width
    }

    //associated functions
    fn build_cube(length: u32) -> Cuboid {
        Cuboid{
            length,
            width: length,
            height: length,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("***"),
        email: String::from("***@gmail.com"),
        sign_in_count: 1000,
        active: true,
    };

    if user1.active {
        user1.email = String::from("***@qq.com");
    }

    let user2 = build_user(String::from("xxx"), String::from("xxx@gmail.com"));

    let user2 = User {
        username: String::from("xxx@qq.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let v = volume(10, 10, 10);
    let v = volume1(CuboidTup(10,20,30));

    let cuboid = Cuboid{
        length: 10,
        width: 20,
        height: 30,
    };
    let v = volume2(&cuboid);

    let v = cuboid.volume();

    let cube = Cuboid::build_cube(5);
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email,
        sign_in_count: 0,
        active: true,
    }
}


fn volume(length: u32, width: u32, height: u32) -> u32 {
    length * width * height
}

fn volume1(c: CuboidTup) -> u32 {
    c.0 * c.1 * c.2
}

fn volume2(c: &Cuboid) -> u32 {
    println!("Cuboid is {:?}", c);
    c.length * c.width * c.height
}

