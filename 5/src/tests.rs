#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_example_correctly() {
        let example = "FBFBBFFRLR";
        let coord = decode_seat(example);
        assert_eq!(coord.row, 44);
        assert_eq!(coord.col, 5);
    }
}

