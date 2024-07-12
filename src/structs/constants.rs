pub const SESSION_DURATION : u16 = 900; // duration of session after login in seconds

pub const DEPLOY_PORT : u16 = 8000; // local port to deploy an API
pub const STANDARD_IP : &'static str = "0.0.0.0"; // standard IP for deploying

// /Users/egorivanov/Desktop/mysql.txt - MacOS
// C:\Users\User\Desktop\mysql.txt - Windows
// mysql.txt - Linux
pub const FILE_LOCATION : &'static str = r#"C:\Users\User\Desktop\mysql.txt"#;

// https://ugo-vape.ru
// http://localhost:8000
// http://localhost:3000
// Replace cors route here! + check if there is a "/" at the end of URL, it shouldn't be there!