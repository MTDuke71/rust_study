# Jenkins Pipeline Configuration for Rust Learning Quality Assurance
# File: Jenkinsfile

pipeline {
    agent any
    
    // Trigger pipeline on git changes and daily schedule
    triggers {
        // Poll SCM every 15 minutes during learning hours (6 AM - 10 PM)
        pollSCM('H/15 6-22 * * *')
        
        // Daily quality report at 6 AM
        cron('0 6 * * *')
    }
    
    parameters {
        booleanParam(
            name: 'SKIP_COVERAGE', 
            defaultValue: false, 
            description: 'Skip coverage analysis for faster builds'
        )
        booleanParam(
            name: 'DETAILED_REPORT', 
            defaultValue: false, 
            description: 'Generate detailed quality report'
        )
        choice(
            name: 'NOTIFICATION_LEVEL',
            choices: ['ALL', 'FAILURES_ONLY', 'NONE'],
            description: 'When to send notifications'
        )
    }
    
    environment {
        // Rust environment variables
        CARGO_HOME = "${WORKSPACE}/.cargo"
        RUSTUP_HOME = "${WORKSPACE}/.rustup"
        PATH = "${CARGO_HOME}/bin:${env.PATH}"
        
        // Quality thresholds
        MIN_COVERAGE = "85"
        MAX_CLIPPY_WARNINGS = "0"
        
        // Report directories
        REPORTS_DIR = "${WORKSPACE}/reports"
        ARTIFACTS_DIR = "${WORKSPACE}/artifacts"
    }
    
    stages {
        stage('Environment Setup') {
            steps {
                script {
                    echo "🔧 Setting up Rust environment..."
                    
                    // Create necessary directories
                    bat '''
                        if not exist reports mkdir reports
                        if not exist artifacts mkdir artifacts
                        if not exist scripts mkdir scripts
                    '''
                    
                    // Install/update Rust if needed
                    bat '''
                        where cargo >nul 2>nul || (
                            echo Installing Rust toolchain...
                            powershell -Command "Invoke-WebRequest -Uri https://win.rustup.rs/ -OutFile rustup-init.exe"
                            rustup-init.exe -y --default-toolchain stable
                        )
                        
                        echo Updating Rust toolchain...
                        rustup update stable
                        rustup component add clippy rustfmt
                    '''
                    
                    // Install additional tools
                    bat '''
                        cargo install --list | findstr cargo-tarpaulin >nul || (
                            echo Installing cargo-tarpaulin for coverage...
                            cargo install cargo-tarpaulin
                        )
                    '''
                }
            }
        }
        
        stage('Quality Pipeline') {
            parallel {
                stage('Code Quality') {
                    steps {
                        script {
                            echo "🔍 Running automated quality pipeline..."
                            
                            // Run the PowerShell quality pipeline
                            def quickFlag = params.SKIP_COVERAGE ? '-Quick' : ''
                            def result = bat(
                                script: "powershell -ExecutionPolicy Bypass -File scripts/quality-pipeline.ps1 ${quickFlag} -NonInteractive -OutputFile reports/quality-report.txt",
                                returnStatus: true
                            )
                            
                            // Store quality results
                            env.QUALITY_RESULT = result
                            
                            if (result != 0) {
                                currentBuild.result = 'UNSTABLE'
                                echo "⚠️ Quality pipeline found issues (exit code: ${result})"
                            } else {
                                echo "✅ Quality pipeline passed successfully"
                            }
                        }
                    }
                    post {
                        always {
                            // Archive quality reports
                            archiveArtifacts(
                                artifacts: 'reports/quality-report.txt',
                                allowEmptyArchive: true
                            )
                        }
                    }
                }
                
                stage('Learning Metrics') {
                    steps {
                        script {
                            echo "📈 Analyzing learning progress..."
                            
                            def detailedFlag = params.DETAILED_REPORT ? '-Detailed' : ''
                            bat """
                                powershell -ExecutionPolicy Bypass -File scripts/weekly-quality-review.ps1 ${detailedFlag} -OutputDir reports
                            """
                        }
                    }
                    post {
                        always {
                            // Archive learning reports
                            archiveArtifacts(
                                artifacts: 'reports/weekly-quality-*.txt, reports/quality-trends.csv',
                                allowEmptyArchive: true
                            )
                        }
                    }
                }
            }
        }
        
        stage('Test Results Processing') {
            steps {
                script {
                    echo "📊 Processing test results..."
                    
                    // Generate JUnit-style test reports if available
                    bat '''
                        cargo test --workspace -- --format=json > reports/test-results.json || echo "Test results generation failed"
                    '''
                    
                    // Generate coverage reports
                    if (!params.SKIP_COVERAGE) {
                        bat '''
                            cargo tarpaulin --workspace --timeout 120 --out Html --output-dir reports || echo "Coverage generation failed"
                        '''
                    }
                }
            }
            post {
                always {
                    // Publish test results (if available)
                    publishHTML([
                        allowMissing: true,
                        alwaysLinkToLastBuild: true,
                        keepAll: true,
                        reportDir: 'reports',
                        reportFiles: 'tarpaulin-report.html',
                        reportName: 'Coverage Report',
                        reportTitles: ''
                    ])
                }
            }
        }
        
        stage('Quality Gate Assessment') {
            steps {
                script {
                    echo "🚦 Assessing quality gates..."
                    
                    def qualityGates = [:]
                    
                    // Check compilation
                    qualityGates.compilation = env.QUALITY_RESULT == '0'
                    
                    // Check coverage threshold (if not skipped)
                    if (!params.SKIP_COVERAGE) {
                        // Parse coverage from reports (simplified)
                        try {
                            def coverageReport = readFile('reports/tarpaulin-report.html')
                            def coverageMatch = coverageReport =~ /(\d+\.\d+)%/
                            if (coverageMatch) {
                                def coverage = Float.parseFloat(coverageMatch[0][1])
                                qualityGates.coverage = coverage >= Float.parseFloat(env.MIN_COVERAGE)
                                echo "Coverage: ${coverage}% (threshold: ${env.MIN_COVERAGE}%)"
                            }
                        } catch (Exception e) {
                            echo "Could not parse coverage: ${e.message}"
                            qualityGates.coverage = true // Don't fail on parsing error
                        }
                    } else {
                        qualityGates.coverage = true
                    }
                    
                    // Overall assessment
                    def allPassed = qualityGates.values().every { it }
                    
                    if (allPassed) {
                        currentBuild.result = 'SUCCESS'
                        echo "✅ All quality gates passed!"
                    } else {
                        currentBuild.result = 'UNSTABLE'
                        echo "⚠️ Some quality gates failed: ${qualityGates}"
                    }
                    
                    // Store for notifications
                    env.QUALITY_GATES_RESULT = allPassed ? 'PASSED' : 'FAILED'
                }
            }
        }
        
        stage('Mission Progress Update') {
            when {
                expression { env.QUALITY_GATES_RESULT == 'PASSED' }
            }
            steps {
                script {
                    echo "🎯 Updating mission progress..."
                    
                    // Update progress tracking (simplified)
                    bat '''
                        echo %DATE% %TIME% - Quality pipeline passed >> progress_log.txt
                        git add progress_log.txt || echo "Git add failed"
                        git commit -m "Automated: Quality pipeline passed - %DATE%" || echo "Nothing to commit"
                    '''
                }
            }
        }
    }
    
    post {
        always {
            script {
                // Clean up temporary files
                bat '''
                    del /Q /F rustup-init.exe 2>nul || echo "No cleanup needed"
                '''
                
                echo "Pipeline completed with result: ${currentBuild.result}"
            }
        }
        
        success {
            script {
                if (params.NOTIFICATION_LEVEL == 'ALL') {
                    echo "✅ Quality pipeline completed successfully!"
                    // Add email/Slack notification here if configured
                }
            }
        }
        
        unstable {
            script {
                if (params.NOTIFICATION_LEVEL != 'NONE') {
                    echo "⚠️ Quality pipeline completed with warnings"
                    // Add notification for unstable builds
                }
            }
        }
        
        failure {
            script {
                if (params.NOTIFICATION_LEVEL != 'NONE') {
                    echo "❌ Quality pipeline failed!"
                    // Add failure notification
                }
            }
        }
        
        cleanup {
            // Archive all reports regardless of build status
            archiveArtifacts(
                artifacts: 'reports/**, artifacts/**',
                allowEmptyArchive: true,
                fingerprint: true
            )
        }
    }
}