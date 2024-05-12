pub const SESSION_DURATION : u16 = 900; // duration of login session in seconds

// /Users/egorivanov/Desktop/mysql.txt - MacOS
// C:\Users\User\Desktop\mysql.txt - Windows
// mysql.txt - Linux
pub const FILE_LOCATION : &'static str = r#"C:\Users\User\Desktop\mysql.txt"#;

// https://ugo-vape.ru
// http://localhost:8000
// http://localhost:3000
// Replace cors route here! + check if "/" at the end of URL, it shouldn't be there!
pub const CORS_ROUTE : &'static str = "http://localhost:3000";