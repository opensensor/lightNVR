# GitHub Actions Docker Publish Fix

## Issue

The GitHub Actions workflow was failing during the manifest creation step with the error:

```
Creating and pushing manifest: ghcr.io/opensensor/lightnvr:-a397c50
ERROR: invalid reference format
Error: Process completed with exit code 1.
```

## Root Cause

The workflow was using `type=sha,prefix={{branch}}-` to generate SHA-based tags. However, when the workflow is triggered by a tag push (e.g., `0.12.6`), there is no branch context, so `{{branch}}` resolves to an empty string, resulting in an invalid tag format: `-a397c50` (starting with a hyphen).

## The Problem Code

```yaml
tags: |
  type=semver,pattern={{version}}
  type=semver,pattern={{major}}.{{minor}}
  type=semver,pattern={{major}}
  type=raw,value=latest,enable={{is_default_branch}}
  type=sha,prefix={{branch}}-  # ❌ This causes the issue
```

When triggered by tag `0.12.6`:
- `{{branch}}` = empty string
- Result: `-a397c50` (invalid!)

## The Fix

Changed the SHA prefix from `{{branch}}-` to a fixed `sha-`:

```yaml
tags: |
  type=semver,pattern={{version}}
  type=semver,pattern={{major}}.{{minor}}
  type=semver,pattern={{major}}
  type=raw,value=latest,enable={{is_default_branch}}
  type=sha,prefix=sha-  # ✅ Fixed with static prefix
```

Now when triggered by tag `0.12.6`:
- Result: `sha-a397c50` (valid!)

## Tag Generation Examples

### For Tag Builds (e.g., `git tag 0.12.6`)

The workflow will create these tags:
- `ghcr.io/opensensor/lightnvr:0.12.6` (full version)
- `ghcr.io/opensensor/lightnvr:0.12` (major.minor)
- `ghcr.io/opensensor/lightnvr:0` (major)
- `ghcr.io/opensensor/lightnvr:latest` (if on default branch)
- `ghcr.io/opensensor/lightnvr:sha-a397c50` (git commit SHA)

### For Main Branch Builds (e.g., `git push origin main`)

The workflow will create these tags:
- `ghcr.io/opensensor/lightnvr:latest`
- `ghcr.io/opensensor/lightnvr:sha-a397c50` (git commit SHA)

### For Pull Request Builds

The workflow will create these tags:
- `ghcr.io/opensensor/lightnvr:pr-123` (PR number)
- `ghcr.io/opensensor/lightnvr:sha-a397c50` (git commit SHA)

## Benefits of SHA Tags

SHA-based tags are useful for:

1. **Reproducibility** - Can reference exact commit that built the image
2. **Debugging** - Easy to correlate image with source code
3. **Testing** - Can test specific commits before tagging
4. **Rollback** - Can quickly identify and rollback to specific commits

## Files Modified

- `.github/workflows/docker-publish.yml` (lines 49 and 124)

## Testing

To test this fix:

1. **Create a new tag:**
   ```bash
   git tag 0.12.7
   git push origin 0.12.7
   ```

2. **Verify the workflow:**
   - Check GitHub Actions tab
   - Ensure all builds complete successfully
   - Verify manifest creation succeeds

3. **Check generated tags:**
   ```bash
   docker pull ghcr.io/opensensor/lightnvr:0.12.7
   docker pull ghcr.io/opensensor/lightnvr:sha-a397c50
   docker pull ghcr.io/opensensor/lightnvr:latest
   ```

## Alternative Solutions Considered

### Option 1: Remove SHA tags for tag builds
```yaml
tags: |
  type=semver,pattern={{version}}
  type=semver,pattern={{major}}.{{minor}}
  type=semver,pattern={{major}}
  type=raw,value=latest,enable={{is_default_branch}}
  type=sha,enable={{is_default_branch}}  # Only for branch builds
```

**Pros:** Cleaner tag list for releases
**Cons:** Lose SHA traceability for tagged releases

### Option 2: Use conditional prefix
```yaml
tags: |
  type=semver,pattern={{version}}
  type=semver,pattern={{major}}.{{minor}}
  type=semver,pattern={{major}}
  type=raw,value=latest,enable={{is_default_branch}}
  type=sha,prefix=${{ github.ref_type == 'tag' && 'sha-' || format('{0}-', github.ref_name) }}
```

**Pros:** Dynamic prefix based on trigger type
**Cons:** More complex, harder to maintain

### Option 3: Fixed prefix (CHOSEN)
```yaml
tags: |
  type=semver,pattern={{version}}
  type=semver,pattern={{major}}.{{minor}}
  type=semver,pattern={{major}}
  type=raw,value=latest,enable={{is_default_branch}}
  type=sha,prefix=sha-
```

**Pros:** Simple, consistent, always works
**Cons:** Doesn't include branch name (but SHA is unique anyway)

## Conclusion

The fix is simple and effective: use a static `sha-` prefix instead of the dynamic `{{branch}}-` prefix. This ensures valid Docker tags are generated regardless of whether the workflow is triggered by a tag push, branch push, or pull request.

## Related Documentation

- [Docker Metadata Action](https://github.com/docker/metadata-action)
- [Docker Tag Naming Conventions](https://docs.docker.com/engine/reference/commandline/tag/)
- [GitHub Actions Context](https://docs.github.com/en/actions/learn-github-actions/contexts)

## Status

✅ **Fixed** - Workflow should now complete successfully for all trigger types.

