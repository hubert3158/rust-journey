fn main() {
    let x = "a";
    fn test(x: &mut String) -> &mut String {
        return x;
    }
}
