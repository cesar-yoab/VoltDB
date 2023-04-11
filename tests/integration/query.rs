#[cfg(test)]
mod tests {
    use crate::core::database::Database;
    use crate::core::query::Query;

    #[test]
    fn test_insert() {
        let mut database = Database::new();
        let key = "test_key".to_string();
        let vector = vec![1.0, 2.0, 3.0];

        let query = Query::Insert(key.clone(), vector.clone());
        let result = query.execute(&mut database);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get() {
        let mut database = Database::new();
        let key = "test_key".to_string();
        let vector = vec![1.0, 2.0, 3.0];

        let query = Query::Insert(key.clone(), vector.clone());
        let _ = query.execute(&mut database);

        let query = Query::Get(key.clone());
        let result = query.execute(&mut database);
        assert_eq!(result, Ok(format!("{:?}", vector)));
    }

    #[test]
    fn test_remove() {
        let mut database = Database::new();
        let key = "test_key".to_string();
        let vector = vec![1.0, 2.0, 3.0];

        let query = Query::Insert(key.clone(), vector.clone());
        let _ = query.execute(&mut database);

        let query = Query::Remove(key.clone());
        let result = query.execute(&mut database);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nearest_neighbors() {
        let mut database = Database::new();
        let key1 = "test_key1".to_string();
        let vector1 = vec![1.0, 2.0, 3.0];
        let key2 = "test_key2".to_string();
        let vector2 = vec![4.0, 5.0, 6.0];
        let key3 = "test_key3".to_string();
        let vector3 = vec![7.0, 8.0, 9.0];

        let query1 = Query::Insert(key1.clone(), vector1.clone());
        let _ = query1.execute(&mut database);
        let query2 = Query::Insert(key2.clone(), vector2.clone());
        let _ = query2.execute(&mut database);
        let query3 = Query::Insert(key3.clone(), vector3.clone());
        let _ = query3.execute(&mut database);

        let query = Query::NearestNeighbors(key1.clone(), 2);
        let result = query.execute(&mut database);
        assert_eq!(
            result,
            Ok(format!(
                "{:?},{:?}",
                (key2.clone(), vector2.clone()),
                (key3.clone(), vector3.clone())
            ))
        );
    }
}
