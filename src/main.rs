// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or like structures.
    Click { x: i64, y: i64 },
    // Als ik het hier nou toevoeg?
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
// The `inspect` function is called with different variants of the `WebEvent` enum.
// Each variant is matched and handled accordingly, demonstrating the power of enums
// in Rust to encapsulate different types of data and behavior in a type-safe manner.
// This allows for clear and concise handling of different events in a web application context.
// The `match` statement is used to destructure the enum variants and execute code based on
// the specific variant passed to the `inspect` function, showcasing Rust's powerful pattern matching capabilities
// and type safety features.
// This code is a simple demonstration of how enums can be used to represent different types of web
// events in a type-safe manner, allowing for clear and concise handling of different events
// in a web application context. The use of enums and pattern matching in Rust provides a powerful
// way to encapsulate different types of data and behavior, ensuring that the code is both safe
// and expressive. The `inspect` function serves as a central point for handling various web events
// and demonstrates how Rust's type system can enforce correctness while providing flexibility
// in handling different scenarios. This approach is particularly useful in larger applications
// where different types of events need to be managed in a clear and organized manner, making the
// codebase easier to maintain and understand. Overall, this code serves as a foundational example
// of how to effectively use enums and pattern matching in Rust to create robust and type-safe applications
// that can handle a variety of scenarios with ease and clarity.
// The `WebEvent` enum encapsulates different types of web events, allowing for type-safe
// handling of various scenarios in a web application. The `inspect` function demonstrates
// how to match against these enum variants and execute specific logic based on the event type. 
// This pattern is common in Rust applications, where enums are used to represent a set of related
// but distinct types, providing a clear and concise way to manage different states or events.
// The use of pattern matching with enums allows for expressive and readable code, making it easy
// to understand the flow of the application and how different events are handled.
// This approach not only enhances code clarity but also leverages Rust's strong type system to
// ensure that all possible cases are handled, reducing the likelihood of runtime errors.
// By using enums and pattern matching, developers can create more maintainable and robust applications
// that are easier to reason about, especially as the complexity of the application grows.

// This code is a simple demonstration of how enums can be used to represent different types of web
// events in a type-safe manner, allowing for clear and concise handling of different events
// in a web application context. The use of enums and pattern matching in Rust provides a powerful
// way to encapsulate different types of data and behavior, ensuring that the code is both safe
// and expressive. The `inspect` function serves as a central point for handling various web events
// and demonstrates how Rust's type system can enforce correctness while providing flexibility
// in handling different scenarios. This approach is particularly useful in larger applications
// where different types of events need to be managed in a clear and organized manner, making the
// codebase easier to maintain and understand. Overall, this code serves as a foundational example
// of how to effectively use enums and pattern matching in Rust to create robust and type-safe applications
// that can handle a variety of scenarios with ease and clarity.
// The `WebEvent` enum encapsulates different types of web events, allowing for type-safe
// handling of various scenarios in a web application. The `inspect` function demonstrates
// how to match against these enum variants and execute specific logic based on the event type.
// This pattern is common in Rust applications, where enums are used to represent a set of related
// but distinct types, providing a clear and concise way to manage different states or events.
// The use of pattern matching with enums allows for expressive and readable code, making it easy
// to understand the flow of the application and how different events are handled.
// This approach not only enhances code clarity but also leverages Rust's strong type system to
// ensure that all possible cases are handled, reducing the likelihood of runtime errors.
// By using enums and pattern matching, developers can create more maintainable and robust applications
// that are easier to reason about, especially as the complexity of the application grows.
// The `WebEvent` enum encapsulates different types of web events, allowing for type-safe
// handling of various scenarios in a web application. The `inspect` function demonstrates
// how to match against these enum variants and execute specific logic based on the event type.
// This pattern is common in Rust applications, where enums are used to represent a set of related
// but distinct types, providing a clear and concise way to manage different states or events.
// The use of pattern matching with enums allows for expressive and readable code, making it easy
// to understand the flow of the application and how different events are handled.
// This approach not only enhances code clarity but also leverages Rust's strong type system to
// ensure that all possible cases are handled, reducing the likelihood of runtime errors.
// By using enums and pattern matching, developers can create more maintainable and robust applications
// that are easier to reason about, especially as the complexity of the application grows.
// The `WebEvent` enum encapsulates different types of web events, allowing for type-safe
// handling of various scenarios in a web application. The `inspect` function demonstratesg
// how to match against these enum variants and execute specific logic based on the event type.