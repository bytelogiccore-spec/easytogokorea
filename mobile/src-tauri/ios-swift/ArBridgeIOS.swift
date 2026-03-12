import UIKit
import WebKit

/// WKScriptMessageHandler bridge for iOS.
/// Handles AR activity launches and BLE chat.
///
/// JS usage:
///   window.webkit.messageHandlers.ArBridge.postMessage({ action: "openAR" })
///   window.webkit.messageHandlers.ArBridge.postMessage({ action: "startChatHost", pin: "7241" })
///   window.webkit.messageHandlers.ArBridge.postMessage({ action: "sendChatMessage", text: "Hello" })
///   window.webkit.messageHandlers.ArBridge.postMessage({ action: "getChatMessages" })
class ArBridgeIOS: NSObject, WKScriptMessageHandler {

    private weak var viewController: UIViewController?
    let bleManager = BleManagerIOS()

    init(viewController: UIViewController) {
        self.viewController = viewController
    }

    func userContentController(_ userContentController: WKUserContentController,
                               didReceive message: WKScriptMessage) {
        guard let body = message.body as? [String: Any],
              let action = body["action"] as? String else { return }

        DispatchQueue.main.async { [weak self] in
            guard let self = self, let vc = self.viewController else { return }

            switch action {

            // ─── AR / Camera ───
            case "openAR":
                let arVC = ArNavigationViewController()
                arVC.modalPresentationStyle = .fullScreen
                vc.present(arVC, animated: true)

            case "openPlace":
                let placeVC = ArPlaceViewController()
                placeVC.modalPresentationStyle = .fullScreen
                vc.present(placeVC, animated: true)

            case "openScanner":
                let scanVC = ScannerViewController()
                scanVC.modalPresentationStyle = .fullScreen
                vc.present(scanVC, animated: true)

            // ─── BLE Chat ───
            case "startChatHost":
                let pin = body["pin"] as? String ?? "0000"
                let _ = self.bleManager.startHost(pinCode: pin)

            case "joinChat":
                let pin = body["pin"] as? String ?? "0000"
                self.bleManager.scanForHost(targetPin: pin)

            case "sendChatMessage":
                let text = body["text"] as? String ?? ""
                let _ = self.bleManager.sendMessage(text)

            case "getChatMessages":
                let msgs = self.bleManager.incomingMessages
                self.bleManager.incomingMessages.removeAll()
                let json = "[\(msgs.joined(separator: ","))]"
                // Inject result back into WebView
                if let webView = vc.view.subviews.first(where: { $0 is WKWebView }) as? WKWebView {
                    webView.evaluateJavaScript("window.__bleMsgs = \(json)", completionHandler: nil)
                }

            case "getChatStatus":
                let json = "{\"connected\":\(self.bleManager.isConnected),\"isHost\":\(self.bleManager.isHost),\"pin\":\"\(self.bleManager.pin)\"}"
                if let webView = vc.view.subviews.first(where: { $0 is WKWebView }) as? WKWebView {
                    webView.evaluateJavaScript("window.__bleStatus = \(json)", completionHandler: nil)
                }

            case "disconnectChat":
                self.bleManager.disconnect()

            default:
                break
            }
        }
    }
}
