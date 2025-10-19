fn main() {
    let age :u8 = 14;
    let hello :&'static str= "Hello world";
    let on_enable:bool = true;
    let time :f32 = 12.34;
    let emoje :char = '😡';


    println!("{}", age);
    println!("{}", hello);
    println!("{}",on_enable);
    println!("{}", time);
    println!("{}", emoje)
}