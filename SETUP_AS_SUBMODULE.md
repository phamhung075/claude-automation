# Setting Up Claude Automation as Git Submodule

This guide shows you how to use `claude-automation` as a git submodule in your project.

---

## 🎯 Why Use as Submodule?

✅ **Reusable** - Use across multiple projects
✅ **Versioned** - Pin to specific version or track latest
✅ **Independent** - Has its own git history
✅ **Updatable** - Pull latest features easily
✅ **Shareable** - Others can use the same system

---

## 🚀 Initial Setup

### Step 1: Initialize Git Repository for claude-automation

```bash
# Navigate to claude-automation directory
cd /home/daihu/__projects__/4genthub/claude-automation

# Initialize git repository
git init

# Add all files
git add .

# Initial commit
git commit -m "Initial commit: Autonomous multi-agent system"

# Create remote repository (GitHub, GitLab, etc.)
# Then add remote
git remote add origin https://github.com/YOUR_USERNAME/claude-automation.git

# Push to remote
git branch -M main
git push -u origin main
```

### Step 2: Remove from Main Project Tracking

```bash
# Go back to main project
cd /home/daihu/__projects__/4genthub

# Remove from git tracking (if it was tracked)
git rm -r --cached claude-automation

# Add to .gitignore temporarily
echo "claude-automation/" >> .gitignore

# Commit the removal
git add .gitignore
git commit -m "Remove claude-automation from main repo (preparing for submodule)"
```

### Step 3: Add as Submodule

```bash
# Still in main project root
cd /home/daihu/__projects__/4genthub

# Remove local directory (we'll re-add as submodule)
rm -rf claude-automation

# Add as submodule
git submodule add https://github.com/YOUR_USERNAME/claude-automation.git claude-automation

# Commit the submodule addition
git add .gitmodules claude-automation
git commit -m "Add claude-automation as git submodule"

# Push to remote
git push
```

---

## 📥 Cloning Project with Submodules

### For New Contributors

When someone clones your main project:

```bash
# Option 1: Clone with submodules in one command
git clone --recursive https://github.com/YOUR_USERNAME/your-project.git

# Option 2: Clone then initialize submodules
git clone https://github.com/YOUR_USERNAME/your-project.git
cd your-project
git submodule update --init --recursive
```

### After Cloning

```bash
# Navigate to project
cd your-project

# Make claude-automation scripts executable
chmod +x claude-automation/scripts/*.sh
chmod +x claude-automation/tests/*.sh

# Test installation
./claude-automation/tests/test_autonomous_system.sh
```

---

## 🔄 Updating the Submodule

### Update to Latest Version

```bash
# From main project root
cd /home/daihu/__projects__/4genthub

# Enter submodule directory
cd claude-automation

# Pull latest changes
git pull origin main

# Go back to main project
cd ..

# Commit the updated submodule reference
git add claude-automation
git commit -m "Update claude-automation submodule to latest version"
git push
```

### Update to Specific Version

```bash
# Enter submodule
cd claude-automation

# Checkout specific tag or commit
git checkout v1.0.0

# Go back to main project
cd ..

# Commit the submodule reference
git add claude-automation
git commit -m "Pin claude-automation to v1.0.0"
git push
```

---

## 🔧 Making Changes to Submodule

### Workflow for Submodule Development

```bash
# 1. Enter submodule
cd claude-automation

# 2. Create feature branch
git checkout -b feature/new-agent-type

# 3. Make changes
vim scripts/autonomous_orchestrator.sh

# 4. Test changes
./tests/test_autonomous_system.sh

# 5. Commit changes
git add scripts/autonomous_orchestrator.sh
git commit -m "feat: add support for new agent type"

# 6. Push to submodule remote
git push origin feature/new-agent-type

# 7. Create Pull Request on GitHub/GitLab
# (Review and merge on the submodule repository)

# 8. After merge, update main project
cd ..
cd claude-automation
git checkout main
git pull origin main

# 9. Update main project reference
cd ..
git add claude-automation
git commit -m "Update claude-automation with new agent type support"
git push
```

---

## 📂 Project Structure with Submodule

```
your-project/                          (main repository)
├── .git/
├── .gitmodules                        (submodule configuration)
├── .claude/                           (Claude Code hooks)
│   ├── hooks/
│   └── commands/
├── claude-automation/                 (git submodule)
│   ├── .git/                          (separate git repository)
│   ├── README.md
│   ├── scripts/
│   ├── docs/
│   └── tests/
├── src/
├── tests/
└── README.md
```

### .gitmodules File

After adding submodule, `.gitmodules` will contain:

```ini
[submodule "claude-automation"]
    path = claude-automation
    url = https://github.com/YOUR_USERNAME/claude-automation.git
    branch = main
```

---

## 🔗 Integration with Main Project

### From Main Project Scripts

```bash
# In your main project script
#!/bin/bash

# Use claude-automation
./claude-automation/scripts/start_autonomous_workflow.sh \
    "$PROJECT_ID" \
    "$GIT_BRANCH" \
    "$GOAL"
```

### From Main Project CI/CD

```yaml
# .github/workflows/autonomous-workflow.yml
name: Autonomous Workflow

on:
  workflow_dispatch:
    inputs:
      goal:
        description: 'Workflow goal'
        required: true

jobs:
  run-autonomous:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: recursive  # Important!

      - name: Make scripts executable
        run: |
          chmod +x claude-automation/scripts/*.sh

      - name: Run autonomous workflow
        run: |
          ./claude-automation/scripts/start_autonomous_workflow.sh \
            "${{ secrets.PROJECT_ID }}" \
            "auto/${{ github.run_number }}" \
            "${{ github.event.inputs.goal }}"
```

---

## 🛠️ Troubleshooting

### Issue: Submodule directory is empty

```bash
# Initialize and update submodules
git submodule update --init --recursive
```

### Issue: Changes in submodule not showing in main project

```bash
# Go to main project root
cd your-project

# Update submodule reference
git add claude-automation
git commit -m "Update submodule reference"
```

### Issue: Submodule detached HEAD state

```bash
# Enter submodule
cd claude-automation

# Checkout main branch
git checkout main

# Pull latest
git pull origin main
```

### Issue: Merge conflicts in submodule

```bash
# Enter submodule
cd claude-automation

# Resolve conflicts normally
git status
git merge --continue

# Update main project
cd ..
git add claude-automation
git commit -m "Resolve submodule conflicts"
```

---

## 📊 Submodule Best Practices

### 1. Pin to Stable Versions

```bash
# Use tags for stable releases
cd claude-automation
git checkout v1.0.0
cd ..
git add claude-automation
git commit -m "Pin claude-automation to v1.0.0"
```

### 2. Document Submodule Usage

In main project README.md:

```markdown
## Setup

This project uses `claude-automation` as a git submodule.

```bash
# Clone with submodules
git clone --recursive <your-repo>

# Or initialize after clone
git submodule update --init --recursive
```
```

### 3. Regular Updates

```bash
# Weekly/monthly
cd claude-automation
git pull origin main
cd ..
git add claude-automation
git commit -m "Update claude-automation to latest"
```

### 4. Test After Updates

```bash
# After updating submodule
./claude-automation/tests/test_autonomous_system.sh
```

---

## 🔄 Alternative: Git Subtree

If submodules feel complex, consider git subtree:

```bash
# Add as subtree
git subtree add --prefix claude-automation \
    https://github.com/YOUR_USERNAME/claude-automation.git main --squash

# Update subtree
git subtree pull --prefix claude-automation \
    https://github.com/YOUR_USERNAME/claude-automation.git main --squash
```

**Pros**:
- Simpler for contributors (no submodule commands)
- Code is directly in main repo

**Cons**:
- Larger repo size
- Harder to contribute back to submodule

---

## 📚 Additional Resources

- [Git Submodules Documentation](https://git-scm.com/book/en/v2/Git-Tools-Submodules)
- [GitHub Submodules Guide](https://github.blog/2016-02-01-working-with-submodules/)
- [Git Subtrees vs Submodules](https://www.atlassian.com/git/tutorials/git-subtree)

---

## ✅ Checklist

Setup:
- [ ] Initialize claude-automation as git repo
- [ ] Push to remote repository
- [ ] Add as submodule to main project
- [ ] Test installation
- [ ] Document in main project README

Development:
- [ ] Create feature branch in submodule
- [ ] Make and test changes
- [ ] Push to submodule remote
- [ ] Update main project reference

Collaboration:
- [ ] Contributors know to use `--recursive`
- [ ] CI/CD configured for submodules
- [ ] Regular update schedule established

---

**You're now ready to use claude-automation as a reusable submodule!** 🎉
