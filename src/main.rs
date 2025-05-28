enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time { 
        // ① Now that the enum variants hold a String, 
        //   you have to provide a String, too, when creating ThingsInTheSky.
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")),
        _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{description}"), 
        // ② Now, when we match on our reference to ThingsInTheSky, 
        // we have access to the data inside (in this case, a String). 
        // Note that we can give the inner String any name we want here: 
        // description, n, or anything else.
        ThingsInTheSky::Stars(n) => println!("{n}"),
    }
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
}