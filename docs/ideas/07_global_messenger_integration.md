# 아이디어 07: 글로벌 메신저 연동 방안

> **문서 위치**: `docs/ideas/07_global_messenger_integration.md`  
> **관련 도메인**: Chat Service, Notification, Marketing  
> **최종 수정**: 2026-02-27

---

## 1. 개요
외국인 관광객은 한국 방문 시 새로운 앱(예: 카카오톡)을 설치하기보다 본국에서 사용하던 익숙한 메신저를 그대로 사용하고자 하는 성향이 강합니다. **EasyToGoKorea**의 기존 인앱 `chat-service`를 글로벌 메신저와 연동하면 사용자의 접근성을 극대화하고 앱 설치 허들을 낮출 수 있습니다.

## 2. 타겟 국가별 주요 타겟 메신저
- **글로벌/북미/인도/동남아**: **WhatsApp** (가장 범용적)
- **일본/대만/태국**: **LINE**
- **중국**: **WeChat** (결제 및 미니앱 생태계 필수)
- **러시아/CIS/유럽 일부**: **Telegram**
- **기타 범용**: **Facebook Messenger** / **Instagram Direct**

---

## 3. 주요 연동 시나리오 (Use Cases)

### A. 옴니채널 실시간 상담 (Omnichannel CS)
현재 설계된 `chat-service`(여행사·통역 매칭)를 외부 메신저와 브릿지(Bridge)합니다.
- **작동 방식**: 사용자가 자신의 WhatsApp으로 메시지를 보내면, EasyToGoKorea의 파트너(여행사/통역사)는 내부 어드민이나 앱의 `chat-service` 인박스에서 수신하고 답변합니다. 답변은 다시 사용자의 WhatsApp으로 전송됩니다.
- **기대 효과**: 사용자가 계속 앱을 켜두지 않아도 익숙한 메신저로 상담 및 통역 서비스를 이용할 수 있습니다.

### B. 중요 알림 푸시 (Notification Outbound)
단순 앱 푸시(FCM/APNs) 외에 메신저 공식 계정을 통한 알림을 제공합니다.
- **발송 내용**: 병원 예약 확정, 셔틀버스 탑승 QR 리마인더, 템플스테이 체크인 안내, 보험 청구 완료 알림 등.
- **기대 효과**: 네트워크 문제나 앱 알림 설정 꺼짐 상태에서도 메시지 도달률(Delivery Rate)을 90% 이상으로 유지할 수 있습니다.

### C. 챗봇 기반 간편 서비스 (Inbound Chatbot)
자연어 처리(NLP) 기반의 챗봇이나 메신저 리치 메뉴(Rich Menu)를 활용합니다.
- **제공 기능**:
  - 간편 길찾기 (명소 이름 입력 시 Valhalla 기반 경로 링크 회신)
  - 1차 FAQ 대응 (응급실 위치, 1330 핫라인 연결, 교통카드 안내)
- **기대 효과**: 앱을 다운로드하지 않은 "라이트 유저(Light User)"도 기본적인 EasyToGoKorea의 서비스를 체험할 수 있습니다.

### D. 소셜 로그인 및 인앱 공유 (Social Auth & Share)
- **소셜 로그인**: LINE Login, WeChat Login 을 Auth Service에 통합하여 회원가입을 3초 이내로 단축.
- **일정 공유**: EasyToGoKorea에서 플래닝한 일정을 "WhatsApp으로 공유하기", "LINE으로 공유하기" 버튼을 통해 친구들에게 딥링크(Deep Link) 형태로 전달.

---

## 4. 시스템 아키텍처 관점의 통합 방안

지금의 모듈러 모놀리스/MSA 구조 중 `chat-service`와 `notification-service`를 확장해야 합니다.

1. **Webhook Gateway 구성**: WhatsApp (Twilio/Meta API), LINE (Messaging API), WeChat (Official Account API) 등에서 들어오는 웹훅 요청을 단일 포맷으로 정규화하는 Gateway 레이어 추가.
2. **CPaaS 솔루션 도입 고려**: 초기 개발 리소스 절감을 위해 직접 연동보다는 **Twilio**, **Sendbird**, **채널톡(Channel Talk)** 등 다중 채널을 통합 지원하는 3rd-party SaaS를 `chat-service`의 백엔드로 활용하는 것도 좋은 전략입니다.

---

## 5. 단계별 도입 로드맵 제안

### Phase 1 (MVP ~ 오픈 초기) : 소셜 인증 및 단방향 알림
- **목표**: 획득(Acquisition) 허들 낮추기
- WhatsApp / LINE 기반 소셜 로그인.
- Twilio 등을 활용한 예약/결제 확정 단방향 알림 발송.

### Phase 2 (MAU 5만 이상) : 양방향 CS 및 챗봇 연동
- **목표**: 리텐션(Retention) 및 사용자 경험 강화
- WhatsApp Business / LINE Official Account 런칭.
- `chat-service`와 통역사/여행사 매칭 대화를 메신저로 양방향 Webhook 연동.
- 1차 FAQ 대응용 단답형 챗봇 적용.

### Phase 3 (글로벌 확장기) : 위챗 미니프로그램 & 미니앱
- **목표**: 중화권 및 주요 아시아 시장 락인(Lock-in)
- WeChat 미니앱 생태계에 편입하여, 앱 설치 없이도 위챗 내에서 식당 메뉴 번역, 결제(WeChat Pay), 특산품 쇼핑까지 원스톱으로 가능하도록 개발.
