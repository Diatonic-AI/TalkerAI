import { useCallback } from 'react'
import ReactFlow, {
  Node,
  Edge,
  addEdge,
  Connection,
  useNodesState,
  useEdgesState,
  Controls,
  MiniMap,
  Background,
  BackgroundVariant,
} from 'reactflow'
import 'reactflow/dist/style.css'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { 
  Activity, 
  Brain, 
  GitBranch, 
  Play, 
  Pause, 
  Square,
  Zap,
  Clock,
  CheckCircle,
  XCircle
} from 'lucide-react'
import { useAppStore } from '@/store/app-store'

// Sample data for demonstration
const initialNodes: Node[] = [
  {
    id: '1',
    type: 'default',
    position: { x: 100, y: 100 },
    data: { 
      label: 'Intent: Deploy App',
      status: 'completed',
      duration: '2m 30s'
    },
    style: { 
      background: '#dcfce7', 
      border: '2px solid #16a34a',
      borderRadius: '8px'
    }
  },
  {
    id: '2',
    type: 'default',
    position: { x: 300, y: 100 },
    data: { 
      label: 'Analyze Requirements',
      status: 'in-progress',
      duration: '45s'
    },
    style: { 
      background: '#fef3c7', 
      border: '2px solid #f59e0b',
      borderRadius: '8px'
    }
  },
  {
    id: '3',
    type: 'default',
    position: { x: 500, y: 100 },
    data: { 
      label: 'Execute Deployment',
      status: 'pending',
      duration: 'Est. 3m'
    },
    style: { 
      background: '#f3f4f6', 
      border: '2px solid #6b7280',
      borderRadius: '8px'
    }
  },
  {
    id: '4',
    type: 'default',
    position: { x: 300, y: 250 },
    data: { 
      label: 'Verify Health Checks',
      status: 'pending',
      duration: 'Est. 1m'
    },
    style: { 
      background: '#f3f4f6', 
      border: '2px solid #6b7280',
      borderRadius: '8px'
    }
  }
]

const initialEdges: Edge[] = [
  { id: 'e1-2', source: '1', target: '2', animated: false },
  { id: 'e2-3', source: '2', target: '3', animated: true },
  { id: 'e3-4', source: '3', target: '4', animated: false }
]

const getStatusIcon = (status: string) => {
  switch (status) {
    case 'completed':
      return <CheckCircle className="w-4 h-4 text-green-600" />
    case 'in-progress':
      return <Activity className="w-4 h-4 text-yellow-600" />
    case 'error':
      return <XCircle className="w-4 h-4 text-red-600" />
    case 'pending':
    default:
      return <Clock className="w-4 h-4 text-gray-600" />
  }
}

const getStatusColor = (status: string) => {
  switch (status) {
    case 'completed':
      return 'bg-green-100 text-green-800 border-green-300'
    case 'in-progress':
      return 'bg-yellow-100 text-yellow-800 border-yellow-300'
    case 'error':
      return 'bg-red-100 text-red-800 border-red-300'
    case 'pending':
    default:
      return 'bg-gray-100 text-gray-800 border-gray-300'
  }
}

export function MissionControl() {
  const [nodes, , onNodesChange] = useNodesState(initialNodes)
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges)
  const { cognitiveKernelStatus } = useAppStore()

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  )

  const executionPlans = [
    {
      id: '1',
      name: 'Deploy Marketing Website',
      status: 'in-progress',
      autonomyTier: 2,
      estimatedDuration: '5m 30s',
      tasksCompleted: 2,
      totalTasks: 4,
      riskLevel: 'medium'
    },
    {
      id: '2',
      name: 'Database Migration',
      status: 'pending',
      autonomyTier: 1,
      estimatedDuration: '8m 15s',
      tasksCompleted: 0,
      totalTasks: 3,
      riskLevel: 'high'
    },
    {
      id: '3',
      name: 'Content Generation',
      status: 'completed',
      autonomyTier: 3,
      estimatedDuration: '2m 45s',
      tasksCompleted: 3,
      totalTasks: 3,
      riskLevel: 'low'
    }
  ]

  return (
    <div className="flex flex-col h-full space-y-6 p-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Mission Control</h1>
          <p className="text-muted-foreground">
            Monitor and orchestrate JARVIS execution plans
          </p>
        </div>
        
        <div className="flex items-center space-x-4">
          <div className="flex items-center space-x-2">
            <Brain className="w-5 h-5" />
            <span className="text-sm font-medium">Cognitive Kernel</span>
            <Badge 
              variant={cognitiveKernelStatus === 'processing' ? 'default' : 'secondary'}
              className="capitalize"
            >
              {cognitiveKernelStatus}
            </Badge>
          </div>
          
          <div className="flex space-x-2">
            <Button variant="outline" size="sm">
              <Play className="w-4 h-4 mr-2" />
              Execute
            </Button>
            <Button variant="outline" size="sm">
              <Pause className="w-4 h-4 mr-2" />
              Pause
            </Button>
            <Button variant="destructive" size="sm">
              <Square className="w-4 h-4 mr-2" />
              Stop All
            </Button>
          </div>
        </div>
      </div>

      <Tabs defaultValue="dag" className="flex-1">
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="dag">
            <GitBranch className="w-4 h-4 mr-2" />
            Execution DAG
          </TabsTrigger>
          <TabsTrigger value="plans">
            <Activity className="w-4 h-4 mr-2" />
            Active Plans
          </TabsTrigger>
          <TabsTrigger value="metrics">
            <Zap className="w-4 h-4 mr-2" />
            Performance
          </TabsTrigger>
        </TabsList>

        <TabsContent value="dag" className="flex-1 mt-6">
          <Card className="h-[600px]">
            <CardHeader>
              <CardTitle>Task Execution Flow</CardTitle>
              <CardDescription>
                Real-time visualization of task dependencies and execution progress
              </CardDescription>
            </CardHeader>
            <CardContent className="h-full p-0">
              <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                onConnect={onConnect}
                fitView
              >
                <Controls />
                <MiniMap />
                <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
              </ReactFlow>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="plans" className="mt-6">
          <div className="grid gap-4">
            {executionPlans.map((plan) => (
              <Card key={plan.id}>
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-3">
                      {getStatusIcon(plan.status)}
                      <div>
                        <CardTitle className="text-lg">{plan.name}</CardTitle>
                        <CardDescription>
                          Autonomy Tier {plan.autonomyTier} • {plan.estimatedDuration}
                        </CardDescription>
                      </div>
                    </div>
                    
                    <div className="flex items-center space-x-2">
                      <Badge 
                        variant="outline" 
                        className={getStatusColor(plan.riskLevel)}
                      >
                        {plan.riskLevel} risk
                      </Badge>
                      <Badge variant="outline" className={getStatusColor(plan.status)}>
                        {plan.status}
                      </Badge>
                    </div>
                  </div>
                </CardHeader>
                
                <CardContent>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-4">
                      <div className="text-sm text-muted-foreground">
                        Progress: {plan.tasksCompleted}/{plan.totalTasks} tasks
                      </div>
                      <div className="w-32 bg-gray-200 rounded-full h-2">
                        <div 
                          className="bg-primary h-2 rounded-full" 
                          style={{ 
                            width: `${(plan.tasksCompleted / plan.totalTasks) * 100}%` 
                          }}
                        />
                      </div>
                    </div>
                    
                    <div className="flex space-x-2">
                      {plan.status === 'pending' && (
                        <Button size="sm">
                          <Play className="w-4 h-4 mr-2" />
                          Start
                        </Button>
                      )}
                      {plan.status === 'in-progress' && (
                        <Button variant="outline" size="sm">
                          <Pause className="w-4 h-4 mr-2" />
                          Pause
                        </Button>
                      )}
                      <Button variant="ghost" size="sm">
                        View Details
                      </Button>
                    </div>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="metrics" className="mt-6">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Active Executions
                </CardTitle>
                <Activity className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">3</div>
                <p className="text-xs text-muted-foreground">
                  +2 from last hour
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
                  +1.2% from last hour
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
                <div className="text-2xl font-bold">3m 42s</div>
                <p className="text-xs text-muted-foreground">
                  -12s from last hour
                </p>
              </CardContent>
            </Card>
            
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Memory Usage
                </CardTitle>
                <Brain className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">847MB</div>
                <p className="text-xs text-muted-foreground">
                  +42MB from last hour
                </p>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  )
} 