# Overlay: Terraform / HCL

## Conventions
- Use Terraform 1.x syntax (no legacy 0.x patterns)
- One `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf` per module
- Use `terraform fmt` formatting — no manual style overrides
- Variable descriptions are required, types are required, defaults are optional
- Use `locals` for computed values, not repeated expressions

## Module Patterns
- Root module calls child modules — root never defines resources directly in large projects
- Module sources pinned to exact versions: `source = "..."` with `version = "= 1.2.3"`
- Use `validation` blocks on variables for input constraints
- Outputs include `description` and `sensitive` where appropriate

## State Management
- Remote state backend (S3, GCS, Azure Blob, Terraform Cloud) — never local state in shared projects
- State locking enabled always
- Workspaces or directory-based separation for environments
- `terraform state list` before any state operations

## Safety
- ALWAYS run `terraform plan` before `terraform apply`
- Use `lifecycle { prevent_destroy = true }` on critical resources
- Use `moved` blocks for refactoring instead of destroy/recreate
- Tag all resources with: `project`, `environment`, `managed_by = "terraform"`
- Use `sensitive = true` on variables and outputs containing secrets

## Testing
- `terraform validate` — syntax and configuration validation
- `terraform plan` — behavioral validation (what will change?)
- `tflint` — linting for best practices and cloud-specific rules
- `checkov` / `tfsec` — security scanning
- `terratest` or `terraform test` (1.6+) for functional testing

## Anti-Patterns
- No `terraform apply -auto-approve` in any script or pipeline
- No `count` with complex conditionals — use `for_each` with maps
- No inline `provisioner` blocks — use configuration management tools instead
- No `data` sources that could be variables or outputs from other modules
- No wildcard provider version constraints (`~>` is fine, `>=` without upper bound is not)

## References

- https://developer.hashicorp.com/terraform/docs
- Search: "terraform best practices"
