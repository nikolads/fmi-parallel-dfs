pub fn seq(lists: &mut Vec<Vec<usize>>) {
    for u in 0..lists.len() {
        let (before, after) = lists.split_at_mut(u);
        let (list, after) = after.split_first_mut().unwrap();

        for &v in list.iter() {
            match v {
                _ if v < u => before[v].push(u),
                _ if v > u => after[v - u - 1].push(u),
                _ => unreachable!()
            }
        }
    }
}
