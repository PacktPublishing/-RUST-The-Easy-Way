#[derive(Debug)]
struct Hero {
    name: String,
    energy: u16,
    strike: bool
}

#[derive(Debug)]
struct Goblin {
    energy: u16,
    strike: bool
}

impl Hero{
	fn jump(&self){
		//some logic for jumping
	}
}


trait StrikeTrait{
	fn strike(&mut self);
}

impl StrikeTrait for Hero{
	fn strike(&mut self){
		self.strike = true;
	}
}

impl StrikeTrait for Goblin {
    fn strike(&mut self){
		self.strike = false;
	}
}


fn main() {
    let mut hero = Hero{
    	name: "Dave".to_string(),
    	energy: 100,
    	strike: false
    };

    let mut goblin = Goblin{
    	energy: 99,
    	strike: true
    };

    println!("{:#?}", hero);

    hero.strike();

    println!("{:#?}", hero);

    println!("{:#?}", goblin);

    goblin.strike();

    println!("{:#?}", goblin);
}
