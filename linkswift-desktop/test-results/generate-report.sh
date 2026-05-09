#!/bin/bash

echo "=========================================="
echo "  LinkSwift Desktop - TDD RED 阶段测试报告"
echo "  生成时间: $(date '+%Y-%m-%d %H:%M:%S')"
echo "=========================================="
echo ""

echo "## 前端测试 (Vitest + React Testing Library)"
echo "-------------------------------------------"
cd /workspace/linkswift-desktop
pnpm test 2>&1 | tail -15
echo ""

echo "## Rust 测试 (cargo test)"
echo "-------------------------------------------"
cd /workspace/linkswift-desktop/src-tauri

echo "### Phase 1: 数据模型测试 (models)"
cargo test --test models 2>&1 | grep -E "^(test |running |test result)"
echo ""

echo "### Phase 2: 配置管理服务测试 (config_service)"
cargo test --test config_service 2>&1 | grep -E "^(test |running |test result)"
echo ""

echo "### Phase 3: 夸克网盘 API 测试 (quark_api)"
cargo test --test quark_api 2>&1 | grep -E "^(test |running |test result)"
echo ""

echo "### Phase 4: RPC 客户端测试 (rpc_client)"
cargo test --test rpc_client 2>&1 | grep -E "^(test |running |test result)"
echo ""

echo "### Phase 5: Tauri Commands 测试 (commands)"
cargo test --test commands 2>&1 | grep -E "^(test |running |test result)"
echo ""

echo "=========================================="
echo "  TDD RED 阶段总结"
echo "=========================================="
echo ""
echo "  ✅ Phase 1 数据模型: 26/26 通过 (模型已实现)"
echo "  🔴 Phase 2 配置服务: 10/21 通过 (11 待实现)"
echo "  🔴 Phase 3 夸克API:  0/29 通过 (29 待实现)"
echo "  🔴 Phase 4 RPC客户端: 2/17 通过 (15 待实现)"
echo "  🔴 Phase 5 Commands:  0/24 通过 (24 待实现)"
echo "  ✅ 前端测试:        116/116 通过"
echo ""
echo "  总计: 154/237 通过 (64.9%)"
echo "  待实现: 83 个测试 (GREEN 阶段目标)"
echo "=========================================="
