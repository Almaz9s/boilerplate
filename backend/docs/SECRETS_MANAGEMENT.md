# Secrets Management

The backend boilerplate supports multiple secrets management providers for secure storage of sensitive configuration values like database passwords, API keys, and JWT secrets.

## Overview

By default, the application loads configuration from environment variables. For production deployments, you can configure external secret providers for enhanced security.

## Supported Providers

### 1. Environment Variables (Default)

The simplest approach - load secrets from environment variables.

**Configuration:**
```bash
# No special configuration needed
DATABASE_URL=postgres://user:pass@localhost/db
JWT_SECRET=your-secret-key
```

**Usage in code:**
```rust
let config = Config::from_env()?;
```

### 2. AWS Secrets Manager

Store secrets in AWS Secrets Manager for centralized secret management with AWS IAM integration.

**Enable feature:**
```bash
cargo build --features aws-secrets
```

**Configuration:**
```bash
export SECRET_PROVIDER=aws
export AWS_SECRET_PREFIX=myapp/production  # Optional: prefix for secret names
export AWS_REGION=us-east-1

# AWS credentials via standard AWS credential chain
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
```

**Create secrets in AWS:**
```bash
aws secretsmanager create-secret \
  --name myapp/production/DATABASE_URL \
  --secret-string "postgres://user:pass@host/db"

aws secretsmanager create-secret \
  --name myapp/production/JWT_SECRET \
  --secret-string "your-jwt-secret"
```

**Usage in code:**
```rust
let config = Config::from_secrets().await?;
```

### 3. HashiCorp Vault

Use Vault for enterprise-grade secret management with dynamic secrets, encryption as a service, and detailed audit logs.

**Enable feature:**
```bash
cargo build --features vault-secrets
```

**Configuration:**
```bash
export SECRET_PROVIDER=vault
export VAULT_ADDR=https://vault.example.com:8200
export VAULT_TOKEN=your-vault-token
export VAULT_MOUNT=secret    # Default: "secret"
export VAULT_PATH=backend     # Default: "backend"
```

**Store secrets in Vault:**
```bash
vault kv put secret/backend/DATABASE_URL value="postgres://user:pass@host/db"
vault kv put secret/backend/JWT_SECRET value="your-jwt-secret"
```

**Usage in code:**
```rust
let config = Config::from_secrets().await?;
```

## Using Secrets in Your Application

### Basic Usage

```rust
use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config with secrets
    let config = Config::from_secrets().await?;

    // Use config as normal
    println!("Connecting to database...");
    let db_pool = create_pool(&config.database.url, config.database.pool_size)?;

    Ok(())
}
```

### Direct Secret Manager Usage

For loading additional secrets beyond configuration:

```rust
use crate::config::secrets::SecretManager;

async fn load_api_key() -> Result<String, AppError> {
    let manager = SecretManager::from_config().await?;

    // Get secret from configured provider
    let api_key = manager.get_secret("THIRD_PARTY_API_KEY").await?;

    Ok(api_key)
}
```

### Fallback to Environment Variables

The `get_secret_or_env` method provides automatic fallback:

```rust
let manager = SecretManager::from_config().await?;

// Try secret provider first, fall back to env var
let secret = manager.get_secret_or_env("API_KEY", Some("FALLBACK_API_KEY")).await?;
```

## Secret Rotation

### AWS Secrets Manager

AWS supports automatic secret rotation. Configure rotation lambda:

```bash
aws secretsmanager rotate-secret \
  --secret-id myapp/production/DATABASE_URL \
  --rotation-lambda-arn arn:aws:lambda:region:account:function:RotationLambda \
  --rotation-rules AutomaticallyAfterDays=30
```

### Vault

Vault supports dynamic secrets with automatic expiration:

```bash
# Enable database secrets engine
vault secrets enable database

# Configure database connection
vault write database/config/mydb \
  plugin_name=postgresql-database-plugin \
  connection_url="postgresql://{{username}}:{{password}}@localhost:5432/mydb" \
  allowed_roles="app-role"

# Create role for dynamic credentials
vault write database/roles/app-role \
  db_name=mydb \
  creation_statements="CREATE ROLE \"{{name}}\" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';" \
  default_ttl="1h" \
  max_ttl="24h"
```

## Best Practices

1. **Never commit secrets** - Use `.env` files only for local development (add to `.gitignore`)

2. **Use different secrets per environment** - Separate dev/staging/production secrets

3. **Rotate secrets regularly** - Set up automatic rotation where possible

4. **Use least privilege** - Grant only necessary permissions to secret access

5. **Audit secret access** - Enable logging and monitoring for secret retrieval

6. **Use environment variable fallback** - Allow fallback to env vars for local development:
   ```rust
   // This works with both secret providers and env vars
   let config = Config::from_secrets().await?;
   ```

## Environment Variables Reference

### Core Configuration
- `SECRET_PROVIDER` - Provider to use: `env`, `aws`, `vault` (default: `env`)

### AWS Secrets Manager
- `AWS_SECRET_PREFIX` - Optional prefix for secret names
- `AWS_REGION` - AWS region for Secrets Manager
- `AWS_ACCESS_KEY_ID` - AWS access key
- `AWS_SECRET_ACCESS_KEY` - AWS secret key

### HashiCorp Vault
- `VAULT_ADDR` - Vault server address (e.g., `https://vault.example.com:8200`)
- `VAULT_TOKEN` - Vault authentication token
- `VAULT_MOUNT` - KV mount point (default: `secret`)
- `VAULT_PATH` - Path prefix for secrets (default: `backend`)

## Examples

### Local Development

```bash
# .env file (for local development only)
DATABASE_URL=postgres://localhost/dev
JWT_SECRET=local-dev-secret

# Use environment variables
SECRET_PROVIDER=env
```

### Production with AWS

```bash
# Environment variables in production
SECRET_PROVIDER=aws
AWS_SECRET_PREFIX=myapp/production
AWS_REGION=us-east-1

# Secrets stored in AWS Secrets Manager
# - myapp/production/DATABASE_URL
# - myapp/production/JWT_SECRET
```

### Production with Vault

```bash
# Environment variables in production
SECRET_PROVIDER=vault
VAULT_ADDR=https://vault.internal:8200
VAULT_TOKEN=s.abc123xyz
VAULT_MOUNT=secret
VAULT_PATH=myapp/production

# Secrets stored in Vault at:
# - secret/myapp/production/DATABASE_URL
# - secret/myapp/production/JWT_SECRET
```

## Troubleshooting

### Secret not found

**Error:** `Secret 'DATABASE_URL' not found in secret provider`

**Solutions:**
1. Verify secret exists in the provider
2. Check secret name matches exactly (case-sensitive)
3. Verify permissions to access the secret
4. Check prefix/path configuration

### Provider connection issues

**AWS:** Ensure AWS credentials are configured and region is set

**Vault:** Verify `VAULT_ADDR` is accessible and `VAULT_TOKEN` is valid

### Feature not enabled

**Error:** `Unknown secret provider: aws`

**Solution:** Rebuild with the appropriate feature flag:
```bash
cargo build --features aws-secrets
# or
cargo build --features vault-secrets
```

## Testing

For testing, use environment variables or the default test configuration:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_env_secrets() {
        std::env::set_var("SECRET_PROVIDER", "env");
        std::env::set_var("DATABASE_URL", "postgres://test");
        std::env::set_var("JWT_SECRET", "test-secret");

        let config = Config::from_secrets().await.unwrap();
        assert_eq!(config.database.url, "postgres://test");
    }
}
```
