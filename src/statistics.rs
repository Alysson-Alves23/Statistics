pub mod statistics{

    use std::iter::Sum;

    trait Numeric {
        fn to_f64(&self) -> f64;
    }

    impl Numeric for i32 {
        fn to_f64(&self) -> f64 {
            *self as f64
        }
    }

    impl Numeric for f64 {
        fn to_f64(&self) -> f64 {
            *self
        }
    }

    pub fn mean<T>(x: &[T]) -> f64
        where
            T: Numeric,
    {
        let count = x.len() as f64;
        x.iter().map(|val| val.to_f64()).sum() / count
    }


    pub fn geometric_mean<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;
    }
    pub fn harmonic_mean<T: Into<f64>>(x: &[T]) -> f64{

        return 0.0;
    }

    pub fn median<T: Into<f64>>(x: &[T]) -> f64{
    return 0.0;
    }
    pub fn median_low<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;
    }
    pub  fn median_high<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;
    }
    pub fn median_grouped<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;
    }
    // pub fn mode<T: Into<f64>>(x: &[T]) -> f64{
    //     return 0.0;
    // }
    // pub fn multimode<T: Into<f64>>(x: &[T]) -> f64{
    //     return 0.0;
    // }
    pub fn pstdev<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
    pub fn pvariance<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
    pub fn stdev<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
    pub fn variance<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
    pub fn quantiles<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
    pub fn covariance<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
    pub fn correlation<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
    pub fn linear_regression<T: Into<f64>>(x: &[T]) -> f64{
        return 0.0;

    }
}