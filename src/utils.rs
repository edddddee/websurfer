pub fn binary_search(b: u8, buf: &[u8]) -> bool {
    let mut low: isize = 0;
    let mut high: isize = buf.len() as isize - 1;
    let mut mid: isize = (high+low)/2;

    while low <= high {
        if b == buf[mid as usize] {
            return true;
        } else if b < buf[mid as usize] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
        mid = (high+low)/2;
    }
    return false;
}
