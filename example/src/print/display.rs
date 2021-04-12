pub fn it_works() {

    let point = Point2D{
        x: 3.3,
        y: 7.2
    };

    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let vec = List(vec![1, 2, 3]);
    println!("{}", vec);
}


use std::fmt;

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y:{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[")?;
        // let vec = &self.0;
        // for (index, item) in vec.iter().enumerate() {
        for (index, item) in (&self.0).iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", index, item)?;
        }
        
        write!(f, "]")
        
    }
} 