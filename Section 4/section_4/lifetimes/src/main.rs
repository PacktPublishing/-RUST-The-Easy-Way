fn increase_by_five<'a>(x: &'a u16) -> u16{
	x + 5
}


#[derive(Debug)]
struct TestStruct<'a> {
    x: &'a u32
}

impl<'a> TestStruct<'a>{
    fn return_x(&self) -> &'a u32{
    	self.x
    }
}


fn main() {
	let ts = TestStruct{
		x: &5
	};

	println!("{:?}", ts.return_x());
}
