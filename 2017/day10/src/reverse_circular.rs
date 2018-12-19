pub fn reverse_circular<T: Copy>(list: &mut Vec<T>, mut start: usize, mut len: usize) {
    while len > 1 {
        let tmp = list[start % 256];
        list[start % 256] = list[(start + len - 1) % 256];
        list[(start + len - 1) % 256] = tmp;
        start += 1;
        len -= 2;
    }
}
