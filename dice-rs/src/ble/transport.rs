use async_trait::async_trait;
use btleplug::api::Central;
use btleplug::api::Manager as BtleplugManager;
use btleplug::api::Peripheral as BtleplugPeripheralApi;
use btleplug::api::ScanFilter;
use btleplug::api::WriteType;
use btleplug::platform::Adapter;
use btleplug::platform::Manager;
use btleplug::platform::Peripheral as BtleplugPeripheral;
use futures::stream::BoxStream;
use uuid::Uuid;

use crate::ble::uuids::NUS_NOTIFY_CHAR_UUID;
use crate::ble::uuids::NUS_WRITE_CHAR_UUID;
use crate::error::DiceError;
use crate::error::Result;

/// Abstraction over a BLE backend (btleplug, bluer, mock).
#[async_trait]
pub trait BleTransport: Send + Sync {
    /// The peripheral type returned by the backend.
    type Peripheral: BlePeripheral + Send + Sync;

    /// Start scanning for BLE devices with the given filter.
    async fn start_scan(&self, filter: ScanFilter) -> Result<()>;

    /// Stop an active scan.
    async fn stop_scan(&self) -> Result<()>;

    /// Return all peripherals discovered so far.
    async fn peripherals(&self) -> Result<Vec<Self::Peripheral>>;

    /// Subscribe to central events (device discovered, lost, etc.).
    async fn events(&self) -> Result<BoxStream<'static, btleplug::api::CentralEvent>>;
}

/// Abstraction over a single BLE peripheral.
#[async_trait]
pub trait BlePeripheral: Send + Sync {
    /// Unique identifier of the peripheral.
    fn id(&self) -> btleplug::platform::PeripheralId;

    /// MAC address of the peripheral.
    fn address(&self) -> btleplug::api::BDAddr;

    /// Current connection state.
    async fn is_connected(&self) -> Result<bool>;

    /// Cached properties (local name, RSSI, etc.).
    async fn properties(&self) -> Result<Option<btleplug::api::PeripheralProperties>>;

    /// Establish a connection.
    async fn connect(&self) -> Result<()>;

    /// Disconnect from the peripheral.
    async fn disconnect(&self) -> Result<()>;

    /// Discover GATT services and characteristics.
    async fn discover_services(&self) -> Result<()>;

    /// Find a characteristic by UUID.
    fn characteristic(&self, uuid: Uuid) -> Option<btleplug::api::Characteristic>;

    /// Write data to a characteristic.
    async fn write(&self, characteristic: &btleplug::api::Characteristic, data: &[u8], write_type: WriteType) -> Result<()>;

    /// Enable notifications on a characteristic.
    async fn subscribe(&self, characteristic: &btleplug::api::Characteristic) -> Result<()>;

    /// Stream of incoming notifications.
    async fn notifications(&self) -> Result<BoxStream<'static, btleplug::api::ValueNotification>>;
}

/// btleplug implementation of `BleTransport`.
///
/// Wraps `btleplug::platform::Adapter` which provides `Central` functionality.
pub struct BtleplugTransport {
    adapter: Adapter,
}

impl BtleplugTransport {
    /// Create a new transport by selecting the first available Bluetooth adapter.
    pub async fn new() -> Result<Self> {
        let manager = Manager::new().await.map_err(|_| DiceError::ScanFailed)?;
        let adapters = manager.adapters().await.map_err(|_| DiceError::ScanFailed)?;
        let adapter = adapters.into_iter().next().ok_or(DiceError::ScanFailed)?;
        Ok(Self { adapter })
    }

    /// Returns the write characteristic UUID (NUS RX).
    pub fn write_char_uuid() -> Uuid {
        NUS_WRITE_CHAR_UUID
    }

    /// Returns the notify characteristic UUID (NUS TX).
    pub fn notify_char_uuid() -> Uuid {
        NUS_NOTIFY_CHAR_UUID
    }
}

#[async_trait]
impl BleTransport for BtleplugTransport {
    type Peripheral = BtleplugPeripheralWrapper;

    async fn start_scan(&self, filter: ScanFilter) -> Result<()> {
        self.adapter.start_scan(filter).await.map_err(|_| DiceError::ScanFailed)
    }

    async fn stop_scan(&self) -> Result<()> {
        self.adapter.stop_scan().await.map_err(|_| DiceError::ScanFailed)
    }

    async fn peripherals(&self) -> Result<Vec<Self::Peripheral>> {
        let peripherals = self.adapter.peripherals().await.map_err(|_| DiceError::ScanFailed)?;
        Ok(peripherals.into_iter().map(BtleplugPeripheralWrapper::new).collect())
    }

    async fn events(&self) -> Result<BoxStream<'static, btleplug::api::CentralEvent>> {
        let events = self.adapter.events().await.map_err(|_| DiceError::ScanFailed)?;
        Ok(Box::pin(events))
    }
}

/// Wrapper around `btleplug::platform::Peripheral` implementing `BlePeripheral`.
#[derive(Clone)]
pub struct BtleplugPeripheralWrapper {
    inner: BtleplugPeripheral,
}

impl BtleplugPeripheralWrapper {
    /// Create a new wrapper from a btleplug peripheral.
    pub fn new(peripheral: BtleplugPeripheral) -> Self {
        Self { inner: peripheral }
    }

    /// Returns a reference to the inner btleplug peripheral.
    pub fn inner(&self) -> &BtleplugPeripheral {
        &self.inner
    }
}

#[async_trait]
impl BlePeripheral for BtleplugPeripheralWrapper {
    fn id(&self) -> btleplug::platform::PeripheralId {
        self.inner.id()
    }

    fn address(&self) -> btleplug::api::BDAddr {
        self.inner.address()
    }

    async fn is_connected(&self) -> Result<bool> {
        self.inner.is_connected().await.map_err(|_| DiceError::ConnectionLost)
    }

    async fn properties(&self) -> Result<Option<btleplug::api::PeripheralProperties>> {
        self.inner.properties().await.map_err(|_| DiceError::ConnectionLost)
    }

    async fn connect(&self) -> Result<()> {
        self.inner.connect().await.map_err(|_| DiceError::ConnectionFailed)
    }

    async fn disconnect(&self) -> Result<()> {
        self.inner.disconnect().await.map_err(|_| DiceError::ConnectionLost)
    }

    async fn discover_services(&self) -> Result<()> {
        self.inner.discover_services().await.map_err(|_| DiceError::DiscoveryFailed)
    }

    fn characteristic(&self, uuid: Uuid) -> Option<btleplug::api::Characteristic> {
        self.inner.characteristics().iter().find(|c| c.uuid == uuid).cloned()
    }

    async fn write(&self, characteristic: &btleplug::api::Characteristic, data: &[u8], write_type: WriteType) -> Result<()> {
        self.inner.write(characteristic, data, write_type).await.map_err(|_| DiceError::WriteFailed)
    }

    async fn subscribe(&self, characteristic: &btleplug::api::Characteristic) -> Result<()> {
        self.inner.subscribe(characteristic).await.map_err(|_| DiceError::SubscribeFailed)
    }

    async fn notifications(&self) -> Result<BoxStream<'static, btleplug::api::ValueNotification>> {
        let notifications = self.inner.notifications().await.map_err(|_| DiceError::SubscribeFailed)?;
        Ok(Box::pin(notifications))
    }
}
