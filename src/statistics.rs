
pub mod statistics{
    use crate::statistics::utils::qs::quick_select;

    pub trait Numeric {
        fn to_f64(&self) -> f64;
        fn pow(&self, x:u32) -> f64;
    }

    impl Numeric for i32 {
        fn to_f64(&self) -> f64 {
            *self as f64
        }
        fn pow(&self,x:u32) -> f64{ (*self as u32).pow(x) as f64 }
    }

    impl Numeric for f64 {
        fn to_f64(&self) -> f64 {
            *self
        }

        fn pow(&self, x: u32) -> f64 {
            { (*self as u32).pow(x) as f64 }
        }
    }

    pub fn mean<T>(x: &[T]) -> f64
        where
            T: Numeric,
    {

        x.iter().map(|e| e.to_f64()).sum::<f64>() / x.len() as f64
    }


    pub fn geometric_mean<T>(x: &[T]) -> f64 where T:Numeric{

        return (x.iter().map(|e| e.to_f64()).product::<f64>()).powf(1.0/(x.len()as f64));
    }
    pub fn harmonic_mean<T>(x: &[T]) -> f64 where T:Numeric{

        return x.len() as f64/x.iter().map(|e| 1.0/e.to_f64()).sum::<f64>();
    }
    pub fn w_harmonic_mean<T,R>(x: &[T],pound:&[R]) -> f64 where T:Numeric,R:Numeric{

        return x.len() as f64/x.iter().zip(pound.iter()).map(|(e,p)| p.to_f64()/e.to_f64()).sum::<f64>();

    }

    pub fn median<T>(x: &mut [T]) -> f64
        where
            T: Numeric + Clone,
    {
        let n = x.len();
        if n % 2 == 0 {
            let median1 = quick_select(x, n / 2);
            let median2 = quick_select(x, n / 2 - 1);
            return (median1.to_f64() + median2.to_f64()) / 2.0;
        } else {
            return quick_select(x, n / 2).to_f64();
        }
    }

    pub fn median_low<T: Into<f64>>(x: &mut [T]) -> f64
        where
            T: Numeric + Clone,
    {
        let n = x.len();

            return quick_select(x, n / 2 -1).to_f64();

    }

    pub fn median_high<T: Into<f64>>(x: &mut [T]) -> f64
        where
            T: Numeric + Clone,
    {
        let n = x.len();

        return quick_select(x, n / 2).to_f64();

    }
    pub fn median_grouped<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;
    }
    // pub fn mode<T: Into<f64>>(x: &[T]) -> f64{
    //     return 0.0;
    // }
    // pub fn multimode<T: Into<f64>>(x: &[T]) -> f64{
    //     return 0.0;
    // }
    pub fn pstdev<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
    pub fn pvariance<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
    pub fn stdev<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
    pub fn variance<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
    pub fn quantiles<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
    pub fn covariance<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
    pub fn correlation<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
    pub fn linear_regression<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;

    }
}

pub(crate) mod utils{

   pub mod qs {
        use rand::Rng;
        use crate::statistics::statistics::Numeric;

        pub fn quick_select<T>(x: &mut [T], k: usize) -> T
            where T: Numeric + Clone,
        {
            let n = x.len();
            if n == 1 {
                return x[0].clone();
            }

            let pivot = partition(x);

            if k == pivot {
                return x[k].clone();
            } else if k < pivot {
                return quick_select(&mut x[0..pivot], k);
            } else {
                return quick_select(&mut x[pivot+1..], k - pivot - 1);
            }
        }


        fn partition<T>(x: &mut [T]) -> usize
            where
                T: Numeric,
        {
            let n = x.len();
            if n <= 1 {
                return 0;
            }

            let pivot = rand::thread_rng().gen_range(0..n);
            x.swap(pivot, n - 1);

            let mut i = 0;
            for j in 0..n - 1 {
                if x[j].to_f64() <= x[n - 1].to_f64() {
                    x.swap(i, j);
                    i += 1;
                }
            }

            x.swap(i, n - 1);

            i
        }
    }
}