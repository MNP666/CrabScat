#[derive(Clone, Copy, Debug)]
pub struct FitQuality {
    pub chi_squared: f64,
    pub reduced_chi_squared: f64,
    pub degrees_of_freedom: usize,
}
