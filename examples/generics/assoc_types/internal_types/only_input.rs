struct I32(i32);
struct I64(i64);

trait Adder<Rhs, Sum> {
    fn adder(&self, input: &Rhs) -> Sum;
}

impl Adder<i32, i32> for i32 {
    fn adder(&self, input: &i32) -> i32 { *self + *input }
}

impl Adder<i64, i64> for i32 {
    fn adder(&self, input: &i64) -> i64 { (*self as i64) + *input }
}

fn main() {
    let i1 = 12;
    let i2 = 4;
    let j1 = 8i64;
    
    println!("{}", i1.adder(&i2));
    println!("{}", i1.adder(&j1));
}
