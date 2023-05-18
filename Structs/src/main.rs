struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        actuve: ture,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another1@example.com"),
        ..user1
    };
    
    user1.email = String::from("newemail@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }


}
