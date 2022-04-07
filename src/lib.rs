#![deny(clippy::all)]

use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum MaskError {
    InvalidCharacter(char),
}

impl Display for MaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MaskError::InvalidCharacter(bad_char) => {
                write!(f, "invalid character '{}'", bad_char)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComputedMask {
    pub mask: String,
    pub size: u64,
    pub count: u64,
    pub cost: f64,
}

fn generate_mask(word: &str) -> Result<String, MaskError> {
    let mut mask = String::with_capacity(word.len());

    for char in word.chars() {
        if char.is_ascii_lowercase() {
            mask.push('l');
        } else if char.is_ascii_uppercase() {
            mask.push('u');
        } else if char.is_ascii_digit() {
            mask.push('d');
        } else if "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".contains(char) {
            mask.push('s');
        } else {
            return Err(MaskError::InvalidCharacter(char));
        }
    }

    Ok(mask)
}

fn compute_mask_size(mask: &str, maximum_size: u64) -> Option<u64> {
    let mut result = 1;

    for char in mask.chars() {
        let multiplier = match char {
            'l' => 26,
            'u' => 26,
            'd' => 10,
            's' => 32,
            _ => panic!("unknown mask char '{}'", char),
        };

        if (maximum_size / multiplier) < result {
            return None;
        } else {
            result *= multiplier;
        }
    }

    Some(result)
}

fn compute_mask_cost(mask_size: u64, occurrences_count: u64) -> f64 {
    (occurrences_count as f64) / (mask_size as f64)
}

fn generate_masks_from_list(wordlist: &[&str]) -> HashMap<String, u64> {
    let mut masks_counts = HashMap::new();

    for &word in wordlist {
        let mask = generate_mask(word).unwrap();
        *masks_counts.entry(mask).or_insert(0) += 1;
    }

    masks_counts
}

fn sort_masks(masks_counts: &HashMap<String, u64>, maximum_size: u64) -> Vec<ComputedMask> {
    let mut sorted_masks = Vec::with_capacity(masks_counts.len());

    for (mask, &mask_count) in masks_counts {
        let mask_size = match compute_mask_size(mask, maximum_size) {
            Some(mask_size) => mask_size,
            None => continue, // mask is too big
        };
        let mask_cost = compute_mask_cost(mask_size, mask_count);
        sorted_masks.push(ComputedMask {
            mask: mask.clone(),
            size: mask_size,
            count: mask_count,
            cost: mask_cost,
        });
    }

    sorted_masks.sort_by(|mask_0, mask_1| mask_1.cost.partial_cmp(&mask_0.cost).unwrap());
    sorted_masks
}

#[cfg(test)]
mod lib_tests {
    use super::{
        compute_mask_cost, compute_mask_size, generate_mask, generate_masks_from_list, sort_masks,
    };

    #[test]
    fn mask_generation() {
        let word = "HelloFriend";
        let mask = generate_mask(word).unwrap();
        assert_eq!(mask, "ullllulllll");
    }

    #[test]
    fn mask_size_computation() {
        let mask = "ullllulllll";
        let mask_size = compute_mask_size(mask, u64::MAX).unwrap();
        assert_eq!(mask_size, 3670344486987776);
    }

    #[test]
    fn mask_cost() {
        let mask = "ullllulllll";
        let mask_size = compute_mask_size(mask, u64::MAX).unwrap();
        let mask_occurrences = 1000;
        let mask_cost = compute_mask_cost(mask_size, mask_occurrences);
        assert_eq!(mask_cost, 2.7245398995795416e-13);
    }

    #[test]
    fn masks_from_list() {
        let wordlist = vec!["Hello", "Friend", "Password", "P@$$w0rd"];
        let mask_map = generate_masks_from_list(&wordlist);
    }

    #[test]
    fn sort_masks_list() {
        let wordlist = vec!["Hello", "Hello", "Friend", "Password", "P@$$w0rd"];
        let mask_map = generate_masks_from_list(&wordlist);
        let mask_list = sort_masks(&mask_map, u64::MAX);

        assert_eq!(mask_list[0].mask, "ullll");
    }
}
