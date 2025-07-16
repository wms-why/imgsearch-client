fn main() {

    // 编译时注入环境变量
    let content = std::fs::read_to_string("../.env").expect(".env not found on building app");
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            println!("cargo:rustc-env={}={}", key.trim(), value.trim());
        }
    }

    tauri_build::build()
}
