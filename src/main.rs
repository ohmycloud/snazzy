use snazzy::{create_large_shirt, serialize_shirt, deserialize_shirt};

fn main() {
    let blue_shirt = create_large_shirt("Blue".to_string());
    println!("{:?}", blue_shirt);

    let serialize_blue_shit = serialize_shirt(&blue_shirt);
    println!("{:?}", serialize_blue_shit);

    let deserialize_blue_shirt = deserialize_shirt(&serialize_blue_shit);
    if let Ok(shirt) = deserialize_blue_shirt {
        println!("{:?}", shirt);
    }
}