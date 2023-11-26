
fn sum(x: u32, y: u32) -> u32 {
    return x + y;
}


fn main() {
    let result = sum(2,3);

    let lightspeed = 299792458;

let fast = &lightspeed;
let still_fast = &&lightspeed;

let speed_of_light = &still_fast;
print!("{fast}");

let starship: Option<String> = Some("Omaha".to_string());

match starship {
    Some(ref name) => println!("{}", name),
    None => {}
}

// Without the use of the `ref` keyword above, this next line would not compile:
println!("{:?}", starship);
let favorite = "orange";
println!("{favorite}");

let favorite = "cerulean";
println!("{favorite}");

let favorite = "yellow";
println!("{favorite}");

println!("{favorite}")
}
