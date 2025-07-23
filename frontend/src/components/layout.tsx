import React, { useState, useEffect } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { 
  Menu, 
  X, 
  Brain, 
  GitBranch, 
  Database, 
  Settings, 
  Activity,
  Zap,
  Shield,
  Users,
  Globe,
  FileText,
  Monitor
} from 'lucide-react'
import { useAppStore } from '@/store/app-store'

interface LayoutProps {
  children: React.ReactNode
}

const navigationItems = [
  {
    id: 'mission-control',
    label: 'Mission Control',
    icon: <Monitor className="w-4 h-4" />,
    href: '/mission-control',
    description: 'Monitor and orchestrate JARVIS execution plans'
  },
  {
    id: 'memory-vault',
    label: 'Memory Vault',
    icon: <Brain className="w-4 h-4" />,
    href: '/memory-vault',
    description: 'Access and manage JARVIS memory systems'
  },
  {
    id: 'intent-processor',
    label: 'Intent Processor',
    icon: <FileText className="w-4 h-4" />,
    href: '/intent',
    description: 'Create and manage execution intents'
  },
  {
    id: 'execution-dag',
    label: 'Execution DAG',
    icon: <GitBranch className="w-4 h-4" />,
    href: '/dag',
    description: 'View and manage task execution graphs'
  },
  {
    id: 'database',
    label: 'Database',
    icon: <Database className="w-4 h-4" />,
    href: '/database',
    description: 'Access and manage data storage systems'
  },
  {
    id: 'performance',
    label: 'Performance',
    icon: <Zap className="w-4 h-4" />,
    href: '/performance',
    description: 'Monitor system performance metrics'
  },
  {
    id: 'security',
    label: 'Security',
    icon: <Shield className="w-4 h-4" />,
    href: '/security',
    description: 'Manage security and access controls'
  },
  {
    id: 'network',
    label: 'Network',
    icon: <Globe className="w-4 h-4" />,
    href: '/network',
    description: 'Monitor network connections and services'
  },
  {
    id: 'users',
    label: 'Users',
    icon: <Users className="w-4 h-4" />,
    href: '/users',
    description: 'Manage user accounts and permissions'
  },
  {
    id: 'settings',
    label: 'Settings',
    icon: <Settings className="w-4 h-4" />,
    href: '/settings',
    description: 'Configure JARVIS and application preferences'
  }
]

export function Layout({ children }: LayoutProps) {
  const { sidebarCollapsed, setSidebarCollapsed, cognitiveKernelStatus } = useAppStore()
  const location = useLocation()
  const [isNavigating, setIsNavigating] = useState(false)
  const [activeItem, setActiveItem] = useState('')

  // Update active item when location changes
  useEffect(() => {
    const currentItem = navigationItems.find(item => item.href === location.pathname)
    setActiveItem(currentItem?.id || '')
  }, [location.pathname])

  // Handle navigation with smooth transition
  const handleNavigation = (href: string) => {
    setIsNavigating(true)
    // Small delay to show transition
    setTimeout(() => {
      setIsNavigating(false)
    }, 150)
  }

  return (
    <div className="flex h-screen bg-background">
      {/* Sidebar */}
      <div className={`flex flex-col bg-card border-r sidebar-transition ${
        sidebarCollapsed ? 'w-16' : 'w-64'
      }`}>
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b">
          {!sidebarCollapsed && (
            <div className="flex items-center gap-2 animate-in">
              <Brain className="w-6 h-6 text-primary" />
              <span className="font-semibold text-lg">JARVIS</span>
            </div>
          )}
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setSidebarCollapsed(!sidebarCollapsed)}
            className="ml-auto hover-lift focus-ring"
          >
            {sidebarCollapsed ? <Menu className="w-4 h-4" /> : <X className="w-4 h-4" />}
          </Button>
        </div>

        {/* Navigation */}
        <nav className="flex-1 p-2 space-y-1">
          {navigationItems.map((item) => {
            const isActive = location.pathname === item.href
            return (
              <Link
                key={item.id}
                to={item.href}
                onClick={() => handleNavigation(item.href)}
                className={`group relative flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium nav-link focus-ring ${
                  isActive
                    ? 'nav-item-active text-primary-foreground'
                    : 'text-muted-foreground hover:text-foreground hover:bg-accent/50'
                } ${isNavigating && isActive ? 'loading-pulse' : ''}`}
                title={sidebarCollapsed ? item.label : undefined}
              >
                {/* Active indicator */}
                {isActive && (
                  <div className="absolute left-0 top-0 bottom-0 w-1 bg-primary rounded-r-full animate-in" />
                )}
                
                {/* Icon with smooth color transition */}
                <div className={`transition-colors duration-200 ${
                  isActive ? 'text-primary-foreground' : 'text-muted-foreground group-hover:text-foreground'
                }`}>
                  {item.icon}
                </div>
                
                {/* Label with smooth opacity transition */}
                {!sidebarCollapsed && (
                  <span className={`truncate transition-all duration-200 ${
                    isActive ? 'text-primary-foreground' : 'text-muted-foreground group-hover:text-foreground'
                  }`}>
                    {item.label}
                  </span>
                )}
                
                {/* Hover effect overlay */}
                <div className="absolute inset-0 rounded-md bg-gradient-to-r from-transparent via-white/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none" />
              </Link>
            )
          })}
        </nav>

        {/* Footer */}
        <div className="p-4 border-t">
          {!sidebarCollapsed && (
            <div className="space-y-2 animate-in">
              <div className="flex items-center justify-between">
                <span className="text-xs text-muted-foreground">Cognitive Kernel</span>
                <Badge 
                  variant={cognitiveKernelStatus === 'processing' ? 'default' : 'secondary'}
                  className="text-xs capitalize transition-all duration-200"
                >
                  {cognitiveKernelStatus}
                </Badge>
              </div>
              <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <Activity className="w-3 h-3" />
                <span>System Active</span>
              </div>
            </div>
          )}
          {sidebarCollapsed && (
            <div className="flex justify-center">
              <Badge 
                variant={cognitiveKernelStatus === 'processing' ? 'default' : 'secondary'}
                className="text-xs transition-all duration-200"
              >
                {cognitiveKernelStatus === 'processing' ? '●' : '○'}
              </Badge>
            </div>
          )}
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Top Bar */}
        <div className="flex items-center justify-between p-4 border-b bg-background">
          <div className="flex items-center gap-4">
            <h1 className="text-xl font-semibold transition-all duration-200">
              {navigationItems.find(item => item.href === location.pathname)?.label || 'JARVIS'}
            </h1>
            {location.pathname !== '/' && (
              <p className="text-sm text-muted-foreground transition-all duration-200">
                {navigationItems.find(item => item.href === location.pathname)?.description}
              </p>
            )}
          </div>
          
          <div className="flex items-center gap-2">
            <Badge variant="outline" className="text-xs">
              v0.1.0
            </Badge>
            <Button variant="ghost" size="sm" className="hover-lift focus-ring">
              <Settings className="w-4 h-4" />
            </Button>
          </div>
        </div>

        {/* Page Content with smooth transitions */}
        <main className={`flex-1 overflow-auto content-fade ${
          isNavigating ? 'content-fade-exit' : 'content-fade-enter-active'
        }`}>
          <div className={`transition-all duration-200 ${
            isNavigating ? 'scale-[0.98] opacity-75' : 'scale-100 opacity-100'
          }`}>
            {children}
          </div>
        </main>
      </div>
    </div>
  )
} 