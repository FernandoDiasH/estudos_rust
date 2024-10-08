fn main() {
    let user1 = User {
        active:true,
        username:String::from("testelsjflakjfteste"),
        email:String::from("sfjklajf@dslfjakf.com"),
        sign_in_count:1,
    };
    
    let user2 = User {
        active: user1.active,
        email: String::from("jsdflkjf@testge.com"),
        ..user1
    };
    
    let black = Color(0,0,0);
    let origin = Point(0,0,0);


}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn build_user(email:String, username:String) ->User{
    User {
        active:true,
        username,
        email,
        sign_in_count:1,
    }
}
