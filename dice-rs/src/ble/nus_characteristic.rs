/// NUS (Nordic UART Service) characteristics used by GoDice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum NusCharacteristic {
    /// NUS RX — write characteristic (client-to-dice commands).
    #[error("NUS write")]
    Write,
    /// NUS TX — notify characteristic (dice-to-client notifications).
    #[error("NUS notify")]
    Notify,
}
