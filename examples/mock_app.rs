use rand::{seq::SliceRandom, Rng};
use rust_knn::{k_nearest_neighbour_sort, upload_strings};
use std::time::Instant;

// Function to generate random product names
fn generate_product_names(count: usize) -> Vec<String> {
    let adjectives = vec![
        "premium",
        "deluxe",
        "luxury",
        "elegant",
        "classic",
        "modern",
        "vintage",
        "smart",
        "portable",
        "compact",
        "lightweight",
        "durable",
        "robust",
        "advanced",
        "professional",
        "digital",
        "analog",
        "wireless",
        "wired",
        "electronic",
        "mechanical",
        "organic",
        "natural",
        "synthetic",
        "handcrafted",
        "innovative",
        "traditional",
        "custom",
        "standard",
        "special",
        "exclusive",
        "essential",
        "ultimate",
        "extreme",
        "practical",
        "functional",
        "ergonomic",
        "stylish",
        "sleek",
        "sporty",
        "rugged",
        "tactical",
        "refined",
        "enhanced",
        "improved",
        "portable",
        "heavy-duty",
        "basic",
        "high-end",
        "budget",
        "affordable",
        "valuable",
    ];

    let nouns = vec![
        "device",
        "gadget",
        "tool",
        "appliance",
        "system",
        "equipment",
        "accessory",
        "component",
        "product",
        "solution",
        "machine",
        "instrument",
        "apparatus",
        "mechanism",
        "contraption",
        "technology",
        "innovation",
        "invention",
        "creation",
        "design",
        "package",
        "set",
        "kit",
        "collection",
        "assortment",
        "bundle",
        "pack",
        "container",
        "box",
        "case",
        "holder",
        "organizer",
        "carrier",
        "bag",
        "pouch",
        "wallet",
        "purse",
        "backpack",
        "laptop",
        "tablet",
        "phone",
        "watch",
        "camera",
        "headphones",
        "speaker",
        "charger",
        "cable",
        "adapter",
        "monitor",
        "keyboard",
        "mouse",
        "controller",
        "console",
        "processor",
    ];

    let categories = vec![
        "pro",
        "lite",
        "mini",
        "max",
        "ultra",
        "plus",
        "premium",
        "elite",
        "executive",
        "signature",
        "limited",
        "special",
        "exclusive",
        "classic",
        "essential",
        "standard",
        "basic",
        "advanced",
        "professional",
        "home",
        "office",
        "travel",
        "outdoor",
        "sport",
        "gaming",
        "entertainment",
        "media",
        "creator",
        "developer",
        "designer",
        "artist",
        "studio",
        "workshop",
        "laboratory",
        "industrial",
        "commercial",
        "enterprise",
        "business",
        "personal",
        "family",
        "kids",
        "teen",
        "adult",
        "senior",
        "universal",
        "global",
        "local",
        "urban",
        "rural",
        "tactical",
        "strategic",
    ];

    let brands = vec![
        "techno",
        "nova",
        "nexus",
        "vertex",
        "apex",
        "zenith",
        "pinnacle",
        "summit",
        "horizon",
        "vortex",
        "quantum",
        "matrix",
        "spectrum",
        "fusion",
        "synergy",
        "echo",
        "pulse",
        "wave",
        "flux",
        "current",
        "core",
        "element",
        "atom",
        "particle",
        "vector",
        "origin",
        "source",
        "essence",
        "vitality",
        "vigor",
        "force",
        "power",
        "energy",
        "momentum",
        "drive",
        "thrust",
        "velocity",
        "speed",
        "pace",
        "tempo",
        "rhythm",
        "harmony",
        "balance",
        "unity",
        "alliance",
        "collective",
        "group",
        "team",
        "squad",
        "ensemble",
        "symphony",
        "orchestra",
        "band",
    ];

    let mut rng = rand::thread_rng();
    let mut names = Vec::with_capacity(count);

    for _ in 0..count {
        let name = format!(
            "{} {} {} {}",
            adjectives.choose(&mut rng).unwrap(),
            nouns.choose(&mut rng).unwrap(),
            categories.choose(&mut rng).unwrap(),
            brands.choose(&mut rng).unwrap()
        );
        names.push(name);
    }

    // Add some names with swapped word order for testing
    if count >= 100 {
        for i in 0..50 {
            if i < names.len() {
                let parts: Vec<&str> = names[i].split_whitespace().collect();
                if parts.len() == 4 {
                    let swapped_name =
                        format!("{} {} {} {}", parts[1], parts[0], parts[2], parts[3]);
                    names.push(swapped_name);
                }
            }
        }
    }

    names
}

fn main() {
    println!("Generating 7000 random product names...");
    let start_time = Instant::now();
    let product_names = generate_product_names(7000);
    let generation_time = start_time.elapsed();
    println!(
        "Generated {} product names in {:?}",
        product_names.len(),
        generation_time
    );

    // Upload strings to our library
    let upload_start = Instant::now();
    match upload_strings(product_names) {
        Ok(_) => {
            let upload_time = upload_start.elapsed();
            println!("Successfully uploaded strings in {:?}", upload_time);
        }
        Err(e) => {
            eprintln!("Error uploading strings: {}", e);
            return;
        }
    }

    // Display some example product names
    println!("\nExample product names:");
    for _ in 0..5 {
        let index = rand::thread_rng().gen_range(0..7000);
        let example =
            k_nearest_neighbour_sort(index.to_string(), 1, Some(true)).unwrap_or_default();
        if !example.is_empty() {
            println!("  {}", example[0]);
        }
    }

    // Test word order sensitivity with specific examples
    println!("\n===== WORD ORDER SENSITIVITY DEMONSTRATION =====");

    let word_order_tests = [
        "premium device pro techno",
        "device premium techno pro",
        "digital headphones gaming wave",
        "headphones digital wave gaming",
    ];

    for &query in &word_order_tests {
        println!("\nQuery: \"{}\"", query);

        // With word order sensitivity (default Levenshtein)
        println!("\n[Word Order Sensitive Search]");
        test_knn(query, 5, true);

        // Without word order sensitivity (word set comparison)
        println!("\n[Word Order Independent Search]");
        test_knn(query, 5, false);
    }

    // Test with larger k values and timing
    println!("\n===== PERFORMANCE TESTING =====");

    let queries = [
        "premium device pro techno",
        "digital component basic nexus",
        "elegant watch signature zenith",
        "portable speaker elite wave",
        "wireless headphones gaming pulse",
    ];

    let k_values = [20, 100];

    for query in &queries {
        println!("\n---------------------------------------------------");
        println!("Query: \"{}\"", query);

        println!("\n[Word Order Sensitive]");
        for &k in &k_values {
            test_knn(query, k, true);
        }

        println!("\n[Word Order Independent]");
        for &k in &k_values {
            test_knn(query, k, false);
        }
    }
}

fn test_knn(query: &str, k: u32, word_order_sensitive: bool) {
    let start_time = Instant::now();

    match k_nearest_neighbour_sort(query.to_string(), k, Some(word_order_sensitive)) {
        Ok(results) => {
            let elapsed = start_time.elapsed();
            let micros = elapsed.as_micros();

            println!(
                "\nK={}: Found {} results in {} microseconds ({:.2} ms)",
                k,
                results.len(),
                micros,
                micros as f64 / 1000.0
            );

            println!("Top 5 results:");
            for (i, result) in results.iter().enumerate().take(5) {
                println!("  {}. {}", i + 1, result);
            }

            if results.len() > 5 {
                println!("  ... and {} more results", results.len() - 5);
            }
        }
        Err(e) => {
            eprintln!("Error in KNN search: {}", e);
        }
    }
}
