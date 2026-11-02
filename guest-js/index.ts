/** Typed Tauri 2 IPC bindings for the Windows-native FXMP Dock. */
import { invoke } from '@tauri-apps/api/core'

export interface DockStatus {
  visible: boolean
  supported: boolean
}
export const showDock = (): Promise<void> => invoke('plugin:dock|show')
export const hideDock = (): Promise<void> => invoke('plugin:dock|hide')
export const toggleDock = (): Promise<void> => invoke('plugin:dock|toggle')
export const closeDock = (): Promise<void> => invoke('plugin:dock|close')
export const openDockSettings = (): Promise<void> => invoke('plugin:dock|open_settings')
export const setDockMonitor = (monitorId: string): Promise<void> =>
  invoke('plugin:dock|set_monitor', { monitorId })
export const getDockStatus = (): Promise<DockStatus> => invoke('plugin:dock|get_status')
