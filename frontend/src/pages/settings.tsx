import React, { useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { 
  Settings as SettingsIcon, 
  Brain, 
  Shield, 
  Database, 
  Globe, 
  Users, 
  Bell,
  Palette,
  Save,
  RefreshCw,
  Eye,
  EyeOff
} from 'lucide-react'

interface SettingSection {
  id: string
  title: string
  description: string
  icon: React.ReactNode
}

const settingSections: SettingSection[] = [
  {
    id: 'general',
    title: 'General',
    description: 'Basic application settings and preferences',
    icon: <SettingsIcon className="w-4 h-4" />
  },
  {
    id: 'cognitive',
    title: 'Cognitive Kernel',
    description: 'Configure JARVIS cognitive processing systems',
    icon: <Brain className="w-4 h-4" />
  },
  {
    id: 'security',
    title: 'Security',
    description: 'Manage security and access controls',
    icon: <Shield className="w-4 h-4" />
  },
  {
    id: 'database',
    title: 'Database',
    description: 'Configure data storage and connection settings',
    icon: <Database className="w-4 h-4" />
  },
  {
    id: 'network',
    title: 'Network',
    description: 'Network and connectivity settings',
    icon: <Globe className="w-4 h-4" />
  },
  {
    id: 'users',
    title: 'Users',
    description: 'User management and permissions',
    icon: <Users className="w-4 h-4" />
  },
  {
    id: 'notifications',
    title: 'Notifications',
    description: 'Configure notification preferences',
    icon: <Bell className="w-4 h-4" />
  },
  {
    id: 'appearance',
    title: 'Appearance',
    description: 'Theme and visual preferences',
    icon: <Palette className="w-4 h-4" />
  }
]

export function Settings() {
  const [activeTab, setActiveTab] = useState('general')
  const [showPasswords, setShowPasswords] = useState(false)

  return (
    <div className="flex flex-col h-full space-y-6 p-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground">
            Configure JARVIS and application preferences
          </p>
        </div>
        
        <div className="flex items-center space-x-2">
          <Button variant="outline" size="sm">
            <RefreshCw className="w-4 h-4 mr-2" />
            Reset
          </Button>
          <Button size="sm">
            <Save className="w-4 h-4 mr-2" />
            Save Changes
          </Button>
        </div>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab} className="flex-1">
        <TabsList className="grid w-full grid-cols-4 lg:grid-cols-8">
          {settingSections.map((section) => (
            <TabsTrigger key={section.id} value={section.id} className="flex items-center gap-2">
              {section.icon}
              <span className="hidden lg:inline">{section.title}</span>
            </TabsTrigger>
          ))}
        </TabsList>

        <TabsContent value="general" className="mt-6">
          <div className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Application Settings</CardTitle>
                <CardDescription>
                  Configure basic application behavior and preferences
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="app-name">Application Name</Label>
                    <Input id="app-name" defaultValue="JARVIS Mission Control" />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="version">Version</Label>
                    <Input id="version" defaultValue="0.1.0" disabled />
                  </div>
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="data-dir">Data Directory</Label>
                  <Input id="data-dir" defaultValue="/var/lib/jarvis" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="log-level">Log Level</Label>
                  <select 
                    id="log-level" 
                    className="w-full p-2 border rounded-md bg-background"
                    defaultValue="info"
                  >
                    <option value="debug">Debug</option>
                    <option value="info">Info</option>
                    <option value="warn">Warning</option>
                    <option value="error">Error</option>
                  </select>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>Performance</CardTitle>
                <CardDescription>
                  Configure performance and resource settings
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="max-memory">Max Memory (GB)</Label>
                    <Input id="max-memory" type="number" defaultValue="8" />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="max-cpu">Max CPU Cores</Label>
                    <Input id="max-cpu" type="number" defaultValue="4" />
                  </div>
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="cache-size">Cache Size (MB)</Label>
                  <Input id="cache-size" type="number" defaultValue="1024" />
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        <TabsContent value="cognitive" className="mt-6">
          <div className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Cognitive Kernel Configuration</CardTitle>
                <CardDescription>
                  Configure JARVIS cognitive processing parameters
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="autonomy-level">Default Autonomy Level</Label>
                    <select 
                      id="autonomy-level" 
                      className="w-full p-2 border rounded-md bg-background"
                      defaultValue="2"
                    >
                      <option value="1">Level 1 - Manual</option>
                      <option value="2">Level 2 - Assisted</option>
                      <option value="3">Level 3 - Semi-Autonomous</option>
                      <option value="4">Level 4 - Autonomous</option>
                      <option value="5">Level 5 - Fully Autonomous</option>
                    </select>
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="learning-rate">Learning Rate</Label>
                    <Input id="learning-rate" type="number" step="0.01" defaultValue="0.01" />
                  </div>
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="memory-capacity">Memory Capacity (GB)</Label>
                  <Input id="memory-capacity" type="number" defaultValue="16" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="processing-threads">Processing Threads</Label>
                  <Input id="processing-threads" type="number" defaultValue="8" />
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        <TabsContent value="security" className="mt-6">
          <div className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Security Settings</CardTitle>
                <CardDescription>
                  Configure security and authentication settings
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="session-timeout">Session Timeout (minutes)</Label>
                  <Input id="session-timeout" type="number" defaultValue="30" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="max-login-attempts">Max Login Attempts</Label>
                  <Input id="max-login-attempts" type="number" defaultValue="5" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="password-policy">Password Policy</Label>
                  <select 
                    id="password-policy" 
                    className="w-full p-2 border rounded-md bg-background"
                    defaultValue="strong"
                  >
                    <option value="basic">Basic</option>
                    <option value="strong">Strong</option>
                    <option value="very-strong">Very Strong</option>
                  </select>
                </div>
                
                <div className="flex items-center space-x-2">
                  <input type="checkbox" id="2fa" defaultChecked />
                  <Label htmlFor="2fa">Enable Two-Factor Authentication</Label>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        <TabsContent value="database" className="mt-6">
          <div className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Database Configuration</CardTitle>
                <CardDescription>
                  Configure database connection and storage settings
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="db-host">Database Host</Label>
                  <Input id="db-host" defaultValue="localhost" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="db-port">Database Port</Label>
                  <Input id="db-port" defaultValue="5432" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="db-name">Database Name</Label>
                  <Input id="db-name" defaultValue="jarvis" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="db-user">Database User</Label>
                  <Input id="db-user" defaultValue="jarvis_user" />
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="db-password">Database Password</Label>
                  <div className="relative">
                    <Input 
                      id="db-password" 
                      type={showPasswords ? "text" : "password"}
                      defaultValue="••••••••"
                    />
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
                      onClick={() => setShowPasswords(!showPasswords)}
                    >
                      {showPasswords ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                    </Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        <TabsContent value="network" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Network configuration settings
          </div>
        </TabsContent>

        <TabsContent value="users" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            User management and permissions settings
          </div>
        </TabsContent>

        <TabsContent value="notifications" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Notification preferences and settings
          </div>
        </TabsContent>

        <TabsContent value="appearance" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Theme and visual appearance settings
          </div>
        </TabsContent>
      </Tabs>
    </div>
  )
} 