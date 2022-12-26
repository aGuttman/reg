fn main() {
    #[cfg(not(windows))]
    macro_rules! main_separator{
        ()=>{"/"}
    }
    
    #[cfg(windows)]
    macro_rules! main_separator{
        ()=>{r#"\"#}
    }

    let filestring = include_str!(concat!("..", main_separator!(),"resources", main_separator!(),"file.txt"));
    println!("{}", filestring);
}