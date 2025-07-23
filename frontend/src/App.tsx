import { useEffect } from 'react'
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom'
import { Toaster } from '@/components/ui/toaster'
import { CommandPalette } from '@/components/command-palette'
import { Layout } from '@/components/layout'
import { PageTransition } from '@/components/page-transition'
import { useAppStore } from '@/store/app-store'
import { MissionControl } from '@/pages/mission-control'
import { MemoryVault } from '@/pages/memory-vault'
import { IntentProcessor } from '@/pages/intent-processor'
import { Settings } from '@/pages/settings'
import './App.css'

function App() {
  const { 
    setElectronAPI, 
    setCommandPaletteOpen,
    commandPaletteOpen 
  } = useAppStore()

  useEffect(() => {
    // Setup Electron API if available
    if (window.electronAPI) {
      setElectronAPI(window.electronAPI)
      
      // Handle menu actions
      window.electronAPI.onMenuAction((action: string) => {
        switch (action) {
          case 'command-palette':
            setCommandPaletteOpen(true)
            break
          case 'new-intent':
            // Handle new intent creation
            break
          case 'mission-control':
            window.location.href = '/mission-control'
            break
          case 'memory-vault':
            window.location.href = '/memory-vault'
            break
        }
      })

      // Cleanup on unmount
      return () => {
        window.electronAPI.removeAllListeners()
      }
    }
  }, [setElectronAPI, setCommandPaletteOpen])

  return (
    <Router>
      <div className="App">
        <Layout>
          <PageTransition>
            <Routes>
              <Route path="/" element={<Navigate to="/mission-control" replace />} />
              <Route path="/mission-control" element={<MissionControl />} />
              <Route path="/intent" element={<IntentProcessor />} />
              <Route path="/memory-vault" element={<MemoryVault />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
          </PageTransition>
        </Layout>
        
        {/* Global Components */}
        <CommandPalette 
          open={commandPaletteOpen}
          onOpenChange={setCommandPaletteOpen}
        />
        <Toaster />
      </div>
    </Router>
  )
}

export default App 