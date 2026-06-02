# GUI Gen Firmware

Tauri + React + Rust 기반의 펌웨어 생성 GUI 초기 프로젝트입니다.

## Stack

- Tauri v2
- React
- Vite
- Rust

## Development

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## Initial Scope

이 초기 scaffold는 다음 기능을 확장하기 위한 기반입니다.

- 펌웨어 payload 입력
- OTA header 생성 및 편집
- header hash 재계산
- header signature 생성
- firmware hash 계산
- 암호화 펌웨어 생성
- BIN/HEX 출력
