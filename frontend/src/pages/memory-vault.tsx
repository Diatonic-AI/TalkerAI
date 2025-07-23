import { useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { 
  Brain, 
 
  Clock, 
  Search, 
  Plus, 
  Trash2, 
  Download,
  Upload,
  RefreshCw,
  MapPin,
  HardDrive,
  Cpu,
  Activity
} from 'lucide-react'

interface MemoryItem {
  id: string
  type: 'episodic' | 'semantic' | 'procedural' | 'spatial'
  content: string
  timestamp: Date
  size: number
  tags: string[]
  priority: 'high' | 'medium' | 'low'
}

const memoryTypes = {
  episodic: { label: 'Episodic', icon: <Clock className="w-4 h-4" />, color: 'bg-blue-100 text-blue-800' },
  semantic: { label: 'Semantic', icon: <Brain className="w-4 h-4" />, color: 'bg-green-100 text-green-800' },
  procedural: { label: 'Procedural', icon: <Cpu className="w-4 h-4" />, color: 'bg-purple-100 text-purple-800' },
  spatial: { label: 'Spatial', icon: <MapPin className="w-4 h-4" />, color: 'bg-orange-100 text-orange-800' }
}

const sampleMemories: MemoryItem[] = [
  {
    id: '1',
    type: 'episodic',
    content: 'User requested deployment of marketing website at 2024-01-15 14:30',
    timestamp: new Date('2024-01-15T14:30:00'),
    size: 1024,
    tags: ['deployment', 'website', 'marketing'],
    priority: 'high'
  },
  {
    id: '2',
    type: 'semantic',
    content: 'Database migration procedure for PostgreSQL to MySQL',
    timestamp: new Date('2024-01-14T10:15:00'),
    size: 2048,
    tags: ['database', 'migration', 'postgresql', 'mysql'],
    priority: 'medium'
  },
  {
    id: '3',
    type: 'procedural',
    content: 'Automated backup process for critical systems',
    timestamp: new Date('2024-01-13T09:45:00'),
    size: 512,
    tags: ['backup', 'automation', 'critical'],
    priority: 'high'
  },
  {
    id: '4',
    type: 'spatial',
    content: 'Network topology for production environment',
    timestamp: new Date('2024-01-12T16:20:00'),
    size: 1536,
    tags: ['network', 'topology', 'production'],
    priority: 'medium'
  }
]

export function MemoryVault() {
  const [selectedType] = useState<string>('all')
  const [searchQuery, setSearchQuery] = useState('')

  const filteredMemories = sampleMemories.filter(memory => {
    const matchesType = selectedType === 'all' || memory.type === selectedType
    const matchesSearch = memory.content.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         memory.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()))
    return matchesType && matchesSearch
  })

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'high': return 'bg-red-100 text-red-800 border-red-300'
      case 'medium': return 'bg-yellow-100 text-yellow-800 border-yellow-300'
      case 'low': return 'bg-green-100 text-green-800 border-green-300'
      default: return 'bg-gray-100 text-gray-800 border-gray-300'
    }
  }

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 Bytes'
    const k = 1024
    const sizes = ['Bytes', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  return (
    <div className="flex flex-col h-full space-y-6 p-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Memory Vault</h1>
          <p className="text-muted-foreground">
            Access and manage JARVIS memory systems
          </p>
        </div>
        
        <div className="flex items-center space-x-2">
          <Button variant="outline" size="sm">
            <Upload className="w-4 h-4 mr-2" />
            Import
          </Button>
          <Button variant="outline" size="sm">
            <Download className="w-4 h-4 mr-2" />
            Export
          </Button>
          <Button size="sm">
            <Plus className="w-4 h-4 mr-2" />
            Add Memory
          </Button>
        </div>
      </div>

      {/* Memory Stats */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Total Memories
            </CardTitle>
            <Brain className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{sampleMemories.length}</div>
            <p className="text-xs text-muted-foreground">
              +12 from last week
            </p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Memory Usage
            </CardTitle>
            <HardDrive className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">5.1 GB</div>
            <p className="text-xs text-muted-foreground">
              +2.3 GB from last week
            </p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Active Processes
            </CardTitle>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">8</div>
            <p className="text-xs text-muted-foreground">
              +2 from last hour
            </p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Retrieval Speed
            </CardTitle>
            <Cpu className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">2.3ms</div>
            <p className="text-xs text-muted-foreground">
              -0.5ms from last week
            </p>
          </CardContent>
        </Card>
      </div>

      <Tabs defaultValue="all" className="flex-1">
        <TabsList className="grid w-full grid-cols-5">
          <TabsTrigger value="all">All Memories</TabsTrigger>
          <TabsTrigger value="episodic">Episodic</TabsTrigger>
          <TabsTrigger value="semantic">Semantic</TabsTrigger>
          <TabsTrigger value="procedural">Procedural</TabsTrigger>
          <TabsTrigger value="spatial">Spatial</TabsTrigger>
        </TabsList>

        <TabsContent value="all" className="mt-6">
          <div className="space-y-4">
            {/* Search and Filters */}
            <div className="flex items-center space-x-4">
              <div className="relative flex-1">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground w-4 h-4" />
                <input
                  type="text"
                  placeholder="Search memories..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 border rounded-md bg-background"
                />
              </div>
              <Button variant="outline" size="sm">
                <RefreshCw className="w-4 h-4 mr-2" />
                Refresh
              </Button>
            </div>

            {/* Memory List */}
            <div className="space-y-4">
              {filteredMemories.map((memory) => (
                <Card key={memory.id}>
                  <CardHeader>
                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-3">
                        {memoryTypes[memory.type].icon}
                        <div>
                          <CardTitle className="text-lg">{memoryTypes[memory.type].label} Memory</CardTitle>
                          <CardDescription>
                            {memory.timestamp.toLocaleString()}
                          </CardDescription>
                        </div>
                      </div>
                      
                      <div className="flex items-center space-x-2">
                        <Badge 
                          variant="outline" 
                          className={getPriorityColor(memory.priority)}
                        >
                          {memory.priority} priority
                        </Badge>
                        <Badge variant="outline">
                          {formatBytes(memory.size)}
                        </Badge>
                      </div>
                    </div>
                  </CardHeader>
                  
                  <CardContent>
                    <p className="text-sm text-muted-foreground mb-4">
                      {memory.content}
                    </p>
                    
                    <div className="flex items-center justify-between">
                      <div className="flex flex-wrap gap-1">
                        {memory.tags.map((tag) => (
                          <Badge key={tag} variant="secondary" className="text-xs">
                            {tag}
                          </Badge>
                        ))}
                      </div>
                      
                      <div className="flex space-x-2">
                        <Button variant="ghost" size="sm">
                          <Search className="w-4 h-4" />
                        </Button>
                        <Button variant="ghost" size="sm">
                          <Download className="w-4 h-4" />
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

        {/* Individual memory type tabs would have similar content */}
        <TabsContent value="episodic" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Episodic memories view - showing memories related to specific events and experiences
          </div>
        </TabsContent>
        
        <TabsContent value="semantic" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Semantic memories view - showing conceptual knowledge and facts
          </div>
        </TabsContent>
        
        <TabsContent value="procedural" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Procedural memories view - showing learned skills and procedures
          </div>
        </TabsContent>
        
        <TabsContent value="spatial" className="mt-6">
          <div className="text-center py-8 text-muted-foreground">
            Spatial memories view - showing spatial relationships and layouts
          </div>
        </TabsContent>
      </Tabs>
    </div>
  )
} 