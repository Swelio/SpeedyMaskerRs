#![deny(clippy::all)]

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const SPECIAL_CHARSET: &str = "! \"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
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
    pub size: usize,
    pub count: usize,
    pub cost: f64,
}

impl Display for ComputedMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mask)
    }
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
        } else if SPECIAL_CHARSET.contains(char) {
            mask.push('s');
        } else {
            return Err(MaskError::InvalidCharacter(char));
        }
    }

    Ok(mask)
}

fn compute_mask_size(mask: &str, maximum_size: usize) -> Option<usize> {
    let mut result = 1;

    for char in mask.chars() {
        let multiplier = match char {
            'l' => 26,
            'u' => 26,
            'd' => 10,
            's' => SPECIAL_CHARSET.len(),
            _ => panic!("unknown mask char '{}'", char),
        };

        if (maximum_size / multiplier) < result {
            return None;
        }

        result *= multiplier;
    }

    Some(result)
}

fn compute_mask_cost(mask_size: usize, occurrences_count: usize) -> f64 {
    (occurrences_count as f64) / (mask_size as f64)
}

pub fn generate_masks_from_bufreader<R>(line_reader: &mut R) -> io::Result<HashMap<String, usize>>
where
    R: BufRead,
{
    let mut masks_counts = HashMap::new();

    for word in line_reader.lines() {
        let word = match word {
            Ok(word) => word,
            Err(error) => return Err(error),
        };

        let mask = match generate_mask(&word) {
            Ok(mask) => mask,
            Err(_) => continue,
        };

        if !mask.is_empty() {
            *masks_counts.entry(mask).or_insert(0) += 1;
        }
    }

    Ok(masks_counts)
}

pub fn sort_masks(masks_counts: &HashMap<String, usize>, maximum_size: usize) -> Vec<ComputedMask> {
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

pub fn parse_file<P>(path: P, maximum_size: usize) -> io::Result<(Vec<ComputedMask>, usize)>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut file_reader = BufReader::new(file);
    let mask_map = generate_masks_from_bufreader(&mut file_reader)?;
    let mut used_space = 0;
    let sorted_masks = sort_masks(&mask_map, maximum_size)
        .into_iter()
        .filter(|mask| {
            if mask.size <= maximum_size - used_space {
                used_space += mask.size;
                return true;
            }
            false
        })
        .collect();

    Ok((sorted_masks, used_space))
}

#[cfg(test)]
mod lib_tests {
    use std::io::Cursor;
    use std::time::Instant;

    use super::{
        compute_mask_cost, compute_mask_size, generate_mask, generate_masks_from_bufreader,
        sort_masks,
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
        let mask_size = compute_mask_size(mask, usize::MAX).unwrap();
        assert_eq!(mask_size, 3670344486987776);
    }

    #[test]
    fn mask_cost() {
        let mask = "ullllulllll";
        let mask_size = compute_mask_size(mask, usize::MAX).unwrap();
        let mask_occurrences = 1000;
        let mask_cost = compute_mask_cost(mask_size, mask_occurrences);
        assert_eq!(mask_cost, 2.7245398995795416e-13);
    }

    #[test]
    fn masks_from_iterator() {
        let mut wordlist = Cursor::new(b"Hello\nFriend\nPassword\nP@$$w0rd");
        generate_masks_from_bufreader(&mut wordlist).unwrap();
    }

    #[test]
    fn sort_masks_list() {
        let mut wordlist = Cursor::new(b"Hello\nFriend\nPassword\nP@$$w0rd");
        let start_time = Instant::now();
        let mask_map = generate_masks_from_bufreader(&mut wordlist).unwrap();
        let mask_generation_duration = start_time.elapsed();
        let start_mask_sort = Instant::now();
        let mask_list = sort_masks(&mask_map, usize::MAX);
        let mask_sort_duration = start_mask_sort.elapsed();

        println!("Generation duration: {:?}", mask_generation_duration);
        println!("Sorting duration: {:?}", mask_sort_duration);

        assert_eq!(mask_list[0].mask, "ullll");
    }
}
