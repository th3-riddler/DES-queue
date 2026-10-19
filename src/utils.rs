pub const DEFAULT_VEC_SIZE: usize = 24;

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

// Compute P_k
pub fn p_k(lambda: f64, mu: f64, rho: f64, c: usize) -> [f64; DEFAULT_VEC_SIZE] {
    let mut v: [f64; DEFAULT_VEC_SIZE] = [1.0; DEFAULT_VEC_SIZE];
    v[0] = 1.0;

    let a: f64 = lambda / mu;

    for k in 1..DEFAULT_VEC_SIZE {
        if k <= c {
            v[k] = v[k - 1] * (a / k as f64);
        } else {
            v[k] = v[k - 1] * rho;
        }
    }

    let v_c = if c < DEFAULT_VEC_SIZE {
        v[c]
    } else {
        let mut curr: f64 = 1.0;

        for k in 0..c {
            curr *= a / k as f64;
        }

        curr
    };

    let c_buond: usize = c.min(DEFAULT_VEC_SIZE);
    let s = v[..c_buond].iter().sum::<f64>() + (v_c / (1.0 - rho));

    let p_0: f64 = 1.0 / s;

    v.map(|f| f * p_0)
}

// Compute L_q
#[inline]
pub fn l_q(lambda: f64, mu: f64, c: usize) -> f64 {
    let rho = lambda / (mu * c as f64);

    erlang_c(c, c as f64 * rho) * (rho / (1.0 - rho))
}

// Compute L_x
#[inline]
pub fn l_x(lambda: f64, mu: f64, c: usize) -> f64 {
    let rho = lambda / (mu * c as f64);

    c as f64 * rho
}

// Compute L_s
#[inline]
pub fn l_s(lambda: f64, mu: f64, c: usize) -> f64 {
    l_q(lambda, mu, c) + l_x(lambda, mu, c)
}

// Compute W_q
#[inline]
pub fn w_q(lambda: f64, mu: f64, c: usize) -> f64 {
    let mu_tot: f64 = c as f64 * mu;
    let rho: f64 = lambda / mu_tot;

    erlang_c(c, c as f64 * rho) / (mu_tot * (1.0 - rho))
}

// Compute W_s
#[inline]
pub fn w_s(lambda: f64, mu: f64, c: usize) -> f64 {
    let mu_tot = c as f64 * mu;
    let rho = lambda / mu_tot;

    (erlang_c(c, c as f64 * rho) + c as f64 * (1.0 - rho)) / (mu_tot * (1.0 - rho))
}
