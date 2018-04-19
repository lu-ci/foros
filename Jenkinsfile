pipeline {
  agent any
  stages {
    stage('Install Rust') {
      steps {
        bat 'rustup install nightly'
      }
    }
    stage('Set Default Rust') {
      steps {
        bat 'rustup default nightly'
      }
    }
    stage('Cargo Test') {
      steps {
        bat 'cargo test'
      }
    }
    stage('Cargo Build') {
      steps {
        bat 'cargo build'
      }
    }
    stage('Complete') {
      steps {
        echo 'All done.'
      }
    }
  }
}