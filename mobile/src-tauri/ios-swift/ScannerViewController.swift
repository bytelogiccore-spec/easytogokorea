import UIKit
import AVFoundation

/// Native camera scanner view controller for iOS.
/// Uses AVCaptureSession for camera preview + scan overlay UI.
class ScannerViewController: UIViewController {

    private var captureSession: AVCaptureSession?
    private var previewLayer: AVCaptureVideoPreviewLayer?
    private var resultCard: UIView!
    private var resultOriginal: UILabel!
    private var resultTranslated: UILabel!
    private var resultPrice: UILabel!
    private var scanTitle: UILabel!
    private var flashOn = false

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .black
        setupCamera()
        setupOverlay()
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            self?.captureSession?.startRunning()
        }
    }

    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        captureSession?.stopRunning()
    }

    override var prefersStatusBarHidden: Bool { true }

    // MARK: - Camera Setup

    private func setupCamera() {
        let session = AVCaptureSession()
        session.sessionPreset = .high

        guard let device = AVCaptureDevice.default(.builtInWideAngleCamera, for: .video, position: .back),
              let input = try? AVCaptureDeviceInput(device: device) else { return }

        if session.canAddInput(input) {
            session.addInput(input)
        }

        let preview = AVCaptureVideoPreviewLayer(session: session)
        preview.videoGravity = .resizeAspectFill
        preview.frame = view.bounds
        view.layer.addSublayer(preview)

        captureSession = session
        previewLayer = preview
    }

    // MARK: - UI Overlay

    private func setupOverlay() {
        // Back button
        let backBtn = UIButton(type: .system)
        backBtn.setTitle("←", for: .normal)
        backBtn.titleLabel?.font = .boldSystemFont(ofSize: 28)
        backBtn.setTitleColor(.white, for: .normal)
        backBtn.translatesAutoresizingMaskIntoConstraints = false
        backBtn.addTarget(self, action: #selector(goBack), for: .touchUpInside)
        view.addSubview(backBtn)

        // Scan frame
        let scanFrame = UIView()
        scanFrame.layer.borderColor = UIColor.white.withAlphaComponent(0.3).cgColor
        scanFrame.layer.borderWidth = 2
        scanFrame.layer.cornerRadius = 24
        scanFrame.translatesAutoresizingMaskIntoConstraints = false
        let tap = UITapGestureRecognizer(target: self, action: #selector(simulateScan))
        scanFrame.addGestureRecognizer(tap)
        view.addSubview(scanFrame)

        // Corner decorations
        addCorners(to: scanFrame)

        // Scan title
        scanTitle = UILabel()
        scanTitle.text = "Scan Menu\nPoint at Korean text"
        scanTitle.font = .boldSystemFont(ofSize: 16)
        scanTitle.textColor = UIColor.white.withAlphaComponent(0.5)
        scanTitle.textAlignment = .center
        scanTitle.numberOfLines = 2
        scanTitle.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(scanTitle)

        // Result card (hidden)
        resultCard = UIView()
        resultCard.backgroundColor = UIColor(white: 0.12, alpha: 0.9)
        resultCard.layer.cornerRadius = 16
        resultCard.isHidden = true
        resultCard.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(resultCard)

        resultOriginal = UILabel()
        resultOriginal.font = .boldSystemFont(ofSize: 24)
        resultOriginal.textColor = .white
        resultOriginal.textAlignment = .center
        resultOriginal.translatesAutoresizingMaskIntoConstraints = false
        resultCard.addSubview(resultOriginal)

        resultTranslated = UILabel()
        resultTranslated.font = .boldSystemFont(ofSize: 13)
        resultTranslated.textColor = UIColor(white: 0.64, alpha: 1)
        resultTranslated.textAlignment = .center
        resultTranslated.translatesAutoresizingMaskIntoConstraints = false
        resultCard.addSubview(resultTranslated)

        resultPrice = UILabel()
        resultPrice.font = .boldSystemFont(ofSize: 16)
        resultPrice.textColor = UIColor(red: 0.145, green: 0.388, blue: 0.922, alpha: 1)
        resultPrice.textAlignment = .center
        resultPrice.translatesAutoresizingMaskIntoConstraints = false
        resultCard.addSubview(resultPrice)

        // Flash button
        let flashBtn = UIButton(type: .system)
        flashBtn.setTitle("FLASH ON", for: .normal)
        flashBtn.titleLabel?.font = .boldSystemFont(ofSize: 12)
        flashBtn.setTitleColor(.white, for: .normal)
        flashBtn.translatesAutoresizingMaskIntoConstraints = false
        flashBtn.addTarget(self, action: #selector(toggleFlash(_:)), for: .touchUpInside)
        view.addSubview(flashBtn)

        // Layout
        NSLayoutConstraint.activate([
            backBtn.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 16),
            backBtn.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 24),

            scanFrame.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            scanFrame.centerYAnchor.constraint(equalTo: view.centerYAnchor, constant: -30),
            scanFrame.widthAnchor.constraint(equalTo: view.widthAnchor, multiplier: 0.8),
            scanFrame.heightAnchor.constraint(equalTo: scanFrame.widthAnchor),

            scanTitle.topAnchor.constraint(equalTo: scanFrame.bottomAnchor, constant: 20),
            scanTitle.centerXAnchor.constraint(equalTo: view.centerXAnchor),

            resultCard.topAnchor.constraint(equalTo: scanFrame.bottomAnchor, constant: 16),
            resultCard.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            resultCard.widthAnchor.constraint(equalTo: view.widthAnchor, multiplier: 0.8),

            resultOriginal.topAnchor.constraint(equalTo: resultCard.topAnchor, constant: 16),
            resultOriginal.centerXAnchor.constraint(equalTo: resultCard.centerXAnchor),

            resultTranslated.topAnchor.constraint(equalTo: resultOriginal.bottomAnchor, constant: 4),
            resultTranslated.centerXAnchor.constraint(equalTo: resultCard.centerXAnchor),

            resultPrice.topAnchor.constraint(equalTo: resultTranslated.bottomAnchor, constant: 8),
            resultPrice.centerXAnchor.constraint(equalTo: resultCard.centerXAnchor),
            resultPrice.bottomAnchor.constraint(equalTo: resultCard.bottomAnchor, constant: -16),

            flashBtn.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -20),
            flashBtn.centerXAnchor.constraint(equalTo: view.centerXAnchor),
        ])
    }

    private func addCorners(to container: UIView) {
        let size: CGFloat = 28
        let width: CGFloat = 4
        for (h, v) in [(CGRectEdge.minXEdge, CGRectEdge.minYEdge),
                        (CGRectEdge.maxXEdge, CGRectEdge.minYEdge),
                        (CGRectEdge.minXEdge, CGRectEdge.maxYEdge),
                        (CGRectEdge.maxXEdge, CGRectEdge.maxYEdge)] {
            let hBar = UIView()
            hBar.backgroundColor = .white
            hBar.translatesAutoresizingMaskIntoConstraints = false
            container.addSubview(hBar)

            let vBar = UIView()
            vBar.backgroundColor = .white
            vBar.translatesAutoresizingMaskIntoConstraints = false
            container.addSubview(vBar)

            var constraints: [NSLayoutConstraint] = [
                hBar.widthAnchor.constraint(equalToConstant: size),
                hBar.heightAnchor.constraint(equalToConstant: width),
                vBar.widthAnchor.constraint(equalToConstant: width),
                vBar.heightAnchor.constraint(equalToConstant: size),
            ]

            if h == .minXEdge {
                constraints.append(hBar.leadingAnchor.constraint(equalTo: container.leadingAnchor))
                constraints.append(vBar.leadingAnchor.constraint(equalTo: container.leadingAnchor))
            } else {
                constraints.append(hBar.trailingAnchor.constraint(equalTo: container.trailingAnchor))
                constraints.append(vBar.trailingAnchor.constraint(equalTo: container.trailingAnchor))
            }

            if v == .minYEdge {
                constraints.append(hBar.topAnchor.constraint(equalTo: container.topAnchor))
                constraints.append(vBar.topAnchor.constraint(equalTo: container.topAnchor))
            } else {
                constraints.append(hBar.bottomAnchor.constraint(equalTo: container.bottomAnchor))
                constraints.append(vBar.bottomAnchor.constraint(equalTo: container.bottomAnchor))
            }

            NSLayoutConstraint.activate(constraints)
        }
    }

    // MARK: - Actions

    @objc private func simulateScan() {
        resultOriginal.text = "된장찌개"
        resultTranslated.text = "Doenjang-jjigae (Soybean paste stew)"
        resultPrice.text = "₩9,000 ≈ $6.70"
        resultCard.isHidden = false
        scanTitle.isHidden = true

        DispatchQueue.main.asyncAfter(deadline: .now() + 4) { [weak self] in
            self?.resultCard.isHidden = true
            self?.scanTitle.isHidden = false
        }
    }

    @objc private func toggleFlash(_ sender: UIButton) {
        guard let device = AVCaptureDevice.default(for: .video), device.hasTorch else { return }
        flashOn.toggle()
        try? device.lockForConfiguration()
        device.torchMode = flashOn ? .on : .off
        device.unlockForConfiguration()
        sender.setTitle(flashOn ? "FLASH OFF" : "FLASH ON", for: .normal)
    }

    @objc private func goBack() { dismiss(animated: true) }
}
