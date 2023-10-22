
pub mod statistics{
    use crate::statistics::utils::quick_sort;

    pub trait Numeric {
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

    pub fn median<T: Into<f64>>(x: &[T]) -> f64 where  T:Numeric,T:Clone{
        let mut bind = x.to_owned();
        let x:&mut [T]= bind.as_mut_slice();
        let mid :usize = x.len()/2;


        if x.len()>5 {
            let branch = x.split_at_mut(x.len() / 5);
            median(branch.1);
        }
        // let to_ord:&mut Vec<f64> = &mut vec![];
        // for e in x{
        //     to_ord.push(e.to_f64());
        // }
        // let mut to_ord = to_ord.to_owned();
        // let x:&mut [f64]= to_ord.as_mut_slice();
       quick_sort(x);

        if x.len()%2 == 0{
            return  (x[mid - 1].to_f64() + x[mid].to_f64())/2.0

        }

       return  x[mid].to_f64();


    }
    pub fn median_low<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;
    }
    pub  fn median_high<T: Into<f64>>(_x: &[T]) -> f64{
        return 0.0;
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

pub mod utils{
    use crate::statistics::statistics::Numeric;

    pub fn quick_sort<T>(x: &mut [T])
        where
            T: Numeric + Clone,
    {
        let len = x.len();

        if len > 1 {
            let pivot_idx = partition(x);
            let (left, right) = x.split_at_mut(pivot_idx);

            quick_sort(left);
            quick_sort(&mut right[1..]);
        }
    }

    fn partition<T>(x: &mut [T]) -> usize
        where
            T: Numeric + Clone,
    {
        let len = x.len();
        let pivot_idx = len / 2;

        x.swap(pivot_idx, len - 1);

        let mut i = 0;
        for j in 0..len - 1 {
            if x[j].to_f64() <= x[len - 1].to_f64() {
                x.swap(i, j);
                i += 1;
            }
        }
        x.swap(i, len - 1);
        i
    }
}