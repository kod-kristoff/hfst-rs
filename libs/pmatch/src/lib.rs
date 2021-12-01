mod compiler;
mod container;

pub use compiler::PmatchCompiler;
pub use container::PmatchContainer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
