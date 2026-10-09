/**
 * 平台适配层：把宿主能力封装成简单函数（UI 不直接调用 Tauri API）。
 *
 * 未来 Web 版在此提供另一份实现（WASM 桥）。
 */
import { invoke } from '@tauri-apps/api/core';

import type { RoundState } from '$lib/types/round';

export function getState(): Promise<RoundState> {
  return invoke<RoundState>('get_state');
}

export function beginDraw(): Promise<RoundState> {
  return invoke<RoundState>('begin_draw');
}

export function finishDraw(): Promise<RoundState> {
  return invoke<RoundState>('finish_draw');
}

export function startSpeaking(): Promise<RoundState> {
  return invoke<RoundState>('start_speaking');
}

export function endSpeaking(): Promise<RoundState> {
  return invoke<RoundState>('end_speaking');
}
