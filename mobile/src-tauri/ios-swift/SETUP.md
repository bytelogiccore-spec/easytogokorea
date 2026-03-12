# iOS ARKit 모듈 설정 가이드

## 파일 구조

```
ios-swift/
├── ArNavigationViewController.swift  — ARKit 내비게이션 (ARSCNView + HUD)
├── ArPlaceViewController.swift       — ARKit 오브젝트 배치 (레이캐스트 + SCNText)
├── ScannerViewController.swift       — AVFoundation 카메라 스캔
└── ArBridgeIOS.swift                 — WKScriptMessageHandler 브릿지
```

## macOS에서 설정 방법

### 1. Tauri iOS 초기화
```bash
cd app
npx tauri ios init
```

### 2. Swift 파일 복사
```bash
cp src-tauri/ios-swift/*.swift src-tauri/gen/ios/Sources/
```

### 3. Info.plist 권한 추가
```xml
<key>NSCameraUsageDescription</key>
<string>EasyToGo needs camera access for AR and scanning features</string>
<key>UIRequiredDeviceCapabilities</key>
<array>
    <string>arkit</string>
</array>
```
> Note: `arkit` capability를 required로 넣으면 ARKit 미지원 기기에서 설치 불가.
> optional로 하려면 `UIRequiredDeviceCapabilities`에서 제외.

### 4. ArBridge 등록 (Tauri iOS delegate)
Tauri 생성 파일에서 WebView 설정 시:
```swift
webView.configuration.userContentController.add(
    ArBridgeIOS(viewController: self),
    name: "ArBridge"
)
```

### 5. JavaScript 호출 (기존 Svelte 페이지)
Android와 iOS 브릿지 호출 방식이 다르므로 Svelte에서 분기:
```javascript
// Android: window.ArBridge.openAR()
// iOS:     window.webkit.messageHandlers.ArBridge.postMessage({ action: "openAR" })
```

## Android vs iOS 대응표

| 기능 | Android | iOS |
|------|---------|-----|
| AR 프레임워크 | ARCore | ARKit |
| GL 렌더링 | GLSurfaceView + GLES20 | ARSCNView (SceneKit) |
| 카메라 | CameraX | AVFoundation |
| JS 브릿지 | @JavascriptInterface | WKScriptMessageHandler |
| 평면 감지 | getAllTrackables(Plane) | ARPlaneAnchor |
| 히트 테스트 | frame.hitTest() | session.raycast() |
| LiDAR | ❌ | ✅ sceneReconstruction |
