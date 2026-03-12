import Foundation
import CoreBluetooth

/// iOS BLE P2P chat manager using CoreBluetooth.
/// Host: CBPeripheralManager (advertise + GATT server)
/// Guest: CBCentralManager (scan + connect as client)
class BleManagerIOS: NSObject, CBPeripheralManagerDelegate, CBCentralManagerDelegate, CBPeripheralDelegate {

    static let serviceUUID = CBUUID(string: "0000FFE0-0000-1000-8000-00805F9B34FB")
    static let msgCharUUID = CBUUID(string: "0000FFE1-0000-1000-8000-00805F9B34FB")

    private var peripheralManager: CBPeripheralManager?
    private var centralManager: CBCentralManager?
    private var connectedPeripheral: CBPeripheral?
    private var msgCharacteristic: CBMutableCharacteristic?
    private var subscribedCentral: CBCentral?

    var pin: String = ""
    var isConnected: Bool = false
    var isHost: Bool = false
    var incomingMessages: [String] = []

    // MARK: - Host

    func startHost(pinCode: String) -> Bool {
        pin = pinCode
        isHost = true
        peripheralManager = CBPeripheralManager(delegate: self, queue: nil)
        return true
    }

    func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        guard peripheral.state == .poweredOn else { return }

        let char = CBMutableCharacteristic(
            type: BleManagerIOS.msgCharUUID,
            properties: [.read, .write, .notify],
            value: nil,
            permissions: [.readable, .writeable]
        )
        msgCharacteristic = char

        let service = CBMutableService(type: BleManagerIOS.serviceUUID, primary: true)
        service.characteristics = [char]
        peripheral.add(service)

        // Advertise with PIN in local name
        peripheral.startAdvertising([
            CBAdvertisementDataServiceUUIDsKey: [BleManagerIOS.serviceUUID],
            CBAdvertisementDataLocalNameKey: "ETG-\(pin)"
        ])
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didSubscribeTo characteristic: CBCharacteristic) {
        subscribedCentral = central
        isConnected = true
        incomingMessages.append("{\"type\":\"system\",\"text\":\"User connected\"}")
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveWrite requests: [CBATTRequest]) {
        for request in requests {
            if let value = request.value, let msg = String(data: value, encoding: .utf8) {
                incomingMessages.append("{\"type\":\"received\",\"text\":\"\(msg)\"}")
            }
            peripheral.respond(to: request, withResult: .success)
        }
    }

    // MARK: - Guest

    func scanForHost(targetPin: String) {
        pin = targetPin
        isHost = false
        centralManager = CBCentralManager(delegate: self, queue: nil)
    }

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        guard central.state == .poweredOn else { return }
        central.scanForPeripherals(withServices: [BleManagerIOS.serviceUUID], options: nil)
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral,
                         advertisementData: [String: Any], rssi RSSI: NSNumber) {
        let name = advertisementData[CBAdvertisementDataLocalNameKey] as? String ?? ""
        if name == "ETG-\(pin)" {
            central.stopScan()
            connectedPeripheral = peripheral
            peripheral.delegate = self
            central.connect(peripheral, options: nil)
        }
    }

    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        isConnected = true
        incomingMessages.append("{\"type\":\"system\",\"text\":\"Connected to host\"}")
        peripheral.discoverServices([BleManagerIOS.serviceUUID])
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        guard let service = peripheral.services?.first(where: { $0.uuid == BleManagerIOS.serviceUUID }) else { return }
        peripheral.discoverCharacteristics([BleManagerIOS.msgCharUUID], for: service)
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        guard let char = service.characteristics?.first(where: { $0.uuid == BleManagerIOS.msgCharUUID }) else { return }
        peripheral.setNotifyValue(true, for: char)
    }

    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        if let data = characteristic.value, let msg = String(data: data, encoding: .utf8) {
            incomingMessages.append("{\"type\":\"received\",\"text\":\"\(msg)\"}")
        }
    }

    // MARK: - Send

    func sendMessage(_ text: String) -> Bool {
        guard let data = text.data(using: .utf8) else { return false }

        if isHost {
            guard let char = msgCharacteristic, let central = subscribedCentral else { return false }
            return peripheralManager?.updateValue(data, for: char, onSubscribedCentrals: [central]) ?? false
        } else {
            guard let peripheral = connectedPeripheral,
                  let service = peripheral.services?.first(where: { $0.uuid == BleManagerIOS.serviceUUID }),
                  let char = service.characteristics?.first(where: { $0.uuid == BleManagerIOS.msgCharUUID }) else { return false }
            peripheral.writeValue(data, for: char, type: .withResponse)
            return true
        }
    }

    // MARK: - Disconnect

    func disconnect() {
        peripheralManager?.stopAdvertising()
        peripheralManager = nil
        if let p = connectedPeripheral { centralManager?.cancelPeripheralConnection(p) }
        centralManager = nil
        isConnected = false
        incomingMessages.removeAll()
    }
}
