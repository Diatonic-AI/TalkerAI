-- Talk++ Database Schema Initialization
-- This script initializes the database for local development

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT true
);

-- Functions table
CREATE TABLE IF NOT EXISTS functions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    source_code TEXT NOT NULL,
    compiled_code TEXT,
    target_language VARCHAR(50) NOT NULL DEFAULT 'rust',
    version VARCHAR(20) DEFAULT '1.0.0',
    status VARCHAR(50) DEFAULT 'draft',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, name)
);

-- Function executions table
CREATE TABLE IF NOT EXISTS executions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    function_id UUID REFERENCES functions(id) ON DELETE CASCADE,
    event_data JSONB,
    response_data JSONB,
    status VARCHAR(50) DEFAULT 'pending',
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    execution_time_ms INTEGER,
    error_message TEXT
);

-- Auth tokens table
CREATE TABLE IF NOT EXISTS auth_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    token_type VARCHAR(50) DEFAULT 'bearer',
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE
);

-- Secrets table (encrypted)
CREATE TABLE IF NOT EXISTS secrets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    encrypted_value TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, name)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_functions_user_id ON functions(user_id);
CREATE INDEX IF NOT EXISTS idx_functions_status ON functions(status);
CREATE INDEX IF NOT EXISTS idx_executions_function_id ON executions(function_id);
CREATE INDEX IF NOT EXISTS idx_executions_started_at ON executions(started_at);
CREATE INDEX IF NOT EXISTS idx_executions_status ON executions(status);
CREATE INDEX IF NOT EXISTS idx_auth_tokens_user_id ON auth_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_tokens_expires_at ON auth_tokens(expires_at);
CREATE INDEX IF NOT EXISTS idx_secrets_user_id ON secrets(user_id);

-- Sample data for development
INSERT INTO users (email, password_hash, full_name) VALUES 
    ('demo@talkpp.dev', crypt('demo123', gen_salt('bf')), 'Demo User'),
    ('admin@talkpp.dev', crypt('admin123', gen_salt('bf')), 'Admin User')
ON CONFLICT (email) DO NOTHING;

-- Sample function for testing
DO $$
DECLARE
    demo_user_id UUID;
BEGIN
    SELECT id INTO demo_user_id FROM users WHERE email = 'demo@talkpp.dev';
    
    IF demo_user_id IS NOT NULL THEN
        INSERT INTO functions (user_id, name, description, source_code, target_language, status) VALUES 
        (demo_user_id, 'hello_world', 'Basic hello world function', 
         'if new user registers then send welcome_email using SendGrid', 
         'rust', 'published')
        ON CONFLICT (user_id, name) DO NOTHING;
    END IF;
END $$; 