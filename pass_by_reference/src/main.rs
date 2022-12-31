struct User {
    username: String,
    email: String,
    active: bool,
    id: u32
}

fn display_user(user: &User){
    println!("Username: {}\nEmail: {}\nActive: {}\nID: {}",
             user.username, user.email, user.active, user.id);
}

fn main() {
    let new_user = User {username: String::from("GulDaniel"), 
                         email: String::from("guldaniel@email.com"),
                         active: true, id: 141};

    display_user(&new_user);
}
