#[derive(Debug, PartialEq, Eq)]
pub enum FactorialError {
    Overflow,
}

// Stable version of b-erlang function:
// B(0, A) = 1
// B(k, A) = A * B(k - 1, A) / (k + A * B(k - 1, A))
pub fn erlang_b(y: usize, a: f64) -> f64 {
    let mut b: f64 = 1.0;

    for k in 1..=y {
        b = a * b / (k as f64 + a * b);
    }

    b
}

// Stable version of c-erlang function:
// B(0, A) = 1
// B(k, A) = A * B(k - 1, A) / (k + A * B(k - 1, A))
//
// C(c, A) = B / (1 - (A / c) * (1 - B))
pub fn erlang_c(c: usize, a: f64) -> f64 {
    if a >= c as f64 {
        return 1.0;
    }

    let b = erlang_b(c, a);
    let rho = a / c as f64;

    b / (1.0 - rho + rho * b)
}

// Compute L_q
#[inline]
pub fn compute_l_q(lambda: f64, mu: f64, c: usize) -> f64 {
    let rho = lambda / (mu * c as f64);

    erlang_c(c, c as f64 * rho) * (rho / (1.0 - rho))
}

// Compute L_x
#[inline]
pub fn compute_l_x(lambda: f64, mu: f64, c: usize) -> f64 {
    let rho = lambda / (mu * c as f64);

    c as f64 * rho
}

// Compute L_s
#[inline]
pub fn compute_l_s(lambda: f64, mu: f64, c: usize) -> f64 {
    compute_l_q(lambda, mu, c) + compute_l_x(lambda, mu, c)
}

// Compute W_q
#[inline]
pub fn compute_w_q(lambda: f64, mu: f64, c: usize) -> f64 {
    let mu_tot: f64 = c as f64 * mu;
    let rho: f64 = lambda / mu_tot;

    erlang_c(c, c as f64 * rho) / (mu_tot * (1.0 - rho))
}

// Compute W_s
#[inline]
pub fn compute_w_s(lambda: f64, mu: f64, c: usize) -> f64 {
    let mu_tot = c as f64 * mu;
    let rho = lambda / mu_tot;

    (erlang_c(c, c as f64 * rho) + c as f64 * (1.0 - rho)) / (mu_tot * (1.0 - rho))
}
