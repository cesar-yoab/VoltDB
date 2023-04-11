#[cfg(test)]
mod tests {
    use crate::core::database::{Database, Key, Vector};
    use crate::core::memory::Memory;
    use crate::core::query::Query;
    use crate::core::settings::Settings;
    use crate::core::vector::VectorSpace;

    #[test]
    fn test_core_integration() {
        let settings = Settings {
            memory_limit: 10_000_000,
        };

        let memory = Memory::new();
        let vector_space = VectorSpace::new(settings.clone());
        let mut database = Database::new();

        let key1 = "key1".to_string();
        let vector1 = vec![1.0, 2.0, 3.0];
        let query = Query::Insert(key1.clone(), vector1.clone());
        let result = query.execute(&mut database);
        assert!(result.is_ok());
        memory.allocate(vector1.len() * std::mem::size_of::<f32>());

        let key2 = "key2".to_string();
        let vector2 = vec![4.0, 5.0, 6.0];
        let query = Query::Insert(key2.clone(), vector2.clone());
        let result = query.execute(&mut database);
        assert!(result.is_ok());
        memory.allocate(vector2.len() * std::mem::size_of::<f32>());

        let key3 = "key3".to_string();
        let vector3 = vec![7.0, 8.0, 9.0];
        let query = Query::Insert(key3.clone(), vector3.clone());
        let result = query.execute(&mut database);
        assert!(result.is_ok());
        memory.allocate(vector3.len() * std::mem::size_of::<f32>());

        vector_space.add_vector(key1.clone(), vector1.clone());
        vector_space.add_vector(key2.clone(), vector2.clone());
        vector_space.add_vector(key3.clone(), vector3.clone());

        let neighbors = vector_space.nearest_neighbors(&key1, 2);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&(key2.clone(), vector2.clone())));
        assert!(neighbors.contains(&(key3.clone(), vector3.clone())));

        let query = Query::Remove(key1.clone());
        let result = query.execute(&mut database);
        assert!(result.is_ok());
        memory.deallocate(vector1.len() * std::mem::size_of::<f32>());

        let query = Query::Get(key1);
        let result = query.execute(&mut database);
        assert_eq!(result, Err(String::from("Key not found")));
    }
}
