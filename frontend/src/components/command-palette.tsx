import React, { useState, useEffect } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'

import { Badge } from '@/components/ui/badge'
import { 
  Search, 
  Command, 
  Settings, 
  Brain, 
  GitBranch, 
  Database, 
  FileText,
  Zap,
  Users,
  Shield,
  Globe,
  Monitor
} from 'lucide-react'
import { useHotkeys } from 'react-hotkeys-hook'

interface CommandItem {
  id: string
  title: string
  description: string
  icon: React.ReactNode
  category: string
  action: () => void
  shortcut?: string
}

interface CommandPaletteProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function CommandPalette({ open, onOpenChange }: CommandPaletteProps) {
  const [search, setSearch] = useState('')
  const [selectedIndex, setSelectedIndex] = useState(0)

  const commands: CommandItem[] = [
    {
      id: 'mission-control',
      title: 'Mission Control',
      description: 'Monitor and orchestrate JARVIS execution plans',
      icon: <Monitor className="w-4 h-4" />,
      category: 'Navigation',
      action: () => {
        window.location.href = '/mission-control'
        onOpenChange(false)
      },
      shortcut: '⌘M'
    },
    {
      id: 'memory-vault',
      title: 'Memory Vault',
      description: 'Access and manage JARVIS memory systems',
      icon: <Brain className="w-4 h-4" />,
      category: 'Navigation',
      action: () => {
        window.location.href = '/memory-vault'
        onOpenChange(false)
      },
      shortcut: '⌘V'
    },
    {
      id: 'intent-processor',
      title: 'Intent Processor',
      description: 'Create and manage execution intents',
      icon: <FileText className="w-4 h-4" />,
      category: 'Tools',
      action: () => {
        window.location.href = '/intent'
        onOpenChange(false)
      },
      shortcut: '⌘I'
    },
    {
      id: 'settings',
      title: 'Settings',
      description: 'Configure JARVIS and application preferences',
      icon: <Settings className="w-4 h-4" />,
      category: 'System',
      action: () => {
        window.location.href = '/settings'
        onOpenChange(false)
      },
      shortcut: '⌘,'
    },
    {
      id: 'cognitive-kernel',
      title: 'Cognitive Kernel',
      description: 'Access JARVIS cognitive processing systems',
      icon: <Brain className="w-4 h-4" />,
      category: 'System',
      action: () => {
        // Trigger cognitive kernel interface
        onOpenChange(false)
      }
    },
    {
      id: 'execution-dag',
      title: 'Execution DAG',
      description: 'View and manage task execution graphs',
      icon: <GitBranch className="w-4 h-4" />,
      category: 'Tools',
      action: () => {
        // Navigate to DAG view
        onOpenChange(false)
      }
    },
    {
      id: 'database',
      title: 'Database',
      description: 'Access and manage data storage systems',
      icon: <Database className="w-4 h-4" />,
      category: 'System',
      action: () => {
        // Open database interface
        onOpenChange(false)
      }
    },
    {
      id: 'security',
      title: 'Security',
      description: 'Manage security and access controls',
      icon: <Shield className="w-4 h-4" />,
      category: 'System',
      action: () => {
        // Open security panel
        onOpenChange(false)
      }
    },
    {
      id: 'network',
      title: 'Network',
      description: 'Monitor network connections and services',
      icon: <Globe className="w-4 h-4" />,
      category: 'System',
      action: () => {
        // Open network monitor
        onOpenChange(false)
      }
    },
    {
      id: 'users',
      title: 'Users',
      description: 'Manage user accounts and permissions',
      icon: <Users className="w-4 h-4" />,
      category: 'System',
      action: () => {
        // Open user management
        onOpenChange(false)
      }
    },
    {
      id: 'performance',
      title: 'Performance',
      description: 'Monitor system performance metrics',
      icon: <Zap className="w-4 h-4" />,
      category: 'System',
      action: () => {
        // Open performance monitor
        onOpenChange(false)
      }
    }
  ]

  const filteredCommands = commands.filter(command =>
    command.title.toLowerCase().includes(search.toLowerCase()) ||
    command.description.toLowerCase().includes(search.toLowerCase()) ||
    command.category.toLowerCase().includes(search.toLowerCase())
  )

  const categories = [...new Set(filteredCommands.map(cmd => cmd.category))]

  // Keyboard navigation
  useEffect(() => {
    if (!open) return

    const handleKeyDown = (e: KeyboardEvent) => {
      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault()
          setSelectedIndex(prev => 
            prev < filteredCommands.length - 1 ? prev + 1 : 0
          )
          break
        case 'ArrowUp':
          e.preventDefault()
          setSelectedIndex(prev => 
            prev > 0 ? prev - 1 : filteredCommands.length - 1
          )
          break
        case 'Enter':
          e.preventDefault()
          if (filteredCommands[selectedIndex]) {
            filteredCommands[selectedIndex].action()
          }
          break
        case 'Escape':
          e.preventDefault()
          onOpenChange(false)
          break
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [open, filteredCommands, selectedIndex, onOpenChange])

  // Reset state when opening
  useEffect(() => {
    if (open) {
      setSearch('')
      setSelectedIndex(0)
    }
  }, [open])

  // Hotkey to open command palette
  useHotkeys('cmd+k, ctrl+k', (e) => {
    e.preventDefault()
    onOpenChange(true)
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[600px] p-0">
        <DialogHeader className="px-6 py-4 border-b">
          <DialogTitle className="flex items-center gap-2">
            <Command className="w-5 h-5" />
            Command Palette
          </DialogTitle>
        </DialogHeader>
        
        <div className="p-6">
          <div className="relative mb-4">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground w-4 h-4" />
            <Input
              placeholder="Search commands..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-10"
              autoFocus
            />
          </div>

          <div className="space-y-4 max-h-[400px] overflow-y-auto">
            {categories.map(category => {
              const categoryCommands = filteredCommands.filter(cmd => cmd.category === category)
              
              return (
                <div key={category} className="space-y-2">
                  <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wide">
                    {category}
                  </h3>
                  <div className="space-y-1">
                                         {categoryCommands.map((command) => {
                      const globalIndex = filteredCommands.findIndex(cmd => cmd.id === command.id)
                      const isSelected = globalIndex === selectedIndex
                      
                      return (
                        <button
                          key={command.id}
                          onClick={command.action}
                          className={`w-full flex items-center justify-between p-3 rounded-md text-left transition-colors ${
                            isSelected 
                              ? 'bg-accent text-accent-foreground' 
                              : 'hover:bg-muted'
                          }`}
                          onMouseEnter={() => setSelectedIndex(globalIndex)}
                        >
                          <div className="flex items-center gap-3">
                            <div className="text-muted-foreground">
                              {command.icon}
                            </div>
                            <div>
                              <div className="font-medium">{command.title}</div>
                              <div className="text-sm text-muted-foreground">
                                {command.description}
                              </div>
                            </div>
                          </div>
                          {command.shortcut && (
                            <Badge variant="secondary" className="text-xs">
                              {command.shortcut}
                            </Badge>
                          )}
                        </button>
                      )
                    })}
                  </div>
                </div>
              )
            })}
          </div>

          {filteredCommands.length === 0 && (
            <div className="text-center py-8 text-muted-foreground">
              No commands found matching "{search}"
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
} 