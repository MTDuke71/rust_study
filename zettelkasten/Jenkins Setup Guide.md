# Jenkins Server Setup Guide for Rust Learning Quality Assurance

*Created: 2025-10-10*  
*Navigation: [[zettel-index]] | [[Quality Assurance]] | [[Time Management Optimization]]*
*Related Concepts: [[Automated Quality Pipeline]] | [[Progress Tracking]] | [[Continuous Integration]]*

---

## 🎯 **Jenkins Setup Overview**

This guide provides step-by-step instructions for setting up Jenkins on Windows to automate your Rust learning quality assurance pipeline.

### **Benefits of Jenkins Integration**
- **Automated Quality Checks**: Run quality pipeline on every git commit
- **Scheduled Learning Reports**: Daily/weekly learning progress analysis  
- **Quality Gate Enforcement**: Prevent poor-quality code from being committed
- **Progress Tracking**: Automated documentation of learning advancement
- **Continuous Feedback**: Immediate notification of quality issues

---

## 🛠️ **Jenkins Installation (Windows)**

### **Step 1: Prerequisites**
```powershell
# Check Java installation (Jenkins requires Java 8 or 11)
java -version

# If Java not installed, download and install:
# https://adoptopenjdk.net/ (recommended)
# or
# https://www.oracle.com/java/technologies/javase-downloads.html
```

### **Step 2: Jenkins Installation**
```powershell
# Download Jenkins Windows installer
# https://www.jenkins.io/download/

# OR install via Chocolatey (if available)
choco install jenkins

# OR install as Windows service (recommended)
# 1. Download jenkins.war from jenkins.io
# 2. Install as service using PowerShell:

# Create Jenkins directory
New-Item -ItemType Directory -Path "C:\Jenkins" -Force

# Download Jenkins WAR (replace URL with latest)
Invoke-WebRequest -Uri "https://get.jenkins.io/war-stable/latest/jenkins.war" -OutFile "C:\Jenkins\jenkins.war"

# Install as Windows service
java -jar "C:\Jenkins\jenkins.war" --httpPort=8080 --install
```

### **Step 3: Initial Jenkins Configuration**
```bash
# Start Jenkins service
net start jenkins

# Open browser to: http://localhost:8080

# Get initial admin password
Get-Content "C:\Users\[USERNAME]\.jenkins\secrets\initialAdminPassword"
# or
Get-Content "C:\Jenkins\secrets\initialAdminPassword"
```

---

## 🔧 **Jenkins Configuration for Rust Learning**

### **Step 1: Install Required Plugins**
Navigate to **Manage Jenkins** → **Manage Plugins** → **Available**

**Essential Plugins**:
```
✅ Git Plugin - Git repository integration
✅ Pipeline Plugin - Pipeline job support  
✅ Blue Ocean - Modern pipeline interface
✅ HTML Publisher - HTML report publishing
✅ Build Timeout - Prevent stuck builds
✅ Timestamper - Add timestamps to logs
✅ Workspace Cleanup - Clean workspace after builds
✅ Email Extension - Email notifications
```

**Optional but Useful**:
```
🔧 Slack Notification - Slack integration
🔧 GitHub Integration - GitHub webhook support
🔧 Parameterized Trigger - Build parameter support
🔧 Build Monitor View - Dashboard view
🔧 AnsiColor - Colorized console output
```

### **Step 2: Global Tool Configuration**
**Manage Jenkins** → **Global Tool Configuration**

**Git Configuration**:
```
Name: Default
Path to Git executable: C:\Program Files\Git\bin\git.exe
```

**PowerShell Configuration** (if not default):
```
Name: PowerShell
Command: powershell.exe
```

### **Step 3: System Configuration**  
**Manage Jenkins** → **Configure System**

**Environment Variables**:
```powershell
# Add these environment variables:
CARGO_HOME = C:\Users\[USERNAME]\.cargo
RUSTUP_HOME = C:\Users\[USERNAME]\.rustup  
PATH = $PATH;%CARGO_HOME%\bin
```

**Email Notification** (optional):
```
SMTP Server: smtp.gmail.com (or your provider)
Port: 587 (TLS) or 465 (SSL)
Username: your-email@gmail.com
Password: your-app-password
```

---

## 📋 **Pipeline Job Setup**

### **Step 1: Create New Pipeline Job**
1. **New Item** → **Pipeline** → Enter name: `rust-learning-qa`
2. Configure pipeline options:

**General Settings**:
```
✅ Build Triggers:
   - Poll SCM: H/15 6-22 * * * (every 15 min during learning hours)
   - Build periodically: 0 6 * * * (daily at 6 AM)

✅ Pipeline:
   - Definition: Pipeline script from SCM
   - SCM: Git  
   - Repository URL: [your-rust-study-repo-url]
   - Branch: */main
   - Script Path: Jenkinsfile
```

**Advanced Options**:
```
✅ Pipeline Speed/Durability Override: Performance-optimized
✅ Build Discard Policy: 
   - Days to keep builds: 30
   - Max # of builds to keep: 50
```

### **Step 2: Pipeline Configuration Parameters**
Add these build parameters for flexibility:

```groovy
// In Jenkins job configuration → This project is parameterized
booleanParam(name: 'SKIP_COVERAGE', defaultValue: false, description: 'Skip coverage analysis')
booleanParam(name: 'DETAILED_REPORT', defaultValue: false, description: 'Generate detailed reports')
choice(name: 'NOTIFICATION_LEVEL', choices: ['ALL', 'FAILURES_ONLY', 'NONE'], description: 'Notification preferences')
```

---

## 🚀 **Testing Your Jenkins Setup**

### **Step 1: Manual Pipeline Test**
```powershell
# Ensure your quality scripts are in place
Test-Path "scripts/quality-pipeline.ps1"
Test-Path "scripts/weekly-quality-review.ps1"
Test-Path "Jenkinsfile"

# Test scripts locally first
powershell -ExecutionPolicy Bypass -File scripts/quality-pipeline.ps1 -Quick
```

### **Step 2: Jenkins Pipeline Test**
1. **Build with Parameters** in Jenkins UI
2. Check **Console Output** for errors
3. Verify **Workspace** has correct files
4. Check **Build Artifacts** are generated

### **Step 3: Webhook Integration** (Optional)
If using GitHub/GitLab:

**GitHub Webhook**:
```
Payload URL: http://[jenkins-server]:8080/github-webhook/
Content type: application/json
Secret: [optional webhook secret]
Events: Just the push event
```

---

## 📊 **Quality Dashboard Setup**

### **Step 1: Create Dashboard View**
1. **New View** → **Dashboard View** → Name: `Rust Learning Dashboard`
2. Add widgets:

**Build Statistics**:
```
✅ Test Result Trend Chart
✅ Build History Widget  
✅ Build Queue Widget
✅ Build Statistics Widget
```

**Custom Widgets**:
```ruby
// Add HTML widget with quality metrics
<h3>📈 Learning Progress</h3>
<iframe src="./lastSuccessfulBuild/artifact/reports/weekly-quality-latest.txt" width="100%" height="300"></iframe>

<h3>📊 Coverage Trend</h3>
<iframe src="./lastSuccessfulBuild/artifact/reports/tarpaulin-report.html" width="100%" height="400"></iframe>
```

### **Step 2: Quality Trends Visualization**
Create additional pipeline for trend analysis:

```groovy
// File: trend-analysis-pipeline.groovy
pipeline {
    agent any
    triggers {
        cron('0 7 * * 1') // Weekly on Monday at 7 AM
    }
    stages {
        stage('Generate Trends') {
            steps {
                powershell '''
                    # Generate quality trend charts
                    python scripts/generate-quality-charts.py
                '''
            }
        }
    }
    post {
        always {
            publishHTML([
                reportDir: 'reports/trends',
                reportFiles: 'quality-trends.html',
                reportName: 'Quality Trends'
            ])
        }
    }
}
```

---

## 🔔 **Notification Setup**

### **Email Notifications**
Configure in **Manage Jenkins** → **Configure System** → **Extended E-mail Notification**:

```yaml
Default Recipients: your-email@domain.com
Default Subject: 🦀 Rust Learning QA: $BUILD_STATUS - $PROJECT_NAME #$BUILD_NUMBER
Default Content: |
  Quality Pipeline Results:
  
  📊 Build: $BUILD_STATUS
  📈 Duration: $BUILD_DURATION  
  🎯 Quality Gates: ${BUILD_LOG_REGEX, regex="OVERALL STATUS: (.*)", linesBefore=0, linesAfter=0}
  
  📋 Details: $BUILD_URL
  📊 Reports: $BUILD_URL/artifact/reports/
```

### **Slack Integration** (Optional)
Install Slack plugin and configure:

```yaml
Workspace: your-workspace.slack.com
Channel: #rust-learning
Bot Token: xoxb-your-bot-token

Custom Message:
🦀 Rust Learning QA Pipeline: $BUILD_STATUS
📊 Quality Score: [parse from reports]
🎯 Mission Progress: [current mission status]  
📈 Details: $BUILD_URL
```

---

## 🛡️ **Security & Best Practices**

### **Security Configuration**
```powershell
# Configure Jenkins security
# Manage Jenkins → Configure Global Security

✅ Enable security: Checked
✅ Security Realm: Jenkins' own user database
✅ Authorization: Logged-in users can do anything
✅ CSRF Protection: Checked
✅ Agent → Master Access Control: Checked
```

### **Backup Strategy**
```powershell
# Automated Jenkins backup script
# File: backup-jenkins.ps1

$BackupDir = "C:\Jenkins\Backups\$(Get-Date -Format 'yyyy-MM-dd')"
$JenkinsHome = "C:\Users\[USERNAME]\.jenkins"

New-Item -ItemType Directory -Path $BackupDir -Force

# Backup essential Jenkins data
Copy-Item "$JenkinsHome\config.xml" $BackupDir
Copy-Item "$JenkinsHome\jobs" $BackupDir -Recurse
Copy-Item "$JenkinsHome\plugins" $BackupDir -Recurse
Copy-Item "$JenkinsHome\users" $BackupDir -Recurse

# Compress backup
Compress-Archive -Path $BackupDir -DestinationPath "$BackupDir.zip"
Remove-Item $BackupDir -Recurse -Force

Write-Host "Jenkins backup created: $BackupDir.zip"
```

### **Performance Optimization**
```groovy
// Add to Jenkinsfile for better performance
pipeline {
    options {
        buildDiscarder(logRotator(numToKeepStr: '50'))
        timeout(time: 30, unit: 'MINUTES')
        skipStagesAfterUnstable()
        parallelsAlwaysFailFast()
    }
    
    // Use more efficient workspace cleanup
    post {
        cleanup {
            cleanWs deleteDirs: true, notFailBuild: true
        }
    }
}
```

---

## 🔧 **Troubleshooting Common Issues**

### **PowerShell Execution Policy**
```powershell
# If PowerShell scripts fail to run
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Or bypass in Jenkins pipeline
powershell -ExecutionPolicy Bypass -File script.ps1
```

### **Cargo/Rust Not Found**
```groovy
// Add to Jenkinsfile environment section
environment {
    PATH = "${env.PATH};C:\\Users\\[USERNAME]\\.cargo\\bin"
    CARGO_HOME = "C:\\Users\\[USERNAME]\\.cargo"
    RUSTUP_HOME = "C:\\Users\\[USERNAME]\\.rustup"
}
```

### **Git Authentication Issues**
```powershell
# Configure git credentials for Jenkins service account
git config --global user.name "Jenkins CI"
git config --global user.email "jenkins@yourdomain.com"

# For private repos, add SSH key or use credential manager
```

### **Build Timeouts**
```groovy
// Add timeout to pipeline stages
stage('Quality Check') {
    options {
        timeout(time: 10, unit: 'MINUTES')
    }
    steps {
        // your steps
    }
}
```

---

## 📈 **Advanced Integration Features**

### **Quality Gate Integration**
```groovy
// Advanced quality gates with SonarQube-style gates
stage('Quality Gates') {
    steps {
        script {
            def qualityGate = sh(script: 'powershell scripts/quality-gate-check.ps1', returnStatus: true)
            if (qualityGate != 0) {
                error("Quality gate failed")
            }
        }
    }
}
```

### **Learning Progress Integration**
```groovy
// Integration with spaced repetition system
stage('Update Learning Metrics') {
    steps {
        powershell '''
            # Update spaced repetition cards based on build success
            if ($env:BUILD_STATUS -eq "SUCCESS") {
                # Mark current concepts as successful
                ./scripts/update-learning-success.ps1
            }
        '''
    }
}
```

### **Mission Completion Automation**
```groovy
// Automatic mission completion detection
stage('Mission Progress Check') {
    when {
        expression { 
            return sh(script: 'powershell scripts/check-mission-complete.ps1', returnStatus: true) == 0 
        }
    }
    steps {
        echo "🎉 Mission completed! Updating progress tracking..."
        powershell 'scripts/mission-completion-handler.ps1'
    }
}
```

---

## 🏷️ **Tags & Cross-References**

*Tags: #jenkins-setup #continuous-integration #automated-quality #windows-jenkins #rust-ci-cd #learning-automation #quality-pipeline #devops-learning*

*Jenkins Configuration:*
- [[Jenkins Installation Guide]] - Step-by-step Windows installation
- [[Pipeline Configuration]] - Complete pipeline setup and parameters
- [[Quality Dashboard Setup]] - Visual monitoring and reporting
- [[Notification Configuration]] - Email and Slack integration setup

*Automation Integration:*
- [[Quality Assurance]] - Main quality assurance framework and standards  
- [[Automated Quality Pipeline]] - PowerShell scripts for quality checking
- [[Progress Tracking]] - Learning advancement measurement and reporting
- [[Time Management Optimization]] - Maximizing efficiency through automation

*Troubleshooting & Maintenance:*
- [[Jenkins Security Best Practices]] - Security configuration and user management
- [[Performance Optimization]] - Jenkins and pipeline performance tuning
- [[Backup and Recovery]] - Data protection and disaster recovery procedures
- [[Common Issues Resolution]] - Solutions for frequent Jenkins problems

---

*Created: October 10, 2025*  
*Navigation: [[zettel-index]] | [[Quality Assurance]] | [[Automated Quality Pipeline]] | [[Progress Tracking]]*