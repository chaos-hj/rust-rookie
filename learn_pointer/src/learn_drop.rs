pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping data: {}", self.data);
    }
}

pub fn do_drop() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    //c.drop();
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointer created.")
}
