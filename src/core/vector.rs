pub trait VectorOps {
    fn dot_product(&self, other: &Self) -> f32;
    fn euclidean_distance(&self, other: &Self) -> f32;
    fn cosine_similarity(&self, other: &Self) -> f32;
    fn normalize(&self) -> Self;
}

impl VectorOps for Vec<f32> {
    fn dot_product(&self, other: &Self) -> f32 {
        self.iter().zip(other).map(|(a, b)| a * b).sum()
    }

    fn euclidean_distance(&self, other: &Self) -> f32 {
        self.iter()
            .zip(other)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    fn cosine_similarity(&self, other: &Self) -> f32 {
        let dot_product = self.dot_product(other);
        let magnitude_a = self.iter().map(|a| a.powi(2)).sum::<f32>().sqrt();
        let magnitude_b = other.iter().map(|b| b.powi(2)).sum::<f32>().sqrt();

        dot_product / (magnitude_a * magnitude_b)
    }

    fn normalize(&self) -> Self {
        let magnitude = self.iter().map(|a| a.powi(2)).sum::<f32>().sqrt();
        self.iter().map(|a| a / magnitude).collect()
    }
}
