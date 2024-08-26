struct Example {
    value: i32,
}

impl Example {
    // 方法接收 &self 作为第一个参数
    fn get_value(&self) -> i32 {
        self.value
    }

    // 可变方法接收 &mut self
    fn set_value(&mut self, new_value: i32) {
        self.value = new_value;
    }

    // 消耗性方法接收 self
    fn consume(self) -> i32 {
        self.value
    }
}

fn main() {
    let mut example = Example { value: 42 };

    println!("Value: {}", example.get_value());
    example.set_value(100);
    println!("New value: {}", example.get_value());

    let final_value = example.consume();
    println!("Final value: {}", final_value);
}
