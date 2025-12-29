#!/bin/bash

# 한글 폰트 다운로드 스크립트

echo "Noto Sans KR 폰트 다운로드 중..."

if command -v wget &> /dev/null; then
    wget https://github.com/google/fonts/raw/main/ofl/notosanskr/NotoSansKR-Regular.ttf
elif command -v curl &> /dev/null; then
    curl -L -O https://github.com/google/fonts/raw/main/ofl/notosanskr/NotoSansKR-Regular.ttf
else
    echo "Error: wget 또는 curl이 필요합니다"
    exit 1
fi

if [ -f "NotoSansKR-Regular.ttf" ]; then
    echo "✓ 폰트 다운로드 완료!"
    echo "이제 'cargo run'으로 프로그램을 실행할 수 있습니다."
else
    echo "Error: 폰트 다운로드 실패"
    exit 1
fi
