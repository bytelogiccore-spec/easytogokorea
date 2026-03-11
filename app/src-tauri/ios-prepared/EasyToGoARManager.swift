import ARKit
import UIKit

/// ARKit bridge for EasyToGo — mirrors the Android Kotlin ARCore plugin.
/// Drop this into the Tauri iOS project when `tauri ios init` is run on macOS.
///
/// Usage: The Tauri Rust commands in lib.rs will delegate to these functions
/// on iOS via the Swift ↔ Rust bridge.

class EasyToGoARManager: NSObject, ARSessionDelegate {

    static let shared = EasyToGoARManager()

    private var arSession: ARSession?
    private var isAvailable = false
    private var detectedPlanes: [UUID: ARPlaneAnchor] = [:]
    private var userAnchors: [ARAnchor] = []

    private override init() {
        super.init()
        isAvailable = ARWorldTrackingConfiguration.isSupported
    }

    // MARK: - Public API (called from Tauri Rust via Swift bridge)

    /// Check if ARKit is available on this device
    func checkAvailability() -> [String: Any] {
        return [
            "available": isAvailable,
            "platform": "ios",
            "supportsWorldTracking": ARWorldTrackingConfiguration.isSupported,
            "supportsLiDAR": ARWorldTrackingConfiguration.supportsSceneReconstruction(.mesh),
            "supportsFaceTracking": ARFaceTrackingConfiguration.isSupported
        ]
    }

    /// Start an AR session with plane detection
    func startSession() -> [String: Any] {
        guard isAvailable else {
            return ["success": false, "reason": "ARKit not supported on this device"]
        }

        let session = ARSession()
        session.delegate = self

        let config = ARWorldTrackingConfiguration()
        config.planeDetection = [.horizontal, .vertical]
        config.environmentTexturing = .automatic

        // Enable LiDAR scene reconstruction if available
        if ARWorldTrackingConfiguration.supportsSceneReconstruction(.mesh) {
            config.sceneReconstruction = .mesh
        }

        // Enable auto focus
        if ARWorldTrackingConfiguration.supportsFrameSemantics(.smoothedSceneDepth) {
            config.frameSemantics.insert(.smoothedSceneDepth)
        }

        session.run(config)
        arSession = session

        print("[EasyToGoAR] AR Session started successfully")
        return ["success": true]
    }

    /// Perform a hit test at normalized screen coordinates (0..1)
    /// Returns anchor position in 3D space
    func hitTest(normalizedX: Float, normalizedY: Float, viewSize: CGSize) -> [String: Any]? {
        guard let session = arSession,
              let frame = session.currentFrame else {
            return nil
        }

        guard frame.camera.trackingState == .normal else {
            return nil
        }

        let screenPoint = CGPoint(
            x: CGFloat(normalizedX) * viewSize.width,
            y: CGFloat(normalizedY) * viewSize.height
        )

        // Use raycast (preferred over deprecated hitTest)
        if let query = session.currentFrame?.raycastQuery(
            from: screenPoint,
            allowing: .estimatedPlane,
            alignment: .any
        ) {
            let results = session.raycast(query)
            if let firstResult = results.first {
                let anchor = ARAnchor(name: "easytogo_object", transform: firstResult.worldTransform)
                session.add(anchor: anchor)
                userAnchors.append(anchor)

                let position = firstResult.worldTransform.columns.3
                return [
                    "hit": true,
                    "x": position.x,
                    "y": position.y,
                    "z": position.z,
                    "anchorId": anchor.identifier.uuidString
                ]
            }
        }

        return ["hit": false]
    }

    /// Get current AR tracking state
    func getTrackingInfo() -> [String: Any] {
        guard let session = arSession,
              let frame = session.currentFrame else {
            return [
                "available": isAvailable,
                "tracking": false,
                "planes": 0
            ]
        }

        let trackingState: Bool
        let trackingReason: String

        switch frame.camera.trackingState {
        case .normal:
            trackingState = true
            trackingReason = "normal"
        case .limited(let reason):
            trackingState = false
            switch reason {
            case .initializing: trackingReason = "initializing"
            case .excessiveMotion: trackingReason = "excessive_motion"
            case .insufficientFeatures: trackingReason = "insufficient_features"
            case .relocalizing: trackingReason = "relocalizing"
            @unknown default: trackingReason = "unknown"
            }
        case .notAvailable:
            trackingState = false
            trackingReason = "not_available"
        @unknown default:
            trackingState = false
            trackingReason = "unknown"
        }

        return [
            "available": true,
            "tracking": trackingState,
            "trackingReason": trackingReason,
            "planes": detectedPlanes.count,
            "anchors": userAnchors.count
        ]
    }

    /// Stop the AR session
    func stopSession() {
        arSession?.pause()
        arSession = nil
        detectedPlanes.removeAll()
        userAnchors.removeAll()
        print("[EasyToGoAR] AR Session stopped")
    }

    /// Remove a specific anchor by ID
    func removeAnchor(id: String) -> Bool {
        guard let session = arSession,
              let uuid = UUID(uuidString: id),
              let anchor = userAnchors.first(where: { $0.identifier == uuid }) else {
            return false
        }
        session.remove(anchor: anchor)
        userAnchors.removeAll { $0.identifier == uuid }
        return true
    }

    // MARK: - ARSessionDelegate

    func session(_ session: ARSession, didAdd anchors: [ARAnchor]) {
        for anchor in anchors {
            if let planeAnchor = anchor as? ARPlaneAnchor {
                detectedPlanes[planeAnchor.identifier] = planeAnchor
                print("[EasyToGoAR] Plane detected: \(planeAnchor.alignment == .horizontal ? "horizontal" : "vertical")")
            }
        }
    }

    func session(_ session: ARSession, didUpdate anchors: [ARAnchor]) {
        for anchor in anchors {
            if let planeAnchor = anchor as? ARPlaneAnchor {
                detectedPlanes[planeAnchor.identifier] = planeAnchor
            }
        }
    }

    func session(_ session: ARSession, didRemove anchors: [ARAnchor]) {
        for anchor in anchors {
            detectedPlanes.removeValue(forKey: anchor.identifier)
        }
    }

    func session(_ session: ARSession, didFailWithError error: Error) {
        print("[EasyToGoAR] Session failed: \(error.localizedDescription)")
    }

    func sessionWasInterrupted(_ session: ARSession) {
        print("[EasyToGoAR] Session interrupted")
    }

    func sessionInterruptionEnded(_ session: ARSession) {
        print("[EasyToGoAR] Session interruption ended, resetting")
        let config = ARWorldTrackingConfiguration()
        config.planeDetection = [.horizontal, .vertical]
        session.run(config, options: [.resetTracking, .removeExistingAnchors])
        detectedPlanes.removeAll()
        userAnchors.removeAll()
    }
}
