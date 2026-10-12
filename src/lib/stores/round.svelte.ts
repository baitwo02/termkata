/**
 * 宿主状态的本地镜像（MVU 的 Model）。
 *
 * 命令返回新状态后赋值，Svelte 的响应式系统自动重渲染；
 * UI 从不自己改状态，只调用动作。
 */
import * as api from '$lib/platform/round';
import type { RoundState } from '$lib/types/round';

let current = $state<RoundState>({ phase: 'ready', word: null, error: null });

/** 宿主调用失败时的提示（IPC 层错误，与"词生成失败"是两回事）。 */
let hostError = $state<string | null>(null);

/** 调用宿主命令并把返回值同步到本地镜像；失败时记录并显示，而不是静默。 */
async function callHost(action: () => Promise<RoundState>): Promise<void> {
  try {
    current = await action();
    hostError = null;
  } catch (error) {
    hostError = error instanceof Error ? error.message : String(error);
    console.error('宿主调用失败：', error);
  }
}

export const round = {
  get state(): RoundState {
    return current;
  },

  get hostError(): string | null {
    return hostError;
  },

  /** 启动时从宿主同步一次状态。 */
  async refresh(): Promise<void> {
    await callHost(api.getState);
  },

  /** 抽取：先进入生成中（渲染加载态），再等待生成结果（真实网络耗时）。 */
  async draw(): Promise<void> {
    await callHost(api.beginDraw);
    if (hostError) return;
    await callHost(api.finishDraw);
  },

  /** 准备结束，开始口播。 */
  async speak(): Promise<void> {
    await callHost(api.startSpeaking);
  },

  /** 结束口播，回到就绪。 */
  async end(): Promise<void> {
    await callHost(api.endSpeaking);
  },
};
