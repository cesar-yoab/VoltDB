#[cfg(test)]
mod tests {
    use crate::config::settings::Settings;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_load_settings() {
        let file_content = r#"
        bind_address = "127.0.0.1:4000"
        memory_limit = 10000000
        "#;

        let test_file = "test_settings.toml";
        let mut file = File::create(test_file).unwrap();
        file.write_all(file_content.as_bytes()).unwrap();

        let result = Settings::from_file(test_file);
        assert!(result.is_ok());
        let settings = result.unwrap();

        assert_eq!(settings.bind_address, "127.0.0.1:4000");
        assert_eq!(settings.memory_limit, 10_000_000);

        std::fs::remove_file(test_file).unwrap();
    }
}
