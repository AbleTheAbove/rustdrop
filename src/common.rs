// use image_base64

//All the supported drop types of rdrop
#[derive(Debug)]
// IDEA(Able): maybe add a pop up gui/ notification showing the image
enum DropTypes {
    Text,
    Image,
    Contact,
    Other,
}
#[derive(Debug)]
struct DropData {
    dtype: DropTypes,
    data: String,
}

fn main() {
    //convert to base64
    let x = DropData {
        dtype: DropTypes::Text,
        data: "String".to_string(),
    };
    // Match type here
    match x.dtype {
        DropTypes::Image => {}   // TODO(Able): Display image in notif
        DropTypes::Text => {} // TODO(Able): display the first 200 characters in the drop in the notification
        DropTypes::Contact => {} // rdrop a contact, add to contacts if accepted
        DropTypes::Other => {} // TODO: Just save file  if dropped
    }

    println!("{:?}", x);
}
