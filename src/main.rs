use rayon::prelude::*;

#[rustfmt::skip]
const SBOX_HWS: [f32; 256] = [
    4.0, 5.0, 6.0, 6.0, 5.0, 5.0, 6.0, 4.0, 2.0, 1.0, 5.0, 4.0, 7.0, 6.0, 5.0, 5.0,
    4.0, 2.0, 4.0, 6.0, 6.0, 4.0, 4.0, 4.0, 5.0, 4.0, 3.0, 6.0, 4.0, 3.0, 4.0, 2.0,
    6.0, 7.0, 4.0, 3.0, 4.0, 6.0, 7.0, 4.0, 3.0, 4.0, 5.0, 5.0, 4.0, 4.0, 3.0, 3.0,
    1.0, 5.0, 3.0, 4.0, 2.0, 4.0, 2.0, 4.0, 3.0, 2.0, 1.0, 4.0, 6.0, 4.0, 4.0, 5.0,
    2.0, 3.0, 3.0, 3.0, 4.0, 5.0, 4.0, 2.0, 3.0, 5.0, 5.0, 5.0, 3.0, 5.0, 5.0, 2.0,
    4.0, 4.0, 0.0, 6.0, 1.0, 6.0, 4.0, 5.0, 4.0, 5.0, 6.0, 4.0, 3.0, 3.0, 3.0, 6.0,
    3.0, 7.0, 4.0, 7.0, 3.0, 4.0, 4.0, 3.0, 3.0, 6.0, 1.0, 7.0, 2.0, 4.0, 6.0, 3.0,
    3.0, 4.0, 1.0, 5.0, 3.0, 5.0, 3.0, 6.0, 5.0, 5.0, 5.0, 2.0, 1.0, 8.0, 6.0, 4.0,
    5.0, 2.0, 3.0, 5.0, 6.0, 5.0, 2.0, 4.0, 3.0, 5.0, 6.0, 5.0, 3.0, 5.0, 3.0, 5.0,
    2.0, 2.0, 5.0, 5.0, 2.0, 3.0, 2.0, 2.0, 3.0, 6.0, 4.0, 2.0, 6.0, 5.0, 3.0, 6.0,
    3.0, 3.0, 4.0, 2.0, 3.0, 2.0, 2.0, 4.0, 3.0, 5.0, 4.0, 3.0, 3.0, 4.0, 4.0, 5.0,
    6.0, 3.0, 5.0, 5.0, 4.0, 5.0, 4.0, 4.0, 4.0, 4.0, 5.0, 5.0, 4.0, 5.0, 5.0, 1.0,
    5.0, 4.0, 3.0, 4.0, 3.0, 4.0, 4.0, 4.0, 4.0, 6.0, 4.0, 5.0, 4.0, 6.0, 4.0, 3.0,
    3.0, 5.0, 5.0, 4.0, 2.0, 2.0, 6.0, 3.0, 3.0, 4.0, 5.0, 5.0, 3.0, 3.0, 4.0, 5.0,
    4.0, 5.0, 3.0, 2.0, 4.0, 5.0, 4.0, 3.0, 5.0, 4.0, 4.0, 5.0, 5.0, 4.0, 2.0, 7.0,
    3.0, 3.0, 3.0, 3.0, 7.0, 5.0, 2.0, 3.0, 2.0, 4.0, 4.0, 4.0, 3.0, 3.0, 6.0, 3.0
];

fn main() -> anyhow::Result<()> {
    let data: Measurements = include_str!("../data.txt").parse()?;
    data.gen_stats();
    let key = data.break_key();
    println!("{}", String::from_utf8_lossy(&key));

    Ok(())
}

#[derive(Default, Debug, Clone)]
struct Measurements {
    pts: [Vec<u8>; 32],
    vts: Vec<f32>,
}

impl Measurements {
    fn break_key(self) -> [u8; 32] {
        self.pts.map(|pt_pos_bytes| {
            (u8::MIN..=u8::MAX)
                .into_par_iter()
                .map(|guess| {
                    let hypot = pt_pos_bytes
                        .iter()
                        .map(|b| SBOX_HWS[(b ^ guess) as usize])
                        .collect::<Vec<_>>();

                    (guess, incremental_pearson_coeff(&hypot, &self.vts))
                })
                .max_by(|(_, corre1), (_, corre2)| corre1.total_cmp(corre2))
                .map(|(guess, _)| guess)
                .expect("Hmm, interestingggg....")
        })
    }

    fn gen_stats(&self) {
        for (i, pt) in self.pts.iter().enumerate() {
            for guess in u8::MIN..=u8::MAX {
                let hypot = pt
                    .iter()
                    .map(|&b| SBOX_HWS[(b ^ guess) as usize])
                    .collect::<Vec<_>>();

                let correlation = incremental_pearson_coeff(&hypot, &self.vts);
                println!("{i},{guess},{correlation}");
            }
        }
    }
}

impl std::str::FromStr for Measurements {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .filter_map(|s| {
                if s.contains("2") {
                    let (pt, vt) = s.split_once("\t")?;
                    let pt = hex::decode(pt).ok()?;
                    let vt = vt.parse().ok()?;
                    Some((pt, vt))
                } else {
                    None
                }
            })
            .fold(Default::default(), |mut acc: Measurements, (pt, vt)| {
                for (pt_vec, &pt_byte) in acc.pts.iter_mut().zip(pt.iter()) {
                    pt_vec.push(pt_byte);
                }
                acc.vts.push(vt);
                acc
            });

        Ok(data)
    }
}

fn incremental_pearson_coeff(x: &[f32], y: &[f32]) -> f32 {
    // implementation of the iterative update formulas from: https://crypto.fit.cvut.cz/sites/default/files/publications/fulltexts/pearson.pdf
    assert_eq!(x.len(), y.len());

    let [_, _, m2x, m2y, c2s] = x.iter().zip(y.iter()).enumerate().fold(
        [0.0_f32; 5],
        |[x_bar_, y_bar_, m2x_, m2y_, c2s_], (n, (&x, &y))| {
            let n = n as f32;

            // update the means
            let x_bar = x_bar_ + ((x - x_bar_) / (n + 1.0));
            let y_bar = y_bar_ + ((y - y_bar_) / (n + 1.0));

            // update the sums
            let m2x = m2x_ + (x - x_bar) * (x - x_bar_);
            let m2y = m2y_ + (y - y_bar) * (y - y_bar_);

            // update the covariance
            let c2s = c2s_ + (n / (n + 1.0)) * (x - x_bar_) * (y - y_bar_);

            [x_bar, y_bar, m2x, m2y, c2s]
        },
    );

    c2s / (m2x.sqrt() * m2y.sqrt())
}

fn parallel_pearson_coeff(x: &[f32], y: &[f32]) -> f32 {
    // implementation of the naive one pass algorithm from: https://crypto.fit.cvut.cz/sites/default/files/publications/fulltexts/pearson.pdf
    // this is too numerically unstable to use with f32 for many samples, but with f64 it can handle 2.4 million just fine
    let [xys, xs, xxs, ys, yys] = x
        .into_par_iter()
        .zip_eq(y.into_par_iter())
        .map(|(&x, &y)| [x * y, x, x * x, y, y * y])
        .reduce(
            || [0.0f32; 5],
            |mut acc, x| {
                for (a, b) in acc.iter_mut().zip(x.iter()) {
                    *a += b;
                }
                acc
            },
        );

    let n = x.len() as f32;
    (n * xys - xs * ys) / (f32::sqrt(n * xxs - xs * xs) * f32::sqrt(n * yys - ys * ys))
}
