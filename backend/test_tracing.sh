#!/bin/bash

# Test script to verify tracing implementation
# This script starts the server and makes a few requests to test tracing output

echo "🚀 Starting server with tracing enabled..."
echo ""

# Start the server in the background
RUST_LOG=debug,backend=trace cargo run &
SERVER_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 8

echo ""
echo "📊 Making test requests to generate traces..."
echo ""

# Test 1: Health check
echo "1️⃣  Testing health check endpoint..."
curl -s http://localhost:2999/api/v1/health | jq '.' || echo "Request sent"
sleep 1

# Test 2: Register user
echo ""
echo "2️⃣  Testing user registration..."
curl -s -X POST http://localhost:2999/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "testpass123"
  }' | jq '.' || echo "Request sent"
sleep 1

# Test 3: Login
echo ""
echo "3️⃣  Testing user login..."
TOKEN=$(curl -s -X POST http://localhost:2999/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "testpass123"
  }' | jq -r '.token')

echo "Token received: ${TOKEN:0:20}..."
sleep 1

# Test 4: Get current user (authenticated)
echo ""
echo "4️⃣  Testing authenticated endpoint..."
curl -s http://localhost:2999/api/v1/auth/me \
  -H "Authorization: Bearer $TOKEN" | jq '.' || echo "Request sent"

echo ""
echo ""
echo "✅ Tests complete! Check the server output above for tracing spans."
echo ""
echo "🔍 Look for:"
echo "  - http_request spans with method, route, status, and timing"
echo "  - Nested spans for handlers (register_handler, login_handler, etc.)"
echo "  - Database operation spans (db_get_connection, etc.)"
echo "  - Service layer spans (auth_register, auth_login, etc.)"
echo ""
echo "🛑 Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "✨ Done!"
