-- Talk++ Database Schema

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Functions table (deployed Talk++ functions)
CREATE TABLE functions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    source_code TEXT NOT NULL,
    compiled_code TEXT NOT NULL,
    language VARCHAR(50) NOT NULL,
    version VARCHAR(50) NOT NULL,
    user_id UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Function executions table
CREATE TABLE function_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    function_id UUID REFERENCES functions(id),
    input_data JSONB,
    output_data JSONB,
    status VARCHAR(50) NOT NULL,
    execution_time_ms INTEGER,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Auth tokens table
CREATE TABLE auth_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Secrets table (encrypted credentials)
CREATE TABLE secrets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    encrypted_value TEXT NOT NULL,
    service VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, name)
);

-- Create indexes
CREATE INDEX idx_functions_user_id ON functions(user_id);
CREATE INDEX idx_function_executions_function_id ON function_executions(function_id);
CREATE INDEX idx_function_executions_created_at ON function_executions(created_at);
CREATE INDEX idx_auth_tokens_user_id ON auth_tokens(user_id);
CREATE INDEX idx_auth_tokens_expires_at ON auth_tokens(expires_at);
CREATE INDEX idx_secrets_user_id ON secrets(user_id);

-- Insert sample data
INSERT INTO users (id, email, name) VALUES 
    ('00000000-0000-0000-0000-000000000001', 'demo@talkpp.dev', 'Demo User');

INSERT INTO functions (id, name, description, source_code, compiled_code, language, version, user_id) VALUES 
    ('00000000-0000-0000-0000-000000000001', 
     'hello-world', 
     'Simple Hello World function',
     'if new user registers then send welcome message using Twilio',
     'pub async fn handler(event: Event) -> Result<Response> { Ok(Response::success("Hello World")) }',
     'rust',
     '0.1.0',
     '00000000-0000-0000-0000-000000000001'); 