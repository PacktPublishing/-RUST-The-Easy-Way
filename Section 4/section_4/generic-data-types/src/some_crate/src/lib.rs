#[derive(Debug)]
pub struct User{
	pub name: String,
	pub email: String,
	pub age: u16,
	pub user_type: UserType
}

impl User{
	pub fn print_user(self){
		println!("The name of the user is {}.\nHis email is: {}.\nHe is {} old.\nType of user: {:?}", self.name, self.email, self.age, self.user_type);
	}
}

#[derive(Debug)]
pub enum UserType{
	Guest,
	Regular,
	Admin
}

