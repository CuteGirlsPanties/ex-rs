fn main() {
    println!("Hi, Release N5!");

    #[cfg(target_os = "linux")]
    print!("Hello, Linux ");
    #[cfg(target_os = "macos")]
    print!("Hello, Macos ");
    #[cfg(target_os = "windows")]
    print!("Hello, Windows ");
    
    #[cfg(target_arch = "x86_64")]
    println!("x86_64!");
    #[cfg(target_arch = "aarch64")]
    println!("aarch64!");
}
