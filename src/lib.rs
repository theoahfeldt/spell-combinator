pub mod mouseclick;
pub mod spellang;
pub mod unit;

#[cfg(test)]
mod tests {
    use super::spellang::*;
    #[test]
    fn test_spellang() {
        let mut state = State { hp: 10 };
        let mut dag = example_dag();
        dag.execute(&mut state).unwrap();
        assert_eq!(state, State { hp: 9 });
    }
}
