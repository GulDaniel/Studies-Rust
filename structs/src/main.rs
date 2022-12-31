struct User {
    username: String,
    email: String,
    active: bool,
    id: u32
}

fn main() {
    let new_user = User {username: String::from("GulDaniel"), 
                         email: String::from("guldaniel@email.com"),
                         active: true, id: 141};

    println!("Username: {}\nEmail: {}\nActive: {}\nID: {}",
             new_user.username, new_user.email, new_user.active, new_user.id);
}
