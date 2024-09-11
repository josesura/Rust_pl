struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn print_user(user: &User) {
	println!("{1}: email={0}, active={2}, sign_in_count={3}, ", user.email, user.username, user.active, user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
	
	let users = [ build_user( String::from("user1@email.com"), String::from("Nombre user1") ),
					build_user( String::from("user2@email.com"), String::from("Nombre user2") ),];

	
//	let mut i = 0;
//    i+=1; 
	for user in users {
		print_user(&user);
		println!("{1}: email={0}", user.email, user.username); 
	}	
	
}
