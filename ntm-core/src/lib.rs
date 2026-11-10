//! Núcleo independente do jogo para o port do HBM's Nuclear Tech.
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exposes_package_version() {
        assert_eq!(PROJECT_VERSION, "0.1.0");
    }
}
