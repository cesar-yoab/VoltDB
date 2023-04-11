#[cfg(test)]
mod tests {
    use crate::core::database::{Database, Key, Vector};
    use crate::core::memory::Memory;
    use crate::core::persistence::Persistence;
    use std::fs;

    #[test]
    fn test_save_to_file() {
        let mut database = Database::new();
        let key = "test_key".to_string();
        let vector = vec![1.0, 2.0, 3.0];
        let filepath = "test_save_to_file.txt";

        let query = crate::core::query::Query::Insert(key.clone(), vector.clone());
        let _ = query.execute(&mut database);

        let result = Persistence::save_to_file(&database, filepath);
        assert!(result.is_ok());

        fs::remove_file(filepath).unwrap();
    }

    #[test]
    fn test_load_from_file() {
        let mut database = Database::new();
        let key = "test_key".to_string();
        let vector = vec![1.0, 2.0, 3.0];
        let filepath = "test_load_from_file.txt";

        let query = crate::core::query::Query::Insert(key.clone(), vector.clone());
        let _ = query.execute(&mut database);

        let result = Persistence::save_to_file(&database, filepath);
        assert!(result.is_ok());

        let result = Persistence::load_from_file(filepath);
        assert!(result.is_ok());
        let loaded_database = result.unwrap();

        let query = crate::core::query::Query::Get(key.clone());
        let result = query.execute(&loaded_database);
        assert_eq!(result, Ok(format!("{:?}", vector)));

        fs::remove_file(filepath).unwrap();
    }
}
