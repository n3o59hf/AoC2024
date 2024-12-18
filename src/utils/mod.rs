pub mod c2;

pub fn binary_find<F>(mut low: usize, mut high: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    while low < high {
        let mid = low + (high - low) / 2;
        if f(mid) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}
