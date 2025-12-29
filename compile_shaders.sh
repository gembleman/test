#!/bin/bash

# Shader 컴파일 스크립트
# Vulkan SDK가 설치되어 있어야 합니다

echo "셰이더 컴파일 중..."

# shaders 디렉토리 존재 확인
if [ ! -d "shaders" ]; then
    echo "Error: shaders 디렉토리를 찾을 수 없습니다"
    exit 1
fi

# glslc 명령어 확인
if ! command -v glslc &> /dev/null; then
    echo "Error: glslc를 찾을 수 없습니다. Vulkan SDK가 설치되어 있는지 확인하세요"
    exit 1
fi

# Vertex shader 컴파일
echo "Vertex shader 컴파일..."
glslc shaders/shader.vert -o shaders/vert.spv
if [ $? -ne 0 ]; then
    echo "Error: Vertex shader 컴파일 실패"
    exit 1
fi

# Fragment shader 컴파일
echo "Fragment shader 컴파일..."
glslc shaders/shader.frag -o shaders/frag.spv
if [ $? -ne 0 ]; then
    echo "Error: Fragment shader 컴파일 실패"
    exit 1
fi

echo "셰이더 컴파일 완료!"
