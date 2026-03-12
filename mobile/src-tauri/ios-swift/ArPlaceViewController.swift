import UIKit
import ARKit
import SceneKit

/// Full-screen ARKit object placement view controller.
/// Tap to place emoji markers on detected planes.
class ArPlaceViewController: UIViewController, ARSCNViewDelegate {

    private var sceneView: ARSCNView!
    private var trackingBadge: UILabel!
    private var hintLabel: UILabel!
    private var clearButton: UIButton!
    private var selectedTypeLabel: UILabel!
    private var typeButtons: [UIButton] = []
    private var trackingTimer: Timer?

    private var selectedType = 0
    private let objectTypes: [(emoji: String, label: String)] = [
        ("📍", "Pin"), ("➡️", "Arrow"), ("⭐", "Star"),
        ("🍜", "Restaurant"), ("🏥", "Hospital"), ("🚇", "Subway"), ("🎨", "Graffiti")
    ]
    private var placedNodes: [SCNNode] = []

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .black

        // ARSCNView
        sceneView = ARSCNView(frame: view.bounds)
        sceneView.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        sceneView.delegate = self
        sceneView.automaticallyUpdatesLighting = true

        // Tap gesture for placement
        let tap = UITapGestureRecognizer(target: self, action: #selector(handleTap(_:)))
        sceneView.addGestureRecognizer(tap)
        view.addSubview(sceneView)

        setupHUD()
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        let config = ARWorldTrackingConfiguration()
        config.planeDetection = [.horizontal, .vertical]
        sceneView.session.run(config)
        startTrackingUpdates()
    }

    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        sceneView.session.pause()
        trackingTimer?.invalidate()
    }

    override var prefersStatusBarHidden: Bool { true }

    // MARK: - Tap to Place

    @objc private func handleTap(_ gesture: UITapGestureRecognizer) {
        let location = gesture.location(in: sceneView)

        // ARKit ray cast
        if let query = sceneView.raycastQuery(from: location, allowing: .existingPlaneGeometry, alignment: .any),
           let result = sceneView.session.raycast(query).first {

            let type = objectTypes[selectedType]
            let node = createEmojiNode(emoji: type.emoji, label: type.label)
            node.simdTransform = result.worldTransform
            sceneView.scene.rootNode.addChildNode(node)
            placedNodes.append(node)
            updateClearButton()

            // Pop animation
            node.scale = SCNVector3(0, 0, 0)
            SCNTransaction.begin()
            SCNTransaction.animationDuration = 0.3
            SCNTransaction.animationTimingFunction = CAMediaTimingFunction(name: .easeOut)
            node.scale = SCNVector3(1, 1, 1)
            SCNTransaction.commit()
        }
    }

    private func createEmojiNode(emoji: String, label: String) -> SCNNode {
        let container = SCNNode()

        // Emoji text
        let emojiText = SCNText(string: emoji, extrusionDepth: 0.1)
        emojiText.font = UIFont.systemFont(ofSize: 12)
        emojiText.firstMaterial?.diffuse.contents = UIColor.white
        let emojiNode = SCNNode(geometry: emojiText)
        emojiNode.scale = SCNVector3(0.005, 0.005, 0.005)

        // Center the text
        let (min, max) = emojiNode.boundingBox
        let dx = (max.x - min.x) / 2
        let dy = (max.y - min.y) / 2
        emojiNode.position = SCNVector3(-dx * 0.005, 0.02, 0)

        container.addChildNode(emojiNode)

        // Label plane
        let labelText = SCNText(string: label, extrusionDepth: 0)
        labelText.font = UIFont.boldSystemFont(ofSize: 4)
        labelText.firstMaterial?.diffuse.contents = UIColor.white
        let labelNode = SCNNode(geometry: labelText)
        labelNode.scale = SCNVector3(0.005, 0.005, 0.005)
        labelNode.position = SCNVector3(-dx * 0.005, 0.0, 0)
        container.addChildNode(labelNode)

        // Billboard constraint so it always faces the camera
        let billboard = SCNBillboardConstraint()
        billboard.freeAxes = [.Y]
        container.constraints = [billboard]

        return container
    }

    // MARK: - HUD

    private func setupHUD() {
        // Back button
        let backBtn = UIButton(type: .system)
        backBtn.setTitle("←", for: .normal)
        backBtn.titleLabel?.font = .boldSystemFont(ofSize: 28)
        backBtn.setTitleColor(UIColor.white.withAlphaComponent(0.6), for: .normal)
        backBtn.translatesAutoresizingMaskIntoConstraints = false
        backBtn.addTarget(self, action: #selector(goBack), for: .touchUpInside)
        view.addSubview(backBtn)

        // Tracking badge
        trackingBadge = UILabel()
        trackingBadge.font = .boldSystemFont(ofSize: 10)
        trackingBadge.textColor = UIColor.white.withAlphaComponent(0.4)
        trackingBadge.text = "  SCANNING...  "
        trackingBadge.backgroundColor = UIColor.black.withAlphaComponent(0.4)
        trackingBadge.layer.cornerRadius = 12
        trackingBadge.clipsToBounds = true
        trackingBadge.textAlignment = .center
        trackingBadge.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(trackingBadge)

        // Clear button
        clearButton = UIButton(type: .system)
        clearButton.setTitleColor(.red, for: .normal)
        clearButton.titleLabel?.font = .boldSystemFont(ofSize: 10)
        clearButton.translatesAutoresizingMaskIntoConstraints = false
        clearButton.addTarget(self, action: #selector(clearAll), for: .touchUpInside)
        view.addSubview(clearButton)

        // Crosshair
        let crosshair = UILabel()
        crosshair.text = "+"
        crosshair.font = .systemFont(ofSize: 32)
        crosshair.textColor = UIColor.white.withAlphaComponent(0.3)
        crosshair.textAlignment = .center
        crosshair.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(crosshair)

        // Footer
        let footer = UIView()
        footer.backgroundColor = UIColor.black.withAlphaComponent(0.8)
        footer.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(footer)

        hintLabel = UILabel()
        hintLabel.font = .boldSystemFont(ofSize: 9)
        hintLabel.textColor = UIColor.white.withAlphaComponent(0.3)
        hintLabel.text = "SCANNING FOR SURFACES..."
        hintLabel.textAlignment = .center
        hintLabel.translatesAutoresizingMaskIntoConstraints = false
        footer.addSubview(hintLabel)

        selectedTypeLabel = UILabel()
        selectedTypeLabel.font = .boldSystemFont(ofSize: 18)
        selectedTypeLabel.textColor = .white
        selectedTypeLabel.text = objectTypes[0].label
        selectedTypeLabel.textAlignment = .center
        selectedTypeLabel.translatesAutoresizingMaskIntoConstraints = false
        footer.addSubview(selectedTypeLabel)

        // Type picker
        let pickerStack = UIStackView()
        pickerStack.axis = .horizontal
        pickerStack.spacing = 8
        pickerStack.alignment = .center
        pickerStack.distribution = .equalCentering
        pickerStack.translatesAutoresizingMaskIntoConstraints = false
        footer.addSubview(pickerStack)

        for (i, type) in objectTypes.enumerated() {
            let btn = UIButton(type: .system)
            btn.setTitle(type.emoji, for: .normal)
            btn.titleLabel?.font = .systemFont(ofSize: 22)
            btn.backgroundColor = i == 0 ? UIColor.systemBlue.withAlphaComponent(0.3) : UIColor.white.withAlphaComponent(0.08)
            btn.layer.cornerRadius = 12
            btn.tag = i
            btn.addTarget(self, action: #selector(selectType(_:)), for: .touchUpInside)
            btn.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([btn.widthAnchor.constraint(equalToConstant: 48), btn.heightAnchor.constraint(equalToConstant: 48)])
            pickerStack.addArrangedSubview(btn)
            typeButtons.append(btn)
        }

        NSLayoutConstraint.activate([
            backBtn.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 16),
            backBtn.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 20),

            trackingBadge.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 20),
            trackingBadge.centerXAnchor.constraint(equalTo: view.centerXAnchor),

            clearButton.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 20),
            clearButton.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -20),

            crosshair.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            crosshair.centerYAnchor.constraint(equalTo: view.centerYAnchor),

            footer.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            footer.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            footer.bottomAnchor.constraint(equalTo: view.bottomAnchor),

            hintLabel.topAnchor.constraint(equalTo: footer.topAnchor, constant: 16),
            hintLabel.centerXAnchor.constraint(equalTo: footer.centerXAnchor),

            selectedTypeLabel.topAnchor.constraint(equalTo: hintLabel.bottomAnchor, constant: 4),
            selectedTypeLabel.centerXAnchor.constraint(equalTo: footer.centerXAnchor),

            pickerStack.topAnchor.constraint(equalTo: selectedTypeLabel.bottomAnchor, constant: 12),
            pickerStack.centerXAnchor.constraint(equalTo: footer.centerXAnchor),
            pickerStack.bottomAnchor.constraint(equalTo: footer.safeAreaLayoutGuide.bottomAnchor, constant: -16),
        ])
    }

    // MARK: - Actions

    @objc private func selectType(_ sender: UIButton) {
        selectedType = sender.tag
        selectedTypeLabel.text = objectTypes[selectedType].label
        for (i, btn) in typeButtons.enumerated() {
            btn.backgroundColor = i == selectedType ? UIColor.systemBlue.withAlphaComponent(0.3) : UIColor.white.withAlphaComponent(0.08)
        }
    }

    @objc private func clearAll() {
        placedNodes.forEach { $0.removeFromParentNode() }
        placedNodes.removeAll()
        updateClearButton()
    }

    @objc private func goBack() { dismiss(animated: true) }

    private func updateClearButton() {
        clearButton.setTitle(placedNodes.isEmpty ? "" : "CLEAR (\(placedNodes.count))", for: .normal)
    }

    private func startTrackingUpdates() {
        trackingTimer = Timer.scheduledTimer(withTimeInterval: 0.1, repeats: true) { [weak self] _ in
            guard let self = self,
                  let frame = self.sceneView.session.currentFrame else { return }
            let planes = frame.anchors.compactMap { $0 as? ARPlaneAnchor }

            switch frame.camera.trackingState {
            case .normal:
                self.trackingBadge.text = "  ARKIT • \(planes.count) planes  "
                self.trackingBadge.textColor = UIColor(red: 0.133, green: 0.773, blue: 0.369, alpha: 1)
                self.hintLabel.text = "TAP ON SURFACE TO PLACE"
            case .limited:
                self.trackingBadge.text = "  SCANNING...  "
                self.trackingBadge.textColor = UIColor.white.withAlphaComponent(0.4)
                self.hintLabel.text = planes.isEmpty ? "SCANNING FOR SURFACES..." : "SURFACES DETECTED, MOVE SLOWLY"
            default:
                self.trackingBadge.text = "  NOT AVAILABLE  "
            }
        }
    }
}
