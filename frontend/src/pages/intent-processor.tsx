import { useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'

import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { 
  FileText, 
  Play, 
  Pause, 
  Plus, 
  Edit, 
  Trash2,
  Clock,
  CheckCircle,
  XCircle,
  Brain,
  Settings
} from 'lucide-react'

interface Intent {
  id: string
  title: string
  description: string
  status: 'draft' | 'active' | 'completed' | 'failed'
  priority: 'low' | 'medium' | 'high' | 'critical'
  createdAt: Date
  updatedAt: Date
  estimatedDuration: string
  autonomyLevel: number
  tags: string[]
}

const sampleIntents: Intent[] = [
  {
    id: '1',
    title: 'Deploy Marketing Website',
    description: 'Deploy the new marketing website to production with automated testing and monitoring',
    status: 'active',
    priority: 'high',
    createdAt: new Date('2024-01-15T10:00:00'),
    updatedAt: new Date('2024-01-15T14:30:00'),
    estimatedDuration: '45 minutes',
    autonomyLevel: 3,
    tags: ['deployment', 'website', 'marketing', 'automation']
  },
  {
    id: '2',
    title: 'Database Migration',
    description: 'Migrate PostgreSQL database to MySQL with data validation and rollback procedures',
    status: 'draft',
    priority: 'critical',
    createdAt: new Date('2024-01-14T09:00:00'),
    updatedAt: new Date('2024-01-14T16:00:00'),
    estimatedDuration: '2 hours',
    autonomyLevel: 2,
    tags: ['database', 'migration', 'postgresql', 'mysql']
  },
  {
    id: '3',
    title: 'Content Generation',
    description: 'Generate marketing content for Q1 campaign using AI assistance',
    status: 'completed',
    priority: 'medium',
    createdAt: new Date('2024-01-13T11:00:00'),
    updatedAt: new Date('2024-01-13T15:00:00'),
    estimatedDuration: '30 minutes',
    autonomyLevel: 4,
    tags: ['content', 'marketing', 'ai', 'generation']
  },
  {
    id: '4',
    title: 'Security Audit',
    description: 'Perform comprehensive security audit of production systems',
    status: 'active',
    priority: 'critical',
    createdAt: new Date('2024-01-12T08:00:00'),
    updatedAt: new Date('2024-01-12T12:00:00'),
    estimatedDuration: '4 hours',
    autonomyLevel: 1,
    tags: ['security', 'audit', 'production', 'compliance']
  }
]

const getStatusIcon = (status: string) => {
  switch (status) {
    case 'completed':
      return <CheckCircle className="w-4 h-4 text-green-600" />
    case 'active':
      return <Play className="w-4 h-4 text-blue-600" />
    case 'draft':
      return <Edit className="w-4 h-4 text-yellow-600" />
    case 'failed':
      return <XCircle className="w-4 h-4 text-red-600" />
    default:
      return <Clock className="w-4 h-4 text-gray-600" />
  }
}

const getStatusColor = (status: string) => {
  switch (status) {
    case 'completed':
      return 'bg-green-100 text-green-800 border-green-300'
    case 'active':
      return 'bg-blue-100 text-blue-800 border-blue-300'
    case 'draft':
      return 'bg-yellow-100 text-yellow-800 border-yellow-300'
    case 'failed':
      return 'bg-red-100 text-red-800 border-red-300'
    default:
      return 'bg-gray-100 text-gray-800 border-gray-300'
  }
}

const getPriorityColor = (priority: string) => {
  switch (priority) {
    case 'critical':
      return 'bg-red-100 text-red-800 border-red-300'
    case 'high':
      return 'bg-orange-100 text-orange-800 border-orange-300'
    case 'medium':
      return 'bg-yellow-100 text-yellow-800 border-yellow-300'
    case 'low':
      return 'bg-green-100 text-green-800 border-green-300'
    default:
      return 'bg-gray-100 text-gray-800 border-gray-300'
  }
}

export function IntentProcessor() {
  const [selectedStatus] = useState<string>('all')
  const [searchQuery, setSearchQuery] = useState('')

  const filteredIntents = sampleIntents.filter(intent => {
    const matchesStatus = selectedStatus === 'all' || intent.status === selectedStatus
    const matchesSearch = intent.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         intent.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         intent.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()))
    return matchesStatus && matchesSearch
  })

  return (
    <div className="flex flex-col h-full space-y-6 p-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Intent Processor</h1>
          <p className="text-muted-foreground">
            Create and manage execution intents for JARVIS
          </p>
        </div>
        
        <div className="flex items-center space-x-2">
          <Button variant="outline" size="sm">
            <Settings className="w-4 h-4 mr-2" />
            Templates
          </Button>
          <Button size="sm">
            <Plus className="w-4 h-4 mr-2" />
            New Intent
          </Button>
        </div>
      </div>

      {/* Intent Stats */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Total Intents
            </CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{sampleIntents.length}</div>
            <p className="text-xs text-muted-foreground">
              +3 from last week
            </p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Active Intents
            </CardTitle>
            <Play className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {sampleIntents.filter(i => i.status === 'active').length}
            </div>
            <p className="text-xs text-muted-foreground">
              Currently executing
            </p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Success Rate
            </CardTitle>
            <CheckCircle className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">94.2%</div>
            <p className="text-xs text-muted-foreground">
              +2.1% from last month
            </p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Avg Duration
            </CardTitle>
            <Clock className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">1h 23m</div>
            <p className="text-xs text-muted-foreground">
              -12m from last month
            </p>
          </CardContent>
        </Card>
      </div>

      <Tabs defaultValue="all" className="flex-1">
        <TabsList className="grid w-full grid-cols-5">
          <TabsTrigger value="all">All Intents</TabsTrigger>
          <TabsTrigger value="draft">Draft</TabsTrigger>
          <TabsTrigger value="active">Active</TabsTrigger>
          <TabsTrigger value="completed">Completed</TabsTrigger>
          <TabsTrigger value="failed">Failed</TabsTrigger>
        </TabsList>

        <TabsContent value="all" className="mt-6">
          <div className="space-y-4">
            {/* Search and Filters */}
            <div className="flex items-center space-x-4">
              <div className="relative flex-1">
                <Input
                  placeholder="Search intents..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                />
              </div>
              <Button variant="outline" size="sm">
                <Brain className="w-4 h-4 mr-2" />
                AI Assist
              </Button>
            </div>

            {/* Intent List */}
            <div className="space-y-4">
              {filteredIntents.map((intent) => (
                <Card key={intent.id}>
                  <CardHeader>
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-3">
                        {getStatusIcon(intent.status)}
                        <div>
                          <CardTitle className="text-lg">{intent.title}</CardTitle>
                          <CardDescription>
                            Created {intent.createdAt.toLocaleDateString()} • Updated {intent.updatedAt.toLocaleDateString()}
                          </CardDescription>
                        </div>
                      </div>
                      
                      <div className="flex items-center space-x-2">
                        <Badge 
                          variant="outline" 
                          className={getPriorityColor(intent.priority)}
                        >
                          {intent.priority} priority
                        </Badge>
                        <Badge variant="outline" className={getStatusColor(intent.status)}>
                          {intent.status}
                        </Badge>
                        <Badge variant="outline">
                          Level {intent.autonomyLevel}
                        </Badge>
                      </div>
                    </div>
                  </CardHeader>
                  
                  <CardContent>
                    <p className="text-sm text-muted-foreground mb-4">
                      {intent.description}
                    </p>
                    
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-4">
                        <div className="flex flex-wrap gap-1">
                          {intent.tags.map((tag) => (
                            <Badge key={tag} variant="secondary" className="text-xs">
                              {tag}
                            </Badge>
                          ))}
                        </div>
                        <div className="text-sm text-muted-foreground">
                          Est. {intent.estimatedDuration}
                        </div>
                      </div>
                      
                      <div className="flex space-x-2">
                        {intent.status === 'draft' && (
                          <Button size="sm">
                            <Play className="w-4 h-4 mr-2" />
                            Execute
                          </Button>
                        )}
                        {intent.status === 'active' && (
                          <Button variant="outline" size="sm">
                            <Pause className="w-4 h-4 mr-2" />
                            Pause
                          </Button>
                        )}
                        <Button variant="ghost" size="sm">
                          <Edit className="w-4 h-4" />
                        </Button>
                        <Button variant="ghost" size="sm">
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          </div>
        </TabsContent>

        {/* Individual status tabs would have similar content */}
        <TabsContent value="draft" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Draft intents view - showing intents that are being prepared
          </div>
        </TabsContent>
        
        <TabsContent value="active" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Active intents view - showing currently executing intents
          </div>
        </TabsContent>
        
        <TabsContent value="completed" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Completed intents view - showing successfully executed intents
          </div>
        </TabsContent>
        
        <TabsContent value="failed" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Failed intents view - showing intents that encountered errors
          </div>
        </TabsContent>
      </Tabs>
    </div>
  )
} 