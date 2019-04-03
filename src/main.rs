use bitvec::*;
use rayon::prelude::*;

//struct ParrallelWindow {}
//
//impl<'data> IntoParallelIterator for ParrallelWindow {
//    type Iter = Iter<'data, Self::Item>;
//    type Item = &'data BitSlice;
//
//    fn into_par_iter(self) -> Self::Iter {
//        Iter { slice: self }
//    }
//}
//
//struct Iter<'data, T: 'data + Sync> {
//    slice: &'data [T],
//}


fn main() {}

fn bit_search<'a>(needle: &'a BitSlice, haystack: &'a BitSlice) -> impl Iterator<Item=&'a usize> {
//    let mut result: Vec<_> = vec![];
//    for (index, window) in haystack.windows(needle.len()).enumerate() {
//        if window == needle {
//            result.push(index);
//        }
//    }

    haystack.windows(needle.len())
        .enumerate()
        .filter(move |(index, window)| *window == needle)
        .map(|(index, _)| &
            index)

    //return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let haystack = bitvec![1,1,1,1,1,0,0,0,1,0,0];
        let needle = bitvec![1,0,0];

        let result: Vec<&usize> = bit_search(&needle, &haystack).collect();


        assert_eq!(result, vec![&4, &8]);
    }
}
