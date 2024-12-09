use std::io::{stdin, stdout, Write};

fn main() {
    let num_rows = get_user_input("Enter the number of rows (default 9): ", 9);
    let num_balls = get_user_input("Enter the number of balls (default 1028): ", 1028);

    let bins = simulate_galton_board(num_rows, num_balls);
    let (mean, std_deviation, variance): (f64, f64, f64) = calculate_statistics(&bins);

    println!("Mean: {:.2}", mean);
    println!("Standard Deviation: {:.2}", std_deviation);
    println!("Variance: {:.2}", variance);

    println!(
        "Galton Board Simulation with {} balls and {} rows:",
        num_balls, num_rows
    );

    visualize_distribution(&bins, mean, std_deviation);
}

fn get_user_input(prompt: &str, default: usize) -> usize {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    input
        .trim()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Invalid input. Using default: {}", default);
            default
        })
}

fn simulate_galton_board(num_rows: usize, num_balls: usize) -> Vec<usize> {
    let mut bins = vec![0; num_rows + 1];
    for _ in 0..num_balls {
        let mut bin_index = 0;
        for _ in 0..num_rows {
            if rand::random() {
                bin_index += 1;
            }
        }
        bins[bin_index] += 1;
    }
    bins
}

fn calculate_statistics(bins: &[usize]) -> (f64, f64, f64) {
    let total_balls: usize = bins.iter().sum();
    let mean: f64 = if total_balls == 0 {
        0.0
    } else {
        bins.iter()
            .enumerate()
            .map(|(i, count)| (i as f64) * (*count as f64))
            .sum::<f64>() / (total_balls as f64)
    };

    let variance: f64 = if total_balls == 0 {
        0.0
    } else {
        bins.iter()
            .enumerate()
            .map(|(i, count)| ((i as f64) - mean).powi(2) * (*count as f64))
            .sum::<f64>() / (total_balls as f64)
    };

    let std_deviation = variance.sqrt();

    (mean, std_deviation, variance)
}

fn visualize_distribution(bins: &[usize], mean: f64, std_deviation: f64) {
    let max_count = bins.iter().max().unwrap_or(&0);
    let max_bar_width = 40;

    for (row_index, &count) in bins.iter().enumerate() {
        let num_chars = if *max_count > 0 {
            (count as f64 / *max_count as f64 * max_bar_width as f64) as usize
        } else {
            0
        };

        let is_outside_std_dev = (row_index as f64) < (mean - 1.5 * std_deviation)
            || (row_index as f64) > (mean + 1.5 * std_deviation);

        let final_bar = (0..num_chars)
            .map(|_| if is_outside_std_dev { '~' } else { '#' })
            .collect::<String>();

        println!("{:2}: {} ({})", row_index, final_bar, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_galton_board() {
        let bins = simulate_galton_board(5, 100);
        assert_eq!(bins.len(), 6); 
        assert_eq!(bins.iter().sum::<usize>(), 100);
    }

    #[test]
    fn test_calculate_statistics() {
        let bins = vec![1, 4, 6, 4, 1];
        let (mean, std_deviation, variance) = calculate_statistics(&bins);
        assert!((mean - 2.0).abs() < 1e-10);
        assert!((std_deviation - 1.0).abs() < 1e-10);
        assert!((variance - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_get_user_input_valid() {
        let input = "5";
        let mut user_input = input.as_bytes();
        std::io::stdin().read_from(&mut user_input).unwrap(); 
        let result = get_user_input("Test prompt: ", 10);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_get_user_input_invalid() {
        let input = "abc";
        let mut user_input = input.as_bytes();
        std::io::stdin().read_from(&mut user_input).unwrap(); 
        let result = get_user_input("Test prompt: ", 10);
        assert_eq!(result, 10);
    }
}
