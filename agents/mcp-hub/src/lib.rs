use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, error};
use uuid::Uuid;

/// MCP Server Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub server_type: McpServerType,
    pub connection: McpConnection,
    pub capabilities: Vec<McpCapability>,
    pub enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpServerType {
    Local,
    Remote,
    Docker,
    Kubernetes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpConnection {
    Http { url: String, headers: HashMap<String, String> },
    WebSocket { url: String },
    Stdio { command: String, args: Vec<String> },
    Unix { socket_path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpCapability {
    Tools,
    Resources,
    Prompts,
    Sampling,
    Notifications,
}

/// MCP Tool Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub server_id: Uuid,
}

/// MCP Hub Manager
pub struct McpHub {
    servers: RwLock<HashMap<Uuid, McpServerConfig>>,
    connections: RwLock<HashMap<Uuid, Box<dyn McpConnection + Send + Sync>>>,
    tools: RwLock<HashMap<String, McpTool>>,
}

#[async_trait]
pub trait McpConnection {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn call_tool(&self, tool_name: &str, params: serde_json::Value) -> Result<serde_json::Value>;
    async fn list_tools(&self) -> Result<Vec<McpTool>>;
    async fn is_connected(&self) -> bool;
}

impl McpHub {
    pub fn new() -> Self {
        Self {
            servers: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
            tools: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new MCP server
    pub async fn register_server(&self, config: McpServerConfig) -> Result<()> {
        info!("Registering MCP server: {}", config.name);
        
        let server_id = config.id;
        
        // Store the server configuration
        {
            let mut servers = self.servers.write().await;
            servers.insert(server_id, config.clone());
        }

        // Initialize connection based on server type
        if config.enabled {
            self.connect_server(server_id).await?;
        }

        Ok(())
    }

    /// Connect to an MCP server
    pub async fn connect_server(&self, server_id: Uuid) -> Result<()> {
        let config = {
            let servers = self.servers.read().await;
            servers.get(&server_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("Server not found: {}", server_id))?
        };

        info!("Connecting to MCP server: {}", config.name);

        let connection: Box<dyn McpConnection + Send + Sync> = match config.connection {
            McpConnection::Http { url, headers } => {
                Box::new(HttpMcpConnection::new(url, headers)?)
            },
            McpConnection::WebSocket { url } => {
                Box::new(WebSocketMcpConnection::new(url)?)
            },
            McpConnection::Stdio { command, args } => {
                Box::new(StdioMcpConnection::new(command, args)?)
            },
            McpConnection::Unix { socket_path } => {
                Box::new(UnixMcpConnection::new(socket_path)?)
            },
        };

        // Store the connection
        {
            let mut connections = self.connections.write().await;
            connections.insert(server_id, connection);
        }

        // Discover and register tools from this server
        self.discover_tools(server_id).await?;

        Ok(())
    }

    /// Discover tools from a connected MCP server
    async fn discover_tools(&self, server_id: Uuid) -> Result<()> {
        let connection = {
            let connections = self.connections.read().await;
            connections.get(&server_id).cloned()
        };

        if let Some(conn) = connection {
            let discovered_tools = conn.list_tools().await?;
            
            let mut tools = self.tools.write().await;
            for tool in discovered_tools {
                let tool_key = format!("{}::{}", server_id, tool.name);
                tools.insert(tool_key, tool);
            }
            
            info!("Discovered {} tools from server {}", tools.len(), server_id);
        }

        Ok(())
    }

    /// Execute a tool call
    pub async fn call_tool(&self, tool_name: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        // Find the tool and its server
        let (server_id, tool) = {
            let tools = self.tools.read().await;
            let tool_entry = tools.iter()
                .find(|(key, tool)| tool.name == tool_name || key.ends_with(&format!("::{}", tool_name)))
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_name))?;
            
            (tool_entry.1.server_id, tool_entry.1.clone())
        };

        // Get the connection for this server
        let connection = {
            let connections = self.connections.read().await;
            connections.get(&server_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("No connection for server: {}", server_id))?
        };

        // Execute the tool call
        connection.call_tool(&tool.name, params).await
    }

    /// List all available tools
    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let tools = self.tools.read().await;
        Ok(tools.values().cloned().collect())
    }

    /// Get server status
    pub async fn get_server_status(&self, server_id: Uuid) -> Result<McpServerStatus> {
        let config = {
            let servers = self.servers.read().await;
            servers.get(&server_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("Server not found: {}", server_id))?
        };

        let is_connected = {
            let connections = self.connections.read().await;
            if let Some(conn) = connections.get(&server_id) {
                conn.is_connected().await
            } else {
                false
            }
        };

        Ok(McpServerStatus {
            id: server_id,
            name: config.name,
            connected: is_connected,
            tools_count: self.get_server_tools_count(server_id).await,
        })
    }

    async fn get_server_tools_count(&self, server_id: Uuid) -> usize {
        let tools = self.tools.read().await;
        tools.values().filter(|tool| tool.server_id == server_id).count()
    }
}

#[derive(Debug, Serialize)]
pub struct McpServerStatus {
    pub id: Uuid,
    pub name: String,
    pub connected: bool,
    pub tools_count: usize,
}

// Connection implementations
pub struct HttpMcpConnection {
    url: String,
    client: reqwest::Client,
    headers: HashMap<String, String>,
}

impl HttpMcpConnection {
    pub fn new(url: String, headers: HashMap<String, String>) -> Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self { url, client, headers })
    }
}

#[async_trait]
impl McpConnection for HttpMcpConnection {
    async fn connect(&mut self) -> Result<()> {
        // HTTP connections are stateless
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        // HTTP connections are stateless
        Ok(())
    }

    async fn call_tool(&self, tool_name: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": params
            }
        });

        let mut request = self.client.post(&self.url).json(&request_body);
        
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        let result: serde_json::Value = response.json().await?;
        
        if let Some(error) = result.get("error") {
            return Err(anyhow::anyhow!("MCP error: {}", error));
        }

        Ok(result.get("result").unwrap_or(&serde_json::Value::Null).clone())
    }

    async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list"
        });

        let mut request = self.client.post(&self.url).json(&request_body);
        
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        let result: serde_json::Value = response.json().await?;
        
        if let Some(error) = result.get("error") {
            return Err(anyhow::anyhow!("MCP error: {}", error));
        }

        // Parse tools from response
        let tools_array = result.get("result").and_then(|r| r.get("tools"))
            .and_then(|t| t.as_array())
            .ok_or_else(|| anyhow::anyhow!("Invalid tools response"))?;

        let tools = tools_array.iter()
            .filter_map(|tool_json| {
                let name = tool_json.get("name")?.as_str()?.to_string();
                let description = tool_json.get("description")?.as_str()?.to_string();
                let input_schema = tool_json.get("inputSchema")?.clone();
                
                Some(McpTool {
                    name,
                    description,
                    input_schema,
                    server_id: Uuid::new_v4(), // This should be set by the hub
                })
            })
            .collect();

        Ok(tools)
    }

    async fn is_connected(&self) -> bool {
        // For HTTP, we can check with a ping or health endpoint
        true // Simplified for now
    }
}

// Additional connection types would be implemented similarly
pub struct WebSocketMcpConnection {
    url: String,
}

impl WebSocketMcpConnection {
    pub fn new(url: String) -> Result<Self> {
        Ok(Self { url })
    }
}

#[async_trait]
impl McpConnection for WebSocketMcpConnection {
    async fn connect(&mut self) -> Result<()> {
        // WebSocket connection implementation
        todo!("WebSocket MCP connection implementation")
    }

    async fn disconnect(&mut self) -> Result<()> {
        todo!("WebSocket disconnect implementation")
    }

    async fn call_tool(&self, _tool_name: &str, _params: serde_json::Value) -> Result<serde_json::Value> {
        todo!("WebSocket tool call implementation")
    }

    async fn list_tools(&self) -> Result<Vec<McpTool>> {
        todo!("WebSocket list tools implementation")
    }

    async fn is_connected(&self) -> bool {
        false // TODO: Implement actual connection status
    }
}

pub struct StdioMcpConnection {
    command: String,
    args: Vec<String>,
}

impl StdioMcpConnection {
    pub fn new(command: String, args: Vec<String>) -> Result<Self> {
        Ok(Self { command, args })
    }
}

#[async_trait]
impl McpConnection for StdioMcpConnection {
    async fn connect(&mut self) -> Result<()> {
        // Stdio connection implementation
        todo!("Stdio MCP connection implementation")
    }

    async fn disconnect(&mut self) -> Result<()> {
        todo!("Stdio disconnect implementation")
    }

    async fn call_tool(&self, _tool_name: &str, _params: serde_json::Value) -> Result<serde_json::Value> {
        todo!("Stdio tool call implementation")
    }

    async fn list_tools(&self) -> Result<Vec<McpTool>> {
        todo!("Stdio list tools implementation")
    }

    async fn is_connected(&self) -> bool {
        false // TODO: Implement actual connection status
    }
}

pub struct UnixMcpConnection {
    socket_path: String,
}

impl UnixMcpConnection {
    pub fn new(socket_path: String) -> Result<Self> {
        Ok(Self { socket_path })
    }
}

#[async_trait]
impl McpConnection for UnixMcpConnection {
    async fn connect(&mut self) -> Result<()> {
        // Unix socket connection implementation
        todo!("Unix socket MCP connection implementation")
    }

    async fn disconnect(&mut self) -> Result<()> {
        todo!("Unix socket disconnect implementation")
    }

    async fn call_tool(&self, _tool_name: &str, _params: serde_json::Value) -> Result<serde_json::Value> {
        todo!("Unix socket tool call implementation")
    }

    async fn list_tools(&self) -> Result<Vec<McpTool>> {
        todo!("Unix socket list tools implementation")
    }

    async fn is_connected(&self) -> bool {
        false // TODO: Implement actual connection status
    }
} 