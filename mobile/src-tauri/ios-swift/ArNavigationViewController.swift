import UIKit
import ARKit
import SceneKit

/// Full-screen ARKit navigation view controller.
/// Shows ARSCNView camera + direction HUD overlay.
class ArNavigationViewController: UIViewController, ARSCNViewDelegate {

    private var sceneView: ARSCNView!

    // HUD
    private var distanceLabel: UILabel!
    private var arrowLabel: UILabel!
    private var directionLabel: UILabel!
    private var trackingBadge: UILabel!
    private var destinationLabel: UILabel!
    private var etaLabel: UILabel!
    private var backButton: UIButton!

    private var trackingTimer: Timer?

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .black

        // --- ARSCNView (ARKit camera + scene) ---
        sceneView = ARSCNView(frame: view.bounds)
        sceneView.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        sceneView.delegate = self
        sceneView.automaticallyUpdatesLighting = true
        view.addSubview(sceneView)

        setupHUD()
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        let config = ARWorldTrackingConfiguration()
        config.planeDetection = [.horizontal, .vertical]
        if ARWorldTrackingConfiguration.supportsSceneReconstruction(.mesh) {
            config.sceneReconstruction = .mesh // LiDAR support
        }
        sceneView.session.run(config)
        startTrackingUpdates()
    }

    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        sceneView.session.pause()
        trackingTimer?.invalidate()
    }

    override var prefersStatusBarHidden: Bool { true }
    override var prefersHomeIndicatorAutoHidden: Bool { true }

    // MARK: - HUD Setup

    private func setupHUD() {
        // Back button
        backButton = UIButton(type: .system)
        backButton.setTitle("←", for: .normal)
        backButton.titleLabel?.font = .boldSystemFont(ofSize: 28)
        backButton.setTitleColor(UIColor.white.withAlphaComponent(0.6), for: .normal)
        backButton.translatesAutoresizingMaskIntoConstraints = false
        backButton.addTarget(self, action: #selector(goBack), for: .touchUpInside)
        view.addSubview(backButton)

        // Tracking badge
        trackingBadge = makeLabel(size: 10, color: .white.withAlphaComponent(0.4), bold: true)
        trackingBadge.text = "SCANNING..."
        trackingBadge.backgroundColor = UIColor.black.withAlphaComponent(0.4)
        trackingBadge.layer.cornerRadius = 12
        trackingBadge.clipsToBounds = true
        trackingBadge.textAlignment = .center
        view.addSubview(trackingBadge)

        // Distance
        distanceLabel = makeLabel(size: 42, color: .white, bold: true)
        distanceLabel.text = "—"
        distanceLabel.textAlignment = .center
        distanceLabel.layer.shadowColor = UIColor.black.cgColor
        distanceLabel.layer.shadowOffset = CGSize(width: 0, height: 4)
        distanceLabel.layer.shadowRadius = 20
        distanceLabel.layer.shadowOpacity = 0.8
        view.addSubview(distanceLabel)

        // Arrow
        arrowLabel = makeLabel(size: 96, color: UIColor(red: 0.145, green: 0.388, blue: 0.922, alpha: 1), bold: true)
        arrowLabel.text = "↑"
        arrowLabel.textAlignment = .center
        view.addSubview(arrowLabel)

        // Direction hint
        directionLabel = makeLabel(size: 12, color: .white.withAlphaComponent(0.4), bold: true)
        directionLabel.text = "POINT CAMERA AT SURROUNDINGS"
        directionLabel.textAlignment = .center
        view.addSubview(directionLabel)

        // Bottom sheet
        let bottomView = UIView()
        bottomView.backgroundColor = UIColor.black.withAlphaComponent(0.7)
        bottomView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(bottomView)

        destinationLabel = makeLabel(size: 28, color: .white, bold: true)
        destinationLabel.text = "Hongdae Stn."
        bottomView.addSubview(destinationLabel)

        etaLabel = makeLabel(size: 14, color: UIColor(white: 0.64, alpha: 1), bold: true)
        etaLabel.text = "Scanning..."
        bottomView.addSubview(etaLabel)

        // Layout constraints
        NSLayoutConstraint.activate([
            backButton.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 16),
            backButton.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 24),

            trackingBadge.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 20),
            trackingBadge.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -24),
            trackingBadge.widthAnchor.constraint(greaterThanOrEqualToConstant: 100),
            trackingBadge.heightAnchor.constraint(equalToConstant: 24),

            distanceLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            distanceLabel.centerYAnchor.constraint(equalTo: view.centerYAnchor, constant: -100),

            arrowLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            arrowLabel.topAnchor.constraint(equalTo: distanceLabel.bottomAnchor, constant: -10),

            directionLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            directionLabel.topAnchor.constraint(equalTo: arrowLabel.bottomAnchor, constant: 12),
            directionLabel.widthAnchor.constraint(equalTo: view.widthAnchor, constant: -48),

            bottomView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            bottomView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            bottomView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
            bottomView.heightAnchor.constraint(equalToConstant: 120),

            destinationLabel.leadingAnchor.constraint(equalTo: bottomView.leadingAnchor, constant: 24),
            destinationLabel.topAnchor.constraint(equalTo: bottomView.topAnchor, constant: 20),

            etaLabel.leadingAnchor.constraint(equalTo: bottomView.leadingAnchor, constant: 24),
            etaLabel.topAnchor.constraint(equalTo: destinationLabel.bottomAnchor, constant: 4),
        ])
    }

    // MARK: - Tracking Updates

    private func startTrackingUpdates() {
        trackingTimer = Timer.scheduledTimer(withTimeInterval: 0.1, repeats: true) { [weak self] _ in
            guard let self = self else { return }
            let state = self.sceneView.session.currentFrame?.camera.trackingState

            switch state {
            case .normal:
                let anchors = self.sceneView.session.currentFrame?.anchors.filter { $0 is ARPlaneAnchor } ?? []
                self.trackingBadge.text = "  ARKIT • \(anchors.count) planes  "
                self.trackingBadge.textColor = UIColor(red: 0.133, green: 0.773, blue: 0.369, alpha: 1)
                self.distanceLabel.text = "150m"
                self.directionLabel.text = "WALK STRAIGHT AHEAD"
                self.etaLabel.text = "2 min walk"
            case .limited(let reason):
                let reasonText: String
                switch reason {
                case .initializing: reasonText = "INITIALIZING"
                case .excessiveMotion: reasonText = "SLOW DOWN"
                case .insufficientFeatures: reasonText = "MORE LIGHT"
                case .relocalizing: reasonText = "RELOCALIZING"
                @unknown default: reasonText = "LIMITED"
                }
                self.trackingBadge.text = "  \(reasonText)  "
                self.trackingBadge.textColor = UIColor.white.withAlphaComponent(0.4)
                self.distanceLabel.text = "—"
                self.directionLabel.text = "SCANNING..."
                self.etaLabel.text = "Scanning..."
            default:
                self.trackingBadge.text = "  NOT AVAILABLE  "
                self.trackingBadge.textColor = UIColor.red
            }
        }
    }

    // MARK: - Helpers

    @objc private func goBack() {
        dismiss(animated: true)
    }

    private func makeLabel(size: CGFloat, color: UIColor, bold: Bool) -> UILabel {
        let label = UILabel()
        label.font = bold ? .boldSystemFont(ofSize: size) : .systemFont(ofSize: size)
        label.textColor = color
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }
}
