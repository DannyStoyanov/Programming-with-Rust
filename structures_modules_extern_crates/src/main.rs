fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_counts: u64,
    }
    let mut username = String::from("Admin");
    let mut email = String::from("myadmin@169.hr");
    let mut sign_in_counts = 54;

    let first_admin = User {
        username,
        email,
        sign_in_counts: 54,
    };

    let second_admin = User { 
        email: String::from("jade@gmail.com"),
        ..first_admin
    };

    let mut _currentUser = &second_admin;
    
    /*
    println!("----------------------------------------");
    println!("Username: {}, Email: {}", admin.username, admin.email);
    println!("----------------------------------------");  
    */
    impl User {
        fn new(username: String, email: String, sign_in_counts: u64) -> Self {
            Self{
                username,
                email,
                sign_in_counts,
            }
        }
        fn info(&self) -> () {
            println!("-----User Info-----");
            println!("Username: {}", self.username );
            println!("Email: {}", self.email);
            println!("Sign in counts: {}", self.sign_in_counts);
        }
    }

    let user = User::new(String::from("Lucas"), String::from("lucas@aubg.usa"), 16);
    _currentUser = &user;
    _currentUser.info();

    struct Color(i32, i32, i32);
    impl Color {
        fn info(&self) -> () {
            println!("-----Color Info-----");
            println!("r: {}, g: {}, b{}", self.0, self.1, self.2 );
        }
    }
    let black = Color(0, 0, 0);
    black.info();

}
