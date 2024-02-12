use rand::Rng;

fn main() {
    let s = Some("Hello, world!");
    println!(
        "If s match Some: {}.",
        match s {
            Some(s) => s,
            None => "",
        }
    );

    let blog_posts = vec![
        Post {
            name: Some("moyashi".to_string()),
            ip_addr: "localhost".to_string(),
            metadata: Some("high".to_string()),
        },
        Post {
            name: Some("yudetamago".to_string()),
            ip_addr: "127.0.0.1".to_string(),
            metadata: Some("middle".to_string()),
        },
        Post {
            name: None,
            ip_addr: "127.0.0.1".to_string(),
            metadata: None,
        },
        Post {
            name: None,
            ip_addr: "127.0.0.1".to_string(),
            metadata: Some("Out of Range".to_string()),
        },
    ];
    for post in blog_posts {
        let display_name = match post.name {
            Some(author) => author,
            None => format!(
                "{}: metadata: {}",
                post.client_address(),
                post.get_metadata().expect("expect msg")
            ),
        };
        println!("blog_post_display_name: {:?}", display_name);
    }

    // definition
    let mut rng = rand::thread_rng();
    let mut users: Vec<User> = vec![];
    for _ in 0..100 {
        let num: u8 = rng.gen_range(0..3);
        let s = User {
            name: User::generate_unique_name(),
            nickname: None,
            pet: match num {
                0 => Some(Pet::Buzzard),
                1 => Some(Pet::Hyena),
                2 => Some(Pet::Fellet),
                _ => None,
            },
        };
        users.push(s)
    }
    users[50].nickname = Some("tomo".to_string());
    users[50].pet = Some(Pet::Fellet);

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
struct User {
    name: String,
    nickname: Option<String>,
    pet: Option<Pet>,
}

impl User {
    fn nickname(&self) -> Option<String> {
        self.nickname.to_owned()
    }
    pub fn generate_unique_name() -> String {
        String::from(nanoid!(8))
    }
}

/*
use std::io;
use std::fs::FileInfo;
use std::cmp::Ordering;

fn show_files() -> io::Result<()> {
    let mut v = vec![];

    fn cmp_by_time_stamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        a.timestamp.cmp(&b.timestamp)
            .reverse()
            .then(a.path.cmp(&b.path))
    }
    v.sort_by(cmp_by_time_stamp_then_name);
    Ok(())
} */
#[derive(Debug)]
enum Pet {
    Buzzard,
    Hyena,
    Fellet,
}


/// loop
/// 
#[test]
fn test_string_vector_loop() {
    let mut vec_string = vec![String::from("foo"), String::from("bar")];

    for rs in &vec_string {
        println!("String {:?} is address is {:p}", *rs, rs)
    }
    
    for rs in &mut vec_string {
        rs.insert_str(rs.len() - 1, "char");
        println!("String {:?} is address is {:p}", *rs, rs)
    }
    
    for rs in &mut vec_string {
        rs.insert_str(rs.len() - 1, "char");
        println!("String {:?} is address is {:p}", *rs, rs)
    }
} 