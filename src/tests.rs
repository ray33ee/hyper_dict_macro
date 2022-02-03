
#[cfg(test)]
mod tests {
    use crate::traits::{AccessKeyValue, AccessDictValueOnly};

    #[test]
    fn operator() {
        let hd = crate::example::OperatorHD::new();

        assert_eq!(hd.get("add"), Some(&1.0));
        assert_eq!(hd.get("sub"), Some(&-1.0));
        assert_eq!(hd.get("mul"), Some(&2.0));
        assert_eq!(hd.get("div"), Some(&3.0));
        assert_eq!(hd.get("sqrt"), Some(&0.5));

        assert_eq!(hd.get("imposter!"), None);
    }

    #[test]
    fn arbitrary() {
        let hd = crate::example::ArbitraryHD::new();

        assert_eq!(*hd.get(0), 1);
        assert_eq!(*hd.get(23), 2);
        assert_eq!(*hd.get(67), 3);
        assert_eq!(*hd.get(32), 4);
        assert_eq!(*hd.get(76), 5);
        assert_eq!(*hd.get(8), 6);

    }

}