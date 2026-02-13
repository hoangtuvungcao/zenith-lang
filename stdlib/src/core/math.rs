//! Zenith Math Module
//! Mathematical functions for Zenith standard library

use std::f64;

/// Basic mathematical operations
pub struct Math;

impl Math {
    /// Absolute value
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }

    /// Round to nearest integer
    pub fn round(x: f64) -> f64 {
        x.round()
    }

    /// Round down to nearest integer
    pub fn floor(x: f64) -> f64 {
        x.floor()
    }

    /// Round up to nearest integer
    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }

    /// Truncate decimal part
    pub fn trunc(x: f64) -> f64 {
        x.trunc()
    }

    /// Minimum of two values
    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }

    /// Maximum of two values
    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    /// Power function
    pub fn pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }

    /// Square root
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    /// Cube root
    pub fn cbrt(x: f64) -> f64 {
        x.cbrt()
    }

    /// Natural logarithm
    pub fn ln(x: f64) -> f64 {
        x.ln()
    }

    /// Base-10 logarithm
    pub fn log10(x: f64) -> f64 {
        x.log10()
    }

    /// Base-2 logarithm
    pub fn log2(x: f64) -> f64 {
        x.log2()
    }

    /// Exponential function
    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    /// Sine function
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    /// Cosine function
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    /// Tangent function
    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    /// Arcsine function
    pub fn asin(x: f64) -> f64 {
        x.asin()
    }

    /// Arccosine function
    pub fn acos(x: f64) -> f64 {
        x.acos()
    }

    /// Arctangent function
    pub fn atan(x: f64) -> f64 {
        x.atan()
    }

    /// Hyperbolic sine
    pub fn sinh(x: f64) -> f64 {
        x.sinh()
    }

    /// Hyperbolic cosine
    pub fn cosh(x: f64) -> f64 {
        x.cosh()
    }

    /// Hyperbolic tangent
    pub fn tanh(x: f64) -> f64 {
        x.tanh()
    }

    /// Degrees to radians
    pub fn to_radians(degrees: f64) -> f64 {
        degrees.to_radians()
    }

    /// Radians to degrees
    pub fn to_degrees(radians: f64) -> f64 {
        radians.to_degrees()
    }
}

/// Trigonometry utilities
pub struct Trigonometry;

impl Trigonometry {
    /// Calculate angle from opposite and adjacent
    pub fn atan2(opposite: f64, adjacent: f64) -> f64 {
        opposite.atan2(adjacent)
    }

    /// Calculate hypotenuse
    pub fn hypot(a: f64, b: f64) -> f64 {
        a.hypot(b)
    }

    /// Convert degrees to radians with modulo 360
    pub fn normalize_angle_degrees(angle: f64) -> f64 {
        angle % 360.0
    }

    /// Convert radians to modulo 2π
    pub fn normalize_angle_radians(angle: f64) -> f64 {
        angle % (2.0 * std::f64::consts::PI)
    }
}

/// Statistics functions
pub struct Statistics;

impl Statistics {
    /// Calculate mean of a slice
    pub fn mean(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            None
        } else {
            let sum: f64 = values.iter().sum();
            Some(sum / values.len() as f64)
        }
    }

    /// Calculate median of a slice
    pub fn median(values: &mut [f64]) -> Option<f64> {
        if values.is_empty() {
            None
        } else {
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let len = values.len();

            if len % 2 == 0 {
                Some((values[len / 2 - 1] + values[len / 2]) / 2.0)
            } else {
                Some(values[len / 2])
            }
        }
    }

    /// Calculate variance
    pub fn variance(values: &[f64]) -> Option<f64> {
        if let Some(mean) = Statistics::mean(values) {
            let sum_squared_diff: f64 = values.iter().map(|x| (x - mean).powi(2)).sum();
            Some(sum_squared_diff / values.len() as f64)
        } else {
            None
        }
    }

    /// Calculate standard deviation
    pub fn std_deviation(values: &[f64]) -> Option<f64> {
        Statistics::variance(values).map(|var| var.sqrt())
    }

    /// Calculate range (max - min)
    pub fn range(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            None
        } else {
            let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            Some(max - min)
        }
    }

    /// Calculate percentile
    pub fn percentile(values: &mut [f64], percentile: f64) -> Option<f64> {
        if values.is_empty() || percentile < 0.0 || percentile > 100.0 {
            None
        } else {
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = ((values.len() - 1) as f64 * (percentile / 100.0)).round() as usize;
            if index >= values.len() {
                Some(values[values.len() - 1])
            } else {
                Some(values[index])
            }
        }
    }
}

/// Number theory functions
pub struct NumberTheory;

impl NumberTheory {
    /// Check if number is prime
    pub fn is_prime(n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    /// Get greatest common divisor
    pub fn gcd(a: u64, b: u64) -> u64 {
        let mut a = a;
        let mut b = b;

        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// Get least common multiple
    pub fn lcm(a: u64, b: u64) -> u64 {
        a / NumberTheory::gcd(a, b) * b
    }

    /// Generate Fibonacci sequence up to n
    pub fn fibonacci(n: usize) -> Vec<u64> {
        if n == 0 {
            return Vec::new();
        }

        let mut fib = vec![0, 1];

        while fib.len() < n {
            let next = fib[fib.len() - 1] + fib[fib.len() - 2];
            fib.push(next);
        }

        fib
    }

    /// Check if number is in Fibonacci sequence
    pub fn is_fibonacci(n: u64) -> bool {
        let mut a = 0;
        let mut b = 1;

        while b < n {
            let temp = a + b;
            a = b;
            b = temp;
        }

        n == a || n == b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert_eq!(Math::abs(-5.0), 5.0);
        assert_eq!(Math::round(3.7), 4.0);
        assert_eq!(Math::floor(3.7), 3.0);
        assert_eq!(Math::ceil(3.2), 4.0);
        assert_eq!(Math::trunc(3.7), 3.0);
        assert_eq!(Math::min(2.0, 5.0), 2.0);
        assert_eq!(Math::max(2.0, 5.0), 5.0);
        assert!((Math::pow(2.0, 3.0) - 8.0).abs() < f64::EPSILON);
        assert_eq!(Math::sqrt(16.0), 4.0);
        assert_eq!(Math::cbrt(27.0), 3.0);
        assert!((Math::ln(std::f64::consts::E) - 1.0).abs() < f64::EPSILON);
        assert!((Math::log10(100.0) - 2.0).abs() < f64::EPSILON);
        assert!((Math::log2(8.0) - 3.0).abs() < f64::EPSILON);
        assert!((Math::exp(1.0) - std::f64::consts::E).abs() < f64::EPSILON);
    }

    #[test]
    fn test_trigonometry() {
        assert!((Math::sin(0.0) - 0.0).abs() < f64::EPSILON);
        assert!((Math::cos(0.0) - 1.0).abs() < f64::EPSILON);
        assert!((Math::tan(0.0) - 0.0).abs() < f64::EPSILON);
        assert!((Math::asin(0.0) - 0.0).abs() < f64::EPSILON);
        assert!((Math::acos(1.0) - 0.0).abs() < f64::EPSILON);
        assert!((Math::atan(0.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_statistics() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(Statistics::mean(&values), Some(3.0));

        let mut values2 = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(Statistics::median(&mut values2), Some(2.5));

        assert!((Statistics::variance(&values).unwrap() - 2.0).abs() < f64::EPSILON);
        assert!(
            (Statistics::std_deviation(&values).unwrap() - 1.4142135623730951).abs() < f64::EPSILON
        );
        assert_eq!(Statistics::range(&values), Some(4.0));

        let mut values4 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(Statistics::percentile(&mut values4, 50.0), Some(3.0));
    }

    #[test]
    fn test_number_theory() {
        assert!(NumberTheory::is_prime(2));
        assert!(NumberTheory::is_prime(3));
        assert!(NumberTheory::is_prime(5));
        assert!(!NumberTheory::is_prime(4));
        assert!(!NumberTheory::is_prime(1));

        assert_eq!(NumberTheory::gcd(48, 18), 6);
        assert_eq!(NumberTheory::lcm(12, 18), 36);

        let fib = NumberTheory::fibonacci(10);
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);

        assert!(NumberTheory::is_fibonacci(34));
        assert!(NumberTheory::is_fibonacci(35));
        assert!(!NumberTheory::is_fibonacci(36));
    }
}
