fn main() {
    let s = Some("Hello, world!");
    println!("If s match Some: {}.", 
        match s {
            Some(s) => s,
            None => ""
        });

    let post_blogs = vec![
                    Post { name: Some("moyashi".to_string()),
                              ip_addr: "localhost".to_string(),
                              metadata: Some("high".to_string())
                            },
                    Post { name: Some("yudetamago".to_string()),
                              ip_addr: "127.0.0.1".to_string(),
                              metadata: Some("middle".to_string())
                            },
                    Post { name: None,
                              ip_addr: "127.0.0.1".to_string(),
                              metadata: None
                            },
                    Post { name: None,
                              ip_addr: "127.0.0.1".to_string(),
                              metadata: Some("Out of Range".to_string())
                            },
                ];
    for post in post_blogs {
        let display_name = match post.name {
            Some(author) => author,
            None => 
                format!("{}: metadata: {}", post.client_address(), post.get_metadata().expect("expect msg"))
            };
        println!("blog_post_display_name: {:?}", display_name);
    }

    // definition

    let mut users: Vec<User>= vec![];
    for i in 0..100 {
        let s = User { name: User::generate_unique_name(), nickname: None };
        users.push(s)
    }
    users[50].nickname = Some("tomo".to_string());

    for i in 0..users.len() {
        if let Some(_n) = &users[i].nickname {
            println!("nickname: {:?}", users[i].nickname);
        } else {
            println!("{:?}", users[i]);
        }
    }

}

struct Post {
    name: Option<String>,
    ip_addr: String,
    metadata: Option<String>,
}
impl Post {
    fn get_metadata(&self) -> Result<String, ()> {

        match &self.metadata {
            None => Ok("not have metadata".to_string()),
            Some(m) => Ok(m.clone()),
        }
    }
    fn client_address(&self) -> &str {
        &self.ip_addr
    }
}

use nanoid::nanoid;

#[derive(Debug)]
struct User{
    name: String,
    nickname: Option<String>
}
impl User {
    fn nickname(&self) -> Option<String> { self.nickname.to_owned() }
    pub fn generate_unique_name() -> String { String::from(nanoid!(8)) }
}