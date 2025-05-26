struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message : {}", msg);
        return msg;
    }

    fn sender(&'a self, msg: &str) -> &str {
        println!("Transmitting message : {}", msg);
        return self.name;
    }
}

fn main() {
    let vehicle = Shuttle { name: "Pioneer" };
    let message = vehicle.send_transmission("Greetings :)");

    println!("Message is {}", message);

    let sender = vehicle.sender("Hello world");

    println!("Senter is {}", sender);
}
