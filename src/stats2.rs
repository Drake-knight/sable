fn sorted_copy(data: &[f64]) -> Vec<f64> {
    let mut v = data.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v
}

pub fn mean(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

pub fn variance(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let m = mean(data);
    let sum_sq: f64 = data.iter().map(|v| (v - m) * (v - m)).sum();
    sum_sq / data.len() as f64
}

pub fn stddev(data: &[f64]) -> f64 {
    variance(data).sqrt()
}

pub fn percentile(data: &[f64], p: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut pp = p;
    if pp.is_nan() {
        pp = 0.0;
    } else if pp < 0.0 {
        pp = 0.0;
    } else if pp > 100.0 {
        pp = 100.0;
    }
    let sorted = sorted_copy(data);
    let n = sorted.len();
    let rank = pp / 100.0 * (n as f64 - 1.0);
    let lower = rank.floor();
    let upper = rank.ceil();
    let mut lower_idx = lower as usize;
    let mut upper_idx = upper as usize;
    if lower_idx >= n {
        lower_idx = n - 1;
    }
    if upper_idx >= n {
        upper_idx = n - 1;
    }
    if lower_idx == upper_idx {
        return sorted[lower_idx];
    }
    let frac = rank - lower;
    sorted[lower_idx] + (sorted[upper_idx] - sorted[lower_idx]) * frac
}

pub fn median(data: &[f64]) -> f64 {
    percentile(data, 50.0)
}

pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0.0;
    }
    let xs = &x[..n];
    let ys = &y[..n];
    let mx = mean(xs);
    let my = mean(ys);
    let mut sum = 0.0;
    for i in 0..n {
        sum += (xs[i] - mx) * (ys[i] - my);
    }
    sum / n as f64
}

pub fn correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0.0;
    }
    let xs = &x[..n];
    let ys = &y[..n];
    let sx = stddev(xs);
    let sy = stddev(ys);
    if sx == 0.0 || sy == 0.0 {
        return 0.0;
    }
    let cov = covariance(xs, ys);
    let result = cov / (sx * sy);
    if result.is_nan() || result.is_infinite() {
        return 0.0;
    }
    result
}

pub fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len().min(y.len());
    if n == 0 {
        return (0.0, 0.0);
    }
    let xs = &x[..n];
    let ys = &y[..n];
    let mx = mean(xs);
    let my = mean(ys);
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        num += (xs[i] - mx) * (ys[i] - my);
        den += (xs[i] - mx) * (xs[i] - mx);
    }
    if den == 0.0 {
        return (0.0, 0.0);
    }
    let slope = num / den;
    let intercept = my - slope * mx;
    if slope.is_nan() || slope.is_infinite() || intercept.is_nan() || intercept.is_infinite() {
        return (0.0, 0.0);
    }
    (slope, intercept)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn test_mean_median_variance() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(approx_eq(mean(&data), 3.0, 1e-9));
        assert!(approx_eq(median(&data), 3.0, 1e-9));
        assert!(approx_eq(variance(&data), 2.0, 1e-9));
        assert!(approx_eq(stddev(&data), 2.0f64.sqrt(), 1e-9));
    }

    #[test]
    fn test_percentile_matches_median() {
        let data = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        assert!(approx_eq(percentile(&data, 50.0), median(&data), 1e-9));
    }

    #[test]
    fn test_correlation_perfect_linear() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        assert!(approx_eq(correlation(&x, &y), 1.0, 1e-9));
    }

    #[test]
    fn test_linear_regression_recovers_params() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y: Vec<f64> = x.iter().map(|v| 3.0 * v + 2.0).collect();
        let (slope, intercept) = linear_regression(&x, &y);
        assert!(approx_eq(slope, 3.0, 1e-9));
        assert!(approx_eq(intercept, 2.0, 1e-9));
    }

    #[test]
    fn test_empty_inputs_return_defaults() {
        let empty: Vec<f64> = Vec::new();
        assert_eq!(mean(&empty), 0.0);
        assert_eq!(variance(&empty), 0.0);
        assert_eq!(stddev(&empty), 0.0);
        assert_eq!(percentile(&empty, 50.0), 0.0);
        assert_eq!(median(&empty), 0.0);
        assert_eq!(covariance(&empty, &empty), 0.0);
        assert_eq!(correlation(&empty, &empty), 0.0);
        assert_eq!(linear_regression(&empty, &empty), (0.0, 0.0));
    }

    #[test]
    fn test_percentile_clamps_p() {
        let data = vec![10.0, 20.0, 30.0];
        assert!(approx_eq(percentile(&data, -50.0), 10.0, 1e-9));
        assert!(approx_eq(percentile(&data, 500.0), 30.0, 1e-9));
    }
}
