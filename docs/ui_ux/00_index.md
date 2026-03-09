# EasyToGoKorea — UI/UX Design 명세 🎨 (Neo-Minimalism)

> **디자인 원칙**: *"Function First, Progressive Disclosure, Neo-Minimalism"*
> **최종 수정**: 2026-02-28

---

## 🧭 심플한 UI 핵심 디자인 원칙 (Best Practices)

1. **상시 절제 (Function First)**: 현재 화면의 주 목적 외의 요소는 모두 숨기거나 제거하여 시각적 피로도를 낮춥니다.
2. **여백과 입체감 (White Space & Soft Shadow)**: 넓은 여백으로 숨통을 틔우고, 날카로운 직사각형`[ ]` 대신 부드러운 곡선과 네오 미니멀리즘(Neo-minimalism)의 둥근 컴포넌트`( )`를 사용합니다.
3. **감성 컬러 (Warm Neutral)**: 퓨어 화이트 대신 크림색, 따뜻한 파스텔 톤 등 감성적인 색채를 베이스로 사용하여 친근감을 줍니다.
4. **점진적 공개 (Progressive Disclosure)**: 복잡한 기능(설정, 상세 통계)은 사용자가 원할 때만 펼쳐지도록 숨깁니다.

---

## 📂 파일 구성 (UI/UX ASCII Mockups)

이 문서는 `docs/ideas` 의 사업 기획을 바탕으로 작성된 모바일 앱(사용자용)의 화면 및 터치 흐름을 ASCII 아트로 시각화한 명세서입니다.

| # | 화면 | 주요 기능 | 연관 기획 문서 |
|---|------|----------|-------------|
| 01 | [홈 화면 (Dashboard)](01_home_screen.md) | 원터치 QR, 상황별 추천 카드 (점진적 노출) | `01_vision_platform`, `03_travel_tips` |
| 02 | [스마트 QR 스캐너](02_smart_qr_scanner.md) | 안심 위치 전송(SOS), AR 길안내, 식당 메뉴 번역 | `12_smart_qr_location`, `06_food_payment` |
| 03 | [일정 및 지도 (Map Hub)](03_map_itinerary.md) | 경로 플로팅, 템플스테이/성지 레이어, 오프라인 지도 | `02_itinerary_info` |
| 04 | [문화 체류 플랫폼 (Culture)](04_culture_booking.md) | K-POP 팁 후원, 템플스테이 원터치 예약 | `04_culture_experience` |
| 05 | [소셜 여행 피드 (Social API)](05_social_feed.md) | 미니멀 피드, 업적 배지 하일라이트, 동행 매칭 | `11_social_travel_feed` |
| 06 | [축제 및 셔틀 버스 (Festivals)](06_festivals_shuttle.md) | 캘린더, 축제 예약, 셔틀 버스 QR 탑승권 | `05_festivals_shuttle` |
| 07 | [식당 메뉴 번역 및 결제 (Food & Pay)](07_food_payment.md) | QR 스캔 후 다국어 메뉴, 분할 결제, 원격 팁 | `06_food_payment` |
| 08 | [지역 특산품 마켓 (Market)](08_local_market.md) | 로컬 마켓 다국어 뷰, 해외 직배송 및 픽업 | `07_local_market` |
| 09 | [의료 및 생명보험 지원 (Medical)](09_medical_insurance.md) | 외국인 친화 병원 예약, 서류 자동 OCR 청구 | `08_medical_living`, `04_medical_insurance` |
| 10 | [AR 네비게이션 가이드 (AR Guide)](10_ar_guide.md) | AR 경로 표시, 떠다니는 정보 핀, 긴급 SOS | `12_smart_qr_location` |
| 11 | [유저 프로필 및 월렛 (Profile View)](11_profile_settings.md) | 배지, 여권 연동 디지털 패스 월렛, 티켓 보관소 | `00_overview`, `01_vision_platform` |
| 12 | [챗 인박스 및 고객센터 (Chat & Support)](12_chat_inbox.md) | 1:1 통역/CS 채팅, AI 통합 검색 챗봇 | `07_global_messenger_integration`, `13_ondevice_ai_android`, `14_ondevice_ai_ios` |
| 13 | [생활 및 비자 가이드 (Living & Visa)](13_living_visa_guide.md) | 실시간 전화 통역, 비자/체류 가이드 | `08_medical_living` |
| 14 | [실시간 오픈 채팅 (Real-time Chat)](14_realtime_location_chat.md) | 오버레이 HUD 채팅, 지역/언어 기반 실시간 커뮤니케이션 | `15_realtime_location_chat` |
| 15 | [스마트 스토어 알림 (Smart Alert)](15_smart_store_alert.md) | 디지털 진동벨, 스마트 웨이팅, 대기열 실시간 확인 | `16_smart_store_alert` |
---

## 📱 공통 레이아웃 (Global Layout)

불필요한 선과 면적을 없애 콘텐츠가 부유하는 듯한 느낌을 줍니다.

```text
╭───────────────────────────────────────╮
│  ( 👑 배지 )                  ( 🔍 )  │
│                                       │
│  ╭─────────────────────────────────╮  │
│  │                                 │  │
│  │        Main Content Area        │  │
│  │      - 둥글고 부드러운 카드들   │  │
│  │      - 뚜렷한 경계선 표현       │  │
│  │                                 │  │
│  ╰─────────────────────────────────╯  │
│                                       │
│  ╭─────────────────────────────────╮  │
│  │    ( 🗺️ )    ( QR )    ( 👤 )    │  │
│  │     Home      Scan    Profile   │  │
│  ╰─────────────────────────────────╯  │
╰───────────────────────────────────────╯
```

### 중앙의 거대한 ( QR ) 버튼
앱의 메뉴 구조를 극단적으로 단순화하여, 사용자는 대부분의 행동을 **'QR 스캔 하나로'** 해결합니다.
- 식당 메뉴판 스캔 → `메뉴 번역 및 결제 화면으로 자동 이동`
- 가로등/전봇대 스캔 → `현재 위치(SOS) 및 AR 네비게이션으로 자동 이동`
- 성지 스캔 → `스탬프 획득 화면으로 자동 이동`

따라서 **모든 화면의 중앙 하단에는 플로팅 형태의 커다란 카메라(QR) 버튼이 항상 존재**합니다.
