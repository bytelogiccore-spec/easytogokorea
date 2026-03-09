# 14. 온디바이스 AI 및 라우팅 제어 (iOS)

## 1. 개요
Apple 기기의 독자적인 프레임워크(App Intents, Translation, Apple Intelligence)를 활용하여 `EasyToGoKorea` 앱의 기능을 시스템 AI(Siri, Spotlight)에 노출시키고, 오프라인 실시간 번역 및 자연어 명령을 통한 앱 내 자동 제어를 구현하는 기술 명세입니다.

## 2. 핵심 기술 스택
### 2.1. App Intents Framework
*   **목적**: 앱의 주요 기능이나 특정 화면으로 이동하는 상호작용 가능한 액션을 시스템 수준(Siri, 단축어 앱, iOS 시스템 검색 뷰)에 제공.
*   **방식**: 코드 레벨에서 `AppIntent` 프로토콜을 구현하여, 사용자의 자연어 발화에 따라 앱 내부의 특정 뷰모델 파라미터를 완성하고 즉석에서 해당 기능 화면으로 라우팅합니다.

### 2.2. Apple Intelligence (iOS 18+)
*   **목적**: 고도의 문맥 이해 능력을 기반으로, 복잡한 사용자 의도를 파악하여 앱 전반에 걸친 크로스-인텐트 동작 연쇄 수행.
*   **방식**: 화면 인지 기능을 통해 사용자가 보고 있는 식당 리뷰를 읽고, "이 식당으로 가는 길을 EasyToGoKorea 플래너에 추가해줘"와 같은 명령을 내렸을 때 앱 내 일정을 자동으로 생성합니다. 별도의 검색 및 클릭 단계를 크게 단축시킵니다.

### 2.3. System Translation Framework (iOS 전용)
*   **목적**: 빠르고 정확한 오프라인 다국어 인터페이스 및 서드파티 앱 내 실시간 번역 경험 제공.
*   **방식**: 외부 API 구축 없이 iOS 15부터 제공되는 시스템 내장 번역 프레임워크나 iOS 18의 전용 Translation API를 네이티브 콜로 가져다 씁니다. 애플의 NPU(Neural Engine) 기반으로 즉각적인 번역이 이루어지며 통신비용이 발생하지 않고 보안(오프라인)이 완벽합니다.

## 3. 주요 유스케이스
1.  **Siri 연동 딥 라우팅**: "Siri야, EasyToGoKorea에서 근처 K-Pop 체험이나 공연 찾아줘" -> 단축어/App Intents를 통해 앱이 백그라운드에서 열리며 즉시 가장 관련 있는 문화 체험(`04_culture_experience.md` 뷰) 리스트로 뷰 자동 점프.
2.  **화면 컨텍스트 기반 상황 자동화**: 약봉지나 의료 정보 화면 캡쳐 후 "여기가 어디 약국이야?" 질의 -> Apple Intelligence가 분석하고 EasyToGoKorea 앱의 헬프데스크(`08_medical_living.md`) 라우터로 전달하여 현재 위치를 기반으로 안내 화면 전개.
3.  **지연 없는 오프라인 Live Text AR 번역**: 외국인 사용자가 오프라인 상태에서 지하철역이나 상가 간판을 스캔(`scanner.html` 뷰 연동) -> iOS Translation 프레임워크 + Vision 프레임워크가 텍스트를 적출하고 실시간으로 AR 렌즈 위에 자국어로 합성 및 렌더링.

## 4. 지원 가능한 기기 목록 (iOS)
iOS 기기들은 기본적으로 신경망 엔진(Neural Engine)을 폭넓게 탑재하여 기기 전반에서 온디바이스 기능을 지원하나, 가장 최신 모델인 '초거대 AI 기술' 적용에는 램(RAM)과 칩셋 제약이 있습니다.

*   **Apple Intelligence 기반 고도화된 컨텍스트 인지 및 자율 제어 지원 기기**:
    *   **OS 요구사항**: iOS 18.1 이상.
    *   **iPhone**: iPhone 15 Pro, iPhone 15 Pro Max, iPhone 16 시리즈 전 모델 (16, 16 Plus, 16 Pro, 16 Pro Max)
    *   **iPad / Mac**: M1 칩(Apple Silicon) 이상 탑재된 최신 iPad Pro/Air 및 Mac 라인업
*   **System Translation 프레임워크 (오프라인 실시간 번역) 및 App Intents 기반 라우팅 지원 기기**:
    *   **하드웨어 요구사항**: A12 Bionic 칩(Neural Engine 2세대) 이상 탑재 기기.
    *   **iPhone 목록**: iPhone XS, XS Max, XR 시리즈 이후 출시된 **모든 iPhone 모델** (iPhone 11, 12, 13, 14, 기본 15 모델군 및 SE 2/3세대 등 포함). 지원 범위가 매우 넓습니다.
    *   **OS 요구사항**: iOS 15 이상 (Live Text 번역 및 강화된 Translation UI 사용).
