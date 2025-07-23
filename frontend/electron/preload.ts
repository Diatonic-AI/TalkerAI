import { contextBridge, ipcRenderer } from 'electron'

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electronAPI', {
  // App controls
  getVersion: () => ipcRenderer.invoke('app:getVersion'),
  getName: () => ipcRenderer.invoke('app:getName'),
  getPlatform: () => ipcRenderer.invoke('app:getPlatform'),
  quit: () => ipcRenderer.invoke('app:quit'),

  // Window controls
  minimize: () => ipcRenderer.invoke('window:minimize'),
  maximize: () => ipcRenderer.invoke('window:maximize'),
  close: () => ipcRenderer.invoke('window:close'),

  // Menu actions
  onMenuAction: (callback: (action: string) => void) => {
    ipcRenderer.on('menu:new-intent', () => callback('new-intent'))
    ipcRenderer.on('navigate:mission-control', () => callback('mission-control'))
    ipcRenderer.on('navigate:memory-vault', () => callback('memory-vault'))
    ipcRenderer.on('show:command-palette', () => callback('command-palette'))
  },

  // Remove listeners
  removeAllListeners: () => {
    ipcRenderer.removeAllListeners('menu:new-intent')
    ipcRenderer.removeAllListeners('navigate:mission-control')
    ipcRenderer.removeAllListeners('navigate:memory-vault')
    ipcRenderer.removeAllListeners('show:command-palette')
  }
})

// Types for TypeScript
export interface ElectronAPI {
  getVersion: () => Promise<string>
  getName: () => Promise<string>
  getPlatform: () => Promise<string>
  quit: () => Promise<void>
  minimize: () => Promise<void>
  maximize: () => Promise<void>
  close: () => Promise<void>
  onMenuAction: (callback: (action: string) => void) => void
  removeAllListeners: () => void
}

declare global {
  interface Window {
    electronAPI: ElectronAPI
  }
} 