/**
 * 宿主状态的本地镜像（MVU 的 Model）。
 *
 * 命令返回新状态后赋值，Svelte 的响应式系统自动重渲染；
 * UI 从不自己改状态，只调用动作。
 */
import * as api from '$lib/platform/round';
import type { RoundState } from '$lib/types/round';

let current = $state<RoundState>({ phase: 'ready', word: null, error: null });

/** 临时延迟：FakeApi 没有网络耗时，便于观察加载态；真实 API 接入后删除。 */
const FAKE_LATENCY_MS = 400;

export const round = {
  get state(): RoundState {
    return current;
  },

  /** 启动时从宿主同步一次状态。 */
  async refresh(): Promise<void> {
    current = await api.getState();
  },

  /** 抽取：先进入生成中（渲染加载态），再等待生成结果。 */
  async draw(): Promise<void> {
    current = await api.beginDraw();
    await new Promise((resolve) => setTimeout(resolve, FAKE_LATENCY_MS));
    current = await api.finishDraw();
  },

  /** 准备结束，开始口播。 */
  async speak(): Promise<void> {
    current = await api.startSpeaking();
  },

  /** 结束口播，回到就绪。 */
  async end(): Promise<void> {
    current = await api.endSpeaking();
  },
};
