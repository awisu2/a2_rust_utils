pub fn say(name: &str) {
    println!("Hello, {}!", name);
}

pub fn night(name: &str) {
    println!("Good night, {}!", name);
}

#[cfg(test)]
mod tests {
    // 親モジュールの関数を使うため super:: を付ける
    use super::*;

    #[test]
    fn test_say() {
        // 出力確認なので assert! は不要だが、
        // とりあえず呼び出しがエラーなく動くことを確認
        say("Alice");
    }
}
