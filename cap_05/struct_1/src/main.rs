struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main() {
	
	//#[warn(dead_code)]
	let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("user1@example.com");
    
    let user2 = build_user( String::from("user2@email.com"), String::from("Nombre user") ) ;

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user3@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    
    let user4 = User {
        email: String::from("user4@example.com"),
        ..user2
    };

    let mut i = 0;
    i+=1; 
    //println!("user{i}.email={0}, user{i}.username={1}", user1.email, user1.username); // value borrowed here after move

    i+=1; 
    //println!("user{i}.email={0}, user{i}.username={1}", user2.email, user2.username); // value borrowed here after move
    i+=1; 
    println!("user{i}.email={0}, user{i}.username={1}", user3.email, user3.username);
    i+=1; 
    println!("user{i}.email={0}, user{i}.username={1}", user4.email, user4.username);
    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
