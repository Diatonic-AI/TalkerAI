import { create } from 'zustand'
import { subscribeWithSelector } from 'zustand/middleware'

interface AppState {
  // UI State
  sidebarCollapsed: boolean
  commandPaletteOpen: boolean
  theme: 'light' | 'dark' | 'system'
  
  // Electron API
  electronAPI: any | null
  
  // JARVIS State
  cognitiveKernelStatus: 'idle' | 'processing' | 'error'
  activeIntents: any[]
  executionPlans: any[]
  
  // Actions
  setSidebarCollapsed: (collapsed: boolean) => void
  setCommandPaletteOpen: (open: boolean) => void
  setTheme: (theme: 'light' | 'dark' | 'system') => void
  setElectronAPI: (api: any) => void
  setCognitiveKernelStatus: (status: 'idle' | 'processing' | 'error') => void
  addActiveIntent: (intent: any) => void
  removeActiveIntent: (intentId: string) => void
  updateExecutionPlan: (plan: any) => void
}

export const useAppStore = create<AppState>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    sidebarCollapsed: false,
    commandPaletteOpen: false,
    theme: 'system',
    electronAPI: null,
    cognitiveKernelStatus: 'idle',
    activeIntents: [],
    executionPlans: [],

    // Actions
    setSidebarCollapsed: (collapsed) => set({ sidebarCollapsed: collapsed }),
    
    setCommandPaletteOpen: (open) => set({ commandPaletteOpen: open }),
    
    setTheme: (theme) => {
      set({ theme })
      
      // Apply theme to document
      const root = document.documentElement
      if (theme === 'dark') {
        root.classList.add('dark')
      } else if (theme === 'light') {
        root.classList.remove('dark')
      } else {
        // System theme
        const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
        if (isDark) {
          root.classList.add('dark')
        } else {
          root.classList.remove('dark')
        }
      }
    },
    
    setElectronAPI: (api) => set({ electronAPI: api }),
    
    setCognitiveKernelStatus: (status) => set({ cognitiveKernelStatus: status }),
    
    addActiveIntent: (intent) => {
      const currentIntents = get().activeIntents
      set({ activeIntents: [...currentIntents, intent] })
    },
    
    removeActiveIntent: (intentId) => {
      const currentIntents = get().activeIntents
      set({ 
        activeIntents: currentIntents.filter(intent => intent.id !== intentId) 
      })
    },
    
    updateExecutionPlan: (plan) => {
      const currentPlans = get().executionPlans
      const existingIndex = currentPlans.findIndex(p => p.id === plan.id)
      
      if (existingIndex >= 0) {
        const updatedPlans = [...currentPlans]
        updatedPlans[existingIndex] = plan
        set({ executionPlans: updatedPlans })
      } else {
        set({ executionPlans: [...currentPlans, plan] })
      }
    },
  }))
)

// Subscribe to theme changes
useAppStore.subscribe(
  (state) => state.theme,
  (theme) => {
    localStorage.setItem('talk-plus-plus-theme', theme)
  }
)

// Initialize theme from localStorage
const savedTheme = localStorage.getItem('talk-plus-plus-theme') as 'light' | 'dark' | 'system' | null
if (savedTheme) {
  useAppStore.getState().setTheme(savedTheme)
} else {
  useAppStore.getState().setTheme('system')
} 