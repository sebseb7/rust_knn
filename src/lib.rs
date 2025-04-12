use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StringData {
    content: String,
}

// Global storage for strings
lazy_static::lazy_static! {
    static ref STRINGS: Mutex<Vec<StringData>> = Mutex::new(Vec::new());
}

// Calculate the Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    // Initialize the first column
    (0..=len1).for_each(|i| matrix[i][0] = i);

    // Initialize the first row
    (0..=len2).for_each(|j| matrix[0][j] = j);

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };

            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1, // deletion
                    matrix[i + 1][j] + 1, // insertion
                ),
                matrix[i][j] + cost, // substitution
            );
        }
    }

    matrix[len1][len2]
}

// Calculate word-order-independent distance between two strings
fn word_independent_distance(s1: &str, s2: &str) -> usize {
    // Split strings into words
    let words1: HashSet<_> = s1.split_whitespace().collect();
    let words2: HashSet<_> = s2.split_whitespace().collect();

    // Calculate Jaccard distance: 1 - (intersection / union)
    let intersection_size = words1.intersection(&words2).count();
    let union_size = words1.union(&words2).count();

    if union_size == 0 {
        return 0; // Both strings are empty
    }

    // Convert to a distance (higher is more different)
    let jaccard_similarity = intersection_size as f64 / union_size as f64;
    let jaccard_distance = 1.0 - jaccard_similarity;

    // Scale and round to make it comparable to Levenshtein
    (jaccard_distance * 100.0).round() as usize
}

pub fn upload_strings(strings: Vec<String>) -> Result<(), String> {
    let mut data = STRINGS
        .lock()
        .map_err(|_| "Failed to acquire lock".to_string())?;

    for string in strings {
        data.push(StringData { content: string });
    }

    Ok(())
}

pub fn k_nearest_neighbour_sort(
    query: String,
    k: u32,
    word_order_sensitive: Option<bool>,
) -> Result<Vec<String>, String> {
    let word_order_sensitive = word_order_sensitive.unwrap_or(true);
    let data = STRINGS
        .lock()
        .map_err(|_| "Failed to acquire lock".to_string())?;

    if data.is_empty() {
        return Ok(Vec::new());
    }

    // Calculate distances for each string
    let mut distances: Vec<(usize, &StringData)> = data
        .iter()
        .map(|item| {
            let distance = if word_order_sensitive {
                levenshtein_distance(&query, &item.content)
            } else {
                word_independent_distance(&query, &item.content)
            };
            (distance, item)
        })
        .collect();

    // Sort by distance (ascending)
    distances.sort_by(|a, b| a.0.cmp(&b.0));

    // Take the k nearest
    let k = std::cmp::min(k as usize, distances.len());
    let result: Vec<String> = distances[0..k]
        .iter()
        .map(|(_, item)| item.content.clone())
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("apple", "aple"), 1);
    }

    #[test]
    fn test_word_independent_distance() {
        assert_eq!(word_independent_distance("", ""), 0);
        assert_eq!(word_independent_distance("word1 word2", "word1 word2"), 0);
        assert_eq!(word_independent_distance("word1 word2", "word2 word1"), 0);

        // With word1 word2 word3 and word1 word2 word4
        // Intersection: word1, word2 (2 words)
        // Union: word1, word2, word3, word4 (4 words)
        // Similarity: 2/4 = 0.5
        // Distance: 1 - 0.5 = 0.5 * 100 = 50
        let distance = word_independent_distance("word1 word2 word3", "word1 word2 word4");
        assert_eq!(distance, 50);

        // With word1 word2 and word1 word3
        // Intersection: word1 (1 word)
        // Union: word1, word2, word3 (3 words)
        // Similarity: 1/3 = 0.33
        // Distance: 1 - 0.33 = 0.67 * 100 = 67
        let distance2 = word_independent_distance("word1 word2", "word1 word3");
        assert_eq!(distance2, 67);
    }

    #[test]
    fn test_upload_and_search() {
        // Reset STRINGS for this test
        {
            let mut data = STRINGS.lock().unwrap();
            *data = Vec::new();
        }

        // Upload test data
        let test_data = vec![
            "apple".to_string(),
            "banana".to_string(),
            "orange".to_string(),
            "pear".to_string(),
            "apricot".to_string(),
        ];

        upload_strings(test_data).unwrap();

        // Test word-order sensitive (Levenshtein) search
        let sensitive_results =
            k_nearest_neighbour_sort("aple".to_string(), 2, Some(true)).unwrap();
        assert_eq!(sensitive_results.len(), 2);
        assert_eq!(sensitive_results[0], "apple");

        // Test word-order independent search
        let word_independent_results =
            k_nearest_neighbour_sort("premium device techno".to_string(), 2, Some(false)).unwrap();
        assert_eq!(word_independent_results.len(), 2);
    }
}
