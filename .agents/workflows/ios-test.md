---
description: iOS 테스트 빌드 (GitHub Actions macOS Runner + AltStore Sideload)
---

# iOS 테스트 빌드 & 설치 워크플로우

Tauri v2 앱을 GitHub Actions macOS Runner에서 iOS용으로 빌드하고,
AltStore를 통해 iPhone에 사이드로딩하여 테스트하는 워크플로우입니다.

---

## 사전 준비 (1회만)

### 1. Apple Developer Account 확인
- **무료 계정**: 7일간 사이드로딩 가능 (테스트용)
- **유료 계정 ($99/년)**: 1년간 유효, TestFlight 배포 가능
- https://developer.apple.com 에서 Apple ID로 로그인

### 2. AltStore 설치 (Windows PC)
// turbo
```
winget install AltStore.AltServer
```
- 또는 https://altstore.io 에서 AltServer for Windows 다운로드
- AltServer 실행 → iPhone을 USB로 연결 → AltStore 설치

### 3. GitHub 리포지토리에 Secrets 등록
GitHub → Settings → Secrets → Actions에 추가:
- `APPLE_ID`: Apple 계정 이메일
- `APPLE_PASSWORD`: 앱 비밀번호 (appleid.apple.com → 앱 암호 생성)
- `APPLE_TEAM_ID`: 팀 ID (developer.apple.com → Membership)

---

## GitHub Actions 워크플로우 설정

### 4. 워크플로우 파일 생성
// turbo
`.github/workflows/ios-build.yml` 파일을 아래 내용으로 생성:

```yaml
name: iOS Test Build

on:
  workflow_dispatch:  # 수동 트리거

env:
  APP_DIR: mobile  # Tauri 앱 디렉토리 (구 app/)

jobs:
  build-ios:
    runs-on: macos-14  # Apple Silicon M1 runner
    
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-ios

      - name: Install Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install Tauri CLI
        run: cargo install tauri-cli --version "^2"

      - name: Install frontend deps
        working-directory: ${{ env.APP_DIR }}
        run: npm install

      - name: Initialize Tauri iOS
        working-directory: ${{ env.APP_DIR }}
        run: cargo tauri ios init

      - name: Build iOS (Debug)
        working-directory: ${{ env.APP_DIR }}
        run: cargo tauri ios build --debug

      - name: Upload IPA
        uses: actions/upload-artifact@v4
        with:
          name: ios-debug-build
          path: ${{ env.APP_DIR }}/src-tauri/gen/apple/build/**/*.ipa
          retention-days: 7
```

---

## iPhone에 설치 (AltStore)

### 5. GitHub에서 IPA 다운로드
- GitHub → Actions → "iOS Test Build" → Run workflow
- 완료 후 Artifacts에서 `ios-debug-build.zip` 다운로드
- .ipa 파일 추출

### 6. AltStore로 iPhone에 설치
1. PC에서 AltServer 실행
2. iPhone을 USB로 PC에 연결
3. iPhone에서 AltStore 앱 열기
4. My Apps → + 버튼 → 다운로드한 .ipa 파일 선택
5. Apple ID 입력 → 설치 완료

### 7. 신뢰 설정 (최초 1회)
- iPhone → 설정 → 일반 → VPN 및 기기 관리
- 개발자 앱 → 해당 Apple ID 선택 → 신뢰

---

## 빠른 로컬 테스트 (macOS가 있는 경우)

Mac이 있으면 GitHub Actions 없이 직접 빌드 가능:
```bash
cd mobile
cargo tauri ios init
cargo tauri ios dev  # iPhone을 USB로 연결한 상태
```

---

## 트러블슈팅

| 문제 | 해결 |
|------|------|
| Code signing 오류 | Xcode에서 Automatic Signing 활성화 |
| AltStore 7일 만료 | AltServer 실행 상태에서 자동 갱신됨 |
| 빌드 실패 (종속성) | `cargo tauri ios init` 재실행 |
| ONNX Runtime iOS | `ort` 크레이트가 iOS aarch64 지원. Cargo.toml에서 target 설정 필요 |
