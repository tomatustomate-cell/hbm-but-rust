//! Camada reservada à integração nativa com Horizon.
//! A API alvo ainda precisa ser confirmada; não declarar bindings fictícios.
pub const INTEGRATION_STATUS: &str = "pending-horizon-api-validation";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn integration_is_explicitly_pending() {
        assert_eq!(INTEGRATION_STATUS, "pending-horizon-api-validation");
    }
}
