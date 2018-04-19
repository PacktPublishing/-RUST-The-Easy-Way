extern crate some_crate;

use some_crate::User;

struct UserCollection<T, P>{
	name: String,
	users: Vec<P>,
	size: T
}

fn main(){
	let new_user = User{
		name: "User 1".to_string(),
		email: "user@mail.com".to_string(),
		age: 30,
		user_type: some_crate::UserType::Guest
	};

	let user_collection: UserCollection<u8, User> = UserCollection{
		name: "user collection 1".to_string(),
		users: vec![new_user],
		size: 1
	};

	println!("{:?}", user_collection.users);


}
